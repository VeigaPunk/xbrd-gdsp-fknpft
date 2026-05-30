# Important Note on Concurrency

Captured 2026-05-30. Diagnostic exchange about xbrd dispatch concurrency dropping from 12 → 4 without local changes. Saved verbatim.

---

## User input (verbatim)

> oh yeah, another thing, the xbrd has only a 12 max agent limit but suddenly its defaulting to 4 concurrent dispatches. this is not intended and was never coded like this; can you take a look into it? on my end im sure its not coded like this; do a quick audit please, i dont think ive changed anything on the repo for weeks, I think its a native setting that was changed with the past CC cli changes or smth | godspeed

---

## Audit output (verbatim)

Audit done. Here's what's actually going on:

**Origin of the change**
- CC `claude.exe` binary date is **May 30 2026 (today)** — version 2.1.158. CC was just rebuilt today as a native Bun ELF binary on your machine. That's the moment the behavior changed.
- xbreed repo itself is unchanged. Git log confirms no orchestration code edits in weeks. The cap isn't from your side.

**What's NOT the cause** (ruled out)
- `~/.claude/settings.json` is clean — no concurrency override knob
- `~/.claude/settings.local.json` empty
- xbreed's pane-preflight (`src/precheck.rs`) at your tmux geometry (46 rows, 0-1 panes in use) computes `practical_cap = 46 − (1 + 12 − 1) = 34` > MIN_ROWS=8 → would happily allow 12. Not the bottleneck.
- Your CPU count is 12. The Workflow tool's documented cap formula is `min(16, max(2, cores−2)) = 10`. So even if that formula were leaking onto Agent(), it'd be 10, not 4.

**What I found in the binary**
- The CC bundle contains a template `${K} agents maximum, but you should try to use the minimum number of agents necessary (usually just 1)` — that's a prompt-side nudge baked into multiple agent personas (Plan, general-purpose). When K resolves to a small number (e.g. 4), the orchestrator-side model reads it as "I should dispatch only ~4 agents per fan-out."
- Workflow tool has its own throw-on-exceed cap via `tengu_workflow_agent_cap_exceeded` telemetry event. xbgst doesn't use Workflow — it uses Agent() — but a similar pattern may have shipped for Agent() in 2.1.158.
- Stripped binary, so I couldn't surface the exact numeric K value for the Agent fan-out path.

**Most likely cause** (in confidence order)
1. **CC 2.1.158 reduced the default `${K}` cap** that gets injected into subagent/orchestrator system prompts — model side now nudges toward ~4 per fan-out. Even though your xbgst persona says "<=12", the CC harness's underlying agent-system-prompt cap overrides because it's at a higher trust layer than your persona instruction.
2. **CC 2.1.158 added an Agent()-tool concurrency hard cap** (separate from Workflow's). Could be hardcoded 4, could be `min(N, cores/3)`.
3. Bun-rewrite changed parallel-tool-call dispatch semantics under the hood (less likely — would have broken more than concurrency).

**How to confirm and what to do**
- Check release notes: `gh release view --repo anthropics/claude-code v2.1.158` or browse https://github.com/anthropics/claude-code/releases
- File an issue at https://github.com/anthropics/claude-code/issues if 2.1.158 introduced a silent agent-concurrency reduction
- Quick test: in the next xbgst run, watch whether the judge's *intent* (the message it emits before tool calls) says 12 but the harness only fires 4 — that'd confirm it's a harness queue cap. If the judge itself only emits 4 Agent() calls, that's the prompt-side `${K}` cap nudging behavior.
- Honest gap: I couldn't extract the exact K value from the stripped binary. The diagnosis points at CC 2.1.158 as the change source with high confidence; the precise mechanism (prompt nudge vs hard cap) needs the release notes or a repro test to pin down.

Want me to actually fetch the 2.1.158 release notes to nail the mechanism?
