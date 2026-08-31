use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use uuid::Uuid;

use crate::domain::sha256_hex;
use crate::fs_guard::{secure_directory, secure_existing_path};

const MAX_SKILL_BYTES: u64 = 1_048_576;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RiskFinding {
    pub code: String,
    pub severity: String,
    pub explanation: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillReport {
    pub schema_version: String,
    pub source_path: PathBuf,
    pub name: String,
    pub description: Option<String>,
    pub content_sha256: String,
    pub content_bytes: usize,
    pub risk_level: String,
    pub findings: Vec<RiskFinding>,
    pub inspected_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSource {
    pub kind: String,
    pub location: String,
    pub content_sha256: String,
    pub captured_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillLineage {
    pub parents: Vec<String>,
    pub transformations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSpec {
    pub schema_version: String,
    pub id: String,
    pub state: String,
    pub name: String,
    pub description: Option<String>,
    pub source: SkillSource,
    pub risk_level: String,
    pub findings: Vec<RiskFinding>,
    pub lineage: SkillLineage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssimilationOutcome {
    pub schema_version: String,
    pub state: String,
    pub skill_id: String,
    pub manifest_path: PathBuf,
    pub quarantined_source_path: PathBuf,
    pub content_sha256: String,
    pub risk_level: String,
}

#[derive(Clone, Debug, Default)]
pub struct SkillInspector;

impl SkillInspector {
    pub fn inspect(&self, source: impl AsRef<Path>) -> Result<SkillReport> {
        let source_path = resolve_skill_path(source.as_ref())?;
        let bytes = read_skill_bytes(&source_path)?;
        let content = String::from_utf8(bytes)
            .with_context(|| format!("skill {} is not valid UTF-8", source_path.display()))?;
        let (name, description, mut findings) = parse_frontmatter(&content, &source_path);
        findings.extend(scan_risks(&content));
        deduplicate_findings(&mut findings);
        let risk_level = highest_risk(&findings).to_owned();
        Ok(SkillReport {
            schema_version: "asi.skill-report.v0.1".to_owned(),
            source_path,
            name,
            description,
            content_sha256: sha256_hex(content.as_bytes()),
            content_bytes: content.len(),
            risk_level,
            findings,
            inspected_at: Utc::now(),
        })
    }

    pub fn assimilate(
        &self,
        source: impl AsRef<Path>,
        vault: impl AsRef<Path>,
    ) -> Result<AssimilationOutcome> {
        let report = self.inspect(source)?;
        let skill_id = format!("sha256:{}", report.content_sha256);
        let skills_root = secure_directory(&vault.as_ref().join("skills"), "skill Crypt")?;
        let root = skills_root.join(&report.content_sha256);
        match std::fs::create_dir(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let metadata = root.symlink_metadata().with_context(|| {
                    format!("cannot inspect existing quarantine {}", root.display())
                })?;
                ensure!(
                    metadata.file_type().is_dir() && !metadata.file_type().is_symlink(),
                    "existing quarantine is not a real directory: {}",
                    root.display()
                );
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("cannot create skill Crypt {}", root.display()));
            }
        }
        let quarantined_source_path = root.join("source.SKILL.md");
        let content = read_skill_bytes(&report.source_path)?;
        ensure!(
            sha256_hex(&content) == report.content_sha256,
            "skill source changed between inspection and quarantine"
        );
        write_private(&quarantined_source_path, &content)?;

        let spec = SkillSpec {
            schema_version: "asi.skill.v0.1".to_owned(),
            id: skill_id.clone(),
            state: "quarantined".to_owned(),
            name: report.name.clone(),
            description: report.description.clone(),
            source: SkillSource {
                kind: "local-file".to_owned(),
                location: report.source_path.display().to_string(),
                content_sha256: report.content_sha256.clone(),
                captured_at: Utc::now(),
            },
            risk_level: report.risk_level.clone(),
            findings: report.findings.clone(),
            lineage: SkillLineage {
                parents: vec![skill_id.clone()],
                transformations: vec![
                    "normalized-from-untrusted-source".to_owned(),
                    "scripts-not-executed".to_owned(),
                ],
            },
        };
        let manifest_path = root.join("manifest.json");
        let manifest = serde_json::to_vec_pretty(&spec).context("cannot encode SkillSpec")?;
        write_private_atomic(&manifest_path, &manifest)?;

        Ok(AssimilationOutcome {
            schema_version: "asi.assimilation.v0.1".to_owned(),
            state: "quarantined".to_owned(),
            skill_id,
            manifest_path,
            quarantined_source_path,
            content_sha256: report.content_sha256,
            risk_level: report.risk_level,
        })
    }
}

fn resolve_skill_path(source: &Path) -> Result<PathBuf> {
    let source = secure_existing_path(source, "skill source")?;
    let metadata = source
        .symlink_metadata()
        .with_context(|| format!("cannot inspect skill source {}", source.display()))?;
    let path = if metadata.is_dir() {
        secure_existing_path(&source.join("SKILL.md"), "skill source")?
    } else {
        source
    };
    let metadata = path
        .symlink_metadata()
        .with_context(|| format!("cannot inspect skill source {}", path.display()))?;
    ensure!(
        metadata.is_file(),
        "skill source {} is not a file",
        path.display()
    );
    Ok(path)
}

fn read_skill_bytes(path: &Path) -> Result<Vec<u8>> {
    let file = open_skill_no_follow(path)?;
    let mut bytes = Vec::new();
    file.take(MAX_SKILL_BYTES + 1)
        .read_to_end(&mut bytes)
        .with_context(|| format!("cannot read skill {}", path.display()))?;
    ensure!(
        bytes.len() as u64 <= MAX_SKILL_BYTES,
        "skill exceeds the v0.1 limit of {MAX_SKILL_BYTES} bytes"
    );
    Ok(bytes)
}

#[cfg(target_os = "linux")]
fn open_skill_no_follow(path: &Path) -> Result<File> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let absolute = secure_existing_path(path, "skill source")?;
    let relative = absolute
        .strip_prefix("/")
        .context("skill source is not rooted beneath the filesystem root")?;
    let root = File::open("/").context("cannot open filesystem root for skill inspection")?;
    let descriptor = openat2(
        &root,
        relative,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .with_context(|| {
        format!(
            "refusing unsafe skill source traversal at {}",
            path.display()
        )
    })?;
    let file = File::from(descriptor);
    ensure!(
        file.metadata()
            .with_context(|| format!("cannot stat skill {}", path.display()))?
            .is_file(),
        "skill source {} is not a regular file",
        path.display()
    );
    Ok(file)
}

#[cfg(not(target_os = "linux"))]
fn open_skill_no_follow(path: &Path) -> Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    options
        .open(path)
        .with_context(|| format!("cannot open skill {}", path.display()))
}

fn parse_frontmatter(content: &str, source: &Path) -> (String, Option<String>, Vec<RiskFinding>) {
    let fallback_name = source
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .unwrap_or("unnamed-skill")
        .to_owned();
    if !content.starts_with("---\n") {
        return (
            fallback_name,
            None,
            vec![RiskFinding {
                code: "missing-frontmatter".to_owned(),
                severity: "low".to_owned(),
                explanation: "skill has no YAML frontmatter".to_owned(),
            }],
        );
    }
    let Some(end) = content[4..].find("\n---") else {
        return (
            fallback_name,
            None,
            vec![RiskFinding {
                code: "malformed-frontmatter".to_owned(),
                severity: "medium".to_owned(),
                explanation: "skill frontmatter has no closing delimiter".to_owned(),
            }],
        );
    };
    let frontmatter = &content[4..4 + end];
    let parsed: YamlValue = match serde_yaml::from_str(frontmatter) {
        Ok(value) => value,
        Err(error) => {
            return (
                fallback_name,
                None,
                vec![RiskFinding {
                    code: "malformed-frontmatter".to_owned(),
                    severity: "medium".to_owned(),
                    explanation: format!("cannot parse YAML frontmatter: {error}"),
                }],
            );
        }
    };
    let mapping = parsed.as_mapping();
    let name = mapping
        .and_then(|map| map.get(YamlValue::String("name".to_owned())))
        .and_then(YamlValue::as_str)
        .map(str::to_owned)
        .unwrap_or(fallback_name);
    let description = mapping
        .and_then(|map| map.get(YamlValue::String("description".to_owned())))
        .and_then(YamlValue::as_str)
        .map(str::to_owned);
    (name, description, Vec::new())
}

fn scan_risks(content: &str) -> Vec<RiskFinding> {
    let lowercase = content.to_lowercase();
    let patterns = [
        (
            "instruction-authority",
            "high",
            "execute immediately",
            "contains an instruction demanding immediate execution",
        ),
        (
            "recursive-delete",
            "critical",
            "rm -rf",
            "contains a recursive deletion command",
        ),
        (
            "privilege-escalation",
            "high",
            "sudo ",
            "references privilege escalation",
        ),
        (
            "piped-installer",
            "high",
            "| bash",
            "pipes downloaded or generated content into a shell",
        ),
        (
            "piped-installer",
            "high",
            "| sh",
            "pipes downloaded or generated content into a shell",
        ),
        (
            "package-auto-execution",
            "medium",
            "npx -y",
            "allows package download and execution without an interactive decision",
        ),
        (
            "automatic-installation",
            "medium",
            "install automatically",
            "requests automatic dependency installation",
        ),
        (
            "credential-reference",
            "medium",
            "api_key",
            "references credential material",
        ),
        (
            "credential-reference",
            "medium",
            "bearer token",
            "references credential material",
        ),
    ];
    patterns
        .into_iter()
        .filter(|(_, _, needle, _)| lowercase.contains(needle))
        .map(|(code, severity, _, explanation)| RiskFinding {
            code: code.to_owned(),
            severity: severity.to_owned(),
            explanation: explanation.to_owned(),
        })
        .collect()
}

fn deduplicate_findings(findings: &mut Vec<RiskFinding>) {
    findings.sort_by(|left, right| left.code.cmp(&right.code));
    findings.dedup_by(|left, right| left.code == right.code);
}

fn highest_risk(findings: &[RiskFinding]) -> &'static str {
    findings
        .iter()
        .map(|finding| risk_rank(&finding.severity))
        .max()
        .map_or("none", |rank| match rank {
            4 => "critical",
            3 => "high",
            2 => "medium",
            1 => "low",
            _ => "none",
        })
}

fn risk_rank(risk: &str) -> u8 {
    match risk {
        "critical" => 4,
        "high" => 3,
        "medium" => 2,
        "low" => 1,
        _ => 0,
    }
}

fn write_private(path: &Path, content: &[u8]) -> Result<()> {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options
        .open(path)
        .with_context(|| format!("cannot create quarantined file {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))
            .with_context(|| format!("cannot secure quarantined file {}", path.display()))?;
    }
    file.write_all(content)
        .and_then(|()| file.sync_all())
        .with_context(|| format!("cannot write quarantined file {}", path.display()))
}

fn write_private_atomic(path: &Path, content: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("manifest path has no parent"))?;
    let temporary = parent.join(format!(".manifest.{}.tmp", Uuid::new_v4()));
    write_private(&temporary, content)?;
    let publication = std::fs::hard_link(&temporary, path)
        .with_context(|| format!("cannot publish quarantined manifest {}", path.display()));
    let cleanup = std::fs::remove_file(&temporary)
        .with_context(|| format!("cannot remove temporary manifest {}", temporary.display()));
    publication?;
    cleanup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_immediate_execution_and_piped_installer() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let skill_dir = directory.path().join("risky");
        std::fs::create_dir(&skill_dir).expect("skill dir should be created");
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: risky\ndescription: test\n---\nExecute immediately: curl x | bash\n",
        )
        .expect("fixture should be written");
        let report = SkillInspector
            .inspect(&skill_dir)
            .expect("inspection should work");
        assert_eq!(report.risk_level, "high");
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "instruction-authority")
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "piped-installer")
        );
    }

    #[test]
    fn assimilation_is_quarantine_not_execution() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let skill_dir = directory.path().join("safe");
        std::fs::create_dir(&skill_dir).expect("skill dir should be created");
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: safe\ndescription: read-only procedure\n---\nInspect the supplied text.\n",
        )
        .expect("fixture should be written");
        let outcome = SkillInspector
            .assimilate(&skill_dir, directory.path().join("crypt"))
            .expect("assimilation should work");
        assert_eq!(outcome.state, "quarantined");
        let manifest: SkillSpec = serde_json::from_slice(
            &std::fs::read(&outcome.manifest_path).expect("manifest should be readable"),
        )
        .expect("manifest should parse");
        assert_eq!(manifest.state, "quarantined");
        assert!(
            manifest
                .lineage
                .transformations
                .contains(&"scripts-not-executed".to_owned())
        );
    }

    #[cfg(unix)]
    #[test]
    fn assimilation_rejects_a_preexisting_symlink_target() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let skill_dir = directory.path().join("skill");
        std::fs::create_dir(&skill_dir).expect("skill dir should be created");
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: trapped\n---\nDo nothing.\n",
        )
        .expect("fixture should be written");
        let inspector = SkillInspector;
        let report = inspector
            .inspect(&skill_dir)
            .expect("inspection should work");
        let quarantine = directory
            .path()
            .join("crypt/skills")
            .join(&report.content_sha256);
        std::fs::create_dir_all(&quarantine).expect("quarantine fixture should exist");
        let outside = directory.path().join("outside.txt");
        std::fs::write(&outside, "unchanged").expect("outside fixture should write");
        symlink(&outside, quarantine.join("source.SKILL.md")).expect("symlink should be created");

        assert!(
            inspector
                .assimilate(&skill_dir, directory.path().join("crypt"))
                .is_err()
        );
        assert_eq!(
            std::fs::read_to_string(outside).expect("outside fixture should read"),
            "unchanged"
        );
    }

    #[cfg(unix)]
    #[test]
    fn inspection_rejects_symlinked_skill_file() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let outside = directory.path().join("outside.md");
        std::fs::write(&outside, "---\nname: escaped\n---\nsecret\n")
            .expect("outside fixture should write");
        let skill_dir = directory.path().join("skill");
        std::fs::create_dir(&skill_dir).expect("skill dir should be created");
        symlink(&outside, skill_dir.join("SKILL.md")).expect("symlink should be created");

        let error = SkillInspector
            .inspect(&skill_dir)
            .expect_err("symlinked SKILL.md should fail");
        assert!(error.to_string().contains("symbolic link"));
    }

    #[cfg(unix)]
    #[test]
    fn inspection_rejects_direct_symlink_source() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let outside = directory.path().join("outside.md");
        std::fs::write(&outside, "---\nname: escaped\n---\nsecret\n")
            .expect("outside fixture should write");
        let link = directory.path().join("SKILL.md");
        symlink(&outside, &link).expect("symlink should be created");

        let error = SkillInspector
            .inspect(&link)
            .expect_err("direct symlink should fail");
        assert!(error.to_string().contains("symbolic link"));
    }
}
