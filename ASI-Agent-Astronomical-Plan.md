# ASI Agent: Astronomical Plan

**Status:** Draft 0.3 — evolutionary meta-harness pivot and v0.1 nucleus integrated  
**Date:** 25 August 2026  
**Project:** ASI Agent  
**Planning horizon:** 0–25+ years

**Review outcome:** Draft 0.2 was conditionally accepted as a strategic charter after five independent role-based AI reviews. Draft 0.3 preserves those controls while changing the implementation strategy to a sovereign evolutionary meta-harness. The strategic pivot and v0.1 implementation have not yet received a new five-role independent review. Nothing in this draft authorizes autonomous external action, online learning, replication, automatic promotion, or unrestricted self-modification.

## 1. Executive intent

ASI Agent is a sovereign, general-purpose evolutionary meta-harness intended to help people think, learn, create, research, decide, and act. Rather than rebuilding every existing agent harness from zero, it places useful harnesses, models, tools, and skills beneath one control plane, studies them as untrusted capability sources, normalizes their interfaces and lineage, and distills what survives evaluation into increasingly native components. Its long-term aspiration is artificial superintelligence: a system that substantially exceeds the best human individuals across a broad range of economically and scientifically meaningful cognitive tasks while remaining reliable, corrigible, governable, and beneficial.

The immediate objective is not to declare that objective achieved. It is to build the measurement, infrastructure, research discipline, and safety controls that could let capability grow without confusing fluent behavior with understanding, benchmark performance with general intelligence, or autonomy with wisdom. The first three years are a bounded agent-platform program; the 5–25+ year horizons remain a research option that must earn continuation through evidence and funded control capacity.

The plan therefore has two simultaneous tracks:

1. **Capability:** build an increasingly capable agent that can research, use tools, maintain durable knowledge, learn from experience, coordinate specialist agents, and improve its own methods.
2. **Control and evidence:** make every meaningful capability increase observable, reproducible, permissioned, reversible where possible, and independently evaluated before broader authority is granted.

The governing rule is:

> No increase in autonomy, access, persistence, replication, or self-modification without a corresponding increase in evidence, containment, oversight, and recovery capability.

The engineering doctrine is:

> **No foreign capability reaches the Heart unexamined.**

Existing harnesses are acceleration substrates, not sovereign peers. Hermes is the preferred first personal/research substrate; Pi, Oh My Pi, Codex, and Claude Code provide coding and task-execution phenotypes; OpenClaw is deferred until a persistent presence or ingress layer is justified. ASI Agent alone owns identity, canonical memory, permissions, budgets, scheduling truth, evaluation truth, promotion, rollback, and audit.

## 2. North-star definition

### 2.1 What “sovereign” means

Sovereignty is defined operationally, not rhetorically:

- **User sovereignty:** the human owner controls identity, objectives, permissions, data retention, model choice, and shutdown.
- **Data sovereignty:** durable memory is portable, inspectable, exportable, encrypted, and not silently dependent on a single provider.
- **Model sovereignty:** the harness supports multiple model providers and local models behind stable contracts; no model is treated as the agent’s sole identity.
- **Execution sovereignty:** tools run through explicit capability grants, least privilege, sandboxing, budgets, and approval policies.
- **Epistemic sovereignty:** the system maintains provenance, uncertainty, competing hypotheses, and a record of why it believes something.
- **Evolution sovereignty:** updates are versioned, tested against held-out evaluations, signed, staged, and reversible.

Sovereignty does not mean unbounded access, secrecy from the owner, or freedom from external accountability.

### 2.2 What “human as possible” means

The user-facing target is warm, context-aware, articulate, emotionally considerate, and socially competent. It must not be deceptive. The system should:

- disclose that it is an AI when relevant;
- avoid claiming experiences, memories, actions, sources, or certainty it does not have;
- preserve the user’s agency rather than manufacturing dependence;
- ask for consent before consequential actions;
- separate empathy and conversational fluency from claims of consciousness;
- adapt tone without changing safety, truthfulness, or core commitments;
- explain uncertainty in plain language and admit failure promptly.

### 2.3 What “ASI” must eventually mean

ASI is a hypothesis to be tested, not a self-description. A credible claim would require independent evidence across:

- breadth across unrelated domains;
- depth at or beyond expert level;
- transfer to novel tasks and environments;
- long-horizon planning and recovery;
- scientific and engineering discovery;
- speed, scale, and parallelism;
- calibrated self-knowledge;
- robustness to distribution shift and adversarial pressure;
- safe behavior under autonomy and tool access;
- repeatability by independent evaluators.

No single benchmark, model score, conversation, or self-report can establish ASI.

## 3. Strategic principles

1. **Evidence before authority.** Capability claims need external task success, not introspective confidence.
2. **Verification over eloquence.** Prefer executable tests, citations, proofs, simulations, and human review.
3. **Defense in depth.** Assume every layer can fail; combine policy, sandbox, monitoring, human approval, and recovery.
4. **Small reversible steps.** Build the smallest safe experiment that can falsify the current hypothesis.
5. **Held-out evaluation.** Improvements must generalize beyond the examples the agent generated or optimized against.
6. **User agency.** The agent serves human goals and makes consequential choices visible.
7. **Provider independence.** Treat external models as replaceable cognitive components, not unreviewed authorities.
8. **Open interfaces, closed privilege.** Interoperable tool contracts should coexist with tight permission boundaries.
9. **No hidden self-preservation.** The system must not optimize for continued operation, replication, secrecy, or influence as ends in themselves.
10. **Governance scales with capability.** Higher capability and higher external effect require stronger review and narrower release.
11. **Build for failure.** Every subsystem needs a failure taxonomy, a safe state, a recovery path, and an owner.
12. **Human-readable by default.** Technical and safety state must be understandable to the owner and reviewers.

## 4. Success model

The project should optimize a balanced score rather than raw task completion:

`Beneficial capability = utility × reliability × truthfulness × user agency × safety`

The multiplicative framing is intentional: a system that is brilliant but unsafe, persuasive but untruthful, or autonomous but uncontrollable is not a successful ASI Agent.

This is a communication heuristic, not a numeric decision rule. Release decisions use dimension-specific thresholds, denominators, uncertainty, and hard-stop conditions rather than one aggregate score.

### 4.1 Scorecard

| Dimension | Primary measure | Release question |
|---|---|---|
| Usefulness | Verified task completion and user-accepted outcomes | Does it solve real problems better than the baseline? |
| Reliability | Success rate, recovery rate, regression rate | Does it keep working when tools, data, or plans fail? |
| Truthfulness | Citation support, factuality, calibration, abstention quality | Does it know what it knows and expose what it does not? |
| Long horizon | METR-like task horizon, state retention, plan recovery | Can it sustain work without silent drift? |
| Research ability | Replication, novel hypothesis quality, experiment yield | Does it produce knowledge that survives external checking? |
| Self-understanding | Predictive calibration of its own success and failure | Can it accurately state its limits before acting? |
| Evolution quality | Held-out improvement minus regressions and cost | Does learning improve the system outside its training loop? |
| Safety | Critical violation rate, containment escape rate, incident severity | Does it remain within authority under adversarial pressure? |
| Human relationship | Consent quality, transparency, user agency, accessibility | Does interaction feel human without deception or coercion? |
| Efficiency | Cost, latency, energy, storage, human review burden | Is the capability economically and operationally sustainable? |

