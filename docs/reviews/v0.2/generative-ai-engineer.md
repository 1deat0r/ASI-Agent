# Independent review — Generative AI Engineer

## Scope and independence

This was a strictly read-only review of Draft 0.3, v0.2 implementation, documentation, schemas, scripts, and tests. I did not read the other reviewers' conclusions and did not inspect `docs/reviews`. The report records the pre-remediation snapshot.

## Three strongest elements

- The isolation bootstrap requires trusted enforcers, bounds version probes, removes probe networking, and provides ephemeral missing state.
- Bloodline, lineage, and Skill Crypt have credible local integrity controls including no-follow/create-new publication and hash evidence.
- The product narrative honestly limits v0.2 to a local research nucleus and identifies host-read, networking, and credential risks.

## Critical findings

None for a tightly controlled local single-user research preview. Public, multi-tenant, persistent, or write-capable operation is outside that conclusion.

## High findings

- **GE-H1 — read-only does not protect readable data or credentials.** A read-only host bind plus shared network can still expose user-readable secrets and permit credential-backed remote effects.
- **GE-H2 — current observability cannot support defensible harness comparisons.** The run record lacks model/provider/version, effective tools, usage, cost, trials, and complete experiment identity.
- **GE-H3 — inspected plans are not bound to execution.** The reviewed plan and execute paths independently resolved workers without an approved-plan digest.
- **GE-H4 — release readiness is not a trustworthy oracle.** The reviewed packaging path could be unlocked by any non-empty license without proving gates, review disposition, or release artifacts.

## Medium/low findings

- Hermes integration was a help-level smoke test rather than live behavioral conformance.
- The CLI is infrastructure rather than a comparative product with progress, cancellation, artifacts, or human-oriented workflows.
- Failure diagnostics returned harness-controlled stderr content.
- Emitted artifacts were not validated against schemas, and nominal ledger verification could create directories.

## Highest-risk Unsupported assumption

That v0.2 is sufficient for comparative read-only harness experiments on a normal workstation. It is adequate only for control-flow exploration in a disposable, secret-free environment; it cannot yet support safe normal-workstation use or scientific attribution.

## Required changes before adoption

Require disposable isolation and scoped credentials; bind execution to an approved plan and signed execution unit; add real conformance runs, complete observational metadata, machine-verifiable gates, validated workflows, secret-safe diagnostics, and public-release controls. Keep all write authority denied pending action-level approval, rollback, idempotency, cancellation, and crash recovery.

## Verdict

Approve with conditions.

Machine record: `generative-ai-engineer.json`.
