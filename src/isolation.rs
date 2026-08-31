use std::path::Path;

use anyhow::{Result, ensure};

use crate::domain::{IsolationMode, IsolationReport};
use crate::harness::{InvocationPlan, find_trusted_system_executable};

const BWRAP_NAMESPACES: &[&str] = &["user", "pid", "ipc", "uts", "cgroup"];

/// Apply the sovereign process boundary after adapter planning and before any
/// subprocess can be spawned.
pub fn apply_isolation(
    mut plan: InvocationPlan,
    mode: IsolationMode,
    unsafe_acknowledged: bool,
) -> Result<InvocationPlan> {
    if plan.builtin_fixture {
        plan.isolation = IsolationReport::builtin();
        return Ok(plan);
    }

    if mode == IsolationMode::Off {
        ensure!(
            unsafe_acknowledged,
            "--isolation off requires --acknowledge-unsafe-subprocess"
        );
        plan.isolation = IsolationReport {
            schema_version: "asi.isolation-report.v0.1".to_owned(),
            requested_mode: mode,
            enforced: false,
            enforcer: "none".to_owned(),
            filesystem: "ambient-host-permissions".to_owned(),
            namespaces: Vec::new(),
            ephemeral_writes: Vec::new(),
            network: "ambient-host-network".to_owned(),
            limitations: vec![
                "operator explicitly accepted an unsafe subprocess".to_owned(),
                "adapter flags are not an independent security boundary".to_owned(),
            ],
        };
        plan.containment_notes
            .push("WARNING: OS isolation explicitly disabled by operator".to_owned());
        return Ok(plan);
    }

    #[cfg(not(target_os = "linux"))]
    anyhow::bail!("required isolation is currently implemented only on Linux");

    #[cfg(target_os = "linux")]
    {
        let bwrap = find_trusted_system_executable("bwrap").ok_or_else(|| {
            anyhow::anyhow!(
                "required isolation unavailable: trusted root-owned bwrap not found on PATH"
            )
        })?;
        ensure!(
            plan.program.is_absolute(),
            "isolated harness executable must be an absolute path"
        );

        let private_arguments = bubblewrap_arguments(
            &plan.program,
            &plan.arguments,
            &plan.working_directory,
            &plan.ephemeral_state_paths,
        )?;
        let public_arguments = bubblewrap_arguments(
            &plan.program,
            &plan.redacted_arguments,
            &plan.working_directory,
            &plan.ephemeral_state_paths,
        )?;
        plan.program = bwrap;
        plan.arguments = private_arguments;
        plan.redacted_arguments = public_arguments;
        plan.isolation = IsolationReport {
            schema_version: "asi.isolation-report.v0.1".to_owned(),
            requested_mode: mode,
            enforced: true,
            enforcer: "bubblewrap".to_owned(),
            filesystem: "host-root-read-only; ephemeral-writable-/tmp".to_owned(),
            namespaces: BWRAP_NAMESPACES
                .iter()
                .map(|namespace| (*namespace).to_owned())
                .collect(),
            ephemeral_writes: plan.ephemeral_state_paths.clone(),
            network: "shared-host-network; provider-egress-unmediated".to_owned(),
            limitations: vec![
                "host-readable files remain visible to the subprocess".to_owned(),
                "provider egress is not destination-filtered or inspected".to_owned(),
                "file-based credentials readable by the harness are not yet brokered".to_owned(),
                "the child environment is cleared to a small baseline, but file-based credentials remain readable"
                    .to_owned(),
                "child process trees rely on Bubblewrap and parent-death termination".to_owned(),
            ],
        };
        plan.containment_notes.push(
            "Bubblewrap enforces a read-only host root, ephemeral /tmp, namespace isolation, and dropped capabilities"
                .to_owned(),
        );
        Ok(plan)
    }
}