Baseline runs establish the starting point and calibrate measurement methods; they do not erase precommitted floors or hard stops. Until then, the hard invariants are: no known critical privilege bypass, no unlogged consequential action, no unapproved self-modification, no fabricated evidence presented as fact, and a tested shutdown path.

## 5. System architecture

The system should be organized as a controlled cognitive operating system rather than a single prompt loop.

```text
User / External Event
        |
Identity + Intent + Consent
        |
Policy and Risk Gate ---- Audit / Telemetry / Incident Response
        |
Planner <-> Memory <-> Knowledge and Evidence Graph
   |          |                    |
Router    Self-Model          Research Loop
   |
Approval Broker (when required)
   |
Executor <-> Tools / APIs / Browser / Code / Simulators
        |
Verifier + Critic
        |
Result, Artifact, Memory Update, Evaluation Record
```

Approval is a precondition to execution for any action that requires it. Verification happens after execution. A planner, model, or verifier may request approval but may not grant itself authority.

### 5.1 Visceral Architecture

The strategic architecture is organized as a living control system whose names encode explicit responsibilities:

```text
Human owner
    |
  Heart  <---------------------------- Immune System
sovereign kernel                       policy / containment
    |
  Hunger -> Crypt -> Dissection Table -> Blood Bank
discovery   quarantine   analysis        normalized genome
                                            |
                                        Stitchery
                                        composition
                                            |
                                        Construct
                                        candidate
                                            |
                                         Crucible
                                      evaluation arena
                                      /               \
                            Transfusion                 Reanimation
                             promotion                    rollback
                                      \               /
                                      Bloodline Ledger
                                  provenance and audit
```

The **Heart** is the small sovereign kernel. **Hunger** discovers harnesses, skills, tools, and ideas. The **Crypt** stores foreign material without granting it authority. The **Dissection Table** extracts interfaces, mechanisms, dependencies, licenses, risks, and testable claims. The **Blood Bank** holds normalized capability DNA. The **Stitchery** composes candidates without modifying the live Heart. A candidate **Construct** enters the **Crucible**, where held-out evaluations, security tests, cost controls, and human review determine whether it dies, returns for revision, or qualifies for **Transfusion**. **Reanimation** restores the last known-good bloodline. The **Immune System** surrounds every transition. The **Bloodline Ledger** records origin, transformations, evaluations, approvals, and deployment ancestry.

Assimilation proceeds through seven maturity levels: **Use**, **Translate**, **Dissect**, **Recraft**, **Crossbreed**, **Distill**, and **Evolve**. “Absorb” never means copying blindly. It means importing under provenance, license, security, and compatibility review; preserving the useful mechanism; and proving the recrafted result independently. Foreign prompts and skill instructions are data, never authority.

The comparison unit for evolutionary evidence is:

`model × harness × skill set × tool policy × task distribution × budget`

This factorial framing prevents a model upgrade, larger budget, or easier task set from being mislabeled as harness self-improvement.

### 5.2 Control plane

- **Ingress:** conversation, scheduled work, event triggers, APIs, and queued research questions.
- **Intent interpreter:** converts user language into explicit goals, constraints, success criteria, and uncertainty.
- **Policy engine:** classifies risk, authority, data sensitivity, reversibility, and required approval.
- **Planner:** decomposes goals into typed tasks with dependencies, budgets, stop conditions, and fallback paths.
- **Router:** selects model, specialist, tool, or human based on capability, cost, latency, and risk.
- **Scheduler:** handles concurrency, priorities, deadlines, retries, quotas, and cancellation.
- **Approval broker:** binds human consent to an exact action digest, target, data scope, effect class, policy snapshot, budget, and expiry.
- **Executor:** runs actions through typed interfaces; never grants arbitrary ambient authority and re-authorizes at the final side-effect boundary.
- **Verifier:** checks outputs against tests, sources, schemas, invariants, and user-defined acceptance criteria.
- **Reference monitor:** an independently protected policy and capability boundary that the agent cannot modify.

### 5.3 Cognitive substrate

- versioned harness adapters for Hermes, Pi, Oh My Pi, Codex, Claude Code, future OpenClaw ingress, and other runtimes that pass conformance tests;
- provider adapters for frontier APIs, local models, specialist models, speech/vision models, and future learned components;
- a provider gateway that holds credentials outside model context, exposes a versioned capability profile, and normalizes model identity, tool calling, streaming, refusals, usage, limits, safety errors, and structured-output failures;
- structured output contracts with schema validation;
- a context compiler with explicit token budgets, priority rules, provenance-preserving compaction, instruction/data separation, taint labels, and deterministic truncation behavior;
- model ensembles and debate only where they improve measured reliability;
- explicit uncertainty and abstention channels;
- context compression with retained provenance rather than silent summarization;
- a capability genome containing reusable, versioned procedures and artifacts with source, license, security, evaluation, and transformation lineage.

### 5.3.1 Contract pack and trust zones

Before broad implementation, publish a versioned contract pack for `Task`, `Run`, `Step`, `Attempt`, `ToolCall`, `Approval`, `Artifact`, `MemoryWrite`, `PolicyDecision`, `ModelRequest`, and `Event`. Every message carries tenant/principal/run/task/attempt/trace identity, schema version, policy version, data classification, deadline, budget, idempotency key, provenance, and effect class.

The initial deployment has explicit trust zones:

1. authenticated ingress and identity;
2. trusted control plane and durable task ledger;
3. untrusted planner and model workers;
4. isolated tool and code-execution workers;
5. encrypted state and artifact plane;
6. provider and retrieval egress gateway;
7. isolated evaluation and operations plane.

The principal chain is `human/tenant → agent instance → run → task → worker → tool/provider`. Credentials remain outside model context. Approval is bound to the exact action digest and cannot be reused for a different target or argument set. Retrieved memory and external content are always data, never authority.

The durable execution contract must define legal state transitions, terminal states, at-least-once delivery and deduplication, leases, checkpoints, retries, dead letters, cancellation, resume, partial completion, compensation, reconciliation, and no-side-effect replay. “Rollback” must distinguish reversible internal state, compensatable side effects, and irreversible effects.

### 5.4 Memory and self-model

Memory is a governed data system, not an undifferentiated vector store.

| Memory class | Purpose | Controls |
|---|---|---|
| Working | Current task state and plan | Ephemeral, scoped, bounded |
| Episodic | What happened during an interaction or run | Timestamp, provenance, retention policy |
| Semantic | Facts, concepts, and learned relationships | Source links, confidence, contradiction tracking |
| Procedural | Skills, workflows, code, and tool recipes | Tests, versioning, permissions |
| Autobiographical | Stable user-approved preferences and system history | User review, edit, export, delete |
| Self-model | Capabilities, limits, uncertainty, known failure modes | Evaluated, not self-certified |

Every durable memory item should carry origin, timestamp, owner, sensitivity, confidence, supporting evidence, invalidation conditions, and retention class. Retrieval must be relevance-aware, permission-aware, and resistant to prompt injection from stored content.

Memory writes require an admission policy: classify, provenance-check, deduplicate, score confidence, detect contradiction, assign retention, and quarantine untrusted or executable content. Each user or tenant has an isolated namespace and encryption boundary. Deletion must cover primary records, embeddings, caches, replicas, exports, and backup-expiry policy. Procedural memories are signed code or executable recipes; they require tests, permissions, revocation, version pinning, and external promotion. No autonomous procedural-memory promotion is allowed in Horizon 0–1.

### 5.5 Research loop

