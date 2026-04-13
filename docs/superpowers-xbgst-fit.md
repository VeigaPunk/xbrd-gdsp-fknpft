# Superpowers × /xbgst — Fit Analysis (DRAFT)

> Produced via `/xbgst` Round 1 — team `superpowers-fit-gst-1744545000`.
> 6 axes, 6 teammates (3 gemini / 3 codex / mix), 1 distiller, Pareto-filtered.
>
> **Round 2 (2026-04-13, team `sp-xbgst-impl-1744545600`):** M1-M5 re-synthesised with 4 teammates + distiller. All five moves survive with amendments — evidence schema shipped as atomic bundle with distiller passthrough + closed-enum exempt allowlist; M2 reformulated to test-output pair (no SHA ceremony) with executor.md rewrite as prerequisite pair; M4 demoted from "canonical" to "reference" to preserve xbreed-shared.md SSoT; M5 taxonomy split into three buckets (10 cuts, not 11).

## AXES JUDGED
integration-surface · correctness-fit · complexity-subset · adversarial-breakage · empirical-probe · cross-axis-callsites

## Core finding

**Don't embed the Superpowers pipeline inside subagent briefs.** The two systems solve the same problem ("too fast, too shallow") with opposite strategies — Superpowers serializes with HARD-GATES, /xbgst parallelizes with Pareto. Forcing both onto one subagent silently drops the Superpowers gates (recency bias wins) and produces frontier pollution by incommensurable artifacts (specs vs. candidate moves).

The fit is **structural, not instructional.** Superpowers contributes *principles* to /xbgst's filter schema; it does not contribute *skills* to subagent briefs.

## SYNTHESIS — five surviving moves

### M1 · Evidence-field Pareto gate (atomic bundle) · [high confidence, Round 2 amended]
**Source principle:** `verification-before-completion`
**Change:** Phase 3 Pareto filter schema requires an `evidence:` field on every proposed move. For executor / mutation-tester / sentinel / reviewer moves, evidence = fresh verification output (test stdout + exit code, diff, xask output). For non-executable axes (scout, connector, synthesis, orchestration, adversarial-design, complexity, reverse-engineering) the allowed form is `evidence: none — <axis reason>`. Moves without required evidence are **dropped, not scored**. The closed-enum exempt allowlist keys on `axis_family`; free-text self-classification is rejected.
**Ships atomically with:** (a) distiller spawn template extension — byte-for-byte `evidence:` passthrough, no prose absorption; (b) `axis_family` closed enum in `xbreed-shared.md`. Landing M1 alone would silently drop all distilled executor moves.

### M2 · Executor-lane `|godspeed-impl` variant · [moderate, Round 2 amended]
**Source principle:** Superpowers TDD red-green-refactor, scoped
**Change:** Replace the raw `|godspeed` suffix for executor spawns only with `|godspeed-impl`. Adds: *"A move is a complete red-before-green cycle — `evidence:` must include failing-test output AND passing-test output (two test runs, no commit SHAs). If no test harness exists in scope, attach diff + rationale as evidence. Non-executable axes are not eligible for the executor lane."* All other teammate roles inherit plain `|godspeed`; TDD-ordering is out-of-scope for research/critique lanes by domain, not waived. Executor.md `Completion is the metric` rule is rewritten to carry the `evidence:` return-format field.

### M3 · Pre-flight brainstorming as user ritual, not pipeline · [high, 3-source]
**Source principle:** Superpowers `brainstorming` HARD-GATE
**Change:** Document in `/xbgst` that **Superpowers brainstorming runs before `/xbgst` is invoked**, producing the approved spec that becomes `/xbgst`'s input prompt. Never inside the walk. This eliminates the "stop asking clarifying questions" contradiction at the architecture level.

### M4 · `dispatching-parallel-agents` as reference (not canonical) · [moderate, Round 2 amended]
**Source principle:** `dispatching-parallel-agents`
**Change:** Cite `superpowers:dispatching-parallel-agents` in `xbreed-shared.md` as a **reference only** for the crafted-brief + isolated-context + concurrent-dispatch pattern. Demoted from "canonical" to "reference" — `xbreed-shared.md` is already declared SSoT in commit 31f5902; dual-canonical is split-brain.

