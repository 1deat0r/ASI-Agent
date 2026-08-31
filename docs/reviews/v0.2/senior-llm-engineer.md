# Independent review — Senior LLM Engineer

## Scope and independence

This was a strictly read-only review of the plans, documentation, Rust implementation, scripts, schemas, and tests. I did not read the other reviewers' conclusions, excluded `docs/reviews`, and edited no files. The report records the pre-remediation snapshot.

## Three strongest elements

- The containment boundary is unusually candid and substantially enforced: trusted enforcers, Bubblewrap host-write containment, ephemeral missing state, and explicit confidentiality/network disclaimers.
- Adapter construction redacts prompts, prevents option injection, and uses bounded, no-network, trusted version probes.
- Bloodline, lineage, and Skill Crypt artifacts use symlink resistance, locking, synchronization, content addressing, and explicit schemas.

## Critical findings

None within the explicitly stated local, single-user, read-only research boundary.

## High findings

- **SE-H1 — adapter behavior is not fail-closed.** The reviewed implementation accepted executables with unavailable or unsupported versions and had no version-specific conformance contract. Upstream flag drift could invalidate containment assumptions.
- **SE-H2 — the advertised comparison unit is not represented at runtime.** Exact model/provider/configuration, effective tools, token use, cost, and experimental identity are absent, preventing reproducible attribution.
- **SE-H3 — the final gate oracle can report success with unmet gates.** The reviewed summary counted gates and artifacts without requiring every gate to be checked and evidenced.

## Medium/low findings

- Exit zero does not distinguish structured completion, refusal, provider fallback, or tool behavior.
- The reviewed runtime held an exclusive Bloodline lock across asynchronous work, and checkpoint verification/hash capture were separately raced operations.
- The Hermes installer claimed pinned `uv` without enforcing its exact version.

## Highest-risk Unsupported assumption

That accepted CLI flags plus filesystem write containment are sufficient to treat heterogeneous, ambiently configured provider processes as comparable read-only workers. They do not establish provider identity, tool semantics, destination limits, spend limits, or behavioral conformance.

## Required changes before adoption

Fail closed on adapter versions, add behavioral conformance tests, record the complete execution unit, use structured outcomes, make ledger/checkpoint operations concurrency-safe, and rerun every gate on a frozen snapshot. Public, multi-user, unattended, sensitive-data, or write-capable use requires credential brokering, destination-scoped egress, minimal mounts, quotas, cancellation/crash tests, write authorization, and licensing.

## Verdict

Approve with conditions.

Machine record: `senior-llm-engineer.json`.