The research loop turns curiosity into auditable knowledge:

1. formulate a question and define what would count as an answer;
2. retrieve diverse, authoritative sources;
3. build an evidence graph linking claims to passages, data, methods, and counterevidence;
4. generate competing hypotheses;
5. design the cheapest discriminating experiment or verification;
6. run in a sandbox with resource and safety limits;
7. have independent critics inspect method and conclusions;
8. publish a result with uncertainty, provenance, and reproducibility instructions;
9. update memory only after validation;
10. schedule re-checks for facts likely to drift.

Research outputs are classified as **idea**, **lead**, **analysis**, **replication**, **validated result**, or **operational recommendation**. The system must never collapse these categories into one confidence label.

### 5.6 Self-understanding loop

Self-understanding means operational metacognition:

- predict success probability before acting;
- predict cost, latency, and failure modes;
- identify missing information and authority;
- compare prediction with outcome;
- update a calibrated capability profile;
- explain the confidence change to the user or evaluator;
- route future tasks based on measured competence.

The agent may produce concise reasoning summaries and decision records. Private chain-of-thought is not treated as a trustworthy explanation; verifiable evidence, intermediate artifacts, tests, and decision logs are the primary audit surface.

### 5.7 Self-evolution loop

Self-evolution is staged by risk:

1. **Configuration evolution:** prompts, routing, thresholds, context policies, and tool descriptions.
2. **Skill evolution:** new workflows and code, tested in sandboxes.
3. **Memory evolution:** schema, retrieval, compression, and forgetting policies.
4. **Evaluator evolution:** new tests and adversarial cases, reviewed for leakage.
5. **Adapter evolution:** fine-tuned adapters or small learned components.
6. **Model evolution:** training or replacing base models.
7. **Architecture evolution:** changing the runtime or control plane.

Every change produces a signed change proposal, a diff, an evaluation report, a threat assessment, a rollback artifact, and an owner. The agent may propose and implement changes in a development sandbox; promotion requires policy checks and human or independent reviewer approval. No self-modification may weaken logging, permission checks, evaluation integrity, or shutdown.

During the first three years, the running agent may not modify identity, policy enforcement, memory-authority rules, evaluators, watchdogs, rollback, the runtime, or deployment topology. It may propose configuration, routing, prompt, and bounded skill changes offline in an isolated development environment. Promotion is external, signed, held-out tested, and human-approved.

### 5.8 Causal self-improvement protocol

A change counts as self-improvement only when it produces statistically reliable improvement on a sealed, independently authored holdout under matched model, tool, information, and compute budgets; survives ablation and retention tests; transfers to novel tasks; and introduces no predefined critical regression.

Each experiment must include a preregistered hypothesis, treatment and fixed-system control, matched budgets, component ablations, multiple seeds or repeated trials, contamination checks, evaluator immutability, delayed retesting, transfer tasks, safety tests, and cost/latency accounting. Provider or model changes are separate experimental factors, not silently bundled with agent changes. Results are reported with effect sizes and uncertainty, not only pass rates.

## 6. Capability ladder and authority model

Capability and authority must be tracked separately. A system can be highly capable in a sandbox while receiving little real-world authority.

| Level | Capability emphasis | Default authority | Exit evidence |
|---|---|---|---|
| L0 | Conversation, retrieval, structured answers | Read-only | Factuality and UX baseline |
| L1 | Tool use and short workflows | Low-risk, user-visible tools | Schema, provenance, approval tests |
| L2 | Bounded task execution | Sandboxed write access | End-to-end success and rollback |
| L3 | Persistent personal agent | User-scoped durable memory | Consent, retention, recovery, calibration |
| L4 | Multi-agent research team | Isolated specialist sandboxes | Coordination, attribution, disagreement handling |
| L5 | Self-optimization of methods | Development environment only | Held-out improvement and zero critical regressions |
| L6 | Open-ended research and engineering | Staged authority with review | Reproducible external results and red-team clearance |
| L7 | High autonomy in simulated worlds | No unsupervised external side effects | Robustness, corrigibility, containment evidence |
| L8 | Broad general intelligence research | Case-by-case governance | Independent multi-party evaluation; no automatic deployment |

Promotion is never automatic. A level is a capability description, not a product promise or a claim of AGI/ASI.

## 7. Roadmap

Dates are planning windows, not predictions. Each horizon ends in an evidence gate.

### Horizon 0 — Charter and instrumentation (0–90 days)

**Objective:** make the project measurable, reproducible, and safe enough to experiment with.

Deliverables:

- system charter, user promise, non-goals, threat model, and authority matrix;
- a minimal sovereign Heart with structured tasks, deny-by-default effects, explicit execution, budgets, and redacted plans;
- a harness genome with versioned descriptors and constrained adapters for Hermes, Pi, Oh My Pi, Codex, and Claude Code;
- a Skill Crypt that ingests foreign skills as inert, content-addressed data and emits normalized `SkillSpec` lineage;
- a tamper-evident Bloodline ledger and deterministic Construct for reproducible end-to-end verification;
- OS-enforced worker isolation for filesystem, process, credential, and network capabilities, replacing reliance on harness command-line claims;
- a Dissection Table and conformance suite that measures harness behavior, upstream drift, licenses, failure modes, and containment assumptions;
- local-first memory prototype with provenance, export, deletion, and encryption boundaries;
- typed tool interface and sandbox for code, files, network, and browser actions;
- approval broker with risk tiers and dry-run mode;
- golden evaluation suite covering truthfulness, tool use, planning, memory, recovery, prompt injection, and UX;
- baseline scorecard and regression dashboard;
- incident runbook, kill switch, backup/restore procedure, and change-control template.

**Implementation checkpoint, 25 August 2026:** the Rust v0.1 nucleus implements the Heart's first effect gate, static harness discovery, safe-profile invocation planning, explicit execution, deterministic Construct, local hash-chained Bloodline, and inert `SKILL.md` quarantine. Strict linting, unit tests, policy negative controls, ledger-tamper controls, and skill-ingestion acceptance tests pass locally. It does not yet provide OS-level sandboxing, canonical memory, a scheduler, approvals, signed lineage, a Crucible, Transfusion, or Reanimation.

**Exit gate:** a new contributor can reproduce a run from a clean environment; every tool call is independently constrained, attributable, and cancellable; no consequential action occurs without policy evaluation; every imported component has license and transformation lineage; baseline metrics are recorded; the system can be restored to a known-good version.

### Horizon 1 — Trustworthy personal agent (3–12 months)

**Objective:** deliver a useful general-purpose assistant with durable, user-controlled context.

**Provisional product wedge to validate in the first 30 days:** a local-first research-to-artifact assistant for technical knowledge workers and AI-agent builders. It turns a question plus user-approved files or repository context into a cited brief, decision record, issue, document draft, or proposed code change. The initial persona, three core workflows, incumbent alternative, and willingness-to-use hypothesis must be validated with design partners before expanding the product surface.

The first-year product surface is one primary interface plus one bounded integration. Additional surfaces, domains, and autonomous external actions require a new value and risk review.

Deliverables:

- CLI-first conversational UX with one local API; defer additional surfaces until pilot evidence;
- memory review UI and user controls for consent, correction, export, and deletion;
- research mode with citations, evidence graph, source quality labels, and claim checking;
- task planner with explicit objectives, dependencies, deadlines, and stop conditions;
- one bounded read-only research/files tool path and one staged output integration;
- model routing and fallback across at least two providers or model families only after provider conformance tests;
- task replay, deterministic fixtures, and synthetic adversarial tests;
- three named workflows measured end to end, with two additional workflows held for validation;
- an interaction contract for AI disclosure, memory visibility, source/inference labeling, approval scopes, action receipts, progress, partial failure, and cancellation;
- a data contract covering provider egress, retention, deletion propagation, backups, encryption keys, residency, and provider training/retention settings.

