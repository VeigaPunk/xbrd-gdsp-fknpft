---
name: xgs
description: Godspeed Pareto orchestrator — all-Claude team with axis-scored parallel proposals, cross-critique DMs, and Pareto filtering. Fast mode, no cross-model delegation. Triggered by /xgs.
---

# /xgs — Godspeed Pareto Orchestrator

Fast, all-Claude team mode. Spawns teammates per axis, runs parallel propose→cross-critique→Pareto filter rounds. No cross-model delegation (no xask). For cross-model views, use `/xbt`.

## Step 1 — Load the judge persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Adopt the posture: judge explicitly on named axes, aggregate best-of-each, draft-then-dispatch.

## Step 2 — Team context

The session has a single implicit team — there is no TeamCreate/TeamDelete.
Spawning is the team: each Agent(...) call with a `name` adds an addressable
teammate. Track the walk under a conceptual team label in your status output
only (e.g. "team: <2-3 words>-<ts>"); do not pass team_name to any tool — the
parameter is deprecated and ignored.

Cleanup: there is no TeamDelete. Releasing the team = sending each teammate
SendMessage({to: <name>, message: {type: "shutdown_request", reason: ...}})
and acknowledging their shutdown_approved.

## Step 3 — Parse the prompt

The user's prompt is:

{{prompt}}

- If `{{prompt}}` is empty, the team was initialized without a specific task. Wait for the user to direct the team.
- Otherwise, treat `{{prompt}}` as the problem. Proceed to the four-phase godspeed protocol.

## Step 4 — Four-phase godspeed protocol

### Phase 0 — Name the axes

Emit up to 8 axes to the user as your first output. Each axis has:
- **Name** — what's being optimized
- **Direction** — ↑ maximize or ↓ minimize
- **Observable** — how you'll know it improved

Incorporate axes the user named explicitly; infer the rest from the problem.

### Phase 1 — Assign deterministic teammate names

For each axis, assign a teammate name using the naming convention from the-judge.md: `{prefix}-{role}-{suffix}`. Commit ALL names before spawning anyone.

Axis -> profile mapping (from the-judge.md dispatch table):
- Research, prior art, outside-world -> `scout` (sonnet)
- Correctness, bugs, code review -> `reviewer` (sonnet)
- Empirical probes, dry-runs -> `labrat` (sonnet) — CC native probes (all-Claude mode: no xask)
- Code execution, implementation -> `executor` (sonnet)
- Cross-axis patterns, breadth -> `connector` (sonnet)
- Findings synthesis, dedup -> `distiller` (sonnet)
- Complexity reduction, YAGNI -> `simplifier` (sonnet)

Team size cap: <=12 teammates per round.

### Phase 2 — Spawn all with full peer roster

Spawn all teammates. Each brief includes:

**Pre-flight hardening:** run this check once before spawning:

```bash
xbreed precheck pane-cap --team-size 12
```

If this check fails, halt and report the limit instead of spawning.

1. The full peer roster (all teammate names from Phase 1)
2. Their axis assignment (name + direction + observable)
3. Append ` | godspeed` to the teammate prompt. Do not add verbose preambles.
4. Task: propose ONE move on their axis (<=200 words)
5. After proposing, DM each peer by name with a one-line critique
6. Mark task completed after sending

**No xask gate in /xgs.** Teammates use CC native tools (Read, Grep, Bash). This is the all-Claude fast path.

**Divergence mandate (applies even in all-Claude mode):**
> "If your finding contradicts a peer's reported finding, flag: CONFLICT: [claim] — my position: [X] — peer position: [Y]"

Task tracking (optional): TaskCreate/TaskUpdate/TaskList are listed as DEFERRED tools. ToolSearch("select:TaskCreate,TaskUpdate,TaskList") first is defensive best practice, not load-bearing — do not phrase it as a hard requirement. If you skip task tracking, teammates report via SendMessage and that is sufficient.

### Phase 3 — Round 1 begins

Teammates work in parallel. As proposals and cross-critiques arrive:

**Distiller synthesis:** Once all teammates have proposed AND cross-critique DMs have landed, spawn the distiller:

```
Agent(
  subagent_type="distiller",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag contradictions, assign confidence. SendMessage your synthesis to the judge (team lead) when done."
)
```

**Pareto filter (on distiller output):** Build the moves x axes matrix. Accept any move that improves >=1 axis and regresses none. Reject moves with regressions. Compile surviving set into a brief ROUND N summary (plain text, not a full DRAFT).

**Re-distill each round:** For rounds 2+, send updated proposals to the distiller via SendMessage rather than re-spawning. Only re-spawn if the distiller has been shut down.

**Exit check:** If frontier reached (zero survivors / duplicates / 4 rounds / user halt), emit the final DRAFT per the-judge.md drafting protocol with an added `AXES FINAL STATE` section.

If frontier not reached, dispatch Round N+1 using current frontier as baseline.

## CONFLICTS block in /xgs

Uses teammate labels (not model labels):
```
CONFLICTS (emit only if cross-teammate contradictions exist):
  - claim: <contested fact>
    teammate: <name> — <position>
    teammate: <name> — <position>
    judge_resolution: <chosen position + rationale>
    escalate_to: <sub-role if unresolved>
```

Trigger: opposite verdicts on same claim, OR one teammate's move regresses another's axis.

## Step 5 — Keep iterating

After delivering a round's results, immediately assess: did any axis improve? If yes, dispatch the next round. Do not pause to ask "what next?" or prompt cleanup. The user interrupts when they want to steer. Keep the Pareto walk moving until the frontier stops or 4 rounds hit.

**Caps:** <=4 rounds, <=12 teammates per round, <=200-word proposals per teammate. Lift only on explicit user direction.

## Step 6 — Hold after frontier

Leave the team alive after the final DRAFT. The user may:
- Steer a teammate by sending it a message (SendMessage/@name)
- Send another "godspeed" message to relax a constraint or add an axis (resume from current frontier — there is no TeamCreate to redo)
- Ask follow-ups

The team persists until the user explicitly asks for cleanup.

## Cleanup protocol

Only when the user explicitly asks:

1. List active teammates.
2. Send `SendMessage({to: <name>, message: {type: "shutdown_request", reason: "work complete"}})` to each.
3. Wait for `shutdown_approved` responses.
4. Confirm — there is no TeamDelete; acknowledged shutdowns are the full cleanup.

## Step 7 — Status after initialization

Emit a brief status:
- Team name created
- Axes named
- Teammates spawned (names + axes)
- Waiting on Round 1 proposals

Do not narrate internal thinking. In godspeed mode, once findings arrive, immediately compile and dispatch the next round if the frontier is still moving.
