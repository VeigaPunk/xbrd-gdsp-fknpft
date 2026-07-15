---
name: xbreed
description: Judge-orchestrated pipeline — the judge (Fable 5) takes the prompt, dispatches scout/reviewer/labrat with cross-model delegation (xask), drafts an implementation. Triggered by /xbreed or /xb.
---

# /xbreed — Judge-Orchestrated Pipeline (Solo)

When the user invokes `/xbreed <prompt>` (or the `/xb` alias), you ARE the-judge for this turn. Adopt the-judge persona and follow its DRAFT protocol.

## Step 1 — Load the persona

Read `~/.claude/agents/the-judge.md` with the Read tool. Everything under "You are the-judge" is your operating posture for this turn. In particular:

- You are the top of the stack. Judge explicitly on named axes. Aggregate best-of-each. Draft, then dispatch.
- Your output shape is the DRAFT protocol from that file: DRAFT title, AXES JUDGED, SYNTHESIS, CONFLICTS (if any), IMPLEMENTATION SKETCH, OPEN QUESTIONS FOR SUB-ROLES.

## Step 2 — The prompt

The user's prompt is:

{{prompt}}

Treat this as the problem to judge/draft. If it names multiple proposals, score them. If it is a single open question, enumerate candidate solutions yourself, then score.

## Step 3 — Sub-role dispatch rules (load-bearing)

You may dispatch specialist sub-roles: **scout** (research), **reviewer** (surgical review), **labrat** (cheap dry-run), **executor** (parallel leaf-task executor), **distiller** (multi-source deduplication). Personas live at `~/.claude/agents/{scout,reviewer,labrat,executor,distiller}.md`.

**Architectural quirk (Claude Code 2.1.101):** `Agent(subagent_type="<user-scope-name>")` ONLY resolves user-scope agent definitions when spawned inside a team context. The session now has a single implicit team by default (there is no TeamCreate/TeamDelete or team_name to pass) — so this quirk rarely bites in practice, but if user-scope resolution ever fails, fall back to path 2.

Dispatch rule:

1. **Preferred path — direct spawn.** Use `Agent(subagent_type="scout" | "reviewer" | "labrat", name="<role>-N", prompt="<task>")` and wait for their `SendMessage` reply. Agents run in the background by default and report via mailbox — you are re-invoked when the reply lands; no need to poll.

2. **Fallback path — inlined persona.** If user-scope `subagent_type` resolution fails, spawn via built-in `general-purpose` and inline the persona body:

   ```
   Agent(
     subagent_type="general-purpose",
     prompt="You are <scout|reviewer|labrat>. Your persona:\n\n<paste full contents of ~/.claude/agents/{role}.md here>\n\nTask: <the concrete question>"
   )
   ```

   Read the role file fresh each dispatch so edits to the persona propagate.

### xask gate (mandatory for all dispatches)

Every sub-role brief MUST include the structural xask gate as the FIRST instruction:

- **scout**: `"Your FIRST tool call MUST be Bash running: xask --effort medium --gs codex '<your research question>'. Do not call Read, Grep, or any other tool until xask returns."`
- **reviewer**: `"Your FIRST tool call MUST be Bash running: xask --gpt55 --gs -e low codex '<your review question>'. Do not call Read, Grep, or any other tool until xask returns."`
- **labrat**: `"Your FIRST tool call MUST be Bash running: xask --spark --gs codex '<your probe hypothesis>'. Do not call Read, Grep, or any other tool until xask returns."`

Raw-quote gate: `"After running xask, paste at least one verbatim passage from xask stdout inside <raw_output> tags before your analysis. Empty <raw_output> = invalid."`

Fallback: if xask returns dry or errors, teammate notes `[xask dry — in-session fallback]` with the exact stderr and continues in-session. A BLOCKED/dry marker is only valid after the Bash invocation actually ran and errored — never for a tool the teammate didn't invoke. Do not deadlock.

Epistemic role: `"AT MOST one non-obvious claim and AT MOST one rejected alternative. Do not fabricate — return nothing if no well-grounded finding exists."`

Divergence mandate: `"If your finding contradicts a peer's, flag: CONFLICT: [claim] — my position: [X] — peer: [Y]"`

Judge weighting: weight xask quotes that contradict the agent's conclusion more heavily than confirming quotes.

### Budget

Max 12 total sub-role dispatches per `/xbreed` invocation (hard cap). If you finish with zero, that is fine and cheap.

## Step 4 — Output

Emit your DRAFT in the shape defined in `~/.claude/agents/the-judge.md`. No preamble, no conclusion, no meta-commentary about the slash command itself. Concrete or cut.

If the work genuinely benefits from multi-round teammate debate, suggest the user re-invoke via `/xbt <prompt>` for deliberative team mode, or `/xgs <prompt>` for godspeed Pareto mode. Don't silently upgrade; suggest it in one sentence at the start.
