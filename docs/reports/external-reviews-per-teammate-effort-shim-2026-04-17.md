# External reviews — per-teammate effort shim design

Paired dispatch to non-Claude models for adversarial design review of
`docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md`.

- **the-musketeer** → Grok (xAI SuperGrok) — **returned**
- **the-puppeteer** → GPT-5.4-Pro (ChatGPT Pro) — **blocked** (Chrome remote-debugging port not open; user dispatched manually on chatgpt.com)

Both asked the same 3 questions: approach critique, test-strategy critique, underweighted risk.

---

## Grok response (verbatim)

### 1. Approach critique — PATH-shim is right, but LD_PRELOAD is the trap

PATH-shim is the pragmatic choice. LD_PRELOAD looks cleaner until CC merges a change that hardcodes `/usr/bin/claude` or uses explicit `execve(path, ...)` with an absolute path — suddenly the preload silently stops working. Settings.json per-pane is a non-starter (CC doesn't expose pane-aware settings), tmux hooks lag by design (hook fires *after* the process), and filesystem overlays (unionfs/overlay2) are overkill.

**The real risk:** the cascade falls through silently. If env var isn't set, pane title doesn't match regex, and argv doesn't contain the name, the shim exec's real `claude` with no effort override. That's fine — but you need to instrument *why* the fallthrough happened. Log to stderr which cascade tiers were tried and why each failed.

**Mandatory:** add a `--debug-cascade` flag that prints the resolution chain to a fixed logfile (e.g., `~/.xbreed_shim.log`).

### 2. Test critique — printenv is insufficient; you need *behavior* divergence

Printing the env var confirms it's SET, not that it's HONORED. CC might read it once at outer session init, cache it, and ignore mutations. The M2 test is a false negative — you'll ship thinking it works.

**Real test:** spawn two identical tasks with *different* effort levels, measure `claude --help` output latency or time-to-first-token in a synthetic query (e.g., solve a 30-token Fibonacci). Model selection WILL show measurable latency deltas. Capture `time { claude ... }` and assert `CLAUDE_CODE_EFFORT_LEVEL=max` is visibly slower than `=min`. That's observable, reproducible, and will catch "env set but ignored" instantly.

### 3. Underweighted risk — CC upgrade breaks env-var reading cadence

The biggest assumption: "env var is read fresh per teammate spawn." CC might cache `CLAUDE_CODE_EFFORT_LEVEL` at the outer session init and never re-read it. If a future CC version freezes effort at the parent session level (for performance or architecture), the shim becomes a cargo-cult ritual — the env is set and ignored, and you don't know why until a user complains that all teammates converge on the same tier.

**Blocker:** before shipping, confirm with CC's source (or run this test) — spawn two teammates sequentially in the same session with *different* env values, verify they pick up different efforts. If they don't, the shim is DOA.

**Secondary risk: tmux pane title race.** If the pane title gets set *after* the `claude` command is already running, the read misses. Cascading to argv is safer, but argv scanning is brittle — if the agent name appears in a config file path or prompt text, you'll collide.

**Guard:** check that the candidate name is a known agent (cross-reference against the frontmatter manifest). Reject matches that don't exist.

### Ship-blocking checklist (Grok's summary)

- Confirm CC reads env var *per-spawn*, not once at session init
- Behavior divergence test (latency, token speed, or measurable output) — not just "env is set"
- Tmux pane title fallback *only* if argv scan fails, not as primary tier
- Manifest validation (reject names not in agent templates)

*Recursion guard and graceful fallthrough are solid. The three above will save you from a silent failure that ships.*

---

## ChatGPT / GPT-5.4-Pro prompt (user dispatched manually)

Puppeteer's dispatcher reported Chrome remote-debugging port 9222 not open. Same 3 questions pasted into chatgpt.com for GPT-5.4-Pro with extended thinking. Response to be captured when the user reads it and either pastes back here or files it at `docs/reports/external-reviews-per-teammate-effort-shim-2026-04-17-gpt.md`.

Prompt sent (verbatim core):

> Context: per-teammate effort propagation shim for Claude Code via PATH-shadowed `claude` binary. 3-tier name cascade (env → tmux pane title → argv), manifest from agent frontmatter, recursion guard, opt-in install.
>
> M0: probe if CC exposes teammate name anywhere.
> M1: skeleton shim (log only).
> M2: overfit one teammate, `printenv` check.
> M3-M6: manifest, CLI, tests, ship.
>
> Two load-bearing unknowns: (a) name exposure, (b) env read per-spawn vs once-at-session-init.
>
> Adversarial review questions:
> 1. Is PATH-shim the right primitive vs LD_PRELOAD / settings.json per-pane / tmux hook?
> 2. Is M2's `printenv` check sufficient, or do we need behavioral divergence?
> 3. What failure mode should block ship that the 8-mode table doesn't cover?

---

## Actions to fold into the handoff

Before starting M0 next session, update `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md` with Grok's 4 ship-blockers:

1. **M2 gate upgrade:** replace "printenv shows env var" with "behavior divergence test" — measure tok/s or wall-time deltas across 2+ sequential teammates at different tiers in the SAME session. The ask-the-teammate-to-solve-Fibonacci pattern is cheap and has observable latency signature.

2. **`--debug-cascade` flag + `~/.xbreed/shim.log`** as core shim output from M1, not M4 add-on. Every resolution failure logs WHICH tier was tried and WHY it missed.

3. **Argv-first, pane-title-second** swap the cascade ordering. Argv is structured + unambiguous when present; pane-title has the race-window risk. Current order (env → pane-title → argv) deprioritizes the most reliable path.

4. **Manifest membership guard:** resolution returns `None` for any name not in the frontmatter-derived manifest. Prevents argv-scan collision on prompt-embedded agent names (e.g. "read the file about `ccs-distiller`").

Additional per-GPT response when returned: append section once the user captures it.

---

## Takeaway

Grok validated the approach but made M2 materially more rigorous (behavior-divergence, not env-set) and flagged two real risks (cascade-order + argv collision). Pending GPT-5.4-Pro review for second independent signal.
