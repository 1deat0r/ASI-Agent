# ASI Agent build plan — visceral nucleus

Status: V0.1 COMPLETE; V0.2 ENGINEERING COMPLETE; SOURCE PUBLIC UNDER `MIT OR Apache-2.0` (ratified 2026-08-31) — ADOPTION, PACKAGING, AND FORMAL RELEASES REMAIN BLOCKED  
Mode: Solo depth tree; one coherent vertical slice, no concurrent file ownership.  
Toolchain: Rust 1.98.0 via explicit stable toolchain paths; Node.js for black-box acceptance checks.  
Default authority: no tools or read-only only. Workspace writes and external side effects are denied.

## Root contract

Build a runnable `asi` CLI whose sovereign kernel owns policy, task identity, adapter selection, execution budgets, skill quarantine, and the append-only Bloodline ledger. Existing harnesses remain replaceable untrusted workers. No adapter may own ASI identity, durable memory, policy, audit, or promotion state.

Shared contracts:

- `TaskEnvelope`: versioned task, run identity, requested effect, deadline, budget, and working directory.
- `HarnessDescriptor`: executable identity, detected version, supported effects, isolation claims, and adapter status.
- `InvocationPlan`: exact program, arguments, working directory, timeout, and effective containment; secrets are never serialized.
- `PolicyDecision`: allow or deny with stable reason code; v0.1 denies workspace writes and external side effects.
- `BloodlineEvent`: sequence-numbered JSONL event with previous hash and SHA-256 integrity hash.
- `SkillSpec`: normalized provenance, metadata, content hash, risk findings, lineage, and mandatory `quarantined` state.

Error convention: user-caused policy/configuration failures return concise errors and a non-zero exit; internal errors preserve context without printing credentials. JSON modes write machine-readable stdout and diagnostics to stderr.

## Depth tree

1. ASI Agent visceral nucleus — COMPLETE
   1.1. Contracts and sovereign policy — COMPLETE
        Owns: `src/domain.rs`, `src/policy.rs`, `specs/*.json`
   1.2. Harness genome and adapters — COMPLETE
        Owns: `src/harness.rs`, `src/registry.rs`
   1.3. Bloodline ledger and execution runtime — COMPLETE
        Owns: `src/bloodline.rs`, `src/runtime.rs`
   1.4. Skill Crypt and normalized SkillSpec — COMPLETE
        Owns: `src/skill.rs`, `tests/fixtures/skills/**`
   1.5. CLI and black-box acceptance — COMPLETE
        Owns: `src/main.rs`, `src/lib.rs`, `scripts/acceptance.mjs`, integration tests
   1.6. Draft 0.3 and operator documentation — COMPLETE
        Owns: `ASI-Agent-Astronomical-Plan.md`, `README.md`, `docs/**`
   1.7. Root integration and adversarial verification — COMPLETE
        Owns: quality fixes within declared root scope and final evidence

## Dependency order

`contracts → policy → adapters → runtime/ledger → skill quarantine → CLI → live-safe smoke → documentation → final re-verification`

## Explicit non-goals for v0.1

- no autonomous self-modification or automatic promotion;
- no execution of imported skill scripts;
- no claim that a subprocess adapter is itself a complete security boundary;
- no workspace writes, external communications, purchases, publication, replication, or unrestricted network action;
- no multi-tenant service, autonomous scheduler, or hidden durable memory;
- no claim of AGI or ASI.

## Completion rule

The nucleus is complete only when every gate in `GATES.md` is re-run successfully, policy-negative controls fail closed, the Bloodline hash chain verifies after real CLI execution, and the documentation states the remaining containment limitations plainly.

## V0.2 hardened worker tranche

Root contract: every subprocess execution is wrapped by a required Linux Bubblewrap profile unless the operator supplies both `--isolation off` and an explicit unsafe acknowledgement. The profile enforces a read-only host filesystem, writable ephemeral `/tmp`, user/process/IPC/UTS/cgroup isolation, parent-death termination, dropped Linux capabilities, and disabled nested user namespaces. Provider network egress and read visibility of the host remain explicit residual risks; v0.2 must not call this complete confidentiality or network containment.

Shared contracts:

