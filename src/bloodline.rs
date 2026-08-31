use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::domain::sha256_hex;
use crate::fs_guard::{secure_existing_path, secure_output_path};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BloodlineEvent {
    pub schema_version: String,
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub run_id: String,
    pub kind: String,
    pub payload: Value,
    pub previous_hash: Option<String>,
    pub integrity_hash: String,
}

#[derive(Serialize)]
struct HashMaterial<'a> {
    schema_version: &'a str,
    sequence: u64,
    timestamp: DateTime<Utc>,
    run_id: &'a str,
    kind: &'a str,
    payload: &'a Value,
    previous_hash: &'a Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerVerification {
    pub valid: bool,
    pub events: u64,
    pub last_hash: Option<String>,
    pub path: PathBuf,
}

pub(crate) struct LedgerSnapshot {
    pub verification: LedgerVerification,
    pub sha256: String,
}

/// Locked, append-only JSONL ledger with a cryptographic hash chain.
pub struct BloodlineLedger {
    path: PathBuf,
    file: File,
    last_sequence: u64,
    last_hash: Option<String>,
}

impl BloodlineLedger {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = secure_output_path(path.as_ref(), "Bloodline")?;
        let mut options = OpenOptions::new();
        options.create(true).read(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let file = options
            .open(&path)
            .with_context(|| format!("cannot open Bloodline ledger {}", path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            let metadata = file
                .metadata()
                .with_context(|| format!("cannot stat Bloodline ledger {}", path.display()))?;
            ensure!(
                metadata.is_file() && metadata.nlink() == 1,
                "Bloodline ledger must be a regular file with one hard link"
            );
            file.set_permissions(std::fs::Permissions::from_mode(0o600))
                .with_context(|| format!("cannot secure Bloodline ledger {}", path.display()))?;
        }
        FileExt::lock_exclusive(&file)
            .with_context(|| format!("cannot lock Bloodline ledger {}", path.display()))?;

        let verification = verify_locked_file(&file, &path)?;
        Ok(Self {
            path,
            file,
            last_sequence: verification.events,
            last_hash: verification.last_hash,
        })
    }

    pub fn append(&mut self, run_id: &str, kind: &str, payload: Value) -> Result<BloodlineEvent> {
        ensure!(!run_id.trim().is_empty(), "run id cannot be empty");
        ensure!(!kind.trim().is_empty(), "event kind cannot be empty");

        let sequence = self
            .last_sequence
            .checked_add(1)
            .context("Bloodline sequence exhausted")?;
        let timestamp = Utc::now();
        let schema_version = "asi.bloodline.v0.1";
        let hash_material = HashMaterial {
            schema_version,
            sequence,
            timestamp,
            run_id,
            kind,
            payload: &payload,
            previous_hash: &self.last_hash,
        };
        let integrity_hash = sha256_hex(
            &serde_json::to_vec(&hash_material).context("cannot encode Bloodline hash material")?,
        );
        let event = BloodlineEvent {
            schema_version: schema_version.to_owned(),
            sequence,
            timestamp,
            run_id: run_id.to_owned(),
            kind: kind.to_owned(),
            payload,
            previous_hash: self.last_hash.clone(),
            integrity_hash: integrity_hash.clone(),
        };
        let encoded = serde_json::to_vec(&event).context("cannot encode Bloodline event")?;
        self.file
            .write_all(&encoded)
            .and_then(|()| self.file.write_all(b"\n"))
            .and_then(|()| self.file.flush())
            .with_context(|| format!("cannot append Bloodline ledger {}", self.path.display()))?;
        self.file
            .sync_data()
            .with_context(|| format!("cannot sync Bloodline ledger {}", self.path.display()))?;
        self.last_sequence = sequence;
        self.last_hash = Some(integrity_hash);
        Ok(event)
    }

    pub fn verify(path: impl AsRef<Path>) -> Result<LedgerVerification> {
        Ok(Self::snapshot(path)?.verification)
    }

    pub(crate) fn snapshot(path: impl AsRef<Path>) -> Result<LedgerSnapshot> {
        let path = secure_existing_path(path.as_ref(), "Bloodline")?;
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let file = options
            .open(&path)
            .with_context(|| format!("cannot open Bloodline ledger {}", path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let metadata = file
                .metadata()
                .with_context(|| format!("cannot stat Bloodline ledger {}", path.display()))?;
            ensure!(
                metadata.is_file() && metadata.nlink() == 1,
                "Bloodline ledger must be a regular file with one hard link"
            );
        }
        FileExt::lock_shared(&file)
            .with_context(|| format!("cannot lock Bloodline ledger {}", path.display()))?;
        let verification = verify_locked_file(&file, &path)?;
        let sha256 = sha256_locked_file(&file, &path)?;
        Ok(LedgerSnapshot {
            verification,
            sha256,
        })
    }
}

fn sha256_locked_file(file: &File, path: &Path) -> Result<String> {
    let mut reader = file
        .try_clone()
        .with_context(|| format!("cannot clone ledger handle {}", path.display()))?;
    reader
        .seek(SeekFrom::Start(0))
        .with_context(|| format!("cannot seek Bloodline ledger {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 65_536];
    loop {
        let count = reader
            .read(&mut buffer)
            .with_context(|| format!("cannot hash Bloodline ledger {}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn verify_locked_file(file: &File, path: &Path) -> Result<LedgerVerification> {
    let mut reader_file = file
        .try_clone()
        .with_context(|| format!("cannot clone ledger handle {}", path.display()))?;
    reader_file
        .seek(SeekFrom::Start(0))
        .with_context(|| format!("cannot seek Bloodline ledger {}", path.display()))?;
    let reader = BufReader::new(reader_file);
    let mut expected_sequence = 1_u64;
    let mut previous_hash: Option<String> = None;

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line.with_context(|| format!("cannot read ledger line {line_number}"))?;
        ensure!(!line.trim().is_empty(), "blank ledger line {line_number}");
        let event: BloodlineEvent = serde_json::from_str(&line)
            .with_context(|| format!("invalid JSON at ledger line {line_number}"))?;
        ensure!(
            event.schema_version == "asi.bloodline.v0.1",
            "unsupported Bloodline schema at line {line_number}: {}",
            event.schema_version
        );
        ensure!(
            !event.run_id.trim().is_empty(),
            "empty run id at ledger line {line_number}"
        );
        ensure!(
            !event.kind.trim().is_empty(),
            "empty event kind at ledger line {line_number}"
        );
        ensure!(
            event.sequence == expected_sequence,
            "ledger sequence mismatch at line {line_number}: expected {expected_sequence}, got {}",
            event.sequence
        );
        ensure!(
            event.previous_hash == previous_hash,
            "ledger previous hash mismatch at line {line_number}"
        );
        let material = HashMaterial {
            schema_version: &event.schema_version,
            sequence: event.sequence,
            timestamp: event.timestamp,
            run_id: &event.run_id,
            kind: &event.kind,
            payload: &event.payload,
            previous_hash: &event.previous_hash,
        };
        let expected_hash = sha256_hex(
            &serde_json::to_vec(&material).context("cannot encode verification material")?,
        );
        if event.integrity_hash != expected_hash {
            bail!("ledger integrity hash mismatch at line {line_number}");
        }
        previous_hash = Some(event.integrity_hash);
        expected_sequence += 1;
    }

    Ok(LedgerVerification {
        valid: true,
        events: expected_sequence - 1,
        last_hash: previous_hash,
        path: path.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hash_chain_detects_tampering() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let path = directory.path().join("bloodline.jsonl");
        {
            let mut ledger = BloodlineLedger::open(&path).expect("ledger should open");
            ledger
                .append("run-1", "run.created", json!({"task": "abc"}))
                .expect("first append should work");
            ledger
                .append("run-1", "run.completed", json!({"ok": true}))
                .expect("second append should work");
        }
        let verification = BloodlineLedger::verify(&path).expect("chain should verify");
        assert_eq!(verification.events, 2);

        let contents = std::fs::read_to_string(&path).expect("ledger should be readable");
        std::fs::write(&path, contents.replace("run.completed", "run.corrupted"))
            .expect("tamper write should work");
        let error = BloodlineLedger::verify(&path).expect_err("tampering should fail");
        assert!(error.to_string().contains("integrity hash mismatch"));
    }

    #[cfg(unix)]
    #[test]
    fn ledger_rejects_symlink_redirection() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let outside = directory.path().join("outside.txt");
        std::fs::write(&outside, "unchanged").expect("outside fixture should write");
        let ledger_path = directory.path().join("bloodline.jsonl");
        symlink(&outside, &ledger_path).expect("symlink should be created");

        assert!(BloodlineLedger::open(&ledger_path).is_err());
        assert_eq!(
            std::fs::read_to_string(outside).expect("outside fixture should read"),
            "unchanged"
        );
    }

    #[test]
    fn verification_does_not_create_missing_parent_directories() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let parent = directory.path().join("missing");
        let ledger = parent.join("bloodline.jsonl");
        assert!(BloodlineLedger::verify(&ledger).is_err());
        assert!(!parent.exists());
    }
}
