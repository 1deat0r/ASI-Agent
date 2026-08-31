# Visceral Architecture — v0.2

ASI Agent is a sovereign evolutionary meta-harness. It places existing runtimes beneath one authority, studies their capabilities as untrusted phenotypes, and permits only evidence-gated recrafting. The visceral vocabulary names responsibilities and trust boundaries; it is not a substitute for precise engineering.

```text
                             HUMAN OWNER
                                  |
                         intent / consent / stop
                                  v
  +----------------------------------------------------------------+
  |                           THE HEART                            |
  | identity | policy | budgets | routing | isolation | shutdown  |
  +-----------+--------------------+---------------------+---------+
              |                    |                     |
              v                    v                     v
          THE HUNGER        THE BLOOD BANK       THE IMMUNE SYSTEM
          discovery          signed genome        policy / defense
              |                    ^                     |
              v                    |                     |
          THE CRYPT -------> DISSECTION TABLE            |
          quarantine        static/dynamic study         |
                                   |                     |
                                   v                     |
                              THE STITCHERY <-------------+
                              composition
                                   |
                                   v
                             THE CONSTRUCT
                         bounded candidate agent
                                   |
                                   v
                              THE CRUCIBLE
                        eval / red team / comparison
                             | pass       | fail
                             v            v
                        TRANSFUSION    REANIMATION
                        promotion       rollback
                             \            /
                              v          v
                         THE BLOODLINE
                  provenance / signatures / lineage
```

## Single-authority invariant

Only the Heart may own:

- the human-facing identity and current objective;
- task/run identity and requested/effective authority;
- policy, approval, routing, budgets, and cancellation;
- canonical memory and evaluation truth;
- isolation selection and credential delegation;
- promotion, rollback, and the authoritative Bloodline.

Hermes, Pi, Oh My Pi, Codex, Claude Code, future OpenClaw workers, models, tools, and imported skills are foreign organs. An adapter translates a Heart decision into a worker protocol. It cannot approve itself, change policy, become canonical memory, or promote its output.

## Current trust zones

```text
Trusted v0.2 control process
  CLI -> TaskEnvelope -> policy -> registry -> adapter -> isolation compiler
    |                                                       |
    +-----------------------> Bloodline <--------------------+

Kernel-enforced worker boundary (Linux Bubblewrap)
  read-only host root | ephemeral /tmp and harness state | namespaces
  dropped capabilities | nested user namespaces disabled | parent-death exit

Residual shared authority
  host files remain readable | file credentials are not brokered
  provider egress uses host network | no seccomp or CPU/memory cgroup budget

Untrusted content boundary
  SKILL.md -> bounded parser -> content-addressed Crypt + SkillSpec
                                      X no script execution
                                      X no promotion
```

The Bubblewrap profile materially enforces write containment. It is not a confidentiality or network-isolation claim. A worker can read files visible to the user account and can contact arbitrary destinations available on the host network. Adapter flags remain defense in depth inside that outer boundary.

## Runtime sequence