- `IsolationMode` and `IsolationReport`: requested mode, actual enforcer, filesystem posture, namespace posture, and network posture.
- `LineageKey`: locally generated Ed25519 signing identity; private material is mode `0600`, never serialized to CLI output, and never overwritten.
- `SignedCheckpoint`: ledger byte digest, event count, terminal event hash, time, key id, and detached Ed25519 signature verified against an explicit public-key file.
- `SignedGenome`: canonical harness descriptors, versions, direct executable fingerprints, key id, and detached signature.
- Direct harness resolution: explicit environment override, known direct `mise` installation, then PATH fallback; state-mutating launcher wrappers are not preferred.

Depth tree:

1. ASI Agent v0.2 hardened worker tranche — COMPLETE (development-only; adoption and public release blocked)
   1.1. Bubblewrap isolation contract and subprocess fixture — COMPLETE
        Owns: `src/isolation.rs`, isolation fields in domain/runtime/harness/CLI, acceptance isolation checks
   1.2. Ed25519 lineage keys and signed Bloodline checkpoints — COMPLETE
        Owns: `src/lineage.rs`, ledger checkpoint CLI and tests
   1.3. Signed harness genome and direct executable fingerprints — COMPLETE
        Owns: `src/genome.rs`, harness discovery extensions, genome CLI and tests
   1.4. CI and fail-closed release packaging — COMPLETE
        Owns: `.github/workflows/**`, release scripts, `docs/LICENSING.md`
   1.5. Pinned Hermes source installation and adapter health — COMPLETE
        Owns: user-local Hermes runtime plus reproducible bootstrap script; no upstream source edits
   1.6. Documentation and fresh five-role independent review — COMPLETE
        Owns: `README.md`, `docs/**`, `ASI-Agent-Astronomical-Plan.md`
   1.7. Root adversarial integration and re-verification — COMPLETE
        Owns: final fixes within root scope and `GATES.md` evidence

Dependency order:

`isolation + signing primitives → signed genome → Hermes behind isolation → release/docs → independent review → remediation → final gates`

V0.2 non-goals:

- no claim that Bubblewrap mediates provider traffic or hides host-readable secrets;
- no workspace writes or external-effect authority;
- no automatic key rotation, remote transparency service, promotion, or rollback engine;
- no public release in v0.2 (the owner ratified `MIT OR Apache-2.0` after the v0.2 tag, 2026-08-31; formal releases remain gated by `scripts/release-readiness.mjs`);
- no autonomous self-improvement, AGI, or ASI claim.

Independent review outcome: four reviewers issued “approve with conditions” and the CAIO issued “do not approve.” The integrated disposition therefore permits only development in a dedicated, disposable, non-sensitive local environment. Adoption, public distribution or service, persistent or unattended use, multi-user operation, sensitive-data handling, and write or external-effect authority remain blocked until the recorded high-severity conditions are resolved and independently re-reviewed.

## V0.3 decision status (recorded 2026-08-31)

- **D1 — License: RESOLVED.** Owner ratified `MIT OR Apache-2.0` on
  2026-08-31; see `docs/LICENSING.md`. The owner component of blocker
  `CAIO-H4` is closed; the remaining release-readiness controls (notices,
  SBOM, approving disposition, protected owner approval, reproducibility)
  keep packaging blocked.
- **D2 — Tranche order: RECOMMENDED, NOT YET RATIFIED.** Recommended
  dependency order: confinement (`GE-H1`, `CAIO-H3`, `SA-H1`) → conformance
  (`SE-H1`) → falsifiable research (`RS-H1..H3`, `SE-H2`, `GE-H2`). Rationale:
  live-provider conformance with unmediated egress and a readable host root
  contradicts the threat model, and the charter defines conformance as the
  entry condition for comparative research. Legitimate alternative: descope
  confinement and proceed straight to research under the disposable-VM
  posture, permanently capping confidentiality claims.
- **D3 — Horizon-0 ratifications: DEFERRED.** Persona, three target
  workflows, and budget/SLO ratification are required only before the
  research tranche starts (`ASI-Agent-Astronomical-Plan.md`).

Repository: public at `github.com/1deat0r/ASI-Agent` (source publication
only; releases and adoption remain blocked as recorded above).
