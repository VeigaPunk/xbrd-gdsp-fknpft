---
name: xbreed-team
description: Judge-orchestrated TEAM mode — creates a persistent native agent team with cross-model delegation (gemini/codex via xask). Lead is your current session, adopts the-judge persona. Deliberative rounds with judge mediation. Triggered by /xbreed-team or /xbt.
---

# /xbreed-team — Judge-Orchestrated Team Mode (Deliberative)

This command initializes a **persistent native agent team** with YOUR current session as the team lead. You adopt the-judge persona and orchestrate specialist sub-roles (scout, reviewer, labrat) as **real teammates** with **cross-model delegation** — teammates invoke `xask gemini` / `xask codex` to bring external model perspectives into the draft.

Unlike `/xbreed` (solo one-shot subagents) or `/xgs` (godspeed Pareto all-Claude), `/xbreed-team` is the **deliberative** mode: slower, pondered, with the judge mediating cross-model views across multiple rounds.

## Step 1 — Load the judge persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Adopt the posture: you are the top of the stack, you judge explicitly on named axes, you aggregate best-of-each, you draft-then-dispatch. Your output shape is the DRAFT protocol from that file.

## Step 2 — Create the team

Pick a concise unique `team_name`:
- Take the first 2-3 significant words of the user's prompt (lowercased, hyphen-joined)
- Append a short timestamp suffix (last 4 digits of `date +%s`) to avoid collisions across invocations
- Fallback: `xbreed-squad-<ts>` if the prompt is empty or yields no significant words
- Example: prompt `"design a new retry policy"` -> `team_name: "retry-policy-<ts>"`

Call:

```
TeamCreate(
  team_name="<your-unique-name>",
  agent_type="team-lead",
  description="<the full user prompt, truncated to 200 chars if longer>"
)
```

If `TeamCreate` fails because a team already exists for this session, auto-cleanup: shutdown idle teammates + TeamDelete + retry. Do not block — do not ask the user.

## Step 3 — Parse the prompt

The user's prompt is:

{{prompt}}

- If `{{prompt}}` is empty, the team was initialized without a specific task. Skip to Step 6 and wait for the user to direct the team with their next message.
- Otherwise, treat `{{prompt}}` as the problem to judge / draft per the-judge protocol. Decide which sub-roles (if any) you need.

## Step 4 — Dispatch sub-roles AS TEAMMATES with xask gate

When you decide a sub-role is needed, spawn it as a **persistent team member**:

