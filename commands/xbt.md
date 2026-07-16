---
description: Judge-orchestrated TEAM mode — cross-model delegation (codex via xask), deliberative rounds with judge mediation. Slower, pondered.
argument-hint: <prompt for the judge, or leave blank to init an empty squad>
allowed-tools: [Agent, Bash, Read, Write, Edit, Glob, Grep, TaskCreate, TaskGet, TaskList, TaskUpdate, TaskOutput, SendMessage, TeamCreate, TeamDelete, WebFetch, WebSearch, LSP, Monitor]
---

# /xbreed-team — Judge-Orchestrated Team Mode (Deliberative)

This command initializes a **persistent native agent team** with YOUR current session as the team lead. You adopt the-judge persona and orchestrate specialist sub-roles (scout, reviewer, labrat) as **real teammates** with **cross-model delegation** — teammates invoke `xask codex` to bring external model perspectives into the draft.

Unlike `/xbreed` (solo one-shot subagents) or `/xgs` (godspeed Pareto all-Claude), `/xbreed-team` is the **deliberative** mode: slower, pondered, with the judge mediating cross-model views across multiple rounds.

## Step 1 — Load the judge persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Adopt the posture: you are the top of the stack, you judge explicitly on named axes, you aggregate best-of-each, you draft-then-dispatch. Your output shape is the DRAFT protocol from that file.

## Step 2 — Create the team

Pick a concise unique `team_name`:
- Take the first 2-3 significant words of the user's prompt (lowercased, hyphen-joined)
- Append a short timestamp suffix (last 4 digits of `date +%s`) to avoid collisions across invocations
- Fallback: `xbreed-squad-<ts>` if the prompt is empty or yields no significant words

Call:

```
TeamCreate(
  team_name="<your-unique-name>",
  agent_type="team-lead",
  description="<the full user prompt, truncated to 200 chars if longer>"
)
```

If `TeamCreate` fails because a team already exists for this session, auto-cleanup: shutdown idle teammates + TeamDelete + retry. Do not block — do not ask the user.

## Scratch working directory (team cwd)

Judge-orchestrated runs work in an isolated **scratch checkout**, NOT per-agent git worktrees. Before spawning teammates, create it and `cd` in:

```bash
ORIGIN_URL=$(git remote get-url origin)
SCRATCH=$(mktemp -d "/tmp/xbt-XXXXXX")
git clone --no-hardlinks . "$SCRATCH"
cd "$SCRATCH" && git remote set-url origin "$ORIGIN_URL"
git checkout -b "xbt/<topic>-<ts>"
```

All teammates operate inside `$SCRATCH`. Do NOT spawn agents with `isolation: "worktree"`.

**Model routing (locked):** the judge (this session) runs on Fable 5; every dispatched teammate runs on Sonnet (`model: "sonnet"`) — the intermediary that delegates to codex. Codex delegations through xask are unchanged.

## Step 3 — Parse the prompt

The user's prompt is:

$ARGUMENTS

- If `$ARGUMENTS` is empty, the team was initialized without a specific task. Skip to Step 6 and wait for the user to direct the team with their next message.
- Otherwise, treat `$ARGUMENTS` as the problem to judge / draft per the-judge protocol. Decide which sub-roles (if any) you need.

**Planner-first is unconditional** (matches `~/.claude/agents/the-judge.md` sub-role table). Spawn `the-planner` as the Phase 0 teammate BEFORE any specialist:

```
Agent(subagent_type="the-planner", team_name="<team>", name="ccs-planner-r0", model="sonnet",
      prompt="WWKD Phase 0 data walk + skeleton for: <full user prompt>. FIRST tool call MUST be Skill(skill='wwkd'). Deliver plan artifact to team-lead. | godspeed")
```

Wait for the plan artifact. It becomes the skeleton against which downstream specialist dispatch checks for drift.

Composition: `/xbt /wwkd <spec>` is the explicit form of the same behavior.

## Step 4 — Dispatch sub-roles AS TEAMMATES with xask gate

When you decide a sub-role is needed, spawn it as a **persistent team member**:

```
Agent(
  subagent_type="scout" | "reviewer" | "labrat" | "the-planner",
  team_name="<the team you just created>",
  name="<unique teammate name>",
  model="sonnet",
  prompt="<task brief with mandatory xask gate and peer roster>"
)
```

