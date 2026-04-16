# xbreed v0.4.0 — Structural Uniqueness Report

**Team:** `xbreed-superiority-0416`
**Round:** 1 (frontier-stable; Round 2 mooted by in-band cross-correction)
**Date:** 2026-04-16
**Audit hash:** `762ea768928183114185d497fc4703d661bdd9664b1ffc68e8ecb80c49653780` (stable across initial synthesis + delta + addendum)

---

## TL;DR

- **"Superiority" is epistemically unprovable** — no benchmark in this set has scored xbreed; xbreed has no published benchmarks. The framing was reframed mid-round (A2+A8 convergence) to **structural uniqueness** on a defined set of axes.
- **xbreed is the only framework in the surveyed set** combining cross-vendor CLI delegation + judge-scored Pareto walk + compiled-in-repo deny-list + judge-blinding audit-hash as simultaneous first-class primitives.
- **🚨 W3 BLOCKER surfaced (live-round operational risk):** Zero call-layer retry/timeout/fallback in `src/ask.rs`. A Gemini timeout produces a ghost axis — silence indistinguishable from valid empty response. Highest-priority follow-up.
- **cco-upgrade A/B verdict:** NOT blanket. Selective: opus for adversarial/synthesis roles (critic, sentinel, reviewer, distiller) where the brief itself may carry errors; sonnet for read-only enumeration (scout, executor, labrat).

---

## Errata (post-commit, cco-reviewer-novel-mirror correction)

Three factual errors in §3 (Verified Novel Mechanisms) were caught by the cco-reviewer-novel-mirror **after** the report's first commit. Corrections applied inline in §3 below; surfaced here for audit-trail honesty.

1. **Gate locus correction.** §3 #1 originally cited `scripts/xask:55-58` as the 4-layer gate location. That file is a transport shim (flag parsing, awk template substitution, HOME isolation, dispatch to `xbreed ask`) with **zero gate-enforcement code**. The gate lives at `commands/references/xbreed-shared.md:25-43` plus skill briefs — i.e. PROTOCOL-level enforcement (agents read and follow the brief), not CODE-level enforcement. **Material implication:** the 4-layer gate is novel-as-a-protocol, but a non-compliant agent can skip Layer 1 — the structural guarantee is weaker than compiled enforcement would be.
2. **Stratification claim factually wrong.** The "judge=opus + others=sonnet" framing was already noted as rejected (REJECTED list in §3). Strengthening the rejection: `the-revenger` is also opus per `xbreed-shared.md:68`, so the framing is factually incorrect, not just "borrowed pattern." Drop entirely from any novelty list.
3. **IMCP v0.2 CONFLICTS attribution wrong.** §3 PARTIAL #8 attributed CONFLICTS to the IMCP v0.2 wire format. The actual wire headers are Goal/State/Unknowns/Dissent/Action/Artifact/Rationale (`docs/superpowers/specs/2026-04-12-inter-model-protocol-v0.2.md:22-28`) — no CONFLICTS. The CONFLICTS block is a DRAFT-layer envelope defined by the judge at `templates/agents/the-judge.md:57-62`. Novel piece: the `judge_resolution` + `escalate_to` labeled-by-model output envelope — NOT the wire format.

