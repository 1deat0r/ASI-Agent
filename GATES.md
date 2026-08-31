# Gates: ASI Agent visceral nucleus and hardened worker tranche

OWNS: GATES.md, PLAN.md, Cargo.toml, Cargo.lock, src/, tests/, scripts/, specs/, docs/, README.md, .gitignore, ASI-Agent-Astronomical-Plan.md

Scope: Preserve the verified v0.1 nucleus, then add the v0.2 OS-enforced worker boundary, signed Bloodline/genome lineage, reproducible release scaffolding, Hermes source installation, and fresh five-role independent review.

- [x] G1: The Rust nucleus is formatted, warning-free under strict Clippy, and fully tested.
  CHECK: env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo fmt --check && env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo clippy --all-targets --all-features -- -D warnings && env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo test --all-targets && printf '%s\\n' 'rust quality verification passed'
  EXPECT: rust quality verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=Running unittests src/lib.rs (target/debug/deps/asi_agent-42e123886d40ea5a) | Running unittests src/main.rs (target/debug/deps/asi-50c09c4d3fbc6a4e)

- [x] G2: The CLI discovers installed harnesses and reports their containment capabilities without granting them authority.
  CHECK: env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --quiet && node scripts/acceptance.mjs doctor && printf '%s\\n' 'harness discovery verification passed'
  EXPECT: harness discovery verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=doctor acceptance passed | harness discovery verification passed\n

- [x] G3: Policy denies unsupported write/external effects, while deterministic no-tool and read-only plans remain inspectable.
  CHECK: node scripts/acceptance.mjs policy && printf '%s\\n' 'policy boundary verification passed'
  EXPECT: policy boundary verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=policy acceptance passed | policy boundary verification passed\n

- [x] G4: A deterministic construct requires an approved plan and signed current genome, then executes end to end, produces a hash-chained Bloodline ledger, and passes independent verification.
  CHECK: node scripts/acceptance.mjs bloodline && printf '%s\\n' 'bloodline verification passed'
  EXPECT: bloodline verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=bloodline acceptance passed | bloodline verification passed\n

- [x] G5: Third-party skills are treated as untrusted data, symlink traversal is rejected, risky bootstrap instructions are detected, and assimilation creates a schema-valid quarantined SkillSpec without execution.
  CHECK: node scripts/acceptance.mjs skills && printf '%s\\n' 'skill crypt verification passed'
  EXPECT: skill crypt verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=skill acceptance passed | skill crypt verification passed\n

- [x] G6: Draft 0.3 and the engineering documentation define the Visceral Architecture, harness/skill assimilation, single-authority rule, current limitations, and runnable operator workflow.
  CHECK: test -s README.md && test -s docs/ARCHITECTURE.md && test -s specs/harness-adapter.schema.json && test -s specs/skill-spec.schema.json && rg -q 'Draft 0.3' ASI-Agent-Astronomical-Plan.md && rg -q 'Visceral Architecture' ASI-Agent-Astronomical-Plan.md && rg -q 'Bloodline' docs/ARCHITECTURE.md && rg -q 'No foreign capability reaches the Heart unexamined' README.md && rg -q 'not a security sandbox' README.md && printf '%s\\n' 'documentation verification passed'
  EXPECT: documentation verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=documentation verification passed\n

- [x] G7: The source/schema inventory contains no unfinished markers and all sixteen gate identifiers are uniquely represented; v0.2 completion is decided only by G16.
  CHECK: node scripts/acceptance.mjs summary && printf '%s\\n' 'release evidence verification passed'
  EXPECT: release evidence verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=summary acceptance passed | release evidence verification passed\n

- [x] G8: The expanded Rust nucleus remains formatted, warning-free under strict Clippy, and fully tested.
  CHECK: env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo fmt --check && env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo clippy --all-targets --all-features -- -D warnings && env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo test --all-targets --all-features && printf '%s\\n' 'v0.2 rust quality verification passed'
  EXPECT: v0.2 rust quality verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=Running unittests src/lib.rs (target/debug/deps/asi_agent-42e123886d40ea5a) | Running unittests src/main.rs (target/debug/deps/asi-50c09c4d3fbc6a4e)

- [x] G9: Executed subprocess harnesses require trusted Bubblewrap by default, clear ambient secrets, resist option injection, suppress raw failure stderr, cannot write the workspace, and disclose residual read/network authority.
  CHECK: env RUSTC=/home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc /home/mustbearnold/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --quiet && node scripts/acceptance.mjs isolation && printf '%s\\n' 'worker isolation verification passed'
  EXPECT: worker isolation verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=isolation acceptance passed | worker isolation verification passed\n

