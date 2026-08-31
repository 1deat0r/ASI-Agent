use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use crate::bloodline::BloodlineLedger;
use crate::domain::sha256_hex;
use crate::fs_guard::{secure_existing_path, secure_output_path};

const MAX_LINEAGE_DOCUMENT_BYTES: u64 = 1_048_576;

#[derive(Deserialize, Serialize)]
struct SecretKeyDocument {
    schema_version: String,
    algorithm: String,
    key_id: String,
    public_key_hex: String,
    secret_key_hex: String,
    created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicKeyDocument {
    pub schema_version: String,
    pub algorithm: String,
    pub key_id: String,
    pub public_key_hex: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyGenerationOutcome {
    pub schema_version: String,
    pub algorithm: String,
    pub key_id: String,
    pub private_key_path: PathBuf,
    pub public_key_path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckpointMaterial {
    pub schema_version: String,
    pub ledger_sha256: String,
    pub events: u64,
    pub last_event_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedCheckpoint {
    pub material: CheckpointMaterial,
    pub algorithm: String,
    pub key_id: String,
    pub signature_hex: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckpointVerification {
    pub valid: bool,
    pub key_id: String,
    pub ledger_sha256: String,
    pub events: u64,
    pub last_event_hash: Option<String>,
    pub checkpoint_path: PathBuf,
}

pub fn generate_keypair(
    private_key_path: impl AsRef<Path>,
    public_key_path: impl AsRef<Path>,
) -> Result<KeyGenerationOutcome> {
    let private_key_path = private_key_path.as_ref().to_path_buf();
    let public_key_path = public_key_path.as_ref().to_path_buf();
    ensure!(
        private_key_path != public_key_path,
        "private and public key paths must differ"
    );
    ensure!(
        !private_key_path.exists(),
        "refusing to overwrite private key {}",
        private_key_path.display()
    );
    ensure!(
        !public_key_path.exists(),
        "refusing to overwrite public key {}",
        public_key_path.display()
    );

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let public_key_hex = hex::encode(verifying_key.as_bytes());
    let key_id = key_id_for(&verifying_key);
    let created_at = Utc::now();
    let secret = SecretKeyDocument {
        schema_version: "asi.lineage-private-key.v0.1".to_owned(),
        algorithm: "Ed25519".to_owned(),
        key_id: key_id.clone(),
        public_key_hex: public_key_hex.clone(),
        secret_key_hex: hex::encode(signing_key.to_bytes()),
        created_at,
    };
    let public = PublicKeyDocument {
        schema_version: "asi.lineage-public-key.v0.1".to_owned(),
        algorithm: "Ed25519".to_owned(),
        key_id: key_id.clone(),
        public_key_hex,
        created_at,
    };

    write_new_json(&private_key_path, &secret, true)?;
    if let Err(error) = write_new_json(&public_key_path, &public, false) {
        return Err(error).context(format!(
            "private key was created at {}; public-key creation failed",
            private_key_path.display()
        ));
    }

    Ok(KeyGenerationOutcome {
        schema_version: "asi.key-generation.v0.1".to_owned(),
        algorithm: "Ed25519".to_owned(),
        key_id,
        private_key_path,
        public_key_path,
    })
}

pub fn create_checkpoint(
    ledger_path: impl AsRef<Path>,
    private_key_path: impl AsRef<Path>,
    checkpoint_path: impl AsRef<Path>,
) -> Result<SignedCheckpoint> {
    let ledger_path = ledger_path.as_ref();
    let checkpoint_path = checkpoint_path.as_ref();
    let snapshot = BloodlineLedger::snapshot(ledger_path)?;
    let signing_key = load_signing_key(private_key_path.as_ref())?;
    let material = CheckpointMaterial {
        schema_version: "asi.bloodline-checkpoint-material.v0.1".to_owned(),
        ledger_sha256: snapshot.sha256,
        events: snapshot.verification.events,
        last_event_hash: snapshot.verification.last_hash,
        created_at: Utc::now(),
    };
    let signed = SignedCheckpoint {
        signature_hex: sign_value(&material, &signing_key)?,
        material,
        algorithm: "Ed25519".to_owned(),
        key_id: key_id_for(&signing_key.verifying_key()),
    };
    write_new_json(checkpoint_path, &signed, false)?;
    Ok(signed)
}

pub fn verify_checkpoint(
    ledger_path: impl AsRef<Path>,
    checkpoint_path: impl AsRef<Path>,
    public_key_path: impl AsRef<Path>,
) -> Result<CheckpointVerification> {
    let ledger_path = ledger_path.as_ref();
    let checkpoint_path = checkpoint_path.as_ref();
    let checkpoint: SignedCheckpoint = read_json(checkpoint_path)?;
    ensure!(
        checkpoint.algorithm == "Ed25519",
        "unsupported signature algorithm"
    );
    ensure!(
        checkpoint.material.schema_version == "asi.bloodline-checkpoint-material.v0.1",
        "unsupported checkpoint material schema"
    );
    let public_document: PublicKeyDocument = read_json(public_key_path.as_ref())?;
    let verifying_key = verifying_key_from_document(&public_document)?;
    ensure!(
        checkpoint.key_id == public_document.key_id,
        "checkpoint key id does not match pinned public key"
    );
    verify_value(
        &checkpoint.material,
        &checkpoint.signature_hex,
        &verifying_key,
    )?;

    let snapshot = BloodlineLedger::snapshot(ledger_path)?;
    let ledger = snapshot.verification;
    let ledger_sha256 = snapshot.sha256;
    ensure!(
        checkpoint.material.ledger_sha256 == ledger_sha256,
        "checkpoint ledger digest mismatch"
    );
    ensure!(
        checkpoint.material.events == ledger.events,
        "checkpoint event count mismatch"
    );
    ensure!(
        checkpoint.material.last_event_hash == ledger.last_hash,
        "checkpoint terminal hash mismatch"
    );

    Ok(CheckpointVerification {
        valid: true,
        key_id: checkpoint.key_id,
        ledger_sha256,
        events: ledger.events,
        last_event_hash: ledger.last_hash,
        checkpoint_path: checkpoint_path.to_path_buf(),
    })
}

pub(crate) fn load_signing_key(path: &Path) -> Result<SigningKey> {
    ensure_private_mode(path)?;
    let document: SecretKeyDocument = read_json(path)?;
    ensure!(
        document.schema_version == "asi.lineage-private-key.v0.1",
        "unsupported private-key schema"
    );
    ensure!(
        document.algorithm == "Ed25519",
        "unsupported private-key algorithm"
    );
    let secret = decode_fixed::<32>(&document.secret_key_hex, "private key")?;
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    ensure!(
        hex::encode(verifying_key.as_bytes()) == document.public_key_hex,
        "private-key document public key mismatch"
    );
    ensure!(
        key_id_for(&verifying_key) == document.key_id,
        "private-key document key id mismatch"
    );
    let _ = document.created_at;
    Ok(signing_key)
}

pub(crate) fn load_verifying_key(path: &Path) -> Result<(PublicKeyDocument, VerifyingKey)> {
    let document: PublicKeyDocument = read_json(path)?;
    let verifying_key = verifying_key_from_document(&document)?;
    Ok((document, verifying_key))
}

pub(crate) fn sign_value<T: Serialize>(value: &T, key: &SigningKey) -> Result<String> {
    let encoded = serde_json::to_vec(value).context("cannot canonicalize signed material")?;
    Ok(hex::encode(key.sign(&encoded).to_bytes()))
}

pub(crate) fn verify_value<T: Serialize>(
    value: &T,
    signature_hex: &str,
    key: &VerifyingKey,
) -> Result<()> {
    let encoded = serde_json::to_vec(value).context("cannot canonicalize signed material")?;
    let signature_bytes = decode_fixed::<64>(signature_hex, "signature")?;
    let signature = Signature::from_bytes(&signature_bytes);
    key.verify(&encoded, &signature)
        .context("Ed25519 signature verification failed")
}

pub(crate) fn key_id_for(key: &VerifyingKey) -> String {
    format!("ed25519:{}", sha256_hex(key.as_bytes()))
}

pub(crate) fn sha256_file(path: &Path) -> Result<String> {
    let mut file =
        File::open(path).with_context(|| format!("cannot open artifact {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 65_536];
    loop {
        let count = file
            .read(&mut buffer)
            .with_context(|| format!("cannot read artifact {}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub(crate) fn write_new_json<T: Serialize>(path: &Path, value: &T, private: bool) -> Result<()> {
    let path = secure_output_path(path, "lineage")?;
    let content = serde_json::to_vec_pretty(value).context("cannot encode lineage document")?;
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(if private { 0o600 } else { 0o644 })
            .custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options
        .open(&path)
        .with_context(|| format!("refusing to overwrite lineage document {}", path.display()))?;
    file.write_all(&content)
        .and_then(|()| file.write_all(b"\n"))
        .and_then(|()| file.sync_all())
        .with_context(|| format!("cannot write lineage document {}", path.display()))
}

pub(crate) fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(read_json_with_sha256(path)?.0)
}

pub(crate) fn read_json_with_sha256<T: DeserializeOwned>(path: &Path) -> Result<(T, String)> {
    let path = secure_existing_path(path, "lineage")?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options
        .open(&path)
        .with_context(|| format!("cannot open lineage document {}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(MAX_LINEAGE_DOCUMENT_BYTES + 1)
        .read_to_end(&mut bytes)
        .with_context(|| format!("cannot read lineage document {}", path.display()))?;
    ensure!(
        bytes.len() as u64 <= MAX_LINEAGE_DOCUMENT_BYTES,
        "lineage document exceeds {MAX_LINEAGE_DOCUMENT_BYTES} bytes"
    );
    let sha256 = sha256_hex(&bytes);
    let value = serde_json::from_slice(&bytes)
        .with_context(|| format!("invalid lineage document {}", path.display()))?;
    Ok((value, sha256))
}

fn verifying_key_from_document(document: &PublicKeyDocument) -> Result<VerifyingKey> {
    ensure!(
        document.schema_version == "asi.lineage-public-key.v0.1",
        "unsupported public-key schema"
    );
    ensure!(
        document.algorithm == "Ed25519",
        "unsupported public-key algorithm"
    );
    let public = decode_fixed::<32>(&document.public_key_hex, "public key")?;
    let verifying_key = VerifyingKey::from_bytes(&public).context("invalid Ed25519 public key")?;
    ensure!(
        key_id_for(&verifying_key) == document.key_id,
        "public-key document key id mismatch"
    );
    Ok(verifying_key)
}

fn decode_fixed<const N: usize>(encoded: &str, label: &str) -> Result<[u8; N]> {
    let bytes = hex::decode(encoded).with_context(|| format!("invalid {label} encoding"))?;
    bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid {label} length"))
}

#[cfg(unix)]
fn ensure_private_mode(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = path
        .symlink_metadata()
        .with_context(|| format!("cannot stat private key {}", path.display()))?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "private key must be a regular non-symlink file"
    );
    let mode = metadata.permissions().mode() & 0o777;
    ensure!(
        mode & 0o077 == 0,
        "private key {} must not be group/world accessible (mode {mode:o})",
        path.display()
    );
    Ok(())
}

#[cfg(not(unix))]
fn ensure_private_mode(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn signed_checkpoint_rejects_ledger_tampering() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let private = directory.path().join("private.json");
        let public = directory.path().join("public.json");
        generate_keypair(&private, &public).expect("keys should generate");
        let ledger_path = directory.path().join("bloodline.jsonl");
        {
            let mut ledger = BloodlineLedger::open(&ledger_path).expect("ledger should open");
            ledger
                .append("run", "run.created", json!({"ok": true}))
                .expect("event should append");
        }
        let checkpoint = directory.path().join("checkpoint.json");
        create_checkpoint(&ledger_path, &private, &checkpoint)
            .expect("checkpoint should be created");
        verify_checkpoint(&ledger_path, &checkpoint, &public).expect("checkpoint should verify");

        let content = std::fs::read_to_string(&ledger_path).expect("ledger should read");
        std::fs::write(&ledger_path, content.replace("run.created", "run.changed"))
            .expect("tamper fixture should write");
        assert!(verify_checkpoint(&ledger_path, &checkpoint, &public).is_err());
    }

    #[test]
    fn private_key_is_not_overwritten() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let private = directory.path().join("private.json");
        let public = directory.path().join("public.json");
        generate_keypair(&private, &public).expect("keys should generate");
        let second_public = directory.path().join("public-2.json");
        let error = generate_keypair(&private, &second_public)
            .expect_err("existing private key should fail");
        assert!(error.to_string().contains("refusing to overwrite"));
    }
}
