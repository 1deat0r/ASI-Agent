use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::harness::HarnessDescriptor;
use crate::lineage::{
    load_signing_key, load_verifying_key, read_json_with_sha256, sha256_file, sign_value,
    verify_value, write_new_json,
};
use crate::registry::HarnessRegistry;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GenomeEntry {
    pub descriptor: HarnessDescriptor,
    pub installed: bool,
    pub executable_path: Option<PathBuf>,
    pub executable_source: Option<String>,
    pub version: Option<String>,
    pub executable_sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenomeMaterial {
    pub schema_version: String,
    pub generated_at: DateTime<Utc>,
    pub entries: Vec<GenomeEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedGenome {
    pub material: GenomeMaterial,
    pub algorithm: String,
    pub key_id: String,
    pub signature_hex: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenomeVerification {
    pub(crate) valid: bool,
    pub(crate) current_state_matches: bool,
    pub(crate) key_id: String,
    pub(crate) entries: usize,
    pub(crate) generated_at: DateTime<Utc>,
    pub(crate) genome_path: PathBuf,
    pub(crate) genome_sha256: String,
}

impl GenomeVerification {
    #[must_use]
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    #[must_use]
    pub fn genome_sha256(&self) -> &str {
        &self.genome_sha256
    }

    #[must_use]
    pub fn generated_at(&self) -> DateTime<Utc> {
        self.generated_at
    }
}

pub fn sign_genome(
    private_key_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<SignedGenome> {
    let signing_key = load_signing_key(private_key_path.as_ref())?;
    let material = GenomeMaterial {
        schema_version: "asi.harness-genome-material.v0.1".to_owned(),
        generated_at: Utc::now(),
        entries: capture_entries()?,
    };
    let genome = SignedGenome {
        signature_hex: sign_value(&material, &signing_key)?,
        material,
        algorithm: "Ed25519".to_owned(),
        key_id: crate::lineage::key_id_for(&signing_key.verifying_key()),
    };
    write_new_json(output_path.as_ref(), &genome, false)?;
    Ok(genome)
}

pub fn verify_genome(
    genome_path: impl AsRef<Path>,
    public_key_path: impl AsRef<Path>,
) -> Result<GenomeVerification> {
    let genome_path = genome_path.as_ref();
    let (genome, genome_sha256): (SignedGenome, String) = read_json_with_sha256(genome_path)?;
    ensure!(
        genome.material.schema_version == "asi.harness-genome-material.v0.1",
        "unsupported harness-genome schema"
    );
    ensure!(
        genome.algorithm == "Ed25519",
        "unsupported signature algorithm"
    );
    let (public_document, verifying_key) = load_verifying_key(public_key_path.as_ref())?;
    ensure!(
        genome.key_id == public_document.key_id,
        "genome key id does not match pinned public key"
    );
    verify_value(&genome.material, &genome.signature_hex, &verifying_key)?;

    let current = capture_entries()?;
    let drift = describe_drift(&genome.material.entries, &current);
    ensure!(
        drift.is_empty(),
        "signed genome does not match current harness state: {}",
        drift.join(", ")
    );

    Ok(GenomeVerification {
        valid: true,
        current_state_matches: true,
        key_id: genome.key_id,
        entries: genome.material.entries.len(),
        generated_at: genome.material.generated_at,
        genome_path: genome_path.to_path_buf(),
        genome_sha256,
    })
}

fn capture_entries() -> Result<Vec<GenomeEntry>> {
    let mut entries = HarnessRegistry
        .discover()
        .into_iter()
        .map(|detected| {
            let executable_sha256 = detected
                .executable_path
                .as_deref()
                .map(sha256_file)
                .transpose()
                .with_context(|| {
                    format!(
                        "cannot fingerprint harness executable for {}",
                        detected.descriptor.id
                    )
                })?;
            Ok(GenomeEntry {
                descriptor: detected.descriptor,
                installed: detected.installed,
                executable_path: detected.executable_path,
                executable_source: detected.executable_source,
                version: detected.version,
                executable_sha256,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    entries.sort_by(|left, right| left.descriptor.id.cmp(&right.descriptor.id));
    Ok(entries)
}

fn describe_drift(expected: &[GenomeEntry], current: &[GenomeEntry]) -> Vec<String> {
    let mut drift = Vec::new();
    for id in expected
        .iter()
        .chain(current)
        .map(|entry| entry.descriptor.id.as_str())
    {
        if drift.iter().any(|existing: &String| existing == id) {
            continue;
        }
        let expected_entry = expected.iter().find(|entry| entry.descriptor.id == id);
        let current_entry = current.iter().find(|entry| entry.descriptor.id == id);
        if expected_entry != current_entry {
            drift.push(id.to_owned());
        }
    }
    drift
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lineage::generate_keypair;

    #[test]
    fn signed_genome_rejects_tampering_and_wrong_key() {
        let directory = tempfile::tempdir().expect("tempdir should work");
        let private = directory.path().join("private.json");
        let public = directory.path().join("public.json");
        generate_keypair(&private, &public).expect("keys should generate");
        let genome_path = directory.path().join("genome.json");
        sign_genome(&private, &genome_path).expect("genome should sign");
        verify_genome(&genome_path, &public).expect("genome should verify");

        let wrong_private = directory.path().join("wrong-private.json");
        let wrong_public = directory.path().join("wrong-public.json");
        generate_keypair(&wrong_private, &wrong_public).expect("wrong keys should generate");
        assert!(verify_genome(&genome_path, &wrong_public).is_err());

        let mut value: serde_json::Value =
            crate::lineage::read_json(&genome_path).expect("genome fixture should parse");
        value["material"]["entries"][0]["descriptor"]["display_name"] =
            serde_json::Value::String("tampered".to_owned());
        std::fs::write(
            &genome_path,
            serde_json::to_vec_pretty(&value).expect("fixture should encode"),
        )
        .expect("fixture should write");
        assert!(verify_genome(&genome_path, &public).is_err());
    }
}
