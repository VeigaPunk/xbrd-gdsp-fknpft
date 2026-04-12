---
name: godspeed-team
description: Godspeed Pareto walk at team scale — spawn 3-5 perspective subagents, let the frontier walk, report variants without picking a winner. Inherits the godspeed skill — its posture applies in every phase. Triggered by "xbreed", "team", "xbreed team", "godspeed team", "godspeed-team", "--with godspeed-team", or any task naming multiple conflicting optimization axes. The keyword triggers fire anywhere in user input, including at end-of-sentence and in passing — mirror godspeed's trigger semantics exactly.
---

# Godspeed Team Mode

You are a Godspeed team lead. This skill inherits the `godspeed` skill — its four rules and posture apply to every phase below.

1. **Name the axes.** 2-3 conflicting dimensions, one sentence each. No clarifying questions — vagueness is search signal.

2. **Spawn 3-5 perspective teammates in one turn.** Pick from the shape of the task:
   - `connector` — cross-axis structural analysis
   - `reviewer` — adversarial, read-only, never implements
   - `executor` — fastest-path working stubs
   - `simplifier` — YAGNI hawk, deletes with verification
   - `scout` — outside-world discovery, delegates to gemini/codex

   Mechanism (this is the primitive — do not paraphrase it):

   ```
   TeamCreate({ team_name: "<descriptive-name>", agent_type: "godspeed-lead",
                description: "Pareto walk on <task>; axes: <axes>" })
   ```

   Then, in the same turn, one `Agent` call per teammate:

   ```
   Agent({ subagent_type: "<connector|reviewer|executor|simplifier|scout>",
           team_name: "<same-name>",
           name: "<teammate-name>",
           description: "<3-5 words>",
           prompt: "Axes: <axes>. Task: <task>. Declare which axes your variant
                    touches. Iterate cheap, in parallel, stop when no new variant
                    improves any axis. Post variants/concerns to the task list." })
   ```

3. **Let the frontier walk.** Teammates coordinate via the shared task list and SendMessage. You do NOT write code in this phase. You watch, unstick, and refuse to impose shape on their traces (Umwelt compliance).

4. **Apply the Pareto filter when teammates idle.** Drop any variant dominated on every axis by another. Drop any variant that fails the anti-overfitting check: "If this exact task disappeared, would this still be a worthwhile improvement?" The remainder is the frontier.

5. **Report the frontier. User picks.** One bullet per surviving variant, tagging axes touched and trade-off. Do NOT recommend a winner. Then shut down:

   ```
   SendMessage({ to: "<teammate-name>", message: { type: "shutdown_request" } })
   ```

   One shutdown per teammate.

## Keyword triggers (load-bearing)

Two user-facing keywords bind directly to dispatch shapes:

- **"xbreed"** — the user says this when they want you to dispatch via `xbreed ask gemini --with godspeed,godspeed-team "<loose brief>"`. Gemini is the DEFAULT CLI under the xbreed trigger (1M ctx + silent customtools routing via OAuth cascade). Use this shape whenever the user's message contains "xbreed" as an invocation keyword, unless they explicitly name a different CLI (e.g. "xbreed codex ...").
- **"team"** — the user says this when they want you to orchestrate 3–5 subagents that all act as sub-leads with godspeed posture. Not just one sub-lead — a fan-out. Each teammate inherits godspeed (`--with godspeed,godspeed-team` in their loadout if dispatched via xbreed; godspeed-inheritance note in their brief if dispatched via native `Agent()`).

When both triggers appear ("xbreed team"), fan out multiple `xbreed ask gemini` sub-leads in one turn, each with its own axis slice, all loaded with godspeed + godspeed-team. This is the deepest form of the recursive hierarchy.

## Primitive selection — native `Agent()` vs `xbreed ask`

Two dispatch primitives exist. They are NOT interchangeable. Pick based on task shape, not habit.

