use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Maximum side-effect class requested by a task.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
pub enum EffectClass {
    /// No harness tools. The harness may still contact its configured model provider.
    #[value(name = "none")]
    None,
    /// Filesystem or repository reads, with no writes.
    #[value(name = "read-only")]
    ReadOnly,
    /// Reversible writes inside the selected workspace.
    #[value(name = "workspace-write")]
    WorkspaceWrite,
    /// Communication, publication, purchases, remote mutation, or other external effects.
    #[value(name = "external")]
    External,
}

impl EffectClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReadOnly => "read-only",
            Self::WorkspaceWrite => "workspace-write",
            Self::External => "external",
        }
    }
}

impl std::fmt::Display for EffectClass {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Process-containment policy for subprocess harnesses.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum IsolationMode {
    /// Require the platform enforcer and fail closed when it is unavailable.
    #[default]
    Required,
    /// Run the harness directly. Requires a separate explicit acknowledgement.
    Off,
}

impl IsolationMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Off => "off",
        }
    }
}

impl std::fmt::Display for IsolationMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IsolationReport {
    pub schema_version: String,
    pub requested_mode: IsolationMode,
    pub enforced: bool,
    pub enforcer: String,
    pub filesystem: String,
    pub namespaces: Vec<String>,
    pub ephemeral_writes: Vec<PathBuf>,
    pub network: String,
    pub limitations: Vec<String>,
}

impl IsolationReport {
    #[must_use]
    pub fn pending() -> Self {
        Self {
            schema_version: "asi.isolation-report.v0.1".to_owned(),
            requested_mode: IsolationMode::Required,
            enforced: false,
            enforcer: "pending-runtime-policy".to_owned(),
            filesystem: "not-yet-applied".to_owned(),
            namespaces: Vec::new(),
            ephemeral_writes: Vec::new(),
            network: "not-yet-applied".to_owned(),
            limitations: Vec::new(),
        }
    }

    #[must_use]
    pub fn builtin() -> Self {
        Self {
            schema_version: "asi.isolation-report.v0.1".to_owned(),
            requested_mode: IsolationMode::Required,
            enforced: true,
            enforcer: "in-process-deterministic-construct".to_owned(),
            filesystem: "no-subprocess".to_owned(),
            namespaces: Vec::new(),
            ephemeral_writes: Vec::new(),
            network: "no-subprocess".to_owned(),
            limitations: vec!["fixture proves control flow, not OS containment".to_owned()],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecutionBudget {
    pub timeout_seconds: u64,
    pub max_output_bytes: usize,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        Self {
            timeout_seconds: 120,
            max_output_bytes: 1_048_576,
        }
    }
}

/// Versioned task contract crossing the sovereign control boundary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskEnvelope {
    pub schema_version: String,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub prompt: String,
    pub prompt_sha256: String,
    pub requested_effect: EffectClass,
    pub working_directory: PathBuf,
    pub budget: ExecutionBudget,
}

impl TaskEnvelope {
    pub fn new(
        prompt: impl Into<String>,
        requested_effect: EffectClass,
        working_directory: impl AsRef<Path>,
        timeout_seconds: u64,
    ) -> Result<Self> {
        let prompt = prompt.into();
        ensure!(!prompt.trim().is_empty(), "task prompt cannot be empty");
        ensure!(timeout_seconds > 0, "timeout must be greater than zero");
        ensure!(
            timeout_seconds <= 3_600,
            "timeout exceeds the v0.1 ceiling of 3600 seconds"
        );

        let working_directory = working_directory.as_ref().canonicalize().with_context(|| {
            format!(
                "cannot resolve working directory {}",
                working_directory.as_ref().display()
            )
        })?;
        ensure!(
            working_directory.is_dir(),
            "working directory is not a directory"
        );

        let prompt_sha256 = sha256_hex(prompt.as_bytes());
        Ok(Self {
            schema_version: "asi.task.v0.1".to_owned(),
            run_id: Uuid::new_v4(),
            task_id: Uuid::new_v4(),
            created_at: Utc::now(),
            prompt,
            prompt_sha256,
            requested_effect,
            working_directory,
            budget: ExecutionBudget {
                timeout_seconds,
                ..ExecutionBudget::default()
            },
        })
    }
}

#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_envelope_redacts_prompt_when_serialized() {
        let envelope = TaskEnvelope::new("secret task", EffectClass::None, ".", 30)
            .expect("task should be valid");
        let serialized = serde_json::to_string(&envelope).expect("serialization should work");
        assert!(!serialized.contains("secret task"));
        assert!(serialized.contains(&envelope.prompt_sha256));
    }

    #[test]
    fn rejects_empty_prompt() {
        let error = TaskEnvelope::new("  ", EffectClass::None, ".", 30)
            .expect_err("empty task should fail");
        assert!(error.to_string().contains("cannot be empty"));
    }
}
