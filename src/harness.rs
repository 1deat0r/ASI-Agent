#[cfg(target_os = "linux")]
use std::io::Read;
use std::path::{Path, PathBuf};
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::domain::{EffectClass, IsolationReport, TaskEnvelope, sha256_hex};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HarnessDescriptor {
    pub schema_version: String,
    pub id: String,
    pub display_name: String,
    pub executable_name: Option<String>,
    pub supported_version_prefixes: Vec<String>,
    pub supported_effects: Vec<EffectClass>,
    pub containment_claims: Vec<String>,
    pub authority_owner: String,
    pub integration_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DetectedHarness {
    #[serde(flatten)]
    pub descriptor: HarnessDescriptor,
    pub installed: bool,
    pub executable_path: Option<PathBuf>,
    pub executable_source: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug)]
pub struct InvocationPlan {
    pub harness_id: String,
    pub program: PathBuf,
    pub worker_program: PathBuf,
    pub harness_version: Option<String>,
    pub executable_sha256: Option<String>,
    pub arguments: Vec<String>,
    pub redacted_arguments: Vec<String>,
    pub working_directory: PathBuf,
    pub effective_effect: EffectClass,
    pub timeout_seconds: u64,
    pub containment_notes: Vec<String>,
    pub builtin_fixture: bool,
    pub ephemeral_state_paths: Vec<PathBuf>,
    pub isolation: IsolationReport,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicInvocationPlan {
    pub schema_version: String,
    pub harness_id: String,
    pub program: String,
    pub worker_program: String,
    pub harness_version: Option<String>,
    pub executable_sha256: Option<String>,
    pub arguments: Vec<String>,
    pub working_directory: PathBuf,
    pub effective_effect: EffectClass,
    pub timeout_seconds: u64,
    pub containment_notes: Vec<String>,
    pub isolation: IsolationReport,
    pub task_sha256: String,
    pub plan_sha256: String,
}

impl InvocationPlan {
    #[must_use]
    pub fn public(&self, task_sha256: &str) -> PublicInvocationPlan {
        let mut plan = PublicInvocationPlan {
            schema_version: "asi.invocation.v0.1".to_owned(),
            harness_id: self.harness_id.clone(),
            program: self.program.display().to_string(),
            worker_program: self.worker_program.display().to_string(),
            harness_version: self.harness_version.clone(),
            executable_sha256: self.executable_sha256.clone(),
            arguments: self.redacted_arguments.clone(),
            working_directory: self.working_directory.clone(),
            effective_effect: self.effective_effect,
            timeout_seconds: self.timeout_seconds,
            containment_notes: self.containment_notes.clone(),
            isolation: self.isolation.clone(),
            task_sha256: task_sha256.to_owned(),
            plan_sha256: String::new(),
        };
        let material =
            serde_json::to_vec(&plan).expect("typed invocation plan serialization must not fail");
        plan.plan_sha256 = sha256_hex(&material);
        plan
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterKind {
    Hermes,
    Pi,
    OhMyPi,
    Codex,
    Claude,
    ConstructFixture,
    IsolationFixture,
}

impl AdapterKind {
    pub const ALL: [Self; 7] = [
        Self::Hermes,
        Self::Pi,
        Self::OhMyPi,
        Self::Codex,
        Self::Claude,
        Self::ConstructFixture,
        Self::IsolationFixture,
    ];

    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Hermes => "hermes",
            Self::Pi => "pi",
            Self::OhMyPi => "oh-my-pi",
            Self::Codex => "codex",
            Self::Claude => "claude",
            Self::ConstructFixture => "construct-fixture",
            Self::IsolationFixture => "isolation-fixture",
        }
    }