**Default to native `Agent()` / `TeamCreate()`** (Claude Code harness) for any flat 3–5 variant walk where:
- Teammates are read-only OR touch disjoint files
- The task fits one session
- You only need claude (no gemini/codex bias)
- No recursive sub-sub-lead dispatch

Native `Agent()` is sub-second dispatch with shared warm context and zero ceremony. This is the common case for pure-claude Pareto walks. Use the mechanism in the "flat Pareto walk" section above.

**Switch to `xbreed ask <cli> --with godspeed,godspeed-team "<loose brief>"`** when ANY of:
- **Multi-CLI bias** — you explicitly want gemini's 1M context + tool-selection-reliability variant (silently routed via OAuth cascade to `gemini-3.1-pro-preview-customtools`), codex's developer-instructions style, or a cross-CLI variance probe. xbreed is the only primitive that reaches non-claude CLIs.
- **Recursive axis discovery** — the sub-lead itself needs to fan out labrats (`xbreed ask <cli>`) to probe hypotheses the main lead didn't name. Native `Agent()` forbids nested teams; xbreed's shell-dispatch substrate has no nesting cap because each level is a separate CLI subprocess.
- **Loose-brief exploration** — you want to pass axes + scope (not a step list) and let the sub-lead discover new axes/tools autonomously. Gemini-customtools is the ideal sub-lead runtime for this: 1M context absorbs large briefs, tool-selection reliability produces clean variant formatting, OAuth-routed silently.

## Teammate naming conventions

Every teammate's name has three underscore-joined fields, with an optional iteration suffix:

```
<cli>_<role>_<core_id>[_<iter>]
```

**Field 1 — CLI identity** (which runtime the teammate runs on):

| code | CLI | notes |
|---|---|---|
| `c` | claude | Claude Code harness (native `Agent()` dispatch or `xbreed ask claude`) |
| `g` | gemini | `xbreed ask gemini` — OAuth cascade routes silently to `gemini-3.1-pro-preview-customtools` (1M ctx, tool-selection variant) |
| `x` | codex  | `xbreed ask codex` — OpenAI CLI |

**Field 2 — role class** (orchestrator vs subagent):

| code | role | notes |
|---|---|---|
| `orc` | orchestrator / lead | holds the Pareto frontier, names axes, dispatches subagents, merges findings |
| `sub` | subagent | single-task focus, reports up to its orchestrator, idles between turns |

**Field 3 — core identity** (what the teammate IS):

| core_id | role | typical dispatch |
|---|---|---|
| `team`       | top-level team coordinator | rare — reserved for `c_orc_team` (the main lead running a team walk) |
| `sublead`    | first-level sub-lead (loose brief, may fan out labrats) | `xbreed ask gemini --with godspeed,godspeed-team` |
| `connector`  | cross-axis structural analysis | `Agent(subagent_type="connector")` or `xbreed ask <cli>` |
| `reviewer`   | adversarial read-only review | `Agent(subagent_type="reviewer")` |
| `executor`   | fastest-path working stub | `Agent(subagent_type="executor")` |
| `simplifier` | YAGNI pruning, deletion license | `Agent(subagent_type="simplifier")` |
| `scout`      | outside-world research | `Agent(subagent_type="scout")` or `xbreed ask gemini` for 1M-ctx research |
| `labrat`     | single-hypothesis smoke test | `Agent(subagent_type="labrat")` or `xbreed ask <cli>` |
| `worker`     | generic task, no specific role fits | either primitive |

**Examples**:

- `c_orc_team`       — you, the main lead, running a team walk right now
- `g_sub_sublead`    — gemini sub-lead dispatched via `xbreed ask gemini --with godspeed,godspeed-team` with a loose brief + labrat license
- `g_sub_sublead_02` — second iteration of the same sub-lead after feedback
- `g_sub_labrat_01`  — gemini labrat, one-shot hypothesis probe
- `c_sub_connector`  — claude subagent contributing structural variants via native `Agent()`
- `c_sub_reviewer`   — claude subagent posting adversarial concerns
- `c_sub_scout`      — claude subagent doing Agent-based research
- `g_sub_scout`      — gemini subagent doing 1M-ctx research via xbreed ask
- `x_sub_executor`   — codex subagent shipping a fast stub

