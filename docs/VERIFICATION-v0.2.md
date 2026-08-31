# Verification record: v0.2 hardened worker tranche

Date: 25 August 2026  
Environment: Linux x86_64, Bubblewrap 0.11.2, Rust 1.98.0 stable, Node.js 26.7.0

## Current verified evidence

| Layer | Evidence | Result |
|---|---|---:|
| Rust formatting | `cargo fmt --all -- --check` | pass |
| Static quality | all targets/features under Clippy `-D warnings` | pass |
| Rust behavior | 25 unit tests | 25 pass, 0 fail |
| Isolation | Bubblewrap plan, missing/spoofed enforcer, unsafe bypass, cleared environment, secret-safe diagnostics, and write-escape controls | pass |
| Plan/genome binding | missing/wrong approval and missing genome negatives; current signed genome and worker fingerprint required | pass |
| Signed Bloodline | Ed25519 positive verification plus ledger, checkpoint, and wrong-key negatives | pass |
| Signed genome | descriptor/executable fingerprints plus descriptor and wrong-key negatives | pass |
| Schema contracts | emitted descriptors, isolation report, checkpoint, genome, SkillSpec, review records, and disposition validated | pass |
| Release guard | license-only negative plus review/gates/notices/SBOM/approval/reproducibility blockers before `dist/` | pass |
| Hermes install | pinned commit, exact uv/Python, clean source, locked dependencies, direct executable, top-level/chat help, isolated plan | pass |
| Live worker | real Codex subprocess under Bubblewrap returned the requested marker through approved-plan + signed-genome chain | pass |
| Independent review | five fresh role-specific reviews plus machine-reconciled disposition | complete; adoption/public release blocked |

The final live Codex smoke returned `ASI_V02_CHAIN_OK` in 5.745 seconds. Execution matched inspected plan `7cf6b71da918d479b0b9c8d53c41f286d0affe5f9a2a905d47187bf8558561ac`, verified signed-genome digest `b7b9c5b92b6c4eccf9b8f971adc81874088d801adce345e58a67328926af5faa`, rechecked Codex executable digest `bbc3341e44c9ead340ed9570c17be936e37870f570751a941699ffd04d672827`, and reported Bubblewrap enforcement, read-only host root, ephemeral `/tmp` and `.codex` state, isolated namespaces, and shared/unmediated provider network. The four-event Bloodline verified afterward. Temporary signing material was removed. This proves one real provider-backed adapter path on this machine; it does not establish every adapter/provider combination.

Hermes Agent reports `Hermes Agent v0.20.0 (2026.8.3)` from commit `b9aa9289a8083f2e9d248ad6837b2938f5ee92d7`. The installer re-ran successfully with exactly `uv 0.12.5 (x86_64-unknown-linux-musl)` and CPython `3.13.15`; current `chat --help` contains every adapter flag and ASI generated an isolated digest-bound Hermes plan. No Hermes provider inference was attempted because provider credentials/model choice are operator-owned and full Hermes conformance remains a release blocker.

The independent verdicts were four **approve with conditions** and one CAIO **do not approve**, with no critical and 18 high findings. Code-level traversal, plan/genome binding, fail-closed versioning, gate-oracle, schema, concurrency, diagnostics, and release-oracle findings were repaired. Research validity, full adapter conformance, host-read confidentiality, mediated egress, organizational operations, and public-service controls remain scope blockers. Review completion therefore did not authorize adoption or publication.

## Reproduction

Use the explicit toolchain paths in `GATES.md` on this host, then run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --locked

node scripts/acceptance.mjs isolation
node scripts/acceptance.mjs policy
node scripts/acceptance.mjs bloodline
node scripts/acceptance.mjs lineage
node scripts/acceptance.mjs genome
node scripts/acceptance.mjs skills
node scripts/acceptance.mjs release
node scripts/acceptance.mjs hermes
node scripts/acceptance.mjs reviews
node scripts/acceptance.mjs summary

# Machine-local, authenticated provider smoke (not run in CI):
node scripts/live-codex-smoke.mjs
```

The CI workflow repeats portable Rust and black-box checks on Ubuntu. Machine-specific discovery assumes the local direct installations described in `doctor`; CI does not pretend those third-party CLIs are preinstalled.

## Negative controls

- Policy rejects `workspace-write` before adapter selection.
- Option-like task text does not alter Pi or isolation-fixture options and is redacted publicly.
- Unknown and unsupported harness versions fail before planning; version probes do not inherit an ambient secret variable.
- Required isolation with no discoverable `bwrap` fails.
- A user-owned executable named `bwrap` cannot impersonate the trusted enforcer.
- `--isolation off` without `--acknowledge-unsafe-subprocess` fails.
- The isolated subprocess does not inherit an ambient secret, cannot create `.asi-isolation-escape`, and can write ephemeral `/tmp`.
- Failed-process stderr is represented by a digest rather than returned raw.
- Execution without an approved plan, with a wrong plan digest, or without the pinned signed genome fails before Bloodline creation.
- Bloodline chain verification rejects an edited event.
- Signed checkpoint verification rejects edited ledger bytes, edited material, and another key.
- Signed genome verification rejects edited descriptors, another key, and current component drift.
- Existing private/signature output paths are never overwritten.
- Direct and nested symlinked Skill sources are rejected; risky skills remain quarantined and non-executable.
- A non-empty placeholder license alone cannot unlock release; packaging exits before creating `dist/`.
- A malformed emitted document fails the local JSON Schema conformance validator.
- Every independent critical/high finding must have one schema-valid resolved or release-blocking disposition.

## Claim limits

This record does not prove confidentiality from a worker, destination-scoped networking, credential safety, syscall confinement, CPU/memory isolation, multi-tenant safety, reproducible builds across platforms, formal correctness, or general intelligence. Provider egress and host read visibility remain explicit. Ed25519 proves signer continuity only; private-key theft or trusted-key replacement defeats that trust model.

Even after engineering gates pass, the release status remains blocked by the CAIO verdict, scope-blocked high findings, missing owner license decision, notices, SBOM, protected release approval, and reproducibility evidence. v0.2 is development-only in a disposable, secret-free single-user environment; it is not approved for organizational adoption, public distribution/service, sensitive data, persistent/unattended operation, multi-user use, or write/external authority.