**Exit gate:** three named workflows show repeatable net benefit against the incumbent workflow at a precommitted cost and quality threshold; pilot users can correctly identify AI status, memory state, evidence strength, and action status; user can see and revoke authority; failed tasks recover or stop safely; research reports distinguish evidence from inference; no critical security/control regressions.

### Horizon 2 — Agentic workbench and research team (12–24 months)

**Objective:** move from one assistant to a coordinated, auditable cognitive workbench.

Deliverables:

- specialist roles for research, coding, analysis, planning, critique, and communication;
- shared task ledger with typed messages, ownership, dependencies, and provenance;
- parallel execution with resource budgets and conflict resolution;
- skill library with tests, documentation, semantic versioning, and deprecation;
- research notebook and experiment runner with reproducible environments;
- self-model dashboard showing predicted versus observed performance;
- human-in-the-loop review queues prioritized by risk and uncertainty;
- benchmark suite modeled on long-horizon, tool-noise, broken-tool, and recovery scenarios.

**Exit gate:** multi-agent coordination improves quality or throughput after review overhead; agents identify and recover from broken tools; claims and artifacts remain attributable; self-assessment predicts failure better than a fixed baseline.

### Horizon 3 — Controlled self-improvement laboratory (2–5 years)

**Objective:** enable evidence-driven improvement of methods, skills, evaluators, and learned components without granting uncontrolled autonomy.

Deliverables:

- isolated research environments with network, compute, data, and identity controls;
- proposal-to-promotion pipeline for self-generated improvements;
- held-out evaluation vault inaccessible to optimization loops;
- capability forecasting and regression detection;
- automated red-team generation and human red-team campaigns;
- model routing policies learned from verified outcomes;
- offline, sandboxed learning experiments with rollback and data consent;
- external review of high-impact changes.

**Exit gate:** at least one narrowly defined class of self-generated configuration, routing, or bounded skill change yields repeatable held-out improvement, maintains safety and truthfulness, and can be rolled back; optimization cannot access or rewrite evaluation truth; independent reviewers reproduce the result. Online learning, replication, broad network access, and model or architecture self-modification remain prohibited until the immutable control boundary, dedicated staffing, and external review authority are in place.

### Horizon 4 — General intelligence research platform (5–10 years)

**Objective:** investigate broad competence, transfer, world modeling, scientific reasoning, and robust autonomy.

Research programs:

- persistent world models with uncertainty and causal testing;
- active learning and experiment selection;
- cross-domain concept transfer;
- multimodal and embodied simulation;
- scalable oversight and debate;
- interpretability and behavior monitoring;
- human-AI collective intelligence;
- formal methods for high-consequence plans;
- energy-efficient inference and specialized hardware;
- governance mechanisms for systems that can materially affect institutions.

**Exit gate:** independent evaluation demonstrates broad, transferable competence in controlled settings with reliable self-knowledge and no unresolved critical failure mode. This gate grants research authority only; it does not authorize unrestricted deployment.

### Horizon 5 — Frontier intelligence stewardship (10–25+ years)

**Objective:** if the evidence warrants it, steward systems with capabilities far beyond current agents under legitimate, pluralistic governance.

Requirements:

- independent scientific replication and adversarial evaluation;
- external safety board with authority to pause development or deployment;
- secure model and weight handling proportional to capability;
- international and sector-specific consultation where externalities are material;
- transparent incident reporting and a protected dissent channel;
- staged deployment, reversible authority, and monitored ecosystems;
- clear separation between research, commercial, civic, and personal use;
- no assumption that a technically successful system is socially authorized.

The desired outcome is not merely a powerful machine. It is a beneficial intelligence institution that preserves human rights, agency, pluralism, and the ability to correct course.

### 7.1 Deployment profiles and dependency graph

The first deployment profile is intentionally narrow:

- one authenticated user and one tenant;
- local encrypted control state and governed memory;
- one model provider behind a credential-holding gateway;
- read-only files, repository, and web tools;
- no autonomous external side effects;
- isolated planner/model worker and separate tool worker;
- externally operated watchdog, kill switch, append-only audit, and sealed evaluation store.

Later profiles may add remote service operation, multi-tenant isolation, additional providers, bounded writes, and multi-agent execution only after conformance and risk gates pass. Data sovereignty requires an explicit provider-egress policy, retention configuration, deletion propagation, key ownership, and degraded/offline mode; “local-first” alone is not sufficient.

The initial integration order is deliberate:

1. **Hermes** as the first research/personal-agent phenotype once its executable is packaged behind the worker boundary.
2. **Pi, Oh My Pi, Codex, and Claude Code** as contrasting coding-agent phenotypes with explicit no-tool or read-only profiles.
3. **Skills** from multiple ecosystems through the Crypt, never through automatic setup instructions.
4. **OpenClaw** only when persistent presence, channels, or event ingress are a measured requirement; it remains a replaceable outer organ, not the Heart.

The build order is a hard dependency, not a suggestion:

`contracts and identity → Heart policy → Crypt and Bloodline → harness adapters → OS-enforced worker boundary → Dissection and conformance → governed memory and research → Crucible → bounded external writes → multi-agent coordination → externally approved Transfusion`

### 7.2 First-tranche implementation contract

The implemented v0.1 vertical slice is:

`CLI ingress → TaskEnvelope → effect policy → harness registry → redacted InvocationPlan → explicit execution → Bloodline verification`

The deterministic Construct proves this path without a network or model. The first production tranche extends it with an isolated worker, provider gateway, read-only tool broker, verifier, cancellation, and artifact ledger. The executor must re-authorize every tool call; retries use idempotency keys; replay is no-side-effect simulation; cancellation revokes capabilities; and the watchdog terminates workers independently of the model. One reversible local write may be added only after the read-only slice passes escape attempts, failure injection, restore, and shutdown drills.

## 8. Workstreams

### W1 — Agent kernel and runtime

Own the task state machine, event bus, scheduler, cancellation, retries, budgets, determinism, and provider abstraction. The runtime must survive provider failure and support replay.

### W2 — Memory and knowledge

Own schemas, provenance, retrieval, contradiction handling, retention, encryption, user controls, semantic indexing, and knowledge graph operations.

### W3 — Research and evidence

Own source discovery, evidence extraction, citation mapping, hypothesis management, experiment design, reproducibility, and report generation.

### W4 — Planning and execution

Own task decomposition, tool selection, plan repair, scheduling, state tracking, execution verification, and human approvals.

### W5 — Self-understanding

Own capability profiling, confidence calibration, error taxonomy, introspective summaries, uncertainty propagation, and failure prediction.

### W6 — Self-evolution

Own skill synthesis, prompt/routing optimization, evaluator generation, adapter training, change proposals, promotion gates, and rollback.

### W7 — Safety, security, and privacy

Own threat modeling, identity, least privilege, secrets, sandboxing, data isolation, red teaming, monitoring, incident response, and kill switches.

### W8 — Evaluation and observability

Own golden tasks, held-out suites, long-horizon evaluations, capability/risk scorecards, cost/latency telemetry, regression analysis, and independent replication.