- [x] G10: An Ed25519 key can create a checkpoint from one locked Bloodline snapshot, while ledger tampering, checkpoint tampering, and the wrong public key all fail verification.
  CHECK: node scripts/acceptance.mjs lineage && printf '%s\\n' 'signed Bloodline verification passed'
  EXPECT: signed Bloodline verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=lineage acceptance passed | signed Bloodline verification passed\n

- [x] G11: The harness genome can be signed and verified with component fingerprints, while descriptor tampering and the wrong public key fail closed.
  CHECK: node scripts/acceptance.mjs genome && printf '%s\\n' 'signed genome verification passed'
  EXPECT: signed genome verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=genome acceptance passed | signed genome verification passed\n

- [x] G12: CI and release scaffolding reproduce portable checks and fail closed before `dist/` unless licensing, reviews, gates, notices, SBOM, protected approval, and reproducibility controls all pass.
  CHECK: node scripts/acceptance.mjs release && printf '%s\\n' 'release scaffolding verification passed'
  EXPECT: release scaffolding verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=release acceptance passed | release scaffolding verification passed\n

- [x] G13: Hermes Agent is installed from the pinned clean local source checkout, is directly discoverable without a state-mutating wrapper, and passes its non-interactive CLI health check.
  CHECK: node scripts/acceptance.mjs hermes && printf '%s\\n' 'Hermes integration verification passed'
  EXPECT: Hermes integration verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=hermes acceptance passed | Hermes integration verification passed\n

- [x] G14: Operator, architecture, threat, release, and verification documents state the enforced boundary, signature trust model, licensing blocker, and remaining credential/network risks without overclaiming.
  CHECK: test -s docs/ARCHITECTURE.md && test -s docs/THREAT-MODEL.md && test -s docs/VERIFICATION-v0.2.md && test -s docs/LICENSING.md && rg -q 'Bubblewrap' README.md docs/ARCHITECTURE.md docs/THREAT-MODEL.md && rg -q 'Ed25519' README.md docs/ARCHITECTURE.md docs/THREAT-MODEL.md && rg -q 'owner decision' docs/LICENSING.md && rg -q 'provider egress remains unmediated' docs/THREAT-MODEL.md && printf '%s\\n' 'v0.2 documentation verification passed'
  EXPECT: v0.2 documentation verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=v0.2 documentation verification passed\n

- [x] G15: Five mutually independent role reviews inspect Draft 0.3 and v0.2, and a schema-valid integrated disposition resolves every critical/high finding or explicitly blocks adoption and release.
  CHECK: node scripts/acceptance.mjs reviews && printf '%s\\n' 'independent review verification passed'
  EXPECT: independent review verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=reviews acceptance passed | independent review verification passed\n

- [x] G16: Final v0.2 verification finds sixteen met gates with no unmet or abandoned work and no unfinished source markers.
  CHECK: node scripts/acceptance.mjs summary-v02 && printf '%s\\n' 'v0.2 release evidence verification passed'
  EXPECT: v0.2 release evidence verification passed
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/run/media/mustbearnold/Projects/AI Agents/ASI-Agent; path=376930f8d5a4/32 entries; output=v0.2 summary acceptance passed | v0.2 release evidence verification passed\n

## Post-tag addendum: license ratification (2026-08-31, owner decision)

The v0.2 tag `v0.2-engineering-complete` (commit `01b3704`) freezes the
verified engineering state. After that tag, the owner ratified
`MIT OR Apache-2.0` (`docs/LICENSING.md`, `LICENSE`, `LICENSE-MIT`,
`LICENSE-APACHE`) and directed public source publication. The release
gate oracle was updated to track this reality — no gate above is reopened:

- `scripts/release-readiness.mjs`: SPDX line now validates recognized
  SPDX expression tokens (`MIT`, `Apache-2.0`, `OR`, `AND`, `WITH`) so the
  license-only negative control still fails on placeholder content.
- `scripts/acceptance.mjs` `release()`: asserts the ratified LICENSE clears
  the license-missing blocker while `Release status: APPROVED` (and all
  other release controls) keep packaging blocked.

Post-change evidence: all ten acceptance scenarios pass (`policy`,
`bloodline`, `skills`, `isolation`, `lineage`, `genome`, `release`,
`reviews`, `summary`, `summary-v02`); `release-readiness.mjs
--expect-blocked` exits 0 with the six remaining release controls
(release status, third-party notices, SBOM, approving disposition,
protected owner approval, reproducibility evidence). `cargo fmt` clean,
strict `clippy -D warnings` clean, 25 unit tests pass on toolchain 1.98.0.
