//! ASI Agent's first sovereign, measurable meta-harness nucleus.

pub mod bloodline;
pub mod domain;
mod fs_guard;
pub mod genome;
pub mod harness;
pub mod isolation;
pub mod lineage;
pub mod policy;
pub mod registry;
pub mod runtime;
pub mod skill;

pub use bloodline::{BloodlineLedger, LedgerVerification};
pub use domain::{EffectClass, IsolationMode, IsolationReport, TaskEnvelope};
pub use genome::{
    GenomeEntry, GenomeMaterial, GenomeVerification, SignedGenome, sign_genome, verify_genome,
};
pub use lineage::{
    CheckpointVerification, KeyGenerationOutcome, SignedCheckpoint, create_checkpoint,
    generate_keypair, verify_checkpoint,
};
pub use policy::{PolicyDecision, PolicyEngine};
pub use registry::HarnessRegistry;
pub use runtime::{RunOutcome, SovereignRuntime};
pub use skill::{AssimilationOutcome, SkillInspector, SkillReport, SkillSpec};