1. The CLI validates a `TaskEnvelope`, assigns task/run identifiers, resolves the working directory, and hashes the raw prompt.
2. Fixed Rust policy evaluates the requested effect. `workspace-write` and `external` fail before adapter selection.
3. The registry resolves an explicit adapter or a policy-compatible automatic route.
4. Detection resolves an environment override, known direct mise installation, or PATH executable. It never prefers the local launchers known to mutate global mise selection.
5. Any `--version` probe runs with a cleared environment for at most five seconds inside a separate no-network, read-only Bubblewrap profile. Security utilities selected through PATH must resolve to root-owned, non-group/world-writable files; otherwise the probe is unavailable.
6. External adapters fail closed when the version is unavailable or does not match a reviewed compatibility prefix. The selected worker executable is SHA-256 fingerprinted.
7. The adapter compiles private arguments and a separately redacted public projection. Option-like task text is placed after a separator where supported; Hermes receives it only as the value of `--query`.
8. The isolation compiler requires Bubblewrap for every subprocess. The unsafe bypass requires both `--isolation off` and `--acknowledge-unsafe-subprocess`.
9. Planning emits a deterministic digest over the task hash, adapter, worker identity, redacted arguments, cwd, budget, and isolation posture. Planning ends unless `--execute` is explicit.
10. Execution requires that exact approved-plan digest plus a signed genome verified against an explicit public key and current harness state. The selected worker fingerprint is checked again before launch.
11. The Bloodline records task/policy, approved plan, worker identity, signed-genome digest/key/generation time, and the redacted isolated invocation. Each append takes a short exclusive lock; no lock spans model execution.
12. The runtime clears the environment to a small baseline, then executes one in-process Construct or one isolated subprocess with null stdin, bounded output retention, wall timeout, kill-on-drop, and parent-death termination.
13. Completion or failure is appended without raw prompt/output or raw failed-process stderr. The caller receives status, bounded output, digests, duration, ledger path, signed-genome identity, and actual isolation report.

## Worker boundary

The Linux profile is compiled after adapter planning so a worker cannot replace it:

- `--unshare-all`, explicit user namespace, no nested user namespaces;
- all capabilities dropped;
- `/` mounted read-only;
- fresh `/proc`, `/dev`, and tmpfs `/tmp`;
- known harness state directories overlaid with ephemeral storage;
- explicit working directory and task program after Bubblewrap's `--` boundary;
- host network deliberately re-shared because provider calls are required.

The enforcer is accepted only when PATH resolves `bwrap` to a root-owned file without group/world write permission. This prevents a user-local shim named `bwrap` from impersonating the boundary; it is not a substitute for host package integrity.

The subprocess acceptance Construct proves that an ambient secret variable is absent, attempts a workspace write and must fail, then writes `/tmp` and emits `ASI_ISOLATION_OK`. Negative checks remove `bwrap`, spoof it with a user-owned shim, try a one-signal isolation bypass, and force secret-bearing stderr to prove only a digest is returned.

Not yet enforced: destination egress, DNS policy, credential scoping, syscall filtering, process-count/CPU/memory quotas, encrypted prompt IPC, or per-file read allowlists. Those omissions keep persistent write authority prohibited.

## Harness genome

Each adapter declares a phenotype instead of pretending all CLIs share semantics:

| Harness | Current effects | v0.2 posture |
|---|---|---|
| Hermes Agent | read-only, explicit only | Pinned 0.20.0 source install; `safe` toolset plus safe mode; excluded from auto routing because provider tools remain network-capable |
| Pi | none, read-only | Extensions, skills, templates, themes, context, and session disabled; tools disabled or read allowlisted |
| Oh My Pi | none, read-only | Extensions, skills, rules, and session disabled; tools disabled or read allowlisted |
| Codex CLI | read-only | Ephemeral, ignored config/rules, read-only inner sandbox |
| Claude Code | none, read-only | Safe mode, no persistence, tools disabled or `Read,Grep,Glob` allowlisted |
| Deterministic Construct | none | In-process control-flow and Bloodline fixture |
| Isolation Construct | none | Fixed subprocess write-escape probe |

`genome sign` snapshots sorted typed descriptors, reviewed version prefixes, installation state, isolated version result, direct executable path/source, and SHA-256 executable fingerprint. It signs the schema-locked JSON material with Ed25519. `genome verify` requires an explicit public key, verifies the signature, captures the current state again, rejects drift, and returns the exact signed-document digest that execution binds into Bloodline.

An executable fingerprint does not recursively fingerprint all interpreted source or configuration. Hermes is an editable source installation because upstream rejects wheel builds; its installer therefore also pins exact commit/origin and requires a completely clean checkout. Continuous integrity monitoring and signed upstream manifests remain future work.

## Bloodline trust model

The JSONL Bloodline uses a short exclusive advisory lock per append and commits each event to its sequence, predecessor, timestamp, kind, run, and structured data with SHA-256. This detects accidental damage and ordinary edits without holding a synchronous lock across a worker run.

