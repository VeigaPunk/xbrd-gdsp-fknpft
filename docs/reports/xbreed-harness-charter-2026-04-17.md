# xbreed harness charter — next mission (post mailbox-r4)

**Charter filed:** 2026-04-17 during mailbox-r4-0417 R2 session
**Priority:** Deferred — queued behind mailbox-r4 completion
**Dispatch model:** `/xbgst /wwkd | godspeed` (same discipline as mailbox mission)
**Owner:** team-lead (orchestrator), fresh xbgst session TBD

---

## Scope

Three harness-level defects surfaced during mailbox-r4-0417 R1 live execution.
Record here so primary-source evidence is not lost when session context rotates.

---

## Item 1 — Effort-tier override precedence (Protocol-tier leak)

### Observation (primary-source)

When spawning teammates via Agent tool with `model: "sonnet"`, teammates are
running at **sonnet high effort** instead of the intended **sonnet medium**
mandated by `feedback_sonnet_effort_tiers.md` for distiller / simplifier / scribe.

User statement: "sonnet medium is spawning as sonnet high (as per user global
configs)."

### Hypothesis

User-global `~/.claude/settings.json` default effort (sonnet=high) beats
agent-level effort specification (sonnet=medium from `templates/agents/*.md`
frontmatter + team-lead brief). Effort routing is **Protocol-tier** (brief
text) rather than **Runtime-tier** (spawn-arg the harness respects).

### Evidence collection plan

1. `cat ~/.claude/settings.json` — capture user-global defaults
2. `grep -n "effort" ~/.claude/agents/*.md templates/agents/*.md` — capture
   per-agent frontmatter
3. Spawn a labrat with `model: "sonnet"` and explicit `effort: "medium"` in
   brief, then instruct it to `Bash: ps -ef | grep claude` and report the
   exact process args / env the harness invoked it with
4. Repeat with `model: "opus"` effort low vs high — verify whether
   `cco-distiller@opus low` (user override this session) actually ran at low
   or fell back to high
5. Enumerate ALL knobs: settings.json, agent frontmatter, `CLAUDE_EFFORT` env,
   Agent tool `model` enum, subagent-type custom-agents dir — map precedence

### Expected fix

Promote effort from **Protocol-tier** (brief string) to **Runtime-tier**
(spawn-arg or env var the harness respects, documented in
`xbreed-shared.md` §Enforcement Tiers). Agent frontmatter + team-lead brief
should WIN over settings.json for team-spawned agents.

### Acceptance gate

- Spawn `ccs-labrat-effort-test` with `model: sonnet`, `effort: medium`
- Teammate reports its own model + effort via introspection
- Reported effort MATCHES brief (medium, not harness-default high)

---

## Item 2 — xask-as-first-class-tool (Layer-1-vs-rest asymmetry)

### Observation

Current xask integration: Layer-1 gate `"Your FIRST tool call MUST be Bash:
xask --effort <tier> <model> '<q>'"`. Teammates reliably obey at boot (ritual
honored) but **never invoke xask again** for the rest of the session, even
when follow-up cross-model probes would be productive (e.g., distiller
resolving CONFLICTS, reviewer verifying executor's fix, critic reframing on
new evidence).

Observed behavior: teammates receive peer DMs, integrate them using
**in-session Claude capacity only** (cco or ccs). This is not wrong — good
rerouting — but it forfeits cross-model second-opinion depth throughout the
round.

### Hypothesis

xask is a **Bash-wrapped CLI invocation**, not a native CC tool. Teammates
see Bash as one of many tools but have no affordance/habit for re-invoking a
specific Bash subcommand mid-session. Promoting xask to a **native CC tool**
(first-class tool exposed to the harness, not MCP) would make cross-model
delegation structurally available, not just ritually-invoked.

**Implementation constraint (per user directive 2026-04-17):** MCP is NOT
the target — a proper native CC tool integration is. MCP is a different tier
(protocol server layer) and would still be one-removed from the harness's
tool-dispatch loop. Native tool = same tier as Read/Write/Grep/Bash.

### Evidence collection plan

1. Grep teammate transcripts (mailbox-r4-0417) for `xask` invocation count
   per teammate. Expect: 1 at Layer 1, 0 after (with occasional second
   invocation under specific reframe directives).
2. Measure proposal quality delta: compare proposals where teammate had
   access to xask-mid-session vs Layer-1-only.
3. Prototype: write an MCP server `mcp__xbreed_xask` exposing `xask_gemini`,
   `xask_codex` tools. Spawn a teammate with this MCP loaded. Measure
   mid-session xask invocation rate.

### Expected fix

- Implement xask as a **native CC tool** (NOT MCP — user-directive
  constraint) exposing:
  - `xask_gemini(effort, question, context?, loadout?)`
  - `xask_codex(effort, question, scope?)`
  - `xask_spark(question)` (codex-5.3-spark fire-and-forget)