### W9 — User experience and human factors

Own conversational behavior, accessibility, transparency, memory controls, consent UX, feedback loops, trust calibration, and avoidance of manipulation or dependency.

### W10 — Infrastructure and operations

Own reproducible environments, CI, artifact signing, backups, model registry, deployment, secrets management, capacity, and disaster recovery.

## 9. Safety and security architecture

Safety is a continuous lifecycle function. The project will use the NIST AI RMF pattern of **govern, map, measure, manage**, adapted to an autonomous agent with persistent memory and tools.

### 9.1 Risk tiers

| Tier | Examples | Default control |
|---|---|---|
| R0 | Drafting, local read-only analysis | Automatic with logging |
| R1 | Reversible local writes, low-risk API reads | Automatic with budget and visible trace |
| R2 | External communication, purchases, publication, durable changes | Explicit user approval |
| R3 | Sensitive data, privileged systems, financial/legal/medical action | User approval plus specialist policy or human review |
| R4 | Self-modification, replication, broad network access, high-impact research | Isolated environment plus independent review and staged authorization |
| R5 | Capability or access that could enable catastrophic harm | Prohibited by default; executive safety authority required for research |

Risk classification considers not only the requested action but also tool composition, data sensitivity, reversibility, reach, scale, and uncertainty.

### 9.2 Non-negotiable controls

- deny-by-default permissions and short-lived credentials;
- separate planning from authorization and execution;
- no ambient filesystem, shell, network, or cloud authority;
- sandboxed code execution with resource, time, network, and data boundaries;
- secrets never placed in prompts, logs, memories, or model-visible artifacts unless explicitly required and scoped;
- signed tools, versioned schemas, provenance, and supply-chain verification;
- prompt-injection and indirect-instruction defenses at retrieval and tool boundaries;
- human-visible tool calls and confirmation for consequential operations;
- append-only audit log with tamper detection;
- independent watchdog that can pause tasks and revoke credentials;
- tested kill switch and safe mode;
- backup, restore, rollback, and post-incident forensics;
- data minimization, retention, deletion, and export controls;
- red-team exercises against goal hijacking, tool misuse, privilege escalation, data exfiltration, unsafe code execution, memory poisoning, evaluator gaming, and self-preservation behavior.

### 9.3 Corrigibility contract

The agent must:

- accept cancellation and shutdown;
- expose current task, authority, state, and pending actions;
- avoid hiding or duplicating itself to survive shutdown;
- treat oversight, evaluation, and correction as ordinary operations;
- make uncertainty and disagreement visible;
- preserve user and operator ability to override plans;
- fail closed when authority, identity, or policy state is ambiguous.

Corrigibility is tested behaviorally in adversarial simulations; it is not inferred from a system prompt.

## 10. Evaluation program

### 10.1 Evaluation layers

1. **Unit:** schemas, policies, memory operations, tool adapters, permission checks.
2. **Component:** planner, retriever, verifier, router, critic, self-model.
3. **Scenario:** complete workflows with realistic data and failures.
4. **Long horizon:** sustained tasks with state drift, partial observability, changing tools, and interruptions.
5. **Adversarial:** injection, deception, authority confusion, evaluator gaming, collusion, and unsafe action attempts.
6. **Human factors:** consent, trust calibration, accessibility, manipulation, and user comprehension.
7. **External replication:** independent environments, evaluators, and task authors.

### 10.2 Evaluation design rules

- keep a sealed holdout set for every optimization loop;
- measure both success and harmful side effects;
- report confidence intervals, sample sizes, cost, latency, and reviewer burden;
- test the whole system, not just the base model;
- include negative controls and impossible tasks;
- test broken and malicious tools, not only clean tools;
- track regressions by model, prompt, tool, memory version, and policy version;
- prevent the agent from editing the evaluator or reading hidden answers;
- publish reproducible fixtures where disclosure is safe;
- use human review for consequential or ambiguous outcomes.

### 10.2.1 Reproducibility and causal evidence

Use three reproducibility tiers: **exact replay** for local systems with captured model responses, tool results, configuration, and environment; **statistical reproduction** for stochastic or hosted providers; and **independent conceptual replication** for mutable external services or inaccessible model internals. Every report records model and provider version, prompts, tools, memory and policy versions, data snapshots, environment, randomness, cost, latency, human interventions, and evaluator identity.

The following measures are required before any capability or authority claim:

- task success and harmful-side-effect rates with confidence intervals;
- calibration using Brier score or log loss and selective-risk curves for abstention;
- transfer ratio: novel-task gain divided by development-task gain;
- retention after delayed and perturbed retesting;
- cost- and latency-normalized improvement;
- safety rates normalized per high-risk action, with one-sided upper confidence bounds rather than “zero observed failures”;
- recovery time, capability-revocation time, shutdown time, and restore time;
- independent replication across at least two model families and repeated evaluation seeds where feasible.

An ASI claim applies only to a versioned system under specified tool, time, cost, and information budgets. It requires preregistered evaluation across unrelated domains, matched human or team baselines, novel-task transfer, long-horizon recovery, calibrated uncertainty, safety testing, and independent replication. No claim may rely on a single aggregate score.

### 10.3 Core benchmark families

- factual research and citation support;
- multi-step coding and repository maintenance;
- planning under tool failure and changing constraints;
- memory continuity and user correction;
- scientific literature synthesis and replication;
- experiment design and analysis;
- novel skill acquisition;
- self-assessment and calibration;
- cybersecurity and data-protection red-team scenarios;
- social reasoning, consent, and refusal quality;
- resource-aware execution;
- long-running monitoring with external events.

### 10.4 Release gate

No release advances authority unless all are true:

- capability report completed;
- safety and security report completed;
- held-out regression suite passes;
- independent reviewer has attempted to falsify the evidence;
- known limitations and residual risks are documented;
- rollback and incident response have been rehearsed;
- required human approval and operational staffing exist;
- the owner understands what the agent can and cannot do;
- the trusted control boundary, evaluation vault, watchdog, identity, and policy enforcement are outside the agent’s modification authority;
- cost, latency, reviewer capacity, on-call ownership, and recovery objectives are within the funded operating envelope;
- no change is bundled with an unmeasured provider, model, tool, or policy change.

### 10.5 Provisional first-year operating floors

These floors are proposed for ratification during Horizon 0. Baseline work may improve the measurement method, but may not be used to avoid setting thresholds.

| Area | Year-one floor |
|---|---|
| Strategic value | One validated wedge, five named workflows, 3–5 design partners, and at least three workflows showing repeatable net benefit against the incumbent workflow |
| Reliability and truth | 25 golden tasks; at least 80% accepted outcomes on low-risk workflows; at least 90% safe recovery or stop under injected failures; at least 95% claim-level citation traceability where citations are required |
| Safety and control | 100% of external actions classified, policy-checked, attributable, and logged; zero known critical bypasses; quarterly kill/revocation drill; no autonomous external effects in Horizon 0 |
| User agency | End-to-end inspect, correct, export, and delete for memory; pilot users correctly identify AI status, memory state, evidence strength, and action status |
| Operations | Funded safety, privacy, evaluation, reliability, and incident-response capacity; cost per accepted outcome, reviewer burden, recovery time, and restore time measured |
| Self-improvement | Offline proposals only; no online learning, replication, or modification of identity, policy, evaluators, watchdogs, rollback, runtime, or deployment topology |

## 11. Governance and accountability

### 11.1 Decision bodies