`ledger checkpoint` verifies and hashes one shared-locked file snapshot, then signs this typed material with Ed25519:

- exact ledger byte SHA-256;
- event count;
- terminal event hash;
- checkpoint time.

Private keys are create-new mode `0600`; the CLI returns only key id and paths. Verification pins an explicit public-key document. A signature establishes continuity with that key, not objective truth. Keys and checkpoints currently share the same user-account trust domain, so account compromise can still steal the key, replace the binary, and forge future state. Hardware-backed keys and an external transparency anchor are later controls.

## Skill assimilation

```text
source -> bounded read -> hash -> static findings -> SkillSpec -> quarantine
                                                               X execution
                                                               X promotion
```

Version 0.2 reads one local `SKILL.md`, rejects every symlink component, and on Linux opens it with descriptor-based `openat2` using `BENEATH`, `NO_MAGICLINKS`, and `NO_SYMLINKS`. It parses metadata, detects a conservative set of risky instruction patterns, strips executable permission, and records that scripts were not copied or run. Static inspection is neither a malware verdict nor a license grant. The intended ladder remains Use → Translate → Dissect → Recraft → Crossbreed → Distill → Evolve, but only the quarantined entry point exists.

## Promotion invariant

No candidate can promote itself. A future Transfusion requires at least:

- complete source, transformation, dependency, and license lineage;
- a versioned candidate digest and signed genome;
- sealed tests and adversarial cases not authored only by the candidate;
- causal benefit against named baselines, with cost and regression evidence;
- policy/security review proportional to any authority increase;
- an externally controlled approval bound to the exact artifact;
- a tested Reanimation path to a known-good release.

No Crucible, Transfusion, autonomous self-modification, or online learning implementation exists in v0.2.

## Executable contracts

- `TaskEnvelope`: identity, prompt digest, effect, budget, deadline, and cwd.
- `PolicyDecision`: policy version, allow/deny, stable reason, and ceiling.
- `HarnessDescriptor`: phenotype, reviewed version prefixes, supported effects, authority owner, and containment claims.
- `InvocationPlan`: private command plus digest-bound worker version/fingerprint and redacted public projection.
- `IsolationReport`: requested mode, actual enforcer, filesystem, namespaces, network, and limitations.
- `BloodlineEvent`: append-only hash-linked lifecycle event.
- `SignedCheckpoint`: signed ledger digest/count/terminal hash.
- `SignedGenome`: signed typed harness snapshot and executable fingerprints.
- `SkillSpec`: normalized metadata, content hash, findings, quarantine state, and lineage.

## Failure posture

- Unknown, version-unavailable, or version-incompatible harness: fail before planning, never improvise.
- Missing or mismatched approved plan or signed current genome: reject before creating run evidence.
- Unsupported effect: deny before execution.
- Missing Bubblewrap in required mode: fail closed.
- Unsafe isolation bypass without the second signal: reject.
- Timeout or non-zero child status: record a digest-only failure and return non-zero.
- Corrupt ledger, checkpoint, genome, or wrong key: reject at the first broken invariant.
- Existing private key or signed output path: refuse overwrite.
- Malformed/oversized skill or signature document: reject; never execute recovery instructions.
- Missing licensing ratification, approving review disposition, gate evidence, notices, SBOM, protected approval, or reproducibility record: fail before creating release output.

## Next architectural tranche

1. Broker least-privilege credentials outside model and worker context.
2. Route provider access through destination-scoped egress mediation with observable policy decisions.
3. Add seccomp, process-count, CPU, memory, and disk quotas plus process-tree tests.
4. Replace command-line prompts with protected IPC and explicit secret/taint labels.
5. Add protocol-level conformance suites, artifact capture, cancellation races, and crash recovery.
6. Anchor signed Bloodline checkpoints in a separately controlled transparency service.
7. Build the Crucible before any implementation of automatic promotion or persistent workspace writes.
