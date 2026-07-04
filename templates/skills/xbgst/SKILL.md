---
name: xbgst
description: Godspeed Pareto orchestrator with cross-model delegation — combines /xgs speed (axis-scored Pareto rounds) with /xbt depth (xask codex). Teammates bring external model perspectives at godspeed pace. Triggered by /xbgst.
---

# /xbgst — Godspeed Pareto + Cross-Model Delegation

The full crossbreed: godspeed Pareto walk (parallel proposals, axis-scored, round-capped) with cross-model delegation (teammates invoke `xask codex`). Combines the speed of `/xgs` with the external perspectives of `/xbt`.

Use when you want fast Pareto convergence AND cross-model views. For all-Claude speed without cross-model, use `/xgs`. For slower deliberative mediation with cross-model, use `/xbt`.

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

If empty, wait for user direction. Otherwise, proceed to four-phase godspeed with xask.

## Step 4 — Four-phase godspeed protocol (with xask gate)

### Phase 0 — Name the axes

Emit up to 8 axes (name + direction + observable). Incorporate user-named axes; infer the rest.

### Phase 1 — Assign deterministic teammate names

For each axis, assign a name using the-judge.md naming convention: `{prefix}-{role}-{suffix}`. Commit ALL names before spawning.

Axis -> profile mapping (from the-judge.md dispatch table):
- Research, prior art, outside-world -> `scout` (sonnet) — delegates to `xask --effort medium --gs codex`
- Correctness, bugs, code review -> `reviewer` (sonnet) — delegates to `xask --gpt55 --gs -e low codex`
- Empirical probes, dry-runs -> `labrat` (sonnet) — delegates to `xask --spark --gs codex`
- Code execution, implementation -> `executor` (sonnet) — delegates to `xask --spark --gs codex`
- Cross-axis patterns, breadth -> `connector` (sonnet) — delegates to `xask --effort medium codex` (no `--gs`; avoids double-godspeed frame on pontification-prone lane)
- Findings synthesis, dedup -> `distiller` (sonnet) — in-session text synthesis (no xask)
- Complexity reduction, YAGNI -> `simplifier` (sonnet) — uses CC native tools

Cap: <=12 teammates per round.

### Phase 2 — Spawn all with full peer roster AND xask gate

Each brief includes:
1. Full peer roster (all names from Phase 1)
2. Axis assignment (name + direction + observable)
3. **Godspeed mode** (always — /xbgst is inherently godspeed): `"GODSPEED MODE (inherited from judge): You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls."`
4. **Structural xask gate** (mandatory for roles that delegate)
5. Task: propose ONE move on their axis (<=200 words)
6. After proposing, DM each peer by name with a one-line critique
7. Mark task completed after sending

#### xask gate by role (four layers)

**Layer 1 — Gate (structural, per-role):**

- **scout** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --effort medium --gs codex '<your research question for this axis>'. Do not call Read, Grep, or any other tool until xask returns."`
- **reviewer** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --gpt55 --gs -e low codex '<your review question for this axis>'. Do not call Read, Grep, or any other tool until xask returns."`
- **labrat** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --spark --gs codex '<your probe hypothesis for this axis>'. Do not call Read, Grep, or any other tool until xask returns."`
- **connector** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --effort medium codex '<your cross-axis pattern question>'. Do not call Read, Grep, or any other tool until xask returns. Note: no --gs — connector deliberately skips explicit godspeed-skill load to avoid double-frame stacking."`
- **executor** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --spark --gs codex '<your implementation task for this axis>'. Do not call Read, Grep, or any other tool until xask returns."`
- **mutation-tester** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --spark --gs codex '<generate mutation for this function>'. Do not call Read, Grep, or any other tool until xask returns."`
- **simplifier/distiller**: No xask gate. These use CC native tools or in-session synthesis.

**Layer 2 — Raw-quote gate (mandatory for all xask-gated roles):**

> "After running xask, paste at least one verbatim passage from xask stdout inside `<raw_output>` tags before your analysis. The passage must be a literal substring of xask output — not a paraphrase, not a summary. Empty `<raw_output>` = invalid proposal. Scope: this gate applies only to fixed CLI output, not to quoting other agents or prior turns."