### M5 · Explicit cut-list · [moderate]
Skip 10 Superpowers skills, grouped by reason:

- **Serialization-incompatible** (serial human-gate checkpoints contradict godspeed): `systematic-debugging`, `executing-plans`, `receiving-code-review`.
- **Lifecycle-position** (pre-`/xbgst` ceremony or post-`/xbgst` cleanup — neither phase exists inside the runtime loop): `writing-plans`, `finishing-a-development-branch`, `requesting-code-review`, `using-git-worktrees`, `using-superpowers`.
- **Domain/superseded** (addressed by /xbgst's own primitives): `subagent-driven-development`, `writing-skills`.

`test-driven-development` is kept selectively via M2 (red-before-green encoded in executor-lane evidence). YAGNI everywhere else.

## CONFLICTS

- **C1 (resolved against connector):** *Distiller + TDD makes distillation executable spec* (gemini / connector) vs. *cut TDD entirely* (simplifier). Resolution: M1's `evidence:` field achieves the empirical-falsifiability property without re-importing the TDD iron law. Reject connector's variant; keep the end-state.
- **C2 (resolved via reviewer's structural bridge):** *Embed verification gate inside the walk* (gemini / scout) vs. *Defer gate entirely to judge post-frontier* (codex / critic). Resolution: gate lives in the **filter schema** (judge-side), teammates only **attach** evidence (subagent-side). Both halves satisfied; neither "embedded in brief" nor "deferred past frontier."
- **C3 (resolved by labrat):** Universal high-leverage claim for `verification-before-completion` FALSIFIED — selective gate is correct framing. Encoded in M1.

## IMPLEMENTATION SKETCH

- **files:**
  - `commands/references/xbreed-shared.md` — add "Pareto filter evidence schema" section; cite `dispatching-parallel-agents` under Phase 2.
  - `commands/xbgst.md` — document pre-flight brainstorming expectation; replace Phase 3 description with evidence-gated filter.
  - `templates/agents/executor.md` — add `|godspeed-impl` variant block.
- **sequencing:** M3 + M4 (documentation, no behavior change) → M1 (schema change, one test) → M2 (executor-only, lane-scoped). M5 is the absence-change: no imports land.
- **tests:** one fixture — a malformed move (executor-lane proposal with no `evidence:` field) reaches the Pareto filter; expected: dropped with reason, not scored.

## OPEN QUESTIONS

- Distiller output schema — should SYNTHESIS_READY carry the `evidence:` field upstream, or is it attached only at teammate-proposal granularity?
- Non-executable axes (scout, connector) need a **negative evidence convention** (e.g., `evidence: none — research axis`) or the filter will silently reject them.

## AXES FINAL STATE
| Axis | Improved? | Note |
|---|---|---|
| integration-surface | ✅ | M1+M4 name the clean joinpoints |
| correctness-fit | ✅ | C1+C2 structurally resolved |
| complexity-subset | ✅ | M5 rejects 10 skills (3-bucket taxonomy) |
| adversarial-breakage | ✅ | M3 dissolves the recency-bias failure mode |
| empirical-probe | ✅ | labrat's falsification preserved as "selective gate" |
| cross-axis-callsites | ◻️ | matrix emitted but connector's top pick was Pareto-rejected |

**Round 2 adds (2026-04-13, commit 9dfc7d9):** `the-judge.md` Pareto-filter description gains explicit evidence-gate step; `xbreed-shared.md` moved into repo + distiller spawn template gains byte-for-byte `evidence:` constraint; strict exit condition + anti-premature-halt rule installed; docs body reconciled.

**Round 3 adds (2026-04-13):** `scripts/install-commands.sh` for fresh-clone portability; `xbreed-shared.md` Pareto schema gains missing scope tag (dropped by a lost Round-2 edit) + `deletion` axis_family row so `simplifier` stops misclassifying as non-executable `complexity`; Exit Condition gains materiality rule (axis-observable change, not prose delta) to kill paraphrase-round thrash; `xbgst.md` Step 4+5 duplication of exit semantics replaced by pointer into the SSoT.