- Same tier as Read/Write/Grep/Bash in the harness's tool-dispatch loop
- Add to agent templates as available tool
- Update `xbreed-shared.md` xask-gate documentation from "Bash-invoked" to
  "native CC tool, Layer-1 gate unchanged, mid-session calls encouraged"
- Preserve Layer-1 boot ritual (it works)
- Investigation needed: what's the CC tool-registration surface? Extend
  existing tool list or add a new category? Native vs Bash-wrapped
  distinction must be measurable (tool call name in transcript should be
  `xask_gemini`, not `Bash: xask --effort ...`).

### Acceptance gate

- Teammate X invokes xask >=2 times in a session: once at Layer 1, once
  mid-session during a peer-DM reframe
- Cross-model call count in a round doubles or better vs current state
- No new deadlock / authorization failure paths

### Non-goals

- Replacing the Layer-1 gate (it's the boot ritual that works)
- Adding xask to the allowed-tools list for ALL agents (still scope by
  axis_family per xbreed-shared.md)

---

## Item 3 — True-concurrent batch dispatch (tmux-pane-cap asymmetry)

### Observation (primary-source, live incident)

Mailbox-r4-0417 R2 dispatch: 6 Agent calls issued in **one message**
(batch). Expected behavior: all 6 spawn concurrently. Actual behavior: 1
succeeded, 5 failed with error `"Failed to create teammate pane: no space
for new pane"`. Required shutdown of 9 R1 teammates to free tmux panes,
then retry spawned all 6 successfully.

### Hypothesis

The harness serially allocates tmux panes for each Agent call even when the
Agent tool calls are batched in one message. When pane cap is hit mid-batch,
remaining spawns fail rather than queue-with-retry-on-free.

### Evidence collection plan

1. `tmux list-panes | wc -l` before and after an N=10 Agent batch — measure
   pane allocation serialization
2. Add timing instrumentation: log pane-alloc start/end per spawn, compute
   overlap factor (if truly parallel, overlap = N; if serial, overlap = 1)
3. Test with N=5 (under cap) vs N=15 (over cap) — measure failure mode:
   N=5 should all succeed; N=15 should gracefully queue or reject EARLY,
   not silently drop 5 spawns with an obscure tmux error
4. Read harness source (if accessible) for pane-allocation implementation

### Expected fix

- If the cap is hardware/tmux-limit-driven → add pre-spawn cap-check with
  clear error ("10 panes in use, cap 12, cannot spawn 6 more — shutdown
  idle teammates first") — Build/CI-tier enforcement
- If the cap is software-enforceable → implement proper batch-spawn with
  queue-on-full OR concurrent-pane-alloc semantics — Runtime-tier
- Document the cap explicitly in xbreed-shared.md + preserve sequential
  logic where role dependencies require it (e.g., the-planner Phase 0 must
  complete before Phase 2 fan-out)

### Acceptance gate

- N=10 Agent batch in one message: all 10 spawn concurrently (timing
  overlap ≈ N, not ≈ 1)
- N=15 batch with cap=12: first 12 spawn, remaining 3 either queue
  gracefully or reject with clear "cap hit" error BEFORE consuming the
  pane-alloc attempts
- Sequential-by-design flows (planner-first-then-specialists) still work

---

## Mission discipline (apply to all 3 items)

Per mailbox-r4 precedent:

- **Phase 0:** the-planner with wwkd Layer 0 — data walk + skeleton
- **Phase 1:** axis + teammate naming (likely: labrat-harness-introspect,
  scout-mcp-tooling, reviewer-xbreed-shared, executor-effort-fix,
  executor-mcp-xask, executor-batch-spawn-cap-check, critic-precedence-rules,
  distiller, scribe-harness-r1)
- **Phase 2:** parallel dispatch (this time *actually* parallel — Item 3 is
  a meta-concern the dispatch itself will stress-test)
- **Phase 3:** rounds + Pareto + commit per round
- **Evidence schema:** Build/CI-tier > Runtime-tier > Protocol-tier claims
  must be primary-source verified (avoid the harness-broker overclaim trap
  from ask-resilience-r3)

---

## Dispatch trigger

Fresh `/xbgst /wwkd | godspeed` after:

1. mailbox-r4-0417 R2 completes (SYNTHESIS_READY + Pareto + commit)
2. R4 TeamDelete cleanup (per `feedback_team_cleanup_on_shutdown.md`)
3. User signals ready (or auto-continues per no-hold-after-frontier policy)

---

## Provenance

- **Charter authored:** team-lead (current session, mailbox-r4-0417 R2)
- **Source:** user directive mid-R2 with `/xbgst /wwkd | godspeed`
- **Deferred:** explicit user instruction — "when we finish mailbox
  optimization, we'll look into it — and test it extensively — using the
  same protocol discipline"
- **Date:** 2026-04-17

Godspeed queued.
