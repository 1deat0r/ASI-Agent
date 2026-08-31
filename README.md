# ASI Agent

> **Absorb. Recraft. Evolve.**

ASI Agent is an experimental sovereign meta-harness: one control plane that can inspect, constrain, invoke, compare, and eventually recraft capabilities from many agent harnesses and skill ecosystems.

Its long-term aspiration is evidence-gated improvement toward more general intelligence. The software here is **not AGI or ASI**. Version 0.2 is a local research nucleus with a policy Heart, quarantined Skill Crypt, OS-isolated harness workers, signed harness genome, and signed Bloodline checkpoints.

The governing doctrine is:

> **No foreign capability reaches the Heart unexamined.**

## What works today

- discovers Hermes Agent, Pi, Oh My Pi, Codex CLI, and Claude Code through explicit adapters;
- resolves direct executables before state-mutating launcher wrappers;
- runs harness version probes inside a no-network, read-only Bubblewrap probe;
- fails closed before planning when an external harness version is unavailable or outside its reviewed compatibility prefixes;
- creates redacted invocation plans without running the requested task;
- permits only `none` and `read-only` effect classes under policy v0.1;
- requires `--execute`, the exact digest from a separately inspected plan, and a signed current genome verified with an explicit public key before running a task;
- requires a root-owned, non-group/world-writable Linux Bubblewrap executable for every subprocess task by default;
- exposes the actual isolation posture and its residual read/network authority;
- records the approved plan, worker version/fingerprint, signed-genome digest/key, and execution outcome in a mode-`0600`, append-only SHA-256 Bloodline;
- creates Ed25519-signed Bloodline checkpoints and harness-genome snapshots;
- reads third-party `SKILL.md` files with no-symlink descriptor resolution, detects risky patterns, and copies only that document into non-executable quarantine;
- provides deterministic in-process and subprocess Constructs for repeatable tests.

Hermes Agent 0.20.0 is installed on the development machine from clean pinned commit `b9aa9289a8083f2e9d248ad6837b2938f5ee92d7`. Because its `safe` toolset still contains provider-backed tools, Hermes currently requires explicit `--harness hermes` selection and is excluded from automatic routing. OpenClaw remains deferred: it may later become a presence or ingress layer, but it will not own identity, policy, memory, evaluation, or promotion.

## Build and verify

Requirements: Linux, Bubblewrap, Rust 1.85 or newer, and Node.js 20 or newer.

```bash
cargo build --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

This machine's ambient `mise` Cargo shim has no selected version, so verification uses the explicit rustup installation recorded in `GATES.md`.

## Establish trust, inspect, execute

Discover the local harness genome:

```bash
target/debug/asi doctor --json
```

Generate a local Ed25519 identity and sign the current harness genome before the first execution:

```bash
target/debug/asi key generate \
  --private-key .asi/keys/lineage-private.json \
  --public-key .asi/keys/lineage-public.json \
  --json

target/debug/asi genome sign \
  --private-key .asi/keys/lineage-private.json \
  --output .asi/genome/signed.json \
  --json
```

Prepare a redacted, non-executing plan. Inspect all fields and retain `plan.plan_sha256`:

```bash
target/debug/asi plan \
  --harness pi \
  --effect none \
  --task "Reply with a one-line health report" \
  --json
```

Exercise the complete runtime offline:

```bash
target/debug/asi run \
  --harness construct-fixture \
  --effect none \
  --task "prove the nucleus is alive" \
  --execute \
  --approved-plan-sha256 '<digest from the matching plan command>' \
  --genome .asi/genome/signed.json \
  --genome-public-key .asi/keys/lineage-public.json \
  --ledger .asi/bloodline.jsonl \
  --json

target/debug/asi ledger verify --path .asi/bloodline.jsonl --json
```

Run a subprocess worker only after inspecting its plan:

```bash
target/debug/asi run \
  --harness codex \
  --effect read-only \
  --task "Reply exactly: ASI_ADAPTER_OK" \
  --execute \
  --approved-plan-sha256 '<digest from an identical codex plan command>' \
  --genome .asi/genome/signed.json \
  --genome-public-key .asi/keys/lineage-public.json \
  --ledger .asi/bloodline.jsonl \
  --json
```

The approved digest binds the redacted task hash, adapter, worker path/version/fingerprint, arguments, cwd, budget, and isolation plan. Execution rebuilds that plan, requires an exact digest match, verifies that the signed genome still matches current harness state, and re-fingerprints the selected worker immediately before launch. There is still a narrow path-to-exec race on a compromised same-account host; v0.2 is not a hostile-host security boundary.

The default subprocess plan is wrapped by Bubblewrap with a read-only host root, ephemeral `/tmp`, isolated user/PID/IPC/UTS/cgroup namespaces, disabled nested user namespaces, dropped capabilities, and parent-death termination. Known harness state directories receive ephemeral overlays so a CLI can run without persisting session mutations.

If Bubblewrap is unavailable, execution fails closed. Deliberately bypassing it requires two signals:

```bash
target/debug/asi plan \
  --harness pi \
  --effect none \
  --task "unsafe example" \
  --isolation off \
  --acknowledge-unsafe-subprocess \
  --json