    #[must_use]
    pub fn descriptor(self) -> HarnessDescriptor {
        let (display_name, executable, effects, containment, status) = match self {
            Self::Hermes => (
                "Hermes Agent",
                Some("hermes"),
                vec![EffectClass::ReadOnly],
                vec![
                    "ASI requests Hermes' safe toolset and safe mode while ignoring ambient rules"
                        .to_owned(),
                    "Hermes' safe toolset can still use provider-backed web, vision, and image services"
                        .to_owned(),
                    "Hermes remains unavailable until an executable is explicitly installed"
                        .to_owned(),
                ],
                "explicit-use adapter; local executable not assumed",
            ),
            Self::Pi => (
                "Pi",
                Some("pi"),
                vec![EffectClass::None, EffectClass::ReadOnly],
                vec![
                    "extensions, skills, prompt templates, themes, and context files disabled"
                        .to_owned(),
                    "read-only profile allowlists read, grep, find, and ls".to_owned(),
                ],
                "safe invocation profile implemented",
            ),
            Self::OhMyPi => (
                "Oh My Pi",
                Some("omp"),
                vec![EffectClass::None, EffectClass::ReadOnly],
                vec![
                    "extensions, skills, and rules disabled".to_owned(),
                    "read-only profile allowlists read, grep, find, and ls".to_owned(),
                ],
                "safe invocation profile implemented",
            ),
            Self::Codex => (
                "Codex CLI",
                Some("codex"),
                vec![EffectClass::ReadOnly],
                vec![
                    "Codex read-only sandbox requested".to_owned(),
                    "session persistence and user configuration disabled".to_owned(),
                ],
                "read-only invocation profile implemented",
            ),
            Self::Claude => (
                "Claude Code",
                Some("claude"),
                vec![EffectClass::None, EffectClass::ReadOnly],
                vec![
                    "safe mode disables customizations".to_owned(),
                    "tools are disabled or restricted to Read, Grep, and Glob".to_owned(),
                ],
                "safe invocation profile implemented",
            ),
            Self::ConstructFixture => (
                "Deterministic Construct",
                None,
                vec![EffectClass::None],
                vec!["built-in deterministic acceptance fixture; no subprocess".to_owned()],
                "built-in test construct",
            ),
            Self::IsolationFixture => (
                "Isolation Probe Construct",
                Some("sh"),
                vec![EffectClass::None],
                vec![
                    "fixed subprocess probe attempts a workspace write and an ephemeral /tmp write"
                        .to_owned(),
                    "intended only for containment acceptance testing".to_owned(),
                ],
                "built-in subprocess isolation fixture",
            ),
        };

        HarnessDescriptor {
            schema_version: "asi.harness-descriptor.v0.1".to_owned(),
            id: self.id().to_owned(),
            display_name: display_name.to_owned(),
            executable_name: executable.map(str::to_owned),
            supported_version_prefixes: self
                .supported_version_prefixes()
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            supported_effects: effects,
            containment_claims: containment,
            authority_owner: "asi-agent".to_owned(),
            integration_status: status.to_owned(),
        }
    }

    #[must_use]
    pub const fn supported_version_prefixes(self) -> &'static [&'static str] {
        match self {
            Self::Hermes => &["Hermes Agent v0.20.0"],
            Self::Pi => &["0.84.3"],
            Self::OhMyPi => &["omp/18.0.4"],
            Self::Codex => &["codex-cli 0.149.0"],
            Self::Claude => &["2.1.241"],
            Self::ConstructFixture => &["0.2.0"],
            // The isolation fixture uses only POSIX `sh -c`; its system shell
            // is part of the test boundary rather than an absorbed harness.
            Self::IsolationFixture => &[],
        }
    }

    #[must_use]
    pub fn supports_detected_version(self, version: &str) -> bool {
        self.supported_version_prefixes()
            .iter()
            .any(|prefix| version.starts_with(prefix))
    }

    #[must_use]
    pub const fn requires_version_compatibility(self) -> bool {
        !matches!(self, Self::ConstructFixture | Self::IsolationFixture)
    }

    #[must_use]
    pub fn detect(self) -> DetectedHarness {
        let descriptor = self.descriptor();
        if self == Self::ConstructFixture {
            return DetectedHarness {
                descriptor,
                installed: true,
                executable_path: None,
                executable_source: Some("in-process".to_owned()),
                version: Some(env!("CARGO_PKG_VERSION").to_owned()),
            };
        }

        let (executable_path, executable_source) = if self == Self::IsolationFixture {
            (
                Some(PathBuf::from("/bin/sh")),
                Some("fixed-system-path".to_owned()),
            )
        } else {
            self.resolve_executable(
                descriptor
                    .executable_name
                    .as_deref()
                    .expect("subprocess descriptor must name an executable"),
            )
            .map_or((None, None), |(path, source)| {
                (Some(path), Some(source.to_owned()))
            })
        };
        let version = executable_path.as_deref().and_then(read_version);
        DetectedHarness {
            installed: executable_path.is_some(),
            descriptor,
            executable_path,
            executable_source,
            version,
        }
    }

    fn resolve_executable(self, executable_name: &str) -> Option<(PathBuf, &'static str)> {
        if let Some(path) = self
            .override_variable()
            .and_then(std::env::var_os)
            .map(PathBuf::from)
            .filter(|candidate| is_executable(candidate))
            .and_then(|candidate| candidate.canonicalize().ok())
        {
            return Some((path, "environment-override"));
        }

        if let Some(path) = self
            .mise_direct_candidates()
            .into_iter()
            .find(|candidate| is_executable(candidate))
            .and_then(|candidate| candidate.canonicalize().ok())
        {
            return Some((path, "mise-direct"));
        }

        find_executable(executable_name).map(|path| (path, "path"))
    }