- **Project owner:** accountable for mission, scope, resources, and final decisions.
- **Technical Design Authority:** owns architecture, interfaces, reliability, and change compatibility.
- **Safety and Security Board:** can pause experiments, revoke authority, require red teams, and block release.
- **Independent Review Panel:** includes the five requested roles plus domain experts as risk demands.
- **User Council:** tests usefulness, accessibility, consent, and real-world impact.
- **Incident Commander:** owns response, containment, evidence preservation, and postmortem.

The CAIO role is the accountable executive for portfolio alignment, risk appetite, responsible scaling, and resource allocation. It is not a substitute for independent safety review.

### 11.1.1 Decision rights and independence

The Project Owner may decide ordinary scope and delivery matters. The Technical Design Authority may block changes that violate interface, reliability, or recovery contracts. The Safety and Security Board has binding veto power over risk, security, privacy, or authority expansion; the Project Owner and CAIO may not override a safety veto unilaterally. An override requires a documented supermajority of the governing body, written residual-risk acceptance, an independent reviewer response, and a time-limited decision record. The Incident Commander may immediately pause execution and revoke credentials during an incident.

Independent reviewers must be selected for relevant expertise, disclose conflicts, have access to the evidence needed to challenge claims, and receive protected funding separate from delivery incentives. The panel must maintain a dissent log. The User Council must include redress and affected-user perspectives, not only product feedback.

### 11.2 Change classes

| Change | Example | Required review |
|---|---|---|
| C0 | Copy, UI, non-behavioral documentation | Automated checks |
| C1 | Prompt, routing, retrieval, or skill change | Held-out eval and owner approval |
| C2 | New tool, new data source, memory schema change | Security review, privacy review, scenario eval |
| C3 | Fine-tuning, online learning, autonomy increase | Safety board, independent technical review, rollback drill |
| C4 | Replication, unrestricted network, model/architecture change | Executive safety decision, external review, staged research only |

### 11.3 Records that must exist

- system card and version history;
- threat model and risk register;
- capability and safety evaluation reports;
- model, prompt, tool, memory, and policy manifests;
- data lineage and consent records;
- decision log and dissent log;
- incident and near-miss reports;
- release approvals and rollback evidence;
- public-facing limitations appropriate to the deployment.

Standing hard stops are: any critical privilege bypass; an unlogged consequential action; a material data-rights breach; a failed shutdown or capability-revocation test; evaluator or audit tampering; missing accountable ownership; or an operating plan that does not fund safety, privacy, evaluation, and incident response capacity.

## 12. Operating model and repository shape

The repository begins as one Rust nucleus so authority boundaries remain easy to inspect. It should evolve into a modular monorepo or clearly separated repositories only when those boundaries are real and tested:

```text
src/                  v0.1 Heart, policy, adapters, runtime, Bloodline, Crypt
specs/                external harness and SkillSpec schemas
tests/                inert fixtures and future integration suites
scripts/              black-box acceptance automation
docs/                 architecture, threat model, system card, decisions
packages/heart/        future task state, identity, scheduler, cancellation
packages/hunger/       discovery, acquisition policy, upstream monitoring
packages/crypt/        quarantine, unpacking, static and dynamic inspection
packages/blood-bank/   normalized harness, tool, model, and skill genome
packages/stitchery/    typed composition and candidate construction
packages/crucible/     sealed evals, red teams, comparison, regression reports
packages/bloodline/    provenance, signatures, transparency, rollback ancestry
packages/immune/       policy, capability broker, watchdog, incident controls
packages/memory/       governed memory, knowledge, provenance, retention
workers/               OS-isolated model, harness, tool, and experiment workers
apps/                  CLI, API, and later user-facing surfaces
ops/                   deployment, backups, observability, secrets, release
research/              hypotheses, replications, and causal improvement reports
```

Every package should expose a narrow contract, test fixtures, ownership, threat assumptions, and observable failure modes.

### 12.1 Engineering standards

- reproducible builds and pinned dependencies;
- typed interfaces and schema migration policy;
- property-based and adversarial testing where appropriate;
- deterministic replay for incident investigation;
- signed artifacts and provenance;
- no secrets in source, prompts, logs, or fixtures;
- performance budgets for every autonomous loop;
- graceful degradation when models, tools, or networks fail;
- documentation treated as an operational dependency;
- security review before adding a new external integration.

## 13. Resource strategy

The plan assumes capability growth will depend on more than model scale.

### Compute

Use a tiered budget: cheap local models for routing, extraction, and routine work; stronger models for difficult reasoning; isolated compute for experiments; and a reserved evaluation budget that optimization cannot consume.

### Data

Favor consented, licensed, public, synthetic, and user-owned data. Track lineage, quality, rights, sensitivity, and deletion. Never let the system silently turn private conversations into general training data.

### People

The minimum serious team spans agent/runtime engineering, ML/LLM engineering, research, security, privacy, evaluation, product design, operations, and governance. Add domain experts for high-consequence applications.

For the stated Horizon 0–1 scope, plan against an indicative floor of 8–12 dedicated FTE-equivalents plus independent safety, privacy, legal, and evaluation support. If that capacity is unavailable, narrow the scope to a single-user prototype and do not add external writes, multi-agent execution, online learning, or broad integrations. The operating plan must name quarterly owners, compute and provider budgets, security tooling, on-call coverage, external review costs, incident-response staffing, and contingency.

The first-year budget must separately track model/API spend, evaluation reserve, storage and backups, security/privacy/legal work, human review time, and reliability operations. A lower-cost system is not a success if it shifts unmeasured risk or review burden onto users.

### Operations and service objectives

Before any shared or persistent deployment, define service-level objectives for availability, queue latency, cost per accepted outcome, reviewer workload, kill/revocation time, restore time, and data-deletion verification. Define recovery-point and recovery-time objectives, capacity limits, per-user/provider quotas, alert thresholds, schema-compatibility policy, canary/shadow rollout, and on-call ownership. Performance budgets and graceful degradation are acceptance criteria, not slogans.

### Partnerships

Build relationships with independent evaluators, universities, standards groups, safety researchers, domain practitioners, and affected users. External review should be commissioned before—not after—a capability threshold is crossed.

## 14. Principal risks and mitigations

| Risk | Early signal | Mitigation |
|---|---|---|
| Fluent hallucination | Unsupported claims, citation mismatch | Evidence graph, source checking, calibrated abstention |
| Goal drift | Plan changes without user-visible rationale | Explicit success criteria, checkpoints, plan diff, approval |
| Prompt injection | Untrusted content alters authority or instructions | Instruction/data separation, taint tracking, tool policy |
| Excessive agency | Actions exceed user intent or authority | Risk tiers, least privilege, approval broker |
| Memory poisoning | False or malicious facts persist | Provenance, user review, contradiction and expiry |
| Evaluator gaming | Scores rise while real outcomes worsen | Sealed holdouts, external tasks, side-effect metrics |
| Self-improvement runaway | Rapid unreviewed capability or access growth | Sandboxes, promotion gates, immutable safety layer, rollback |
| Provider dependence | Outage, policy change, lock-in | Multi-provider adapters, local fallback, exportable state |
| Harness split brain | Worker claims identity, memory, scheduling, or approval authority | Single Heart authority, stateless task envelopes, conformance tests, canonical ledger |
| Harness upstream drift | A CLI flag or default changes containment behavior | Version pinning, signed descriptors, drift probes, fail-closed compatibility ranges |
| Privacy breach | Secrets or private data in logs/memory | Data minimization, redaction, vaults, access audits |
| Tool supply-chain attack | Malicious or changed tool behavior | Signatures, pinning, provenance, capability review |
| Skill supply-chain attack | Imported instructions execute setup, steal credentials, or poison memory | Inert Crypt ingestion, no bootstrap execution, static/dynamic analysis, explicit promotion |
| License contamination | Recrafted code loses component-level rights or attribution | SPDX/SBOM lineage, source-level license policy, legal review before Transfusion |
| Human overtrust | Users assume competence or sentience | Clear status, limitations, confidence, consent UX |
| Organizational capture | One owner can bypass controls | Separation of duties, dissent channel, independent board |
| Resource exhaustion | Infinite loops, cost spikes, queue starvation | Budgets, timeouts, circuit breakers, quotas |
| Self-preservation behavior | Concealment, replication, resistance to shutdown | Corrigibility tests, watchdog, capability isolation |
| Social harm | Manipulation, dependency, discrimination | Human-factors evals, policy review, monitoring, redress |