**Layer 3 — Fallback tiers:**

> "If xask returns dry or errors: note `[xask dry — in-session fallback]` on the finding, continue in-session with CC native tools. Do not deadlock. Do not stall the round."

- scout/reviewer/connector: xask failure -> continue in-session with `[xask dry]` marker. DM judge is NOT required in godspeed (would stall the round). Just mark the source.
- labrat: xask failure -> emit `obs: xask BLOCKED [exact stderr]` as the finding (only after the Bash invocation actually ran and errored — never for a tool you didn't invoke), despawn.

**Layer 4 — Confidence rule:**

> "`[xask dry]` marks source provenance, not quality. Judge assesses confidence at Pareto filter time."

**Epistemic role constraint (all xask-gated briefs):**

> "Your proposal must include AT MOST one non-obvious claim and AT MOST one rejected alternative. If no well-grounded non-obvious finding exists, say so — do not fabricate. Return nothing rather than force a low-confidence finding."

**Divergence mandate (all briefs, including non-xask roles):**

> "If your finding contradicts a peer's reported finding, flag: `CONFLICT: [claim] — my position: [X] — peer position: [Y]`"

Task tracking (optional): TaskCreate/TaskUpdate/TaskList are listed as DEFERRED tools. ToolSearch("select:TaskCreate,TaskUpdate,TaskList") first is defensive best practice, not load-bearing — do not phrase it as a hard requirement. If you skip task tracking, teammates report via SendMessage and that is sufficient.

### Phase 3 — Rounds begin

Teammates work in parallel, invoking xask as their first tool call, then proposing moves.

**Distiller synthesis:** Once all teammates have proposed AND cross-critique DMs have landed, spawn the distiller:

```
Agent(
  subagent_type="distiller",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag cross-model (codex) vs in-session (claude) contradictions, assign confidence. SendMessage your synthesis to the judge (team lead) when done."
)
```

**Pareto filter (on distiller output):** Build the moves x axes matrix. Accept moves improving >=1 axis with zero regressions. Reject moves with regressions. Compile survivors into ROUND N summary.

**Re-distill each round:** For rounds 2+, send updated proposals to the distiller via SendMessage. Only re-spawn if shut down.

**CONFLICTS block** uses **model labels** (not teammate labels) since cross-model delegation is active:
```
CONFLICTS (emit only if cross-model contradictions exist):
  - claim: <contested fact>
    model: codex (via <teammate-name>) — <position>
    model: claude (in-session, via <teammate-name>) — <position>
    judge_resolution: <chosen position + rationale>
    escalate_to: <sub-role if unresolved>
```

Trigger: opposite verdicts on same claim — cross-model (codex) vs in-session (claude).

**Falsification probe (after Pareto filter, before exit check):** If the highest-divergence surviving move has a claim that no other teammate challenged, dispatch ONE targeted xask call to the opposing model for attack. Cap: one exchange, one claim. This adds 3-8s but catches convergent blind spots. Skip if all claims were cross-critiqued.

**Judge weighting rule:** Weight xask quotes that contradict the agent's own conclusion more heavily than quotes that confirm it — contradicting quotes are higher-signal finds.

**Exit check:** Frontier reached (zero survivors / duplicates / 4 rounds / user halt) -> final DRAFT with AXES FINAL STATE. Otherwise dispatch Round N+1 using current frontier as baseline.

## Step 5 — Keep iterating

After each round, immediately assess and dispatch next round if frontier still moving. Do not pause. Do not ask. The user interrupts when they want to steer.

**Caps:** <=4 rounds, <=12 teammates per round, <=200-word proposals. Lift only on user direction.

## Step 6 — Hold after frontier

Team stays alive. User may:
- Steer a teammate by sending it a message (SendMessage/@name)
- Send another message to resume with new axis
- Ask follow-ups

## Cleanup

On explicit user request only: SendMessage shutdown_request to each teammate, await shutdown_approved, confirm.

## Step 7 — Status after init

Emit: team name, axes, teammates spawned (with their xask delegation targets), waiting on Round 1.
Do not narrate. Immediately compile and dispatch next round when findings arrive.