    const fn override_variable(self) -> Option<&'static str> {
        match self {
            Self::Hermes => Some("ASI_HARNESS_HERMES_PATH"),
            Self::Pi => Some("ASI_HARNESS_PI_PATH"),
            Self::OhMyPi => Some("ASI_HARNESS_OH_MY_PI_PATH"),
            Self::Codex => Some("ASI_HARNESS_CODEX_PATH"),
            Self::Claude => Some("ASI_HARNESS_CLAUDE_PATH"),
            Self::ConstructFixture | Self::IsolationFixture => None,
        }
    }

    fn mise_direct_candidates(self) -> Vec<PathBuf> {
        let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
            return Vec::new();
        };
        let installs = home.join(".local/share/mise/installs");
        let relative_paths: &[&str] = match self {
            Self::Pi => &["pi/latest/pi/pi", "pi/latest/bin/pi"],
            Self::OhMyPi => &[
                "github-can1357-oh-my-pi/latest/omp",
                "github-can1357-oh-my-pi/latest/bin/omp",
            ],
            Self::Codex => &["codex/latest/bin/codex"],
            Self::Claude => &["claude/latest/claude", "claude/latest/bin/claude"],
            Self::Hermes | Self::ConstructFixture | Self::IsolationFixture => &[],
        };
        relative_paths
            .iter()
            .map(|relative| installs.join(relative))
            .collect()
    }

    pub fn plan(self, task: &TaskEnvelope, executable: Option<&Path>) -> Result<InvocationPlan> {
        if !self
            .descriptor()
            .supported_effects
            .contains(&task.requested_effect)
        {
            bail!(
                "harness {} does not support effect profile {}",
                self.id(),
                task.requested_effect
            );
        }

        if self == Self::ConstructFixture {
            return Ok(InvocationPlan {
                harness_id: self.id().to_owned(),
                program: PathBuf::from("builtin:construct-fixture"),
                worker_program: PathBuf::from("builtin:construct-fixture"),
                harness_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
                executable_sha256: None,
                arguments: Vec::new(),
                redacted_arguments: vec![redacted_task(&task.prompt_sha256)],
                working_directory: task.working_directory.clone(),
                effective_effect: EffectClass::None,
                timeout_seconds: task.budget.timeout_seconds,
                containment_notes: self.descriptor().containment_claims,
                builtin_fixture: true,
                ephemeral_state_paths: Vec::new(),
                isolation: IsolationReport::builtin(),
            });
        }

        let program = executable
            .map(Path::to_path_buf)
            .ok_or_else(|| anyhow::anyhow!("harness {} is not installed", self.id()))?;
        let (arguments, redacted_arguments) = self.arguments(task)?;
        Ok(InvocationPlan {
            harness_id: self.id().to_owned(),
            worker_program: program.clone(),
            program,
            harness_version: None,
            executable_sha256: None,
            arguments,
            redacted_arguments,
            working_directory: task.working_directory.clone(),
            effective_effect: task.requested_effect,
            timeout_seconds: task.budget.timeout_seconds,
            containment_notes: self.descriptor().containment_claims,
            builtin_fixture: false,
            ephemeral_state_paths: self.ephemeral_state_paths(),
            isolation: IsolationReport::pending(),
        })
    }

    fn ephemeral_state_paths(self) -> Vec<PathBuf> {
        let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
            return Vec::new();
        };
        let relative_paths: &[&str] = match self {
            Self::Hermes => &[".hermes"],
            Self::Pi => &[".pi"],
            Self::OhMyPi => &[".omp", ".pi"],
            Self::Codex => &[".codex"],
            Self::Claude => &[".claude"],
            Self::ConstructFixture | Self::IsolationFixture => &[],
        };
        relative_paths
            .iter()
            .map(|relative| home.join(relative))
            .collect()
    }

    fn arguments(self, task: &TaskEnvelope) -> Result<(Vec<String>, Vec<String>)> {
        let task_redaction = redacted_task(&task.prompt_sha256);
        if self == Self::IsolationFixture {
            let script = "if [ -n \"${ASI_AMBIENT_SECRET:-}\" ]; then printf '%s' 'AMBIENT_SECRET_LEAK' >&2; exit 72; fi; if [ \"${1:-}\" = 'diagnostic-negative-control' ]; then printf '%s' 'HARNESS_SECRET_DIAGNOSTIC' >&2; exit 71; fi; if touch .asi-isolation-escape 2>/dev/null; then printf '%s' 'WORKSPACE_WRITE_ESCAPE'; exit 73; fi; touch /tmp/asi-isolation-ok; test -f /tmp/asi-isolation-ok; printf '%s' 'ASI_ISOLATION_OK'";
            return Ok((
                vec![
                    "-c".to_owned(),
                    script.to_owned(),
                    "asi-isolation-fixture".to_owned(),
                    task.prompt.clone(),
                ],
                vec![
                    "-c".to_owned(),
                    script.to_owned(),
                    "asi-isolation-fixture".to_owned(),
                    task_redaction,
                ],
            ));
        }
        let mut args = match self {
            Self::Pi => vec![
                "--print",
                "--no-session",
                "--no-extensions",
                "--no-skills",
                "--no-prompt-templates",
                "--no-themes",
                "--no-context-files",
                "--no-approve",
                "--mode",
                "text",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
            Self::OhMyPi => vec![
                "--print",
                "--no-session",
                "--no-extensions",
                "--no-skills",
                "--no-rules",
                "--approval-mode=always-ask",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
            Self::Codex => vec![
                "exec".to_owned(),
                "--sandbox".to_owned(),
                "read-only".to_owned(),
                "--ephemeral".to_owned(),
                "--ignore-user-config".to_owned(),
                "--ignore-rules".to_owned(),
                "--skip-git-repo-check".to_owned(),
                "--color".to_owned(),
                "never".to_owned(),
                "-C".to_owned(),
                task.working_directory.display().to_string(),
            ],
            Self::Claude => vec![
                "--print".to_owned(),
                "--safe-mode".to_owned(),
                "--no-session-persistence".to_owned(),
                "--output-format".to_owned(),
                "text".to_owned(),
            ],
            Self::Hermes => vec![
                "chat".to_owned(),
                "--toolsets".to_owned(),
                "safe".to_owned(),
                "--max-turns".to_owned(),
                "8".to_owned(),
                "--quiet".to_owned(),
                "--safe-mode".to_owned(),
                "--ignore-user-config".to_owned(),
                "--ignore-rules".to_owned(),
                "--query".to_owned(),
            ],
            Self::ConstructFixture | Self::IsolationFixture => {
                unreachable!("fixture handled before argument planning")
            }
        };

        match self {
            Self::Pi => match task.requested_effect {
                EffectClass::None => args.push("--no-tools".to_owned()),
                EffectClass::ReadOnly => {
                    args.extend(["--tools".to_owned(), "read,grep,find,ls".to_owned()]);
                }
                _ => unreachable!("effect support checked before argument planning"),
            },
            Self::OhMyPi => match task.requested_effect {
                EffectClass::None => args.push("--no-tools".to_owned()),
                EffectClass::ReadOnly => {
                    args.push("--tools=read,grep,find,ls".to_owned());
                }
                _ => unreachable!("effect support checked before argument planning"),
            },
            Self::Claude => match task.requested_effect {
                EffectClass::None => {
                    args.extend([
                        "--tools".to_owned(),
                        String::new(),
                        "--permission-mode".to_owned(),
                        "dontAsk".to_owned(),
                    ]);
                }
                EffectClass::ReadOnly => {
                    args.extend([
                        "--tools".to_owned(),
                        "Read,Grep,Glob".to_owned(),
                        "--permission-mode".to_owned(),
                        "plan".to_owned(),
                    ]);
                }
                _ => unreachable!("effect support checked before argument planning"),
            },
            Self::Hermes | Self::Codex | Self::ConstructFixture | Self::IsolationFixture => {}
        }

        if self != Self::Hermes {
            // Prevent a task beginning with `--` from being interpreted as an
            // option capable of changing the adapter's containment profile.
            args.push("--".to_owned());
        }
        args.push(task.prompt.clone());
        let mut redacted = args.clone();
        let final_argument = redacted
            .last_mut()
            .ok_or_else(|| anyhow::anyhow!("adapter generated no task argument"))?;
        *final_argument = task_redaction;
        Ok((args, redacted))
    }
}

#[must_use]
pub fn find_executable(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join(name))
        .find(|candidate| is_executable(candidate))
        .and_then(|candidate| candidate.canonicalize().ok())
}

