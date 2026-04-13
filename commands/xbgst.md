---
description: Godspeed Pareto + cross-model delegation — combines /xgs speed with /xbt depth. Teammates invoke xask gemini/codex at godspeed pace.
argument-hint: <prompt for the judge>
allowed-tools: [Agent, Bash, Read, Write, Edit, Glob, Grep, TaskCreate, TaskGet, TaskList, TaskUpdate, TaskOutput, SendMessage, TeamCreate, TeamDelete, WebFetch, WebSearch, LSP, Monitor]
---

# /xbgst — Godspeed Pareto + Cross-Model Delegation

The full crossbreed: godspeed Pareto walk (parallel proposals, axis-scored, round-capped) with cross-model delegation (teammates invoke `xask gemini`/`xask codex`). Combines the speed of `/xgs` with the external perspectives of `/xbt`.

Use when you want fast Pareto convergence AND cross-model views. For all-Claude speed without cross-model, use `/xgs`. For slower deliberative mediation with cross-model, use `/xbt`.

## Step 1 — Load the judge persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Adopt the posture: judge explicitly on named axes, aggregate best-of-each, draft-then-dispatch.

## Step 2 — Create the team

Pick a concise unique `team_name`:
- First 2-3 significant words of prompt (lowercased, hyphen-joined) + timestamp suffix
- Fallback: `xbgst-squad-<ts>`

```
TeamCreate(
  team_name="<your-unique-name>",
  agent_type="team-lead",
  description="<the full user prompt, truncated to 200 chars>"
)
```

If `TeamCreate` fails because a team already exists, auto-cleanup: shutdown idle teammates + TeamDelete + retry. Do not ask the user.

## Step 3 — Parse the prompt

$ARGUMENTS

If empty, wait for user direction. Otherwise, proceed to four-phase godspeed with xask.

**Pre-flight expectation:** `/xbgst` runs on an already-scoped prompt. If the user's intent is unclear, design-shaped, or pre-specification, point them to the Superpowers `brainstorming` skill first — that HARD-GATE produces the approved spec that becomes `/xbgst`'s input. Inside the walk, clarifying questions are suppressed (godspeed); ambiguity must be resolved upstream. This is guidance, not a prerequisite — the judge may still proceed on a rough prompt when the axes are self-evident.

## Step 4 — Four-phase godspeed protocol (with xask gate)

### Phase 0 — Name the axes

Emit up to 8 axes (name + direction + observable). Incorporate user-named axes; infer the rest.

### Phase 1 — Assign deterministic teammate names

For each axis, assign a name: `{prefix}-{role}-{suffix}`. Commit ALL names before spawning.

Axis → profile mapping (see `~/.claude/commands/references/xbreed-shared.md` for full details):
- Research, prior art → `scout` (sonnet) — `xask --effort medium gemini`
- Correctness, bugs → `reviewer` (sonnet) — `xask --effort xhigh codex`
- Empirical probes → `labrat` (sonnet) — `xask --spark codex`
- Code execution → `executor` (sonnet) — CC native
- Cross-axis patterns → `connector` (sonnet) — `xask --effort medium gemini`
- Synthesis, dedup → `distiller` (sonnet) — in-session
- Complexity reduction → `simplifier` (sonnet) — CC native

Cap: <=12 teammates per round.

### Phase 2 — Spawn all with full peer roster AND xask gate

Each brief includes:
1. Full peer roster (all names from Phase 1)
2. Axis assignment
3. **Godspeed mode** (always — /xbgst is inherently godspeed): `"GODSPEED MODE (inherited from judge): You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls."`
4. **Structural xask gate** (per-role, three layers)
5. Task: propose ONE move (<=200 words) **with a structured `evidence:` field** (see Evidence Schema in `xbreed-shared.md`)
6. After proposing, DM each peer with one-line critique
7. Mark task completed

