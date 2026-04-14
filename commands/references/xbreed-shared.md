# Shared Orchestration Protocol

Referenced by `/xbreed`, `/xbt`, `/xgs`, `/xbgst`. Do not duplicate — load this file.

## Godspeed Mode Block

Append to every teammate brief when operating in godspeed:

> **GODSPEED MODE (inherited from judge):** You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls.

## Escalation: advisor() (Layer 0)

All sonnet teammates can call `advisor()` (CC-native, zero parameters) for in-session opus-max escalation. The teammate's full conversation context is forwarded automatically.

**When to use advisor():** Before committing to non-obvious architectural decisions, when stuck, when a finding contradicts a peer, or before declaring work complete.

**advisor() vs xask:** advisor() is Layer 0 — it runs before and independently of the 4-layer xask gate. It is NOT cross-model delegation; it's in-session reasoning review. Use `xask claude` for contamination-controlled cross-model dispatch; use `advisor()` for full-context reasoning escalation.

Include in teammate briefs: `"You have access to advisor() — call it before substantive decisions for opus-max review of your full context. Zero parameters, blocks until response."`

## xask Gate (4 layers)

Include as FIRST instruction in every teammate brief that requires cross-model delegation.

**Layer 1 — Gate (structural):**
- **scout**: `"Your FIRST tool call MUST be Bash: xask --effort medium gemini '<research question>' 'context' 'librarian'. No other tool before xask returns."`
- **reviewer**: `"Your FIRST tool call MUST be Bash: xask --effort xhigh codex '<review question>'. No other tool before xask returns."`
- **labrat**: `"Your FIRST tool call MUST be Bash: xask --spark codex '<probe hypothesis>'. No other tool before xask returns."`
- **connector**: `"Your FIRST tool call MUST be Bash: xask --effort medium gemini '<pattern question>'. No other tool before xask returns."`
- **the-revenger**: `"Your FIRST tool call MUST be Bash: xask --effort medium gemini '<surface enumeration question>'. No other tool before xask returns."` (when dispatched for recon on unfamiliar systems; skip gate for in-repo reverse engineering)
- **sentinel**: `"Your FIRST tool call MUST be Bash: xask --effort xhigh codex '<exploit/vulnerability analysis question>'. No other tool before xask returns."`
- **critic**: `"Your FIRST tool call MUST be Bash: xask --effort high codex '<design review question>'. No other tool before xask returns."`
- **mutation-tester**: No xask gate (operates locally in worktrees).
- **executor**: No structural xask gate (no single fixed first-call), BUT executor is a sonnet **mediator** that delegates heavy code drafting to `xask --spark codex "<subtask>"` — fan out in parallel when subtasks are independent. Rationale: sonnet-as-direct-implementer is too slow at /xbgst pace; codex-spark is cheap and fast. Sonnet validates outputs, writes final commits, preserves red-before-green evidence discipline, and can escalate to `advisor()` for non-obvious design decisions. **Executor sharding rule**: when implementation work decomposes into multiple files or independent modules, the judge spawns MULTIPLE executor teammates in parallel (same dispatch message) with strict file-boundary briefs. Bundling into one executor is a godspeed anti-pattern; pipeline decomposition + git index lock handle coordination (no worktree isolation required). See `/xbgst` Step 4 Phase 2 "Executor sharding" for the full rule.
- **simplifier/distiller/mutation-tester/Plan**: No xask gate.

**Layer 2 — Raw-quote gate:** `"After xask, paste verbatim passage in <raw_output> tags. Must be literal substring of xask stdout. Empty = invalid. CLI output only."`

**Layer 3 — Fallback:**
- scout/reviewer: xask failure → DM judge with `BLOCKED: xask [reason]`, then continue in-session with `[xask dry — in-session fallback]` marker. Do not deadlock.
- labrat: xask failure → emit `obs: xask BLOCKED [reason]` as the finding, despawn. Failure IS the result.

**Layer 4 — Confidence:** `[xask dry]` marks source provenance, not quality. Judge assesses confidence case-by-case.

## Epistemic Constraints

Include in every teammate brief:

- **Epistemic role:** `"AT MOST one non-obvious claim + AT MOST one rejected alternative. Do not fabricate — return nothing if no well-grounded finding exists."`
- **Divergence mandate:** `"If your finding contradicts a peer's, flag: CONFLICT: [claim] — my position: [X] — peer: [Y]"`
- **Judge weighting:** Weight xask quotes contradicting agent's conclusion more heavily than confirming quotes.