```

That mode is for diagnosis, not routine use.

## Signed Bloodline and genome

The identity and genome commands above establish the execution trust root. The private document is created with mode `0600`, is never printed, and is never overwritten.

Sign and verify a Bloodline checkpoint:

```bash
target/debug/asi ledger checkpoint \
  --path .asi/bloodline.jsonl \
  --private-key .asi/keys/lineage-private.json \
  --output .asi/checkpoints/bloodline.json \
  --json

target/debug/asi ledger verify-checkpoint \
  --path .asi/bloodline.jsonl \
  --checkpoint .asi/checkpoints/bloodline.json \
  --public-key .asi/keys/lineage-public.json \
  --json
```

Sign and verify every adapter descriptor, detected version, direct executable path/source, and executable SHA-256 fingerprint:

```bash
target/debug/asi genome sign \
  --private-key .asi/keys/lineage-private.json \
  --output .asi/genome/signed.json \
  --json

target/debug/asi genome verify \
  --genome .asi/genome/signed.json \
  --public-key .asi/keys/lineage-public.json \
  --json
```

Verification requires an explicitly selected public-key file and rejects signature tampering, the wrong key, and drift in the currently detected harness state. These signatures prove continuity with whoever controls the private key; they do not prove that the signer or signed software is trustworthy. A same-account attacker who can steal the private key can forge new checkpoints.

## Skill Crypt

Inspection treats `SKILL.md` as untrusted data:

```bash
target/debug/asi skill inspect /path/to/skill --json
target/debug/asi skill assimilate /path/to/skill --vault .asi/crypt --json
```

Assimilation is not installation, approval, or execution. Scripts, hooks, dependencies, binaries, and referenced files are not copied or run. Source path components and the final file must not be symlinks; Linux reads use descriptor-based `openat2` resolution. Promotion does not exist in v0.2.

## Authority boundary

ASI Agent owns task identity, effects, policy, budgets, adapter selection, isolation, and the Bloodline. Harnesses receive bounded work and return output; they never become the sovereign agent.

| Effect | Current policy | Meaning |
|---|---:|---|
| `none` | allowed | No harness tools requested; model-provider traffic may still occur |
| `read-only` | allowed | Adapter requests its constrained read posture |
| `workspace-write` | denied | No persistent workspace mutation authority |
| `external` | denied | No user-facing messages, publication, purchases, or remote mutation authority |

Adapter flags alone are **not a security sandbox**. Bubblewrap now independently enforces the principal filesystem/process boundary, and the runtime clears the child environment to a small baseline instead of inheriting ambient API keys or proxy secrets. It still does not hide host-readable files, broker file-based credentials, filter provider destinations, inspect traffic, apply seccomp, or impose CPU/memory cgroups. Provider inference—and, for explicitly selected Hermes, provider-backed web/vision/image operations—uses the shared host network. Run v0.2 only from a dedicated non-sensitive account or VM, and do not expose files the selected harness should not be able to read.

## Release status

This repository is **not ready for public distribution or adoption**. The owner has not selected a license, but licensing alone is deliberately insufficient: readiness also requires a public-release-approving review disposition, all gates, notices, an SBOM, protected owner approval, and reproducibility evidence. The independent CAIO review says **do not approve**; the integrated disposition allows only tightly isolated local development and blocks public, persistent, multi-user, sensitive-data, and write-capable modes. See `docs/LICENSING.md`, `docs/THREAT-MODEL.md`, and `docs/reviews/v0.2/integrated-disposition.md`.

## Project map

- [Architecture](docs/ARCHITECTURE.md)
- [Threat model](docs/THREAT-MODEL.md)
- [v0.2 verification](docs/VERIFICATION-v0.2.md)
- [Hermes installation](docs/HERMES.md)
- [Licensing gate](docs/LICENSING.md)
- [Five-role review disposition](docs/reviews/v0.2/integrated-disposition.md)
- [Astronomical plan](ASI-Agent-Astronomical-Plan.md)
- [Acceptance gates](GATES.md)
- [Implementation plan](PLAN.md)
- [Harness adapter schema](specs/harness-adapter.schema.json)
- [SkillSpec schema](specs/skill-spec.schema.json)
- [Independent review schema](specs/independent-review.schema.json)
- [Review disposition schema](specs/review-disposition.schema.json)

The next authority milestone is credential brokering and destination-scoped network mediation, followed by syscall/resource controls and protocol-level harness conformance. Workspace writes remain denied until those controls are independently demonstrated.