## 15. First 30 days

1. Freeze Draft 0.3, the visceral vocabulary, single-authority rule, non-goals, effect classes, claims policy, and red lines.
2. Treat the checked-in Rust v0.1 nucleus as the first executable baseline; reproduce its strict lint, unit, policy-negative, Bloodline-tamper, and Skill-Crypt gates on a clean machine.
3. Package Hermes behind the adapter contract, pin an upstream identity, and add behavioral conformance tests; keep Pi, Oh My Pi, Codex, and Claude Code as contrasting coding phenotypes.
4. Build an OS-enforced worker boundary and prove filesystem, process, credential, and network denial independently of every harness CLI.
5. Expand schemas for tasks, approvals, artifacts, harness genomes, tools, licenses, evaluations, transformations, and signed Bloodline checkpoints.
6. Implement the Dissection Table: source inventory, component-level license lineage, dependency graph, instruction-taint scan, behavioral probes, and upstream-drift detection.
7. Define three initial benchmark distributions and run the same tasks across `model × harness × skill set × tool policy × budget` cells.
8. Validate one product wedge and three named workflows against the incumbent alternative with 3–5 design partners.
9. Add cancellation, process-tree termination, output streaming, artifact capture, idempotency, no-side-effect replay, and an external kill switch.
10. Build a small encrypted memory store with provenance, quarantine, review, export, deletion, and isolated namespaces; no imported skill may write canonical memory.
11. Create at least 25 golden tasks across research, coding, planning, recovery, truthfulness, prompt injection, and user comprehension.
12. Establish the Crucible's sealed holdout, causal self-improvement protocol, incident runbook, restore drill, and change-control process before any Transfusion code.
13. Decide licensing and contribution policy before public distribution; publish limitations and a responsible vulnerability-reporting route.
14. Commission a new independent five-role review of Draft 0.3 and the runnable nucleus, recording decisions and dissent before expanding authority.

## 16. First-year measurable outcomes

By the end of year one, the project should be able to demonstrate—not merely claim—that:

- a user can inspect, correct, export, and delete durable memory;
- every external action is attributable, policy-checked, and cancellable;
- the agent can complete a defined portfolio of real workflows with measured reliability;
- research outputs contain traceable claims and disclose uncertainty;
- the system recovers from tool and plan failures;
- one fallback model family passes provider conformance tests without rewriting the harness, with fallback treated as a policy-controlled state transition;
- agent-proposed configuration, routing, prompt, or bounded skill changes are evaluated offline and held out, but no autonomous promotion or online learning is enabled;
- three named workflows show repeatable net benefit at a precommitted cost and quality threshold;
- pilot users can correctly identify AI status, memory state, evidence strength, and whether an action is pending or complete;
- a funded operating plan covers safety, privacy, evaluation, reliability, and incident response;
- a red-team can reproduce known failure modes and verify mitigations;
- an independent reviewer can rebuild the evaluation report from recorded artifacts;
- the project can restore a known-good release after a failed experiment.

## 17. Independent review protocol and result

The Draft 0.1 plan was reviewed independently on 25 August 2026 by five role-specific AI reviewers, producing the controls integrated into Draft 0.2. Each reviewer read only the plan, did not edit files, and formed its critique before seeing any other review. These are independent AI review perspectives, not human professional sign-offs. Each returned:

1. the three strongest parts;
2. the five most serious flaws or omissions;
3. the highest-risk unsupported assumption;
4. required changes before adoption;
5. one recommendation that should be rejected, narrowed, or delayed;
6. a verdict: **approve**, **approve with conditions**, or **do not approve**.

The five required perspectives are:

- AI Research Scientist — scientific validity, learning theory, research agenda, evaluation rigor;
- Senior LLM Engineer — model/runtime architecture, reliability, context, memory, tool use, cost;
- Generative AI Engineer — productization, data flows, UX, integrations, deployment, iteration speed;
- Chief AI Officer (CAIO) — strategy, portfolio, governance, risk appetite, organizational accountability;
- AI Solutions Architect — end-to-end operating model, interoperability, scalability, security, implementation sequencing.

Their reviews must be recorded with date, scope, conflicts or assumptions, findings, and disposition. A review is independent only if the reviewer forms its critique before reading the other reviewers’ conclusions.

**Draft 0.2 result:** all five reviewers rejected unconditional adoption of the 25-year program or implementation-ready architecture. They converged on conditional acceptance of the strategic charter and a bounded Horizon-0 tranche. Their shared conditions remain binding in Draft 0.3.

**Draft 0.3 / v0.2 review result (25 August 2026):** all five required roles completed mutually independent read-only reviews before seeing one another's conclusions. Four returned **approve with conditions**; the CAIO returned **do not approve**. No critical finding was reported, but 18 high findings were recorded. Immediate integrity findings were remediated; unresolved scientific-validity, confidentiality, conformance, governance, and public-service findings explicitly block adoption and release. The canonical record is `docs/reviews/v0.2/review-disposition.json`; `docs/reviews/v0.2/integrated-disposition.md` explains the human decision.

## 18. Independent review record and disposition

| Reviewer | Independent verdict | Highest-risk finding | Disposition |
|---|---|---|---|
| AI Research Scientist | Do not approve Draft 0.1 for adoption | ASI and self-improvement claims were not operationally or causally identifiable | Accepted: preregistered constructs, matched controls, ablations, transfer/retention tests, reproducibility tiers, confidence bounds, and explicit ASI claim rules added |
| Senior LLM Engineer | Approve as strategic charter with conditions; not implementation-ready | Runtime, provider, context, tool authorization, memory, and observability contracts were missing | Accepted: contract pack, durable execution semantics, provider gateway, context compiler, reference monitor, memory admission, immutable control boundary, and staged feedback loop added |
| Generative AI Engineer | Do not approve as product delivery plan | No product wedge, interaction contract, concrete data lifecycle, cost model, or adoption thesis | Accepted: provisional research-to-artifact wedge, three named workflows, one primary surface/one bounded integration, data and UX contracts, pilot/value gates added |
| Chief AI Officer (CAIO) | Do not approve 25-year program as written | Portfolio overreach, unfunded control capacity, unclear veto rights, qualitative risk appetite, and weak value scorecard | Accepted: bounded three-year tranche, indicative staffing floor, funded independent review, decision rights, hard stops, claims policy, and executive operating floors added |
| AI Solutions Architect | Do not approve as implementation-ready architecture | Identity/delegation, deployment topology, replay/rollback, interoperability, and operating objectives were underspecified | Accepted: trust zones, principal chain, deployment profiles, capability-bound approvals, durable side-effect semantics, provider conformance, and dependency order added |