## Axis → Profile Mapping

**This table is the single source of truth for agent routing.** AGENTS.md and the-judge.md carry read-only copies for discoverability. On any edit here, update those two.

Allowed `axis_family` values (must match frontmatter in `templates/agents/*.md`): `research`, `correctness`, `empirical`, `execution`, `cross-axis`, `synthesis`, `complexity`, `reverse-engineering`, `security`, `orchestration`, `adversarial-design`, `test-validation`, `deletion`.

| Axis family | Role | Model | xask target | Tools |
|---|---|---|---|---|
| Research, prior art | `scout` | sonnet | `xask --effort medium gemini` | All |
| Correctness, bugs | `reviewer` | sonnet | `xask --effort xhigh codex` | All |
| Empirical probes | `labrat` | sonnet | `xask --spark codex` | All |
| Code execution | `executor` | sonnet (mediator) | `xask --spark codex` (parallel fan-out) | All |
| Cross-axis patterns | `connector` | sonnet | `xask --effort medium gemini` | All |
| Synthesis, dedup | `distiller` | sonnet | in-session | All |
| Deletion, YAGNI | `simplifier` | sonnet | CC native | All |
| Reverse engineering | `the-revenger` | opus | `xask gemini` for surface enum | All |
| Security auditing | `sentinel` | sonnet | `xask --effort xhigh codex` + `xask gemini` | All |
| Implementation planning | `Plan` | CC built-in | CC native | All |
| Adversarial design | `critic` | sonnet | `xask --effort high codex` | All |
| Test validation | `mutation-tester` | sonnet | `xask --spark codex` | All |

## Naming Convention

`{prefix}-{role}-{suffix}` where prefix = `g-` (Gemini), `ccs-` (Claude Sonnet), `cco-` (Claude Opus), `cdx-` (Codex).

## Labrat Invocation (Universal)

Any agent can spawn a labrat probe. Two paths:

1. **Subagent spawn:** `Agent(subagent_type="labrat", name="cdx-labrat-<hypothesis>", model="sonnet", prompt="<probe>")`
2. **Bash call:** `xask --spark codex "<probe hypothesis>"` — codex-5.3-spark, fire-and-forget

Default labrat delegation is `xask --spark codex` (fast, cheap, expendable). For long-context probes or swarms, fall back to `xask gemini`.

**Gemini fanout/swarm (universal):** Gemini CLI has a native `fanout` skill. Any agent can invoke via `xask gemini "trigger a fanout on: <hypothesis>. Vary angle per probe. Report HYPOTHESIS/METHOD/RESULT."` — 1 Gemini call runs N probes inside Gemini's context. Use for labrat swarms AND mutation-tester generations. Up to 3 refire rounds (30 probes total) — independent of judge rounds.

