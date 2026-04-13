---
description: Godspeed Pareto + cross-model delegation — combines /xgs speed with /xbt depth. Teammates invoke xask gemini/codex at godspeed pace.
argument-hint: <prompt for the judge>
allowed-tools: [Agent, Bash, Read, Write, Edit, Glob, Grep, TaskCreate, TaskGet, TaskList, TaskUpdate, SendMessage, TeamCreate, TeamDelete, WebFetch, WebSearch]
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

## Step 4 — Four-phase godspeed protocol (with xask gate)

### Phase 0 — Name the axes

Emit 3-5 axes (name + direction + observable). Incorporate user-named axes; infer the rest.

### Phase 1 — Assign deterministic teammate names

For each axis, assign a name: `{prefix}-{role}-{suffix}`. Commit ALL names before spawning.

Axis -> profile mapping:
- Research, prior art -> `scout` (sonnet) — `xask gemini`
- Correctness, bugs -> `reviewer` (sonnet) — `xask codex`
- Empirical probes -> `labrat` (haiku) — `xask gemini`
- Code execution -> `executor` (sonnet) — CC native
- Cross-axis patterns -> `connector` (sonnet) — `xask gemini`
- Synthesis, dedup -> `distiller` (sonnet) — in-session
- Complexity reduction -> `simplifier` (sonnet) — CC native

Cap: <=6 teammates per round.

### Phase 2 — Spawn all with full peer roster AND xask gate

Each brief includes:
1. Full peer roster (all names from Phase 1)
2. Axis assignment
3. **Structural xask gate** (per-role, three layers)
4. Task: propose ONE move (<=200 words)
5. After proposing, DM each peer with one-line critique
6. Mark task completed

#### xask gate by role

**Layer 1 — Gate (structural):**
- scout: `"Your FIRST tool call MUST be Bash: xask gemini '<question>'. No other tool before xask returns."`
- reviewer: `"Your FIRST tool call MUST be Bash: xask codex '<question>'. No other tool before xask returns."`
- labrat: `"Your FIRST tool call MUST be Bash: xask gemini '<hypothesis>'. No other tool before xask returns."`
- connector: `"Your FIRST tool call MUST be Bash: xask gemini '<pattern question>'. No other tool before xask returns."`
- executor/simplifier/distiller: No xask gate.

**Layer 2 — Raw-quote gate:** `"After xask, paste verbatim passage in <raw_output> tags. Must be literal substring of xask stdout. Empty = invalid. CLI output only."`

**Layer 3 — Fallback:** `[xask dry — in-session fallback]` marker, continue. No deadlock.

**Layer 4 — Confidence:** `[xask dry]` = provenance marker, not quality demotion.

**Epistemic role:** `"AT MOST one non-obvious claim + AT MOST one rejected alternative. Do not fabricate."`

**Divergence mandate:** `"CONFLICT: [claim] — my position: [X] — peer: [Y]"`

Create TaskCreate per teammate.

### Phase 3 — Rounds begin

**Pareto filter:** Accept strict improvers, reject regressions. Compile ROUND N summary.

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

**Exit:** Frontier reached -> final DRAFT with AXES FINAL STATE. Otherwise Round N+1.

## Step 5 — Keep iterating

Do not pause. Do not ask. User interrupts to steer.
Caps: <=4 rounds, <=6 teammates, <=200-word proposals.

## Step 6 — Hold after frontier

Team stays alive for follow-ups.

## Cleanup

On explicit request: shutdown all -> TeamDelete -> confirm.

## Step 7 — Status after init

Emit: team name, axes, teammates + xask targets, waiting on Round 1.
Immediately compile and dispatch when findings arrive.