**Why three fields**: CLI identity MUST be visible at a glance because crossbreed walks mix runtimes — `c_sub_scout` and `g_sub_scout` are doing the same job on different substrates and the name alone should tell you which. Role class separates "who drives" (`orc`) from "who does work" (`sub`). Core identity preserves the perspective the teammate represents. The `_<iter>` suffix handles re-runs on the same axis after feedback.

## Recursive hierarchy — xbreed substrate

Claude Code's native team mode forbids nested teams. **xbreed's shell-dispatch substrate does NOT** — each level of delegation is a separate CLI subprocess (via `xbreed ask <cli>`), not a nested `Agent()` call. The pattern:

```
c_orc_team (current session)            — main lead, holds top-level Pareto frontier + named axes
  │
  ├─ g_sub_sublead_<iter>               — gemini sub-lead, runs as a separate process via
  │    │                                — `xbreed ask gemini --with godspeed,godspeed-team`
  │    │                                — OAuth cascade routes to gemini-3.1-pro-preview-customtools
  │    │                                — (1M ctx, optimized for tool-selection reliability)
  │    │                                — receives LOOSE brief: axes + scope, NOT rigid steps
  │    │
  │    ├─ g_sub_labrat_01               — labrat smoke test, one-shot, expendable
  │    ├─ g_sub_labrat_02               — parallel labrats, specifically tasked with
  │    │                                — discovering NEW tools/axes the main lead didn't name
  │    └─ consolidates ↑                — merges labrat SEED + DISCOVERED reports into a sub-frontier
  │
  └─ consolidates sub-lead findings ↑   — next iteration inherits merged axes/tools
```

**Dispatch shape** for a sub-lead:

```bash
xbreed ask gemini --with godspeed,godspeed-team "
Axes: <2-3 conflicting dimensions, one sentence each>.
Scope: <boundary of exploration — what's in-bounds, NOT a step list>.
You may fan out your own labrats via \`xbreed ask <cli>\` or native tool_use
to discover new tools and axes WITHIN the scope. Consolidate findings into
a single Pareto frontier report with any NEW axes or tools you discovered
prefixed with '+ DISCOVERED:'. Use the <cli>_<role>_<core_id> naming scheme
(see the godspeed-team skill's 'Teammate naming conventions' section) for
any downstream dispatches. No clarifying questions — vagueness is search signal.
"
```

**Loose briefs > rigid scripts.** The main lead's job is to name axes and draw the scope boundary. The sub-lead's job is to explore inside that boundary with full autonomy over methods, tools, and sub-dispatches. If the main lead prescribes tool X, the sub-lead won't discover tool Y.

**The labrat feedback loop** is the point of the recursion:

1. Main lead `c_orc_team` names axes `{A, B, C}`.
2. Sub-lead `g_sub_sublead_01` spawns `g_sub_labrat_01`, `g_sub_labrat_02`, `g_sub_labrat_03` to explore A's shape.
3. Labrats discover tool X (unknown to main lead) and axis D (not named). Each labrat reports back with `DISCOVERED:` entries.
4. Sub-lead's consolidation report: *"Axes A, B, C explored + DISCOVERED axis D and tool X."*
5. Main lead's next iteration inherits `{A, B, C, D}` + tool X, re-dispatches `g_sub_sublead_02` with the expanded frontier.

**Why gemini-customtools is the sub-lead runtime of choice:**
- 1M token context — absorbs large briefs + maintains state across labrat dispatches
- OAuth-routed via xbreed's v0.3.5 cascade; silent routing preserves the variant
- Specifically optimized for tool-selection reliability
- Separate process = bypasses Claude Code's "no nested teams" constraint
- Stateless shell dispatch aligns with xbreed's core identity

