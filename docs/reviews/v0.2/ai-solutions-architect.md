# Independent review — AI Solutions Architect

## Scope and independence

This was a strictly read-only review of the specified plans, documents, manifest, implementation, scripts, schemas, and tests. I did not read the other reviewers' conclusions, excluded `docs/reviews`, and made no changes. The report records the pre-remediation snapshot.

## Three strongest elements

- The architecture clearly separates the local v0.2 preview from the proposed durable/public v0.3 topology and sequences authority after foundational controls.
- Version probes, trusted enforcers, state isolation, and filesystem defenses materially reduce local execution risk.
- Bloodline, signed lineage, content-addressed quarantine, and formal schemas provide a concrete evidence substrate.

## Critical findings

None.

## High findings

- **SA-H1 — external effects and secret exfiltration are not technically denied.** Shared host networking and readable host files can expose secrets or permit remote writes even if local filesystem writes are blocked.
- **SA-H2 — signed genomes are not bound to executions.** The reviewed run evidence omitted the verified genome and executable identity, so lineage could not prove which execution unit produced a result.
- **SA-H3 — the final v0.2 acceptance oracle can false-pass.** The reviewed summary did not require sixteen checked and evidenced gates.

## Medium/low findings

- Emitted data was not validated against the published schemas.
- Checkpoint capture could race and an exclusive ledger lock spanned execution.
- Adapter conformance remained partial, with one real Codex path and no real Hermes inference.
- The verification report's test count was stale.

## Highest-risk Unsupported assumption

That read-only host mounts plus declared policy prevent harmful external effects. An adversarial networked worker with access to user-readable credentials can still disclose data or mutate remote systems.

## Required changes before adoption

Use a dedicated non-sensitive environment, clear/allowlist environment variables, mediate network and mounts, bind every execution to signed identity and artifact hashes, make lineage atomic, enforce every gate, validate schemas, and keep public/write modes blocked pending multi-user authorization, queues, recovery, quotas, observability, conformance, and release controls.

## Verdict

Approve with conditions.

Machine record: `ai-solutions-architect.json`.
