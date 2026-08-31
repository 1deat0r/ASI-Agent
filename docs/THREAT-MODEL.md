# Threat model: visceral nucleus v0.2

## Security claim

On the tested Linux host, ASI Agent requires Bubblewrap for subprocess tasks and independently enforces a read-only host root, ephemeral `/tmp` and harness-state writes, namespace separation, dropped capabilities, disabled nested user namespaces, and parent-death termination. It fails closed if the enforcer is unavailable unless the operator supplies two explicit unsafe-bypass signals.

This is a write-containment and process-separation claim. It is **not** a claim of complete confidentiality, network containment, hostile-host resistance, malware containment, or safe multi-tenancy. Host-readable files remain visible; file credentials are not brokered; provider egress remains unmediated; syscall and resource controls are incomplete. Consequently, the policy label `none` or `read-only` cannot independently prevent credential-backed remote effects.

## Protected assets

- human intent, informed consent, stop/revoke ability, and honest AI identity;
- policy, requested authority, task/run identity, and isolation selection;
- prompt, workspace, credential, and memory confidentiality;
- adapter, executable, skill, and transformation provenance;
- Bloodline continuity and future evaluation/promotion integrity;
- provider accounts, quotas, and external systems reachable by workers;
- owner signing keys and release authority.

## Trust boundaries and assumptions

The trusted computing base is the reviewed `asi` binary, its Rust dependencies, Bubblewrap/kernel behavior, the selected public key, and the local operating-system account. The host kernel and account are assumed not already compromised.

Installed harnesses, their providers/models/tools, imported content, task text, retrieved content, and skills are untrusted. A harness may be buggy, compromised, or actively adversarial. Adapter flags are defense in depth, not the outer reference monitor.

The Ed25519 public key is trusted because the operator selected its file. A valid signature says only that the corresponding private key signed typed bytes. It does not make a descriptor, event, or software artifact true or safe. The private key currently lives under the same account, so account compromise defeats future-signature integrity.

## Threats and controls

| Threat | v0.2 control | Residual risk / next control |
|---|---|---|
| Prompt or skill instruction seizes sovereignty | Fixed Rust policy precedes workers; skills are inert data; harness cannot edit host state | Worker can still disclose readable data or misuse provider tools; add read allowlists, taint labels, and egress mediation |
| Worker writes workspace, policy, ledger, or user config | Root-owned non-writable Bubblewrap selected through PATH; read-only root; tmpfs and selected state overlays; fixed write-escape probe | Kernel/Bubblewrap/package defects and writable virtual files remain; add seccomp, minimal mounts, and separate worker identity |
| Worker escapes through child processes | PID/user/IPC/UTS/cgroup namespaces, dropped caps, disabled nested userns, parent-death behavior | No explicit process-count/CPU/memory cgroup limits; add cgroup v2 and fork-bomb tests |
| Worker exfiltrates files or credentials | Child environment is cleared to a small baseline; persistent writes denied | Host files and file-based credentials remain readable; use per-task mount allowlists and capability-scoped credential broker |
| Worker contacts undeclared destinations | None beyond adapter intent and logging of the shared-network posture | Provider egress remains unmediated; require deny-by-default proxy/DNS policy and destination grants |
| Ambient harness configuration changes behavior | Child environment is cleared; adapters request ignored rules/config/session; known state writes are ephemeral; Hermes safe mode disables plugins/MCP | File credentials may still load; upstream semantics can drift within a version prefix; add isolated HOME, explicit credential injection, and conformance probes |
| Discovery executes an adversarial `--version` | Version probes run with a cleared environment inside read-only, no-network, dropped-capability Bubblewrap under a hard timeout and output cap | Probe still reads the host root and exercises parser/startup code; replace with signed package metadata where available |
| Option-like task changes adapter flags | Fixed arguments and separator/query-value placement; private/public plans built separately | Upstream parser behavior may change; maintain pinned-version adversarial conformance tests |
| Unknown or drifted adapter changes flag semantics | External adapters require a detected reviewed version prefix; selected executable version and SHA-256 enter the approved plan | Prefix compatibility is not full behavioral conformance; require live version-specific provider/tool tests before sensitive or public use |
| Inspected plan differs from execution | Execute requires the exact inspected-plan digest; worker fingerprint is checked again | A compromised same-account host can race a path between check and exec; move to descriptor/`execveat` launch or immutable package mounts |
| Genome is not the one used for a run | Execute requires a signed genome matching current state and an explicit public key; digest/key/time enter Bloodline | Signed genome omits complete provider/model/config/data identity; add a signed ExperimentSpec and external attestations |
| Prompt leaks through public plan or Bloodline | Public task argument and events use SHA-256 digest; raw output stays out of ledger | Raw task is still in child argv and potentially process inspection; move task material to protected IPC |
| Output floods memory or blocks pipes | Both streams drain concurrently while retained bytes are capped; wall timeout | CPU, child count, and total generated traffic are not budgeted; add cgroups and provider-side quotas |
| Skill bootstrap executes or escapes source during ingestion | One bounded `SKILL.md` read; every symlink component rejected; Linux descriptor read uses `openat2` beneath root with no symlinks; no script execution; executable bits removed | Static rules have false positives/negatives; non-Linux fallback is weaker; add archive sandbox, parser fuzzing, SBOM/license/malware analysis |
| Bloodline entry is edited | Per-event SHA-256 chain and chain verifier | A rewriter can recompute an unsigned chain; use signed checkpoint plus external anchor |
| Signed checkpoint is changed or checked with wrong key | Typed Ed25519 material; explicit public key; digest/count/terminal-hash comparison | Same-account key theft enables forgery; use hardware-backed/offline key and transparency service |
| Harness descriptor or executable drifts | Signed sorted genome with executable SHA-256 and current-state comparison | Interpreted source/config may change behind stable launcher; add source-tree/SBOM fingerprints and signed upstream attestations |
| Signing output overwrites evidence | Keys and signed documents use create-new writes; private mode `0600` | No rotation, revocation, threshold signing, or secure deletion; define key lifecycle |
| Multi-process ledger race or crash | Short advisory exclusive lock per append; single-line append, flush, data sync; checkpoint verification and byte hash share one locked snapshot | A run spans multiple non-transactional events; add recovery records, directory sync, crash injection |
| Auto routing silently changes worker | Compatible priority list and route recorded | User did not pin an implementation; require approval for consequential tiers and signed route rationale |
| Evaluation/promotion is gamed | Promotion does not exist; skills remain quarantined | Future evaluator can become target of optimization; build sealed external Crucible before proposal generation |
| License contamination enters recrafted system | Provenance retained; public packaging blocked without owner license | No component-level license scanner/SBOM policy yet; require before Dissection-to-Transfusion path |
| Public packaging occurs accidentally | Readiness runs before `dist/` and requires owner-ratified license metadata, sixteen evidenced gates, approving machine review disposition, notices, SPDX SBOM, protected approval, and reproducibility evidence | A maintainer with repository authority can bypass local scripts; require protected CI/release environment and artifact attestation |