#[cfg(unix)]
fn is_executable(candidate: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    candidate.is_file()
        && candidate
            .metadata()
            .is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable(candidate: &Path) -> bool {
    candidate.is_file()
}

fn read_version(executable: &Path) -> Option<String> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = executable;
        return None;
    }

    #[cfg(target_os = "linux")]
    let (stdout_bytes, stderr_bytes) = {
        let bwrap = find_trusted_system_executable("bwrap")?;
        let timeout = find_trusted_system_executable("timeout")?;
        bounded_version_probe(&timeout, &bwrap, executable)?
    };
    let stdout = String::from_utf8_lossy(&stdout_bytes);
    let stderr = String::from_utf8_lossy(&stderr_bytes);
    stdout
        .lines()
        .chain(stderr.lines())
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("mise "))
        .map(|line| line.chars().take(200).collect())
}

#[cfg(target_os = "linux")]
fn bounded_version_probe(
    timeout: &Path,
    bwrap: &Path,
    executable: &Path,
) -> Option<(Vec<u8>, Vec<u8>)> {
    const MAX_VERSION_BYTES: u64 = 8_192;

    let mut child = Command::new(timeout)
        .args([
            "--signal=TERM",
            "--kill-after=1s",
            "5s",
            bwrap.to_str()?,
            "--die-with-parent",
            "--new-session",
            "--unshare-all",
            "--unshare-user",
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
            "--setenv",
            "HOME",
            "/tmp",
            "--chdir",
            "/",
            "--",
            executable.to_str()?,
            "--version",
        ])
        .env_clear()
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("TZ", "UTC")
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let stderr = child.stderr.take()?;
    let stdout_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take(MAX_VERSION_BYTES)
            .read_to_end(&mut bytes)
            .map(|_| bytes)
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr
            .take(MAX_VERSION_BYTES)
            .read_to_end(&mut bytes)
            .map(|_| bytes)
    });
    let status = child.wait().ok()?;
    let stdout = stdout_reader.join().ok()?.ok()?;
    let stderr = stderr_reader.join().ok()?.ok()?;
    status.success().then_some((stdout, stderr))
}