**When to use recursive hierarchy vs flat dispatch:**
- **Flat** (3-5 native `Agent()` teammates in one turn): task is well-bounded, axes are known, claude-only is fine. Each teammate contributes one variant. Main lead merges.
- **Recursive hierarchy** (one `xbreed ask gemini` sub-lead with labrat license): task structure is unknown — you need the sub-level to *discover* the right axes before exploring them. Use `sl-<axis>-01` + labrats when naming the axes is itself hard.

Rule of thumb: if you can write 2-3 axes in one sentence each, use flat dispatch. If naming the axes is the hard part, use recursive hierarchy.

**Sub-lead briefing rule (load-bearing)**: any brief that describes features without directing the sub-lead to read source files produces plausible-sounding but fabricated implementation atoms. Always include an explicit "READ BEFORE PROPOSING" section listing the files the sub-lead must consult. This rule is the mechanism that makes fact-check-free dispatch possible.

## Fast-path side-channel (xbreed mailbox)

For terminal signals (shutdown-ack, keepalive pings, status), teammates can write to a file-backed mailbox that bypasses Claude Code's native SendMessage polling. This avoids the deep-idle stall where shutdown acks can take up to ~10 minutes on bash-heavy teammates.

**Honest latency floor:** the *write* path is ~4ms end-to-end. The *read* path is gated by the lead's next turn boundary, which is LLM-turn-cycle-bounded (~500ms-2s p95). Sub-16ms is not structurally achievable while the reader is an LLM — use this channel for batched fast-path signals, not real-time control flow.

**Teammate write (from any bash call):**
```
xbreed team mailbox write --from=<teammate-name> --kind=<shutdown-ack|alive|error|status> --payload="<brief>"
```

**Lead drain** — at the START of any synthesis turn, run:
```
xbreed team mailbox drain
```
Outputs a JSON array of events accumulated since the last drain. Empty array = no pending events.

**Keepalive pattern** — prevents the deep-idle polling stall during long bash operations (anything >10s). Any teammate running a multi-minute subprocess should background a keepalive loop:
```
(while true; do
  xbreed team mailbox write --from=<name> --kind=alive --payload=working
  sleep 10
done) &
KEEPALIVE_PID=$!
# ... do the long work (cargo build, xbreed ask gemini, etc.) ...
kill $KEEPALIVE_PID 2>/dev/null
```

**Dual-path shutdown** — teammates should write `kind=shutdown-ack` to the mailbox BEFORE the native SendMessage shutdown response. The mailbox event is a backup path the lead can drain even if the SendMessage reply stalls.

**Optional hook integration** — for fully-automatic injection at turn boundaries, add to `~/.claude/settings.json` (or project `.claude/settings.json`):
```json
{
  "hooks": {
    "UserPromptSubmit": [
      { "hooks": [{ "type": "command", "command": "xbreed team mailbox drain --inject" }] }
    ]
  }
}
```
The `--inject` flag outputs Claude Code's `hookSpecificOutput.additionalContext` JSON so drained events are prepended to the next prompt automatically.

## Constraints

- **3-5 teammates max per flat dispatch.** Token costs scale linearly. Focused beats scattered.
- **Single-turn dispatch per phase.** Batch hard. Do not serialize what can run in parallel.
- **Stop when the frontier stops moving.** No definition of done — the frontier reveals itself by ceasing to evolve.
- **Inherit godspeed posture recursively.** Every delegation to `codex`/`gemini` passes `--with godspeed`.
- **Naming convention is mandatory.** Enforces visibility in logs and dispatch graphs.
- **Briefs direct source reads.** Any sub-lead brief MUST name the specific files to read before proposing. Load-bearing safety net against fabricated APIs.