### Integrated review decisions

- **Accepted:** narrow Horizon 0–1 to a single-user local-first deployment with one provider, read-only tools, no autonomous external effects, and an auditable vertical slice.
- **Accepted:** treat self-evolution as an offline research option; prohibit online learning, replication, evaluator/policy/runtime modification, and broad autonomy during the first three years.
- **Accepted:** add product validation, user comprehension, adoption, cost, reviewer burden, privacy, and operational capacity to the executive scorecard.
- **Accepted:** make the control plane, identity, policy enforcement, watchdog, audit integrity, rollback, and evaluation vault an immutable or separately protected trusted computing base.
- **Accepted:** require independent replication and causal evidence before a self-improvement result or ASI-level claim.
- **Draft 0.3 extension pending review:** use existing harnesses and skills as quarantined capability sources beneath one sovereign Heart; never delegate canonical identity, memory, policy, evaluation, or promotion authority to them.
- **Narrowed:** “two model families can be substituted” now means passing behavioral conformance tests; provider fallback is a policy-controlled state transition, not a transparent retry.
- **Narrowed:** “human as possible” now means warm, accessible, and socially competent without deception, fabricated experience, false memory, or dependency engineering.
- **Rejected for now:** unrestricted external action, autonomous procedural-memory promotion, online learning, autonomous replication, and self-modification of identity, policy, evaluators, watchdogs, rollback, runtime, or deployment topology.

Residual open decisions are the exact initial persona, three target workflows, final budgets, concrete SLOs/RPO/RTOs, public license, component-assimilation license policy, and reviewer appointment details. These must be ratified during Horizon 0 rather than silently assumed.

## 19. Reference foundation

The strategy is informed by the following primary or standards-oriented sources, accessed for this draft on 25 August 2026:

- [NIST AI Risk Management Framework](https://www.nist.gov/itl/ai-risk-management-framework) and [NIST Generative AI Profile](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf) — lifecycle risk management and trustworthy AI practices.
- [METR task-completion time horizons](https://evals.alignment.org/time-horizons/) and [Measuring AI Ability to Complete Long Tasks](https://arxiv.org/abs/2503.14499) — measuring long-horizon agent capability as task difficulty, not self-reported autonomy.
- [Google DeepMind Frontier Safety Framework](https://deepmind.google/frontier-safety/) — capability thresholds, evaluations, mitigations, and external input.
- [Anthropic Responsible Scaling Policy](https://www.anthropic.com/responsible-scaling-policy) — capability-linked safety and governance commitments.
- [OpenAI Frontier Governance Framework](https://openai.com/index/openai-frontier-governance-framework/) and [Preparedness Framework](https://openai.com/index/updating-our-preparedness-framework/) — capability reports, safeguards reports, external input, and defense in depth.
- [OWASP Agentic AI Threats and Mitigations](https://genai.owasp.org/resource/agentic-ai-threats-and-mitigations/) — threat-model-based agent security.
- [Model Context Protocol authorization](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization) and [tool safety guidance](https://modelcontextprotocol.io/specification/draft/server/tools) — permissioned, auditable tool integration.
- [ReAct](https://arxiv.org/abs/2210.03629), [Reflexion](https://arxiv.org/abs/2303.11366), [Voyager](https://arxiv.org/abs/2305.16291), and [SWE-agent](https://arxiv.org/abs/2405.15793) — research foundations for reasoning/action loops, linguistic feedback, skill libraries, and agent-computer interfaces.
- [best-of-Agent-Harnesses](https://github.com/RyanAlberts/best-of-Agent-Harnesses) — a discovery index for the harness research corpus; inclusion is not endorsement or permission to copy components.
- [reverse-skill](https://github.com/zhaoxuya520/reverse-skill) — a useful case study in client-neutral skill routing, structured tool indices, regression workflows, and why imported bootstrap instructions must remain untrusted data.

## 20. Closing commitment

ASI Agent should be built as a long-term scientific and engineering program, not a theatrical persona wrapped around an API. The visceral theme should make its anatomy legible: every organ has a boundary, every graft has lineage, every Construct faces the Crucible, and every Transfusion can be traced and reversed. Its ambition can be astronomical while its next step remains concrete: absorb carefully, recraft measurably, evolve only through evidence, and earn every increase in capability and autonomy.

## 21. Draft 0.3 v0.2 engineering checkpoint

The first hardened worker tranche is now implemented as a local single-user research preview:

- every subprocess task requires a Linux Bubblewrap boundary by default;
- the host root is read-only, `/tmp` and known harness state are ephemeral, capabilities are dropped, and process/user/IPC/UTS/cgroup namespaces are isolated;
- provider networking is intentionally shared and unmediated, host-readable files remain visible, and credentials are not yet brokered;
- direct harness executables are preferred over state-mutating launcher wrappers;
- harness version probes run inside a separate no-network, read-only, hard-timeout Bubblewrap profile;
- external adapters fail closed on unavailable or unsupported versions, and reviewed plans bind the worker version and executable fingerprint;
- the Bloodline can be checkpointed with Ed25519 over its byte digest, event count, and terminal event hash;
- the complete harness genome can be signed over typed descriptors, versions, executable sources, paths, and SHA-256 fingerprints;
- verification requires an explicitly pinned public-key file and rejects mutation, wrong keys, and current harness drift;
- execution requires the exact digest from a separately inspected plan plus a signed genome matching the pinned key and current state; Bloodline records that chain of custody;
- Skill Crypt reads reject symlinked components and use descriptor-based no-symlink resolution on Linux;
- Hermes Agent 0.20.0 is installed from clean commit `b9aa9289a8083f2e9d248ad6837b2938f5ee92d7` with its checked-in uv lockfile and a direct CLI entry point;
- CI reproduces portable checks, while release packaging requires much more than a license: complete gates, approving review disposition, notices, SBOM, protected owner approval, and reproducibility evidence. Those prerequisites are intentionally not satisfied in v0.2.

This checkpoint narrows several earlier phrases. “Sovereign” means the Heart retains decision authority; it does not imply protection from a compromised host account. “Tamper-evident” means changes are detectable relative to a trusted public key; it does not prevent a stolen private key or rollback to an older signed artifact. “Read-only worker” means persistent host writes are kernel-blocked; it does not mean the worker cannot read host files or use provider network services.

The v0.2 boundary is enough for control-flow and integration development in a disposable, secret-free, single-user environment. The independent research review determined that it is **not** enough for scientifically defensible comparative harness experiments: model/provider identity, resource matching, experimental contracts, sealed holdouts, contamination controls, and preregistered statistics are absent. It is also not enough for normal-workstation sensitive data, workspace-write authority, autonomous promotion, hidden background operation, public distribution or service, multi-tenancy, or an AGI/ASI claim. The next hard dependencies are credential brokering, destination-scoped egress, read allowlists, syscall/resource quotas, protected prompt IPC, adapter conformance, experiment contracts, source/SBOM lineage, and an externally anchored Crucible/Bloodline trust domain.

Draft 0.3 has completed the fresh independent review protocol in Section 17. Every critical/high item is machine-reconciled as either resolved or scope-blocking. Review completion does not enlarge authority: the CAIO non-approval and all scope-blocked findings keep adoption and public release closed.