## Distiller Spawn Template

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag contradictions (cross-model if xask used, cross-teammate if all-Claude), assign confidence. Preserve each surviving move's `evidence:` field verbatim (see Pareto Filter Evidence Schema) — do not absorb into prose; the filter reads it post-synthesis. Use SYNTHESIS_READY mapping for judge consumption. SendMessage your synthesis to the judge (team lead) when done."
)
```

## Pareto Filter Evidence Schema

> **Scope:** Enforced in `/xgs` and `/xbgst` (Pareto-walk modes). Informational in `/xbt` (deliberative) and `/xbreed` / `/xb` (solo pipeline) — judge mediates directly, no drop gate runs. Distiller passthrough still fires, so the field travels intact if present.

The Pareto filter reads a structured `evidence:` field on every proposed move. Moves without required evidence are **dropped, not scored** — the verification discipline is enforced by the filter, not by the agent's willingness to comply.

**Schema (task-aware by role):**

| Role axis_family | Required evidence form |
|---|---|
| `execution` (executor) | failing-test output + passing-test output (red-before-green); OR diff + rationale if no harness |
| `correctness` (reviewer), `test-validation` (mutation-tester), `security` (sentinel) | verbatim xask output OR test/lint stdout + exit code |
| `empirical` (labrat) | probe HYPOTHESIS/METHOD/RESULT triple |
| `deletion` (simplifier) | diff of removed symbols + test pass/fail output (pre- and post-removal) |
| `research` (scout), `cross-axis` (connector), `synthesis` (distiller), `orchestration`, `adversarial-design` (critic), `complexity`, `reverse-engineering` | `evidence: none — <axis reason>` (non-executable) |

**Exempt-role allowlist is a closed enum keyed on `axis_family`**, not free-text self-classification. Any new role must land with a schema update to this table or ship with executable evidence. Distiller passes the field through verbatim.

## Parallel Dispatch Reference

Phase 2 concurrent dispatch follows the crafted-brief + isolated-context + parallel-Agent pattern documented in the Superpowers `dispatching-parallel-agents` skill — cited as reference only; this file remains the SSoT for xbreed dispatch.

## DESPAWN Protocol

Any agent (labrat, reviewer, or other) may send DESPAWN signal after completing all assigned work. Judge acknowledges and releases the session slot. Format:

```
DESPAWN: <agent-name> — signal delivered. Send me shutdown_request.
```

## Team Cleanup

**Graceful path:** `SendMessage shutdown_request` to each teammate → wait for `shutdown_approved` → `TeamDelete`.

**Force path (when TeamDelete fails with "active members"):** Run `xbreed-cleanup <team-name>` via Bash. This kills stale processes and removes team + task dirs. Use when:
- A teammate process hung or was killed externally
- `TeamDelete` refuses due to stale config.json member entries
- Orphan task dirs accumulate from prior sessions

**Periodic maintenance:** `xbreed-cleanup --stale` cleans all teams with no live processes + orphan UUID task dirs.

## Round Limits

- **Godspeed Pareto** (xgs, xbgst): 4 rounds max
- **Deliberative** (xbt): 4 rounds max (sequential depth)
- **Solo pipeline** (xbreed, xb): 8 sub-role dispatches max
- **Labrat Gemini swarm**: 3 refire rounds (30 probes) — independent of judge rounds

## Exit Condition (strict, applies to xgs/xbgst/xbt)

The frontier has stopped moving **iff Round N produced zero axis improvements vs Round N-1** (all survivors duplicate prior-round survivors, or filter accepted nothing new). "Distiller reports no open questions" is NOT the exit condition — clean synthesis still typically moves axes off the pre-walk baseline.

**Materiality rule.** A surviving move counts as an improvement only if at least one axis observable (the triplet defined in Phase 0: name + direction + observable) has changed state vs Round N-1. Proposal-prose difference alone does not qualify — paraphrased findings against unchanged observables are not improvements.

**Anti-premature-halt rule.** After each round, before declaring frontier-stable, judge MUST:
1. Compare Round N survivors to Round N-1 survivors (or pre-walk baseline for Round 1).
2. If any axis improved → dispatch Round N+1 immediately. Do not emit final DRAFT. Do not ask the user.
3. Only on true zero-improvement OR round cap → emit final DRAFT + auto-cleanup.

Round 1 by construction improves axes off baseline, so **Round 2 always runs** unless the user halts or a cap triggers. Jumping to cleanup after Round 1 is a protocol violation.

**Wake-tick forcing function.** `ScheduleWakeup` with the `<<autonomous-loop-dynamic>>` sentinel exists to catch premature-halt rationalization. When the wake fires after the judge has declared DRAFT or called `TeamDelete`, it is NOT a stale tick — it is the protocol asking "did you actually satisfy the exit condition?" Discarding it ("walk already over") re-commits the original halt violation. On receipt, re-evaluate: was R2 actually dispatched? Was R_N compared against R_{N-1}? If either answer is no, the wake is live; dispatch the missing round. The sentinel is load-bearing, not cosmetic.

## Coherence Check

After parallel execution rounds (multiple executors editing files concurrently), spawn a reviewer for cross-file consistency before committing. Checklist:

1. **Cross-file reference consistency** — dispatch tables, xask gate strings, and tool lists agree across xbreed-shared.md, AGENTS.md, the-judge.md, and skill templates
2. **Stale agent name/model references** — no haiku/sonnet mismatches, no removed agent names, delegation targets current
3. **Template-vs-installed sync** — `templates/agents/*.md` matches `~/.claude/agents/*.md`

This is not a blocking gate — the judge decides when the scope of changes warrants it. Multi-file parallel edits always warrant it.
