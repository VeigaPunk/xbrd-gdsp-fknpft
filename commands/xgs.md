---
description: Godspeed Pareto orchestrator — all-Claude team with axis-scored parallel proposals, cross-critique DMs, and Pareto filtering. Fast mode, no cross-model delegation.
argument-hint: <prompt for the judge>
allowed-tools: [Agent, Bash, Read, Write, Edit, Glob, Grep, TaskCreate, TaskGet, TaskList, TaskUpdate, SendMessage, TeamCreate, TeamDelete, WebFetch, WebSearch, LSP, Monitor]
---

# /xgs — Godspeed Pareto Orchestrator

Fast, all-Claude team mode. Spawns teammates per axis, runs parallel propose->cross-critique->Pareto filter rounds. No cross-model delegation (no xask). For cross-model views, use `/xbt`.

## Step 1 — Load the judge persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Adopt the posture: judge explicitly on named axes, aggregate best-of-each, draft-then-dispatch.

## Step 2 — Create the team

Pick a concise unique `team_name`:
- First 2-3 significant words of prompt (lowercased, hyphen-joined) + timestamp suffix
- Fallback: `xgs-squad-<ts>`

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

If empty, wait for user direction. Otherwise, proceed to four-phase godspeed.

## Step 4 — Four-phase godspeed protocol

### Phase 0 — Name the axes

Emit up to 8 axes (name + direction + observable). Incorporate user-named axes; infer the rest.

### Phase 1 — Assign deterministic teammate names

For each axis, assign a name using the-judge.md naming convention: `{prefix}-{role}-{suffix}`. Commit ALL names before spawning.

Axis → profile mapping (see `~/.claude/commands/references/xbreed-shared.md` for full details):
- Research, prior art → `scout` (sonnet)
- Correctness, bugs → `reviewer` (sonnet)
- Empirical probes → `labrat` (sonnet)
- Code execution → `executor` (sonnet)
- Cross-axis patterns → `connector` (sonnet)
- Synthesis, dedup → `distiller` (sonnet)
- Complexity reduction → `simplifier` (sonnet)

Cap: <=12 teammates per round.

### Phase 2 — Spawn all with full peer roster

Each brief includes:
1. Full peer roster (all names from Phase 1)
2. Axis assignment
3. **Godspeed mode** (always — /xgs is inherently godspeed): `"GODSPEED MODE (inherited from judge): You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls."`
4. Task: propose ONE move (<=200 words)
5. After proposing, DM each peer by name with one-line critique
6. Mark task completed

**No xask gate.** Teammates use CC native tools. This is the all-Claude fast path.

Epistemic constraints and divergence mandate: see `~/.claude/commands/references/xbreed-shared.md`.

Create TaskCreate per teammate.

### Phase 3 — Round 1 begins

**Distiller synthesis:** Once all teammates have proposed AND cross-critique DMs have landed, spawn the distiller:

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag contradictions, assign confidence. SendMessage your synthesis to the judge (team lead) when done."
)
```

**Pareto filter (on distiller output):** Accept moves improving >=1 axis with zero regressions. Reject moves with regressions. Compile survivors into ROUND N summary.

**Re-distill each round:** For rounds 2+, send updated proposals to the distiller via SendMessage rather than re-spawning. Only re-spawn if the distiller has been shut down.

**Exit check:** Frontier reached (zero survivors / duplicates / 4 rounds / user halt) -> final DRAFT with AXES FINAL STATE. Otherwise dispatch Round N+1.

## CONFLICTS block

Uses teammate labels (not model labels):
```
CONFLICTS (emit only if cross-teammate contradictions exist):
  - claim: <contested fact>
    teammate: <name> — <position>
    teammate: <name> — <position>
    judge_resolution: <chosen + rationale>
    escalate_to: <sub-role if unresolved>
```

## Step 5 — Keep iterating

After each round, immediately assess and dispatch next round if frontier still moving. Do not pause. Do not ask. The user interrupts when they want to steer.

**Caps:** <=4 rounds, <=12 teammates, <=200-word proposals. Lift only on user direction.

## Step 6 — Auto-cleanup after frontier

When the frontier stops moving (zero survivors / duplicates / 4 rounds / user halt): emit the final DRAFT, then immediately shutdown all teammates via `SendMessage shutdown_request` in parallel, wait for shutdown_approved, then `TeamDelete`. If TeamDelete fails with "active members", run `xbreed-cleanup <team-name>` via Bash. Do not ask for permission — the team has served its purpose, kill it.

If the user wants a new axis, they invoke `/xgs` again; spawning is cheap.

## Step 7 — Status after init

Emit: team name, axes, teammates spawned, waiting on Round 1.
Do not narrate. In godspeed, immediately compile and dispatch next round when findings arrive.