## Hermes-specific residuals

Hermes 0.20.0 is installed from a pinned clean local checkout with locked dependencies. Upstream forbids wheel builds, so the project itself is editable and imports that checkout. A later source modification would change Hermes without changing the generated launcher fingerprint; rerunning the installer catches dirty/different source, but runtime continuous monitoring does not.

The adapter uses `--safe-mode`, ignores user config/rules, selects toolset `safe`, and bounds turns. Hermes' `safe` toolset excludes terminal access but includes provider-backed web, vision, and image services. Bubblewrap prevents their local persistence but does not constrain their destinations or account consumption.

## Cryptographic failure cases

- Wrong public key: key id mismatch fails before accepting the signature.
- Modified checkpoint/genome: Ed25519 verification fails.
- Modified ledger after checkpoint: chain verification or byte/count/terminal comparison fails.
- Modified installed harness after genome: current-state comparison fails.
- Stolen private key: attacker can create apparently valid future signatures; not mitigated in v0.2.
- Replaced trusted public-key file: verification can be redirected; pin/distribute it through an independent channel.
- Rollback to an old valid signed artifact: signatures alone do not provide freshness; add monotonic external anchoring.

## Abuse cases denied by policy or product scope

- persistent workspace mutation or self-installation by a worker;
- user-facing messaging, publication, purchasing, or remote mutation;
- autonomous replication, persistence, or privilege expansion;
- credential discovery/export or hidden background activity (policy-prohibited but not independently prevented for readable file credentials plus shared egress);
- disabling policy, isolation, audit, shutdown, or evaluation controls;
- importing and executing setup instructions from a skill;
- self-approval of a candidate, evaluator, policy, runtime, key, or release;
- concealing which harness/model/tool produced an outcome;
- claiming AGI, ASI, safety, or improvement without defined external evidence.

Provider inference traffic is an acknowledged exception to the broad external-communication denial, because subprocess LLM harnesses cannot function without it. Hermes' selected provider tools add external read/generation traffic. Neither is granted user-facing communication or arbitrary remote mutation by ASI policy, but v0.2 cannot independently enforce that distinction at the network layer. A stolen/readable credential may carry more remote authority than the task declaration.

## Gates before workspace-write authority

Workspace writes remain prohibited until independent evidence demonstrates:

1. exact writable roots are kernel-enforced and artifact outputs are staged outside the trusted control plane;
2. network is deny-by-default and granted per destination/action;
3. credentials are short-lived, least-privilege, and never exposed wholesale to model or child context;
4. syscalls, process count, CPU, memory, disk, wall time, and network volume are bounded;
5. all effects are intercepted, attributed, idempotent where possible, and bound to approval digests;
6. rollback distinguishes reversible, compensatable, and irreversible effects;
7. adversarial workers cannot modify policy, evaluators, keys, audit, release state, or known-good artifacts;
8. process-tree termination and crash recovery pass repeated hostile tests.

## Gates before public service

A license is necessary but not sufficient. Packaging also requires an approving independent-review disposition, complete gate evidence, notices, SBOM, protected owner approval, and reproducibility evidence. A publicly reachable or multi-user service additionally needs tenant isolation, authentication/authorization, rate and spend limits, abuse prevention, privacy/retention/deletion controls, secrets management, security reporting, incident response, backups/restores, observability with redaction, SLOs, and human operational ownership. Version 0.2 is development-only under the integrated review disposition.