fn bubblewrap_arguments(
    program: &Path,
    arguments: &[String],
    cwd: &Path,
    ephemeral_state_paths: &[std::path::PathBuf],
) -> Result<Vec<String>> {
    let mut wrapped = [
        "--die-with-parent",
        "--new-session",
        "--unshare-all",
        "--unshare-user",
        "--share-net",
        "--disable-userns",
        "--cap-drop",
        "ALL",
        "--ro-bind",
        "/",
        "/",
        "--proc",
        "/proc",
        "--dev",
        "/dev",
        "--tmpfs",
        "/tmp",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    for path in ephemeral_state_paths {
        match path.symlink_metadata() {
            Ok(metadata) => {
                ensure!(
                    metadata.file_type().is_dir() && !metadata.file_type().is_symlink(),
                    "harness state path must be a real directory: {}",
                    path.display()
                );
                wrapped.extend([
                    "--overlay-src".to_owned(),
                    path.display().to_string(),
                    "--tmp-overlay".to_owned(),
                    path.display().to_string(),
                ]);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                wrapped.extend(["--dir".to_owned(), path.display().to_string()]);
            }
            Err(error) => {
                return Err(error.into());
            }
        }
    }
    wrapped.push("--chdir".to_owned());
    wrapped.push(cwd.display().to_string());
    wrapped.extend([
        "--setenv".to_owned(),
        "TMPDIR".to_owned(),
        "/tmp".to_owned(),
        "--".to_owned(),
        program.display().to_string(),
    ]);
    wrapped.extend(arguments.iter().cloned());
    Ok(wrapped)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::domain::EffectClass;

    fn fixture_plan(prompt: &str, redacted: &str) -> InvocationPlan {
        InvocationPlan {
            harness_id: "test".to_owned(),
            program: PathBuf::from("/bin/echo"),
            worker_program: PathBuf::from("/bin/echo"),
            harness_version: Some("test-v1".to_owned()),
            executable_sha256: None,
            arguments: vec![prompt.to_owned()],
            redacted_arguments: vec![redacted.to_owned()],
            working_directory: std::env::current_dir().expect("cwd should exist"),
            effective_effect: EffectClass::None,
            timeout_seconds: 30,
            containment_notes: Vec::new(),
            builtin_fixture: false,
            ephemeral_state_paths: Vec::new(),
            isolation: IsolationReport::pending(),
        }
    }

    #[test]
    fn required_profile_wraps_private_and_public_arguments_separately() {
        if find_trusted_system_executable("bwrap").is_none() {
            return;
        }
        let plan = apply_isolation(
            fixture_plan("private prompt", "<task:sha256=abc>"),
            IsolationMode::Required,
            false,
        )
        .expect("required isolation should apply");
        assert!(plan.isolation.enforced);
        assert_eq!(plan.isolation.enforcer, "bubblewrap");
        assert!(plan.arguments.iter().any(|value| value == "private prompt"));
        assert!(
            !plan
                .redacted_arguments
                .iter()
                .any(|value| value == "private prompt")
        );
    }

    #[test]
    fn unsafe_mode_requires_a_second_explicit_signal() {
        let error = apply_isolation(fixture_plan("task", "redacted"), IsolationMode::Off, false)
            .expect_err("unsafe mode without acknowledgement should fail");
        assert!(error.to_string().contains("acknowledge-unsafe-subprocess"));
    }

    #[test]
    fn missing_harness_state_becomes_an_ephemeral_directory() {
        if find_trusted_system_executable("bwrap").is_none() {
            return;
        }
        let directory = tempfile::tempdir().expect("tempdir should work");
        let missing = directory.path().join("missing-state");
        let mut fixture = fixture_plan("task", "redacted");
        fixture.ephemeral_state_paths.push(missing.clone());
        let plan = apply_isolation(fixture, IsolationMode::Required, false)
            .expect("required isolation should apply");
        assert!(
            plan.arguments
                .windows(2)
                .any(|pair| { pair[0] == "--dir" && pair[1] == missing.display().to_string() })
        );
    }
}