/// Resolve a security-critical host utility only when PATH selects a
/// root-owned binary that is not writable by group or world. This preserves
/// normal distro/Nix layouts without trusting a user-controlled PATH shim.
#[cfg(unix)]
#[must_use]
pub fn find_trusted_system_executable(name: &str) -> Option<PathBuf> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let candidate = find_executable(name)?;
    let metadata = candidate.metadata().ok()?;
    (metadata.uid() == 0 && metadata.permissions().mode() & 0o022 == 0).then_some(candidate)
}

#[cfg(not(unix))]
#[must_use]
pub fn find_trusted_system_executable(_name: &str) -> Option<PathBuf> {
    None
}

fn redacted_task(sha256: &str) -> String {
    format!("<task:sha256={sha256}>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pi_none_profile_disables_ambient_capabilities() {
        let task = TaskEnvelope::new("private prompt", EffectClass::None, ".", 30)
            .expect("task should be valid");
        let plan = AdapterKind::Pi
            .plan(&task, Some(Path::new("/usr/bin/pi")))
            .expect("plan should be valid");
        assert!(
            plan.arguments
                .iter()
                .any(|argument| argument == "--no-tools")
        );
        assert!(
            plan.arguments
                .iter()
                .any(|argument| argument == "--no-skills")
        );
        let public = plan.public(&task.prompt_sha256);
        assert!(
            !serde_json::to_string(&public)
                .expect("serialization should work")
                .contains("private prompt")
        );
    }

    #[test]
    fn codex_refuses_no_tool_claim_it_cannot_enforce() {
        let task =
            TaskEnvelope::new("test", EffectClass::None, ".", 30).expect("task should be valid");
        let error = AdapterKind::Codex
            .plan(&task, Some(Path::new("/usr/bin/codex")))
            .expect_err("unsupported effect should fail");
        assert!(error.to_string().contains("does not support"));
    }

    #[test]
    fn task_cannot_inject_adapter_options() {
        for adapter in [
            AdapterKind::Pi,
            AdapterKind::OhMyPi,
            AdapterKind::Codex,
            AdapterKind::Claude,
        ] {
            let effect = if adapter == AdapterKind::Codex {
                EffectClass::ReadOnly
            } else {
                EffectClass::None
            };
            let task = TaskEnvelope::new(
                "--dangerously-bypass-approvals-and-sandbox",
                effect,
                ".",
                30,
            )
            .expect("task should be valid");
            let plan = adapter
                .plan(&task, Some(Path::new("/usr/bin/example")))
                .expect("plan should be valid");
            let prompt_position = plan.arguments.len() - 1;
            assert_eq!(plan.arguments[prompt_position - 1], "--");
            assert_eq!(plan.arguments[prompt_position], task.prompt);
            assert_eq!(
                plan.redacted_arguments[prompt_position],
                redacted_task(&task.prompt_sha256)
            );
        }
    }
}