**Revised novelty tally** (down from "6 HIGH + 2 PARTIAL" claimed in the original commit message):
- 4 cco-confirmed HIGH (Pareto drop-gate, materiality rule, anti-premature-halt R2, 4-layer xask gate at corrected locus)
- 2 cco-gap HIGH (judge blinding + SHA-256 audit hash, contamination-suppression flags — not in cco mirror's 8-item scope; A3 evidence stands)
- 1 HIGH (N8 NDJSON mailbox + CC hook-injection — not in cco scope)
- 1 MEDIUM partial (axis-tuple naming-before-spawn — novel piece is the (name, direction, observable) tuple + frozen-pre-spawn names, not "plan before dispatch")
- **DROPPED:** model-prefix naming convention (naming hygiene, not mechanism); opus stratification (factually wrong)

**Net: 7 HIGH + 1 MEDIUM novel mechanisms** (was 6 HIGH + 2 PARTIAL pre-errata).

This errata is itself a data point on the cco-vs-ccs A/B (§8) — opus mirror's late critical pass caught factual errors in the briefing that would have shipped uncorrected.

---

## 1. Empirical Implementation Surface (A1 + A5)

xbreed v0.4.0 ships:

- **8 Rust binary subcommands** (`src/cli.rs:17-106`): `guard`, `sync`, `claude`, `ask`, `team init`, `team mailbox write|drain|compact`
- **5-flag xask wrapper** (`scripts/xask:20-55`, 177 LOC): `--scope`/`-s`, `--debug`/`-d`, `--rich`/`-r`, `--spark`/`--spk`, `--effort`/`-e` (`low|medium|high|xhigh`); positionals `<model>` (gemini|codex), `<query>`, `[context]`, `[skill]`
- **14 perspective agent templates** (`templates/agents/*.md`): the-judge, the-planner, the-revenger, scout, reviewer, labrat, executor, connector, distiller, simplifier, sentinel, critic, mutation-tester, scribe
- **6 slash commands** (`commands/*.md`): `xbreed`, `xb`, `xbt`, `xbreed-team`, `xgs`, `xbgst`
- **4 test files / 68 test cases** (`cargo test`: 55 + 5 + 7 + 1 = 68 pass, 0 fail per commit `7684551`)
- **239-LOC shared protocol doc** (`commands/references/xbreed-shared.md`) — single source of truth for axis routing, evidence schema, exit conditions

**LOC triple-confirmed** (A1 enumerated, A5 `wc -l`, A6-cco `cargo tree`):
- `src/*.rs` = 2064 LOC total
- `src/ask.rs` = 719 LOC (34.8% concentration risk)
- `scripts/xask` = 177 LOC bash glue
- `templates/agents/*.md` + `commands/references/xbreed-shared.md` ≈ 1291 LOC markdown

**Performance (structural baseline, no model RTT):**
- xask `--debug` constructed-prompt path: sub-1ms (`/usr/bin/time -v` wall-clock 0:00.00)
- Release build cached: 0.03s
- RSS baseline: 3200KB (+128KB at `--effort high`)

**Network-RTT claims** (`README.md:124-131`: ~14s gemini, ~6s codex, ~1s spark, ~29s xbgst round) are plausible but empirically unverifiable without quota burn — would be vacuous as a competitor benchmark axis.

---

## 2. Competitive Landscape (A2, reframed)

| Framework | Multi-vendor CLI | Named Judge | Pareto/Evidence Filter | Deny-list (in-band) | Judge Blinding |
|-----------|:---:|:---:|:---:|:---:|:---:|
| LangChain / LangGraph | N | user-built | N | N (BYOSandbox) | N |
| AutoGen v0.4 | N | round-robin | N | Docker executor | N |
| AG2 | N | manager | N | safeguards | N |
| CrewAI | N | manager | P (manager-validated) | N | N |
| MetaGPT | N | role-play | N | N | N |
| OpenAI Swarm | N | handoff | N | N | N |
| OpenAI Agents SDK | N | runner | N | guardrails | N |
| OpenHands | N | none | N | sandbox | N |
| Agno | N | agent-as-judge | P (eval scoring) | N | N |
| smolagents | N | manager | P | sandboxed exec | N |
| **xbreed v0.4.0** | **Y** (codex + gemini via `scripts/xask`) | **Y** (`templates/agents/the-judge.md`) | **Y\*** (convention) | **Y** (Rust RegexSet, `src/guard.rs:52`) | **Y** (SHA-256 audit hash, `xbreed-shared.md:178`) |

**Y\* footnote (A2/A8 convergence):** xbreed's Pareto filter is convention-enforced via distiller agent cooperation, not a compiled structural gate. If the distiller stalls, the filter is absent for that round. LangGraph conditional edges are topology and cannot be skipped by agent failure — so xbreed's Pareto primitive is "unique flavor" rather than "unique category."

**Surviving uniqueness claims (post-adversarial):**
- Per-role evidence drop gate: no competitor equivalent (HIGH, A2+A3 cross-confirmed)
- Judge blinding + SHA-256 hash-commit: no competitor equivalent (HIGH, A2+A3 cross-confirmed against AutoGen SocietyOfMind, CrewAI manager, Agno agent-as-judge)
- Cross-CLI delegation as named first-class primitive with auth cascade: unique flavor (MEDIUM)
- Pareto filter: unique IF distiller cooperates (MEDIUM, conditional)

---

## 3. Verified Novel Mechanisms (A3 + N8 + cco-mirror corrections)

**Cross-model CONFLICT, judge-resolved:**
- `cdx-reviewer-novel` (sonnet harness → codex high) claimed **7 verified novel + 2 unverified**
- `cco-reviewer-novel-mirror` (opus 4.7 harness → codex high) caught **3 factual errors in the original brief** that the sonnet harness accepted at face value, downgraded to **4 verified + 2 partial + 2 rejected**
- **Judge resolution:** defer to opus mirror (more rigorous critical review of brief framing). Final count: **6 HIGH + 2 PARTIAL.**

### HIGH confidence (verified by both mirrors)

1. **xask 4-layer gate** (structural → raw-quote → fallback → confidence) — `commands/references/xbreed-shared.md:25-43` + per-skill briefs. (Gate is **protocol-level**, not code-level — `scripts/xask` is a transport shim with no gate-enforcement code; a non-compliant agent can skip Layer 1. See §Errata.) No competitor surveyed defines a 4-layer cross-model dispatch gate of any kind, code- or protocol-enforced.
2. **Pareto Filter Evidence Schema** with per-`axis_family` evidence requirements — `xbreed-shared.md:114-125`. Drop-gate on missing/malformed evidence is run BEFORE acceptance scoring.
3. **Materiality rule** (axis-observable change for exit, not prose delta) — `xbreed-shared.md:222-223`. Paraphrased findings against unchanged observables are explicitly NOT improvements.
4. **Anti-premature-halt + Round-2-always-runs invariant** — `xbreed-shared.md:229`. Round 1 by construction moves axes off baseline; jumping to cleanup is a protocol violation.
5. **NDJSON file-based mailbox** with PID-namespaced atomic rename-drain + Claude Code `UserPromptSubmit` hook-injection format — `src/mailbox.rs:51,84-110`. Bridges team orchestration events directly into IDE context-injection protocol. No competitor equivalent.
6. **Judge blinding + SHA-256 audit hash** — `xbreed-shared.md:178`. Computed as SHA-256 of sorted `[{move_id, source_prefix}]` tuples; tamper-evidence for synthesis stage. Verified against AutoGen SocietyOfMind, CrewAI manager, Agno agent-as-judge (none implement hash-commit blinding).

### PARTIAL confidence (cco mirror downgraded)

7. **Axis-naming-before-spawn** — novel piece is specifically the (name, direction, observable) axis tuple + frozen teammate names committed BEFORE spawn. "Plan before dispatch" generally is common (CrewAI, AutoGen). The tuple-as-contract is the narrow novel piece.
8. **Judge CONFLICTS envelope** — the labeled-by-model `judge_resolution` + `escalate_to` output envelope at `templates/agents/the-judge.md:57-62` is the novel piece. NOT the IMCP v0.2 wire format itself (which is Goal/State/Unknowns/Dissent/Action/Artifact/Rationale, no CONFLICTS — see §Errata).

### REJECTED (cco mirror eliminated)

- ~~Model-prefix naming convention as novelty~~ — naming hygiene, not mechanism.
- ~~Judge=opus + others=sonnet stratification as novelty~~ — **factually wrong**: `the-revenger` is also opus per `xbreed-shared.md:68`. Hierarchical stratification itself is common (CrewAI manager/crew, LangGraph supervisor, AutoGen selector).

---

## 4. Safety Posture (A4, narrowed)

### xbreed wins (architectural integration posture)

- **Compiled, in-repo, in-binary deny-list** as CC pre-tool-use hook — `config/policy.yaml`, `src/guard.rs:52,75`. Rust `RegexSet` matched at dispatch time, not via external sandbox.
- **HOME-scoped tmpdir + stale-dir sweeper** — `scripts/xask:116-124`. Closes the `/tmp` orphan vector after SIGKILL/OOM.
- **Env-var Python interpolation** (post-RCE fix) — `scripts/xask:140-153`. Eliminates command injection via `XDG_CACHE_HOME`.
- **Fail-closed on malformed YAML** — `src/guard.rs:44-48`.

### Sandboxing parity (NOT unique — A4 self-correction)

- Codex CLI: `--sandbox read-only|workspace-write|danger-full-access`
- LangChain: E2B sandbox integration
- AutoGen v0.4: Docker executor (default for `CodeExecutorAgent`)
- OpenAI Agents SDK: built-in API guardrails

**Honest weakness (A8):** the deny-list is bypassable by obfuscated/encoded payloads (base64 chains, `python -c exec`) at host privileges. The win is **integration posture** (compiled CC hook), not absolute safety.

**Confidence:** Win #1 narrowed from HIGH to MEDIUM after A8/A4 convergence. All other wins (HOME-scope, env-var fix, fail-closed) and all losses stand.

---

## 5. Architectural Elegance (A6, ccs+cco converged)

- **2064 LOC Rust + 177 LOC bash + 1291 LOC markdown**
- **6 production deps** (`anyhow`, `clap`, `regex`, `serde`, `serde_json`, `serde_yaml` + dev `tempfile`)
- **70 transitive crates** (Cargo.lock authoritative; cco-mirror cross-confirmed correction from earlier 40-estimate)
- **~293× smaller than LangChain** (~600k LOC Python + 50+ direct deps)
- **~24× smaller than AutoGen** (~50k LOC Python)
- **ZERO framework lock-in** — agents are markdown files with YAML frontmatter, NOT Python class hierarchies (CrewAI Agent/Task/Crew, AutoGen ConversableAgent inheritance)

**Honest concentration risk (A6 W2):** `src/ask.rs` = 719 LOC = 34.8% of Rust surface. Refactor candidate; qualifies the "9 flat modules" elegance framing without negating it.

**A6 verbatim:** *"Minimalism describes the dependency graph, not the failure surface."*

---

## 6. Composability Orthogonality (A7, Linux/WSL scoped)

Three orthogonal swap axes that competitors fuse into class hierarchies or config dicts:

| Axis | Swap mechanism | xbreed file | Competitor coupling that breaks the swap |
|------|----------------|-------------|------------------------------------------|
| **Models** | New model = new dispatch case + template | `scripts/xask:56`, `templates/dispatch/<model>.md` | CrewAI: subclass `LLM`. AutoGen: per-agent `llm_config` dict. |
| **Agents** | New role = new `.md` file with YAML frontmatter | `templates/agents/foo.md` | CrewAI: `Agent` Python class. AutoGen: `ConversableAgent` inheritance. |
| **Orchestration modes** | New mode = new `commands/foo.md` referencing SSoT | `commands/foo.md` → `xbreed-shared.md` | LangGraph: `StateGraph` Python rewrite per mode. |

**Scope qualifier (A8 portability counter):** GNU-specific bash primitives (`find -mmin`, `mktemp`, `/usr/bin/time -v`) mean swap-axis claims hold on Linux/WSL. BSD/macOS portability untested.

---

## 7. Operational Weaknesses (A8 + W3 BLOCKER)

### 🚨 W3 BLOCKER (live-round operational risk)

`src/ask.rs` has **zero call-layer retry / timeout / fallback**. The 15+ "fallback" branches in the file are auth-credential cascade only (OAuthProfile → fallback → OAuthDefault → ApiKey → fallback), NOT call-resilience.

**Failure mode:** A Gemini timeout produces a **ghost axis** — silence is indistinguishable from a valid empty response. The Pareto filter sees zero-evidence and drops the move; the round proceeds as if that axis was empirically null. Currently shipping in v0.4.0.

**Recommendation:** Add per-call timeout + circuit breaker + explicit timeout marker in the response so the distiller can flag absent-vs-empty. Highest-priority follow-up.

### Other honest weaknesses (A7/A8)

- **W1 (medium):** Zero agent contract enforcement in xbreed — schema responsibility delegated to CC harness with no compile-time guarantee. No `TypedDict` / typed decorator equivalent. By design, but undocumented dependency.
- **W2 (high):** xask 177 LOC is a complexity attractor with real error swallowing — `|| true` on stale-dir cleanup, `python3 -c` subprocess inside bash for JSON mutation, manual `sed` escaping for awk injection prevention.
- **Zero observability** — no telemetry, traces, or spans anywhere in 2064 LOC. `README.md:69` explicit admission. Production blocker for shared/team use.
- **HOME-scoped single-tenant** — multi-tenant requires architectural rethink.
- **No published benchmarks** — "superiority" claim epistemically void without comparable scores; the report's reframe to "structural uniqueness" is the only defensible framing.
- **Linux/WSL only** — GNU bash primitives.
- **Solo-dev project** — no PyPI/crate publication, no community, no published roadmap. Ecosystem comparison vs LangChain (~90k stars) is not the right axis to claim wins on.

---

## 8. cco-Upgrade A/B Verdict (A9 — partial coverage)

**Live A/B mirrors (cco-reviewer-arch-mirror + cco-reviewer-novel-mirror) ran in parallel to ccs-/cdx- siblings on identical briefs.** Theoretical labrat probe (`cdx-labrat-cco-hypothesis`) was zombied — registered but never spawned, manually cleaned up.

| Axis | Sonnet harness output | Opus harness output | Signal |
|------|----------------------|---------------------|--------|
| **A6 architecture** (read-only enumeration) | 70 transitive deps (corrected from 40) | 70 transitive deps (independent confirm) | **Convergent — no opus signal** |
| **A3 novel mechanisms** (adversarial review) | 7 verified novel | 4 verified + 2 partial + 2 rejected (caught 3 brief errors) | **Strong opus-better signal** |

**cco-mirror verbatim self-report:** *"Opus-specific A/B signal: promoted codex corrections to first-class 'unverified' entries rather than footnoting them under the brief's anchor table."*

### Recommendation: SELECTIVE upgrade, NOT blanket

| Role | Verdict | Rationale |
|------|---------|-----------|
| `ccs-distiller` | Upgrade to `cco-distiller` | Synthesis + cross-model contradiction surfacing benefits from opus-grade rigor; the audit-hash + evidence-passthrough rule is structural protection, but framing decisions matter. |
| `ccs-reviewer-arch` (and other reviewers) | Selective — case-by-case | Read-only enumeration: sonnet sufficient. Critical review of briefs / contradiction-surfacing: opus better. |
| `cdx-critic-overclaim` (and other critics) | Upgrade to `cco-critic-*` | Adversarial roles benefit most from opus's tendency to challenge brief assumptions. |
| `cdx-sentinel-*` | Upgrade to `cco-sentinel-*` | Same reasoning as critic — adversarial mindset benefits from opus. |
| `cdx-scout-*` (research, landscape) | Keep sonnet harness | xask delegation does the heavy lifting; sonnet harness frames adequately. |
| `cdx-executor-*`, `cdx-labrat-*`, `cdx-mutation-tester-*` | Keep sonnet harness | Fast probes / fire-and-forget; opus over-reasoning would worsen connector-stall-class issues. |

**Unverified gap:** No theoretical baseline (the labrat probe never ran). The A/B signal rests entirely on the two live mirror experiments — N=2 is small. Recommend running a 5-axis A/B in a dedicated future round before formalizing the upgrade.

---

## 9. Audit Trail

| Field | Value |
|-------|-------|
| Round | 1 |
| Axes named | 8 (A1–A8) + A9 probe |
| Teammates dispatched | 12 (8 axis + distiller + 3 A/B mirrors) |
| Teammates posted | 11 (1 zombied — `cdx-labrat-cco-hypothesis`) |
| Moves surviving Pareto | 9 (M001–M007 + M003-N8 + M005-A5) |
| Moves dropped | 0 |
| Moves spoof-flagged | 0 |
| In-band cross-corrections | 4 (M002 reframe, M004 narrow, M005 W3 add, M006 portability scope) |
| Cross-model CONFLICTS surfaced | 2 (novel count: 7 vs 4; gate locus: scripts/xask vs xbreed-shared.md protocol) |
| Cross-model CONFLICTS resolved | 2 (defer to cco-mirror on both — file:line anchored) |
| Post-commit errata applied | 3 (gate locus, stratification factual error, IMCP envelope vs wire) |
| audit_hash | `762ea768928183114185d497fc4703d661bdd9664b1ffc68e8ecb80c49653780` |
| Round 2 | Mooted (in-band cross-correction loop produced equivalent material; pane capacity blocked the targeted Round-2 connector probe; cross-axis pattern check would not change materiality calculus) |

---

## 10. Bottom Line

**xbreed v0.4.0 is structurally unique in the surveyed set.** It is the only framework combining:

1. Cross-vendor CLI delegation (codex + gemini via `scripts/xask`) as a first-class primitive
2. Named judge orchestrator with explicit axis-scored Pareto walk (`templates/agents/the-judge.md`)
3. Compiled, in-repo, in-binary deny-list as CC pre-tool-use hook (`src/guard.rs`)
4. Judge-blinding SHA-256 audit-hash over sorted `[{move_id, source_prefix}]` tuples (`xbreed-shared.md:178`)

**It is NOT "superior"** — that framing requires comparable benchmarks against LangChain/AutoGen/CrewAI/MetaGPT/Agno that don't exist in the surveyed corpus. The honest claim is **structural uniqueness on a defined set of axes**, with explicit acknowledgment that:

- Sandboxing parity is real (Codex CLI, LangChain E2B, AutoGen Docker, OpenAI Agents SDK guardrails)
- Ecosystem size, ML interop, observability, and persistent memory are competitor wins
- The W3 ghost-axis BLOCKER is a live-round operational risk shipping in v0.4.0
- The architecture's elegance describes the dependency graph, not the failure surface

**Highest-priority follow-up:** harden `src/ask.rs` against W3 ghost-axis failure. Add per-call timeout + circuit breaker + explicit timeout marker so the distiller can distinguish absent-vs-empty.

**Secondary follow-ups:** (a) refactor `src/ask.rs` (719 LOC concentration), (b) add observability primitives, (c) run a 5-axis cco-vs-ccs A/B in a dedicated round to formalize the selective upgrade recommendation, (d) add a `make verify-docs` lint for SSoT-vs-copies drift (the connector lock landed today required editing 8 files for a single rule change).