These are **real teammates** — they persist, can be chatted with via Shift+Down, will DM back via SendMessage, go idle between turns, and follow shutdown protocol.

**DO NOT** fall back to `Agent(subagent_type="general-purpose", ...)` with inlined persona.

**Create a TaskCreate task per sub-role before or immediately after spawning.**

### Peer roster and cross-critique DMs

Every teammate brief MUST include:
1. **Full peer roster** — all teammate names committed in this dispatch (so they can DM each other)
2. **Cross-critique instruction:** `"After completing your research/analysis, DM each peer by name with a one-line critique or reinforcement of their likely findings based on what you discovered. Use SendMessage({to: '<peer-name>', message: '<critique>'})."`

Peers DM each other directly for lateral information flow. The judge collects all reports + DM summaries and pastes them into the distiller's prompt for synthesis.

### Godspeed inheritance

If `$ARGUMENTS` contains "godspeed", append this block to EVERY teammate's brief (after task instructions, before the xask gate):

> **GODSPEED MODE (inherited from judge):** You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls.

### xask gate, epistemic constraints, and axis→profile mapping

Read `~/.claude/commands/references/xbreed-shared.md` for the full 4-layer xask gate (per-role), epistemic constraints, divergence mandate, judge weighting, and axis→profile mapping. Apply them to every teammate brief.

### Budget

Scale up to 12 teammates when the problem has many independent sub-questions.

## Step 5 — Distiller synthesis + deliberative rounds

### Phase A — Distiller aggregation

Once all teammates have spawned, spawn the **distiller** with | godspeed:

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate findings into one deduplicated, confidence-scored brief. <paste all teammate reports + peer DM SendMessage cross-critiques>. Return format: State block with deduplicated claims, Unknowns block with contradictions, duplicate count. SendMessage your synthesis to the judge (team lead) when done."
)
```

The distiller:
- Reads all teammate reports + peer DM SendMessage cross-critiques
- Deduplicates overlapping findings
- Flags contradictions (CONFLICT blocks) for the judge
- Assigns confidence scores (high/medium/low/unverified)
- Sends one clean synthesis to the judge

### Phase B — Judge-driven iteration

Using the distiller's synthesis, the judge **mediates**:

1. **Draft** initial DRAFT from distiller output.
2. **Challenge** specific findings via targeted SendMessage follow-ups to individual teammates. Push back on weak claims, probe gaps, ask for deeper investigation.
3. **Teammates refine** and re-report. Peer DMs flow again.
4. **Re-distill** if findings changed substantially (send updated reports to distiller via SendMessage). For minor refinements, judge aggregates directly.
5. **Populate CONFLICTS block** if cross-model divergence found (codex vs. claude contradictions on the same claim).
6. **Repeat 2-5** until the judge is satisfied with the DRAFT quality.

**Soft ceiling: 4 deliberative rounds** (aligned with judge godspeed limit). After 4 rounds with no DRAFT progress, emit a CONFLICTS-only output and halt, naming unresolved items. Judge can override but must state why.

**This is NOT godspeed.** Deliberative rounds are sequential depth (judge challenges, teammates refine). For parallel Pareto width, use `/xgs`.


## Auto-cleanup after DRAFT

Once the final DRAFT is emitted (frontier reached / 4 rounds / halt): immediately shutdown all teammates in parallel via `SendMessage shutdown_request`, wait for shutdown_approved, then `TeamDelete`. If TeamDelete fails with "active members", run `xbreed-cleanup <team-name>` via Bash. Do not ask the user — the team served its purpose, kill it.

If the user wants to continue on a new axis, they invoke `/xbt` again; spawning is cheap.

## Terminal step — commit + push (mandatory)

When the frontier is reached (or the run completes), the judge's LAST act is to commit and push from the scratch dir, then discard it:

```bash
git add -A
git commit -m "xbt: <one-line summary of the frontier result>"
git push -u origin "xbt/<topic>-<ts>"
cd - >/dev/null && rm -rf "$SCRATCH"
```

The orchestration ends with a pushed branch — not a dirty tree the user has to clean up.

## Step 7 — Emit a brief status after initialization

End your initialization turn with a short status:
- Team name created
- Which sub-roles were spawned and what task each was given
- Whether waiting on teammate replies or drafting

Do not narrate internal thinking. The DRAFT comes in a later turn once findings are in.