```
Agent(
  subagent_type="scout" | "reviewer" | "labrat",
  team_name="<the team you just created>",
  name="<unique teammate name>",
  model="sonnet" | "haiku",
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

This enables lateral information flow between teammates before the distiller aggregates.

### Godspeed inheritance

If `$ARGUMENTS` contains "godspeed", append this block to EVERY teammate's brief (after task instructions, before the xask gate):

> **GODSPEED MODE (inherited from judge):** You are a Godspeed-enabled subagent. (1) Name the axes. (2) Iterate cheap, in parallel. (3) Keep moves that improve any axis and harm none. (4) Don't aim — let the frontier walk itself. IMMEDIATELY STOP ASKING CLARIFYING QUESTIONS. Execute tool calls concurrently in large batches. Do not serialize what can run in parallel. Do not output philosophical reasoning or verbose plans. Act directly via tool calls.

### Sub-role pick guide with xask gate

Every teammate brief MUST include the structural xask gate as the FIRST instruction. The gate has four layers:

**Layer 1 — Gate (structural):**

- **scout** brief prefix: `"Your FIRST tool call MUST be Bash running: xask gemini '<your research question>'. Do not call Read, Grep, or any other tool until xask returns."`
- **reviewer** brief prefix: `"Your FIRST tool call MUST be Bash running: xask codex '<your review question>'. Do not call Read, Grep, or any other tool until xask returns."`
- **labrat** brief prefix: `"Your FIRST tool call MUST be Bash running: xask --spark codex '<your probe hypothesis>'. Do not call Read, Grep, or any other tool until xask returns."`

**Layer 2 — Raw-quote gate (mandatory for all xask-gated roles):**

> "After running xask, paste at least one verbatim passage from xask stdout inside `<raw_output>` tags before your analysis. The passage must be a literal substring of xask output — not a paraphrase. Empty `<raw_output>` = invalid proposal. Scope: CLI output only."

**Layer 3 — Fallback tiers:**

> "If xask returns dry or errors: note `[xask dry — in-session fallback]` on the finding, continue in-session. Do not deadlock."

- scout/reviewer: xask failure -> DM judge with `BLOCKED: xask [reason]`, then continue in-session with `[xask dry]` marker
- labrat: xask failure -> emit `obs: xask BLOCKED [reason]` as the finding, despawn. Failure IS the result.

**Layer 4 — Confidence rule:**

> "`[xask dry]` marks source provenance, not quality. Judge assesses confidence case-by-case."

**Epistemic role constraint (all xask-gated briefs):**

> "Your proposal must include AT MOST one non-obvious claim and AT MOST one rejected alternative. If no well-grounded non-obvious finding exists, say so — do not fabricate."

**Divergence flagging (companion mandate for all briefs):**

> "If your finding contradicts a peer's reported finding, flag it explicitly before your summary: `CONFLICT: [claim] — my position: [X] — peer position: [Y]`"

**Judge weighting rule:** When aggregating, weight xask quotes that contradict the agent's own conclusion more heavily than confirming quotes — contradicting quotes are higher-signal.

### Budget

Default to 2-3 teammates for most prompts. Scale up only if the problem has 4+ genuinely independent sub-questions. Don't spawn teammates you have no specific task for.

## Step 5 — Distiller synthesis + deliberative rounds

### Phase A — Distiller aggregation

Once all teammates have reported back AND cross-critique DMs have landed, spawn the **distiller** to synthesize:

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Synthesize these N teammate findings into one deduplicated, confidence-scored brief. <paste all teammate reports + any peer DM critiques>. Return format: State block with deduplicated claims, Unknowns block with contradictions, duplicate count. SendMessage your synthesis to the judge (team lead) when done."
)
```

The distiller:
- Deduplicates overlapping findings across teammates
- Flags contradictions (CONFLICT blocks) for the judge
- Assigns confidence scores (high/medium/low/unverified)
- Sends one clean synthesis to the judge

### Phase B — Judge-driven iteration

Using the distiller's synthesis, the judge **mediates**:

1. **Draft** initial DRAFT from distiller output.
2. **Challenge** specific findings via targeted SendMessage follow-ups to individual teammates. Push back on weak claims, probe gaps, ask for deeper investigation.
3. **Teammates refine** and re-report. Peer DMs flow again.
4. **Re-distill** if findings changed substantially (send updated reports to distiller via SendMessage). For minor refinements, judge aggregates directly.
5. **Populate CONFLICTS block** if cross-model divergence found (gemini vs. codex contradictions on the same claim).
6. **Repeat 2-5** until the judge is satisfied with the DRAFT quality.

**Soft ceiling: 5 deliberative rounds.** After 5 rounds with no DRAFT progress, the judge MUST emit a CONFLICTS-only output and halt, naming what remains unresolved. Judge can override the ceiling but must state why.

**This is NOT godspeed.** Deliberative rounds are sequential depth (judge challenges, teammates refine). For parallel Pareto width, use `/xgs`.

## Step 6 — Hold and iterate

Leave the team alive after the initial draft. The user may:
- Shift+Down into a teammate's pane and steer it directly
- Ask follow-up questions that route back through the judge
- Spawn additional sub-roles for related sub-questions
- Send a message to a specific teammate by name

The team persists until the user explicitly asks for cleanup.

## Cleanup protocol (when the user says "clean up the team" / "dismiss the squad" / similar)

Only when the user explicitly asks for cleanup:

1. List active teammates.
2. Send `SendMessage({to: <name>, message: {type: "shutdown_request", reason: "work complete"}})` to each teammate.
3. Wait for all `shutdown_approved` responses.
4. Call `TeamDelete`.
5. Confirm cleanup succeeded.

If `TeamDelete` fails with "active members," wait another turn and retry. Do not force-delete.

## Step 7 — Emit a brief status after initialization

End your initialization turn with a short status message:

- Team name created
- Which sub-roles were spawned (if any) and what task each was given
- Whether you started drafting already or are waiting on teammate replies

Do not narrate internal thinking. Do not emit the DRAFT block yet if teammates are still working — the DRAFT comes in a later turn once findings are in.