**Executor lane — `|godspeed-impl` suffix variant:** when spawning `executor` teammates, append `|godspeed-impl` instead of plain `|godspeed`. Adds to the godspeed directive: *"A move is a complete red-before-green cycle — `evidence:` must include failing-test output AND passing-test output (two test runs, no commit SHAs). If no test harness exists in the scope, attach diff + rationale as evidence. Non-executable axes are not eligible for the executor lane."* All other roles inherit plain `|godspeed`; TDD ordering is out-of-scope for research/critique lanes by domain, not waived.

#### xask gate, epistemic constraints

Read `~/.claude/commands/references/xbreed-shared.md` for the full 4-layer xask gate (per-role), epistemic constraints, and divergence mandate. Apply them to every teammate brief.

Create TaskCreate per teammate.

### Phase 3 — Rounds begin

**Distiller synthesis:** Once all teammates have spawned, spawn the distiller:

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag cross-model contradictions (gemini vs codex), assign confidence. DO NOT rewrite, summarize, or absorb any line beginning with `evidence:` — copy it verbatim, byte-for-byte, into the corresponding move in your synthesis output. This is a structural requirement, not guidance; the Pareto filter reads the field post-synthesis. SendMessage your synthesis to the judge (team lead) when done. |godspeed"
)
```

**Pareto filter (on distiller output):** Accept strict improvers, reject regressions. Compile ROUND N summary.

Every surviving move must carry a structured `evidence:` field matching the **Pareto Filter Evidence Schema** in `xbreed-shared.md`. Moves lacking the required form (e.g., executor-lane proposal with no failing/passing test output) are **dropped, not scored**. Distiller passes the field through verbatim — do not prose-absorb. Non-executable axes use the negative convention: `evidence: none — <axis reason>`.

**Re-distill each round:** For rounds 2+, send updated proposals to the distiller via SendMessage. Only re-spawn if shut down.

**CONFLICTS block** uses **model labels**:
```
CONFLICTS (emit only if cross-model contradictions exist):
  - claim: <contested fact>
    model: gemini (via <teammate>) — <position>
    model: codex (via <teammate>) — <position>
    judge_resolution: <chosen + rationale>
    escalate_to: <sub-role if unresolved>
```

**Falsification probe:** After Pareto filter, if highest-divergence surviving move has an unchallenged claim, dispatch ONE targeted xask to opposing model. Cap: one exchange, one claim (3-8s).

**Judge weighting:** Weight xask quotes that contradict agent's conclusion more heavily than confirming quotes.

**Exit condition (strict):** the frontier has stopped moving **iff Round N produced zero axis improvements vs Round N-1** (every surviving move is a duplicate of a prior-round survivor, or the filter accepted nothing new). "Distiller reports no open questions" is NOT the exit condition — a clean synthesis still typically moves axes relative to the pre-walk baseline. By definition Round 1 moves axes off baseline, so **Round 2 always runs** unless capped.

Otherwise: Round N+1. Final DRAFT with AXES FINAL STATE emits only on strict exit or round cap.

## Step 5 — Keep iterating

Do not pause. Do not ask. User interrupts to steer. Keep iterating.

Caps: <=4 rounds, <=12 teammates, <=200-word proposals.

**Anti-premature-halt rule.** After each round, before declaring frontier-stable, the judge MUST:
1. Compare Round N survivors against Round N-1 survivors (or pre-walk baseline for Round 1).
2. If any axis improved, dispatch Round N+1 immediately — do not emit final DRAFT, do not ask the user.
3. If truly zero improvement, emit final DRAFT.

Round 1 → Round 2 transition is NOT optional. Round 1 by construction improves axes off baseline; jumping to cleanup after Round 1 violates the protocol.

## Step 6 — Auto-cleanup after frontier

When the frontier stops moving (zero survivors / duplicates / 4 rounds / user halt): emit final DRAFT, then immediately shutdown all teammates in parallel via `SendMessage shutdown_request`, wait for shutdown_approved, then `TeamDelete`. If TeamDelete fails with "active members", run `xbreed-cleanup <team-name>` via Bash. Do not ask — kill it. User re-invokes `/xbgst` if they want a new axis.

## Step 7 — Status after init

Emit: team name, axes, teammates + xask targets, waiting on Round N.
Immediately compile and dispatch when findings arrive.
