use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;

use crate::bloodline::BloodlineLedger;
use crate::domain::{IsolationMode, IsolationReport, TaskEnvelope, sha256_hex};
use crate::genome::GenomeVerification;
use crate::harness::{AdapterKind, InvocationPlan, PublicInvocationPlan};
use crate::isolation::apply_isolation;
use crate::lineage::sha256_file;
use crate::policy::{PolicyDecision, PolicyEngine};
use crate::registry::HarnessRegistry;

#[derive(Clone, Debug)]
pub struct PreparedRun {
    pub task: TaskEnvelope,
    pub policy: PolicyDecision,
    pub adapter: AdapterKind,
    pub invocation: InvocationPlan,
}

impl PreparedRun {
    #[must_use]
    pub fn public_plan(&self) -> PublicInvocationPlan {
        self.invocation.public(&self.task.prompt_sha256)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RunOutcome {
    pub schema_version: String,
    pub run_id: String,
    pub harness_id: String,
    pub status: String,
    pub output: String,
    pub output_sha256: String,
    pub output_bytes: usize,
    pub truncated: bool,
    pub duration_ms: u128,
    pub ledger_path: String,
    pub plan_sha256: String,
    pub genome_sha256: String,
    pub genome_key_id: String,
    pub isolation: IsolationReport,
}

#[derive(Debug)]
struct CapturedOutput {
    value: String,
    truncated: bool,
}

#[derive(Debug)]
struct CapturedStream {
    bytes: Vec<u8>,
    truncated: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SovereignRuntime {
    policy: PolicyEngine,
    registry: HarnessRegistry,
}

impl SovereignRuntime {
    pub fn prepare(&self, task: TaskEnvelope, requested_harness: &str) -> Result<PreparedRun> {
        self.prepare_with_isolation(task, requested_harness, IsolationMode::Required, false)
    }

    pub fn prepare_with_isolation(
        &self,
        task: TaskEnvelope,
        requested_harness: &str,
        isolation_mode: IsolationMode,
        unsafe_acknowledged: bool,
    ) -> Result<PreparedRun> {
        let policy = self.policy.evaluate(task.requested_effect);
        if !policy.allowed {
            bail!(
                "policy denied [{}]: {}",
                policy.reason_code,
                policy.explanation
            );
        }
        let adapter = self
            .registry
            .resolve(requested_harness, task.requested_effect)?;
        let detected = adapter.detect();
        if adapter.requires_version_compatibility() {
            let version = detected.version.as_deref().context(format!(
                "harness {} version is unavailable; refusing an unverified adapter contract",
                adapter.id()
            ))?;
            ensure!(
                adapter.supports_detected_version(version),
                "harness {} version {version:?} is outside the reviewed compatibility prefixes: {}",
                adapter.id(),
                adapter.supported_version_prefixes().join(", ")
            );
        }
        let executable_sha256 = if adapter.requires_version_compatibility() {
            detected
                .executable_path
                .as_deref()
                .map(sha256_file)
                .transpose()
                .with_context(|| format!("cannot fingerprint selected harness {}", adapter.id()))?
        } else {
            None
        };
        let mut unisolated = adapter.plan(&task, detected.executable_path.as_deref())?;
        unisolated.harness_version = detected.version;
        unisolated.executable_sha256 = executable_sha256;
        let invocation = apply_isolation(unisolated, isolation_mode, unsafe_acknowledged)?;
        Ok(PreparedRun {
            task,
            policy,
            adapter,
            invocation,
        })
    }

    pub async fn execute(
        &self,
        prepared: PreparedRun,
        ledger_path: impl AsRef<Path>,
        approved_plan_sha256: &str,
        verified_genome: &GenomeVerification,
    ) -> Result<RunOutcome> {
        let ledger_path = ledger_path.as_ref();
        let run_id = prepared.task.run_id.to_string();
        let public_plan = prepared.public_plan();
        ensure!(
            public_plan.plan_sha256 == approved_plan_sha256,
            "approved plan digest does not match the current invocation plan"
        );
        ensure!(
            verified_genome.valid && verified_genome.current_state_matches,
            "execution requires a valid signed genome matching current harness state"
        );
        if let Some(expected) = prepared.invocation.executable_sha256.as_deref() {
            let current = sha256_file(&prepared.invocation.worker_program)
                .context("cannot re-fingerprint the approved worker executable")?;
            ensure!(
                current == expected,
                "worker executable changed after invocation planning"
            );
        }
        let isolation = prepared.invocation.isolation.clone();
        append_event(
            ledger_path,
            &run_id,
            "run.created",
            json!({
                "task_id": prepared.task.task_id,
                "task_sha256": prepared.task.prompt_sha256,
                "requested_effect": prepared.task.requested_effect,
                "harness_id": prepared.adapter.id(),
                "plan_sha256": public_plan.plan_sha256,
                "genome_sha256": verified_genome.genome_sha256,
                "genome_key_id": verified_genome.key_id,
                "genome_generated_at": verified_genome.generated_at,
            }),
        )?;
        append_event(
            ledger_path,
            &run_id,
            "policy.allowed",
            serde_json::to_value(&prepared.policy).context("cannot encode policy decision")?,
        )?;
        append_event(
            ledger_path,
            &run_id,
            "harness.started",
            serde_json::to_value(&public_plan).context("cannot encode invocation plan")?,
        )?;

        let started = Instant::now();
        let execution = if prepared.invocation.builtin_fixture {
            Ok(CapturedOutput {
                value: format!("ASI_CONSTRUCT_OK:{}", prepared.task.prompt_sha256),
                truncated: false,
            })
        } else {
            execute_subprocess(&prepared.invocation, prepared.task.budget.max_output_bytes).await
        };
        let duration_ms = started.elapsed().as_millis();

        let (output, truncated) = match execution {
            Ok(captured) => {
                let (output, utf8_truncated) =
                    truncate_utf8(captured.value, prepared.task.budget.max_output_bytes);
                (output, captured.truncated || utf8_truncated)
            }
            Err(error) => {
                let error_text = error.to_string();
                append_event(
                    ledger_path,
                    &run_id,
                    "harness.failed",
                    json!({
                        "duration_ms": duration_ms,
                        "error_kind": "harness-execution-error",
                        "error_sha256": sha256_hex(error_text.as_bytes()),
                    }),
                )?;
                return Err(error);
            }
        };
        let output_sha256 = sha256_hex(output.as_bytes());
        append_event(
            ledger_path,
            &run_id,
            "harness.completed",
            json!({
                "duration_ms": duration_ms,
                "output_sha256": output_sha256,
                "output_bytes": output.len(),
                "truncated": truncated,
            }),
        )?;

        Ok(RunOutcome {
            schema_version: "asi.run-outcome.v0.1".to_owned(),
            run_id,
            harness_id: prepared.adapter.id().to_owned(),
            status: "completed".to_owned(),
            output_sha256,
            output_bytes: output.len(),
            output,
            truncated,
            duration_ms,
            ledger_path: ledger_path.display().to_string(),
            plan_sha256: public_plan.plan_sha256,
            genome_sha256: verified_genome.genome_sha256.clone(),
            genome_key_id: verified_genome.key_id.clone(),
            isolation,
        })
    }
}

fn append_event(
    ledger_path: &Path,
    run_id: &str,
    kind: &str,
    payload: serde_json::Value,
) -> Result<()> {
    let mut ledger = BloodlineLedger::open(ledger_path)?;
    ledger.append(run_id, kind, payload)?;
    Ok(())
}

async fn execute_subprocess(
    plan: &InvocationPlan,
    max_output_bytes: usize,
) -> Result<CapturedOutput> {
    let mut command = Command::new(&plan.program);
    command
        .args(&plan.arguments)
        .current_dir(&plan.working_directory)
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    if let Some(home) = std::env::var_os("HOME") {
        command.env("HOME", home);
    }
    let mut child = command
        .spawn()
        .with_context(|| format!("cannot launch harness {}", plan.harness_id))?;
    let stdout = child
        .stdout
        .take()
        .context("harness stdout pipe was unavailable")?;
    let stderr = child
        .stderr
        .take()
        .context("harness stderr pipe was unavailable")?;
    let stdout_task = tokio::spawn(read_bounded(stdout, max_output_bytes));
    let stderr_task = tokio::spawn(read_bounded(stderr, max_output_bytes));

    let status =
        match tokio::time::timeout(Duration::from_secs(plan.timeout_seconds), child.wait()).await {
            Ok(status) => {
                status.with_context(|| format!("cannot wait for harness {}", plan.harness_id))?
            }
            Err(_) => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                let _ = stdout_task.await;
                let _ = stderr_task.await;
                bail!(
                    "harness {} exceeded timeout of {} seconds",
                    plan.harness_id,
                    plan.timeout_seconds
                );
            }
        };
    let stdout = stdout_task
        .await
        .context("harness stdout collector failed")?
        .context("cannot read harness stdout")?;
    let stderr = stderr_task
        .await
        .context("harness stderr collector failed")?
        .context("cannot read harness stderr")?;

    if !status.success() {
        let stderr_sha256 = sha256_hex(&stderr.bytes);
        bail!(
            "harness {} exited with {}; stderr_sha256={stderr_sha256}",
            plan.harness_id,
            status
        );
    }

    let selected = if stdout.bytes.is_empty() {
        stderr
    } else {
        stdout
    };
    Ok(CapturedOutput {
        value: String::from_utf8_lossy(&selected.bytes).into_owned(),
        truncated: selected.truncated,
    })
}

async fn read_bounded<R>(mut reader: R, max_bytes: usize) -> std::io::Result<CapturedStream>
where
    R: AsyncRead + Unpin,
{
    let mut retained = Vec::with_capacity(max_bytes.min(65_536));
    let mut buffer = [0_u8; 8_192];
    let mut truncated = false;
    loop {
        let count = reader.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        let remaining = max_bytes.saturating_sub(retained.len());
        let keep = remaining.min(count);
        retained.extend_from_slice(&buffer[..keep]);
        truncated |= keep < count;
    }
    Ok(CapturedStream {
        bytes: retained,
        truncated,
    })
}

fn truncate_utf8(mut value: String, max_bytes: usize) -> (String, bool) {
    if value.len() <= max_bytes {
        return (value, false);
    }
    let mut boundary = max_bytes;
    while boundary > 0 && !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    value.truncate(boundary);
    (value, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::EffectClass;
    use crate::genome::{sign_genome, verify_genome};
    use crate::lineage::generate_keypair;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn fixture_executes_through_policy_and_bloodline() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let task = TaskEnvelope::new("hello construct", EffectClass::None, ".", 30)
            .expect("task should be valid");
        let runtime = SovereignRuntime::default();
        let prepared = runtime
            .prepare(task, "construct-fixture")
            .expect("fixture should prepare");
        let ledger = directory.path().join("bloodline.jsonl");
        let private = directory.path().join("private.json");
        let public = directory.path().join("public.json");
        let genome_path = directory.path().join("genome.json");
        generate_keypair(&private, &public).expect("keys should generate");
        sign_genome(&private, &genome_path).expect("genome should sign");
        let genome = verify_genome(&genome_path, &public).expect("genome should verify");
        let outcome = runtime
            .execute(
                prepared.clone(),
                &ledger,
                &prepared.public_plan().plan_sha256,
                &genome,
            )
            .await
            .expect("fixture should execute");
        assert_eq!(outcome.status, "completed");
        assert!(outcome.output.starts_with("ASI_CONSTRUCT_OK:"));
        let verification = BloodlineLedger::verify(&ledger).expect("ledger should verify");
        assert_eq!(verification.events, 4);
    }

    #[tokio::test]
    async fn output_collection_drains_but_retains_only_the_budget() {
        let (mut writer, reader) = tokio::io::duplex(64);
        let writer_task = tokio::spawn(async move {
            writer
                .write_all(&vec![b'x'; 4_096])
                .await
                .expect("fixture output should write");
        });
        let captured = read_bounded(reader, 128)
            .await
            .expect("capture should succeed");
        writer_task.await.expect("writer should finish");
        assert_eq!(captured.bytes.len(), 128);
        assert!(captured.truncated);
    }
}
