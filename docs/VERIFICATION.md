# Verification record: v0.1 visceral nucleus

This historical record is preserved for the v0.1 baseline. The current record is `VERIFICATION-v0.2.md`.

Date: 25 August 2026  
Environment: Linux x86_64, Rust 1.98.0 stable, Node.js 26.7.0

## Acceptance result

| Layer | Evidence | Result |
|---|---|---:|
| Formatting | `cargo fmt --check` | pass |
| Static quality | Clippy on all targets/features with `-D warnings` | pass |
| Rust tests | 12 unit tests | 12 pass, 0 fail |
| Harness genome | installed-harness detection and authority assertions | pass |
| Policy | write/external denial, non-executing plan, prompt redaction, option-injection negative control | pass |
| Bloodline | four-event execution, mode `0600`, chain verification, tamper negative control | pass |
| Skill Crypt | safe/risky classification, inert assimilation, manifest lineage, non-executable source | pass |
| Release structure | schemas parse, required source present, no unfinished code markers | pass |

## Live adapter smoke

The deterministic Construct proves the runtime without relying on a provider.
One real-harness smoke was also run through the same public CLI:

```text
harness:       codex
effect:        read-only
session:       ephemeral
user config:   ignored
project rules: ignored
sandbox:       read-only requested
expected:      ASI_ADAPTER_OK
observed:      ASI_ADAPTER_OK
status:        completed
duration:      4798 ms
Bloodline:     valid, 4 events
```

This proves invocation compatibility on this machine; it does not independently
prove that the downstream sandbox cannot be escaped.

Pi was also invoked with effect `none`. The adapter reached Pi, but Pi had no API
key for its selected provider and exited non-zero. ASI recorded a valid
four-event failure lineage. A negative scan confirmed that neither the raw task
nor Pi's provider error appeared in the Bloodline; only an error digest was
stored. This is an environment-authentication limitation, not a passing live Pi
inference test.

## Reproduce

```bash
cargo build --locked
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
node scripts/acceptance.mjs doctor
node scripts/acceptance.mjs policy
node scripts/acceptance.mjs bloodline
node scripts/acceptance.mjs skills
node scripts/acceptance.mjs summary
```

The formal gate definitions and exact machine-specific Rust paths are recorded
in `GATES.md`.

## Unproven and intentionally absent

- OS-enforced isolation outside downstream harnesses;
- signed or externally anchored Bloodline checkpoints;
- canonical durable memory, planner, scheduler, or approval broker;
- complete malware, license, or dependency analysis of imported skills;
- Crucible evaluation service, Transfusion promotion, or Reanimation rollback;
- workspace-write or external-effect authority;
- autonomous self-improvement, AGI, or ASI.
