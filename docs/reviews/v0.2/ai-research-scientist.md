# Independent review — AI Research Scientist

## Scope and independence

This was a strictly read-only review of Draft 0.3, the v0.2 workspace source, and the complete `src/`, `scripts/`, and `specs/` trees. I did not read the other reviewers' conclusions and did not inspect `docs/reviews`. The report records the pre-remediation snapshot and has not been retroactively softened.

## Three strongest elements

- The plan treats self-improvement as a causal system-level claim and proposes matched controls, ablations, multiple seeds, transfer, retention, and contamination checks.
- AGI/ASI claim discipline is responsible: evidence must be multidimensional, repeatable, and independently replicated; the repository does not claim that the current system is AGI or ASI.
- v0.2 supplies a credible defensive measurement substrate through bounded isolated version probes, signed genome/checkpoint contracts, hash-chained Bloodline evidence, and hardened output paths.

## Critical findings

None for the explicitly scoped local, single-user, non-confidential research preview. This conclusion does not extend to public or write-capable deployment.

## High findings

- **RS-H1 — v0.2 does not yet encode a falsifiable experiment.** Task and run records omit treatment/control assignment, benchmark distribution and split, model/provider revision, seed, evaluator, contamination status, and a statistical plan. It is engineering instrumentation, not yet self-improvement research evidence.
- **RS-H2 — proposed harness comparisons are causally confounded.** Adapters receive materially different tools and information, while budgets do not normalize tokens, turns, tool calls, compute, or cost.
- **RS-H3 — sealed-holdout and contamination claims cannot currently be supported.** Workers can read the host root and use shared networking, so host-resident holdouts and uncontrolled online information are not sealed.
- **RS-H4 — the final gate oracle can report readiness without proving it.** The reviewed oracle did not require all sixteen gates to be checked and evidenced.

## Medium/low findings

- AGI is not operationally defined and ASI thresholds remain qualitative rather than numerically preregistered.
- Exact scientific replay is unavailable because model/provider, sampling, source/data snapshots, evaluator, and random state are incomplete.
- The reviewed schemas were documentation rather than emitted-artifact conformance gates.

## Highest-risk Unsupported assumption

That a common read-only label plus signed executables makes heterogeneous harness runs comparable enough to attribute improvement. Provider drift, unequal tools and information, differing budgets, online access, hidden configuration, and benchmark exposure can explain an apparent gain without any causal improvement.

## Required changes before adoption

Add versioned experiment/evaluator/result contracts, an externally controlled Crucible, preregistered statistical thresholds, contamination defenses, matched resources, and quantitative AGI/ASI claim thresholds. Until then, the local preview may proceed only as an exploratory engineering instrument with sanitized data and credentials and no comparative or confirmatory claims.

## Verdict

Approve with conditions.

Machine record: `ai-research-scientist.json`.
