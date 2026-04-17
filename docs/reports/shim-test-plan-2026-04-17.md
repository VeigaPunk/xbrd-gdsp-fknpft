# Per-teammate effort shim — test plan (new session)

Installed 2026-04-17 via /xbgst. See `shim-handoff-xbgst-r2r3-2026-04-17.md` for the design rationale.

## 0. What was installed

- `~/.bashrc` has a DEBUG trap block (bottom of file, marked `xbreed per-teammate effort shim`)
- Memory `feedback_teammate_mode_effort_caveat.md` updated — tier mandates are now OPERATIVE
- No xbreed binary changes, no Rust compilation needed

## 1. Restart required

Current Claude Code session (this one) won't pick up the trap — its outer bash was started before the `.bashrc` edit. Exit this CC session and start a fresh one:

```bash
# exit claude code (Ctrl+D or /exit)
claude
```

## 2. Sanity-check the trap is installed

In the fresh session, open a terminal pane (or the Bash tool) and run:

```bash
grep -c __xbreed_effort_trap ~/.bashrc
# expect: 2  (the function def + the trap line)
```

## 3. End-to-end: spawn a teammate and check env

Spawn any teammate via TaskCreate + Agent, or via `/xbgst` / `/xbt` / `/xb`. A quick one-off:

```
TeamCreate(team_name="shim-verify", description="verify shim")
Agent(subagent_type="labrat", team_name="shim-verify", name="ccs-labrat-verify",
      prompt="Run: env | grep -E '^CLAUDE_CODE_EFFORT_LEVEL|^TMUX_PANE' and SendMessage the output to team-lead. Then DESPAWN.")
```

Expected from the labrat:

```
CLAUDE_CODE_EFFORT_LEVEL=high
TMUX_PANE=%...
```

Because `ccs-labrat-verify` matches the `ccs-*|cdx-*|g-*` case → `high`.

Swap the name to test other branches:
- `ccs-distiller-verify` → `medium`
- `ccs-scribe-verify` → `medium`
- `ccs-simplifier-verify` → `medium`
- `cco-critic-verify` → `high`
- anything else `ccs-*` / `cdx-*` / `g-*` → `high`

## 4. Failure modes & their fingerprints

| Symptom | Cause | Fix |
|---|---|---|
| `CLAUDE_CODE_EFFORT_LEVEL` empty in labrat env | Trap didn't fire (pane bash didn't source ~/.bashrc in interactive mode) | Check `echo $BASH_SUBSHELL`; if `>0`, tmux pane is a non-interactive subshell — investigate pane creation mode |
| Var is set but to the wrong tier | Name didn't match any case branch | Add branch to the case statement in ~/.bashrc |
| All teammates get the same tier | Outer session has `CLAUDE_CODE_EFFORT_LEVEL` already exported before CC starts | Unset it in outer shell: `unset CLAUDE_CODE_EFFORT_LEVEL` (trap only exports; CC reads env precedence > settings, so an outer preset sticks session-wide) |
| Trap fires but effort doesn't feel different | Only Opus 4.7 actually honors effort tier (per `LNH()` gating) — sonnet teammates see env but may not modulate | Expected for sonnet — var is set but reasoning-budget gating is opus-specific |

## 5. Uninstall

```bash
# Remove the block from ~/.bashrc
sed -i '/# ---- xbreed per-teammate effort shim/,/# ---- end xbreed per-teammate effort shim ----/d' ~/.bashrc
```

Then restart CC. Teammates revert to outer session's effort.

## 6. What to report back

If you want to know whether it's actually making a difference in practice, compare round-times on an xbgst run:
- Before (today, pre-install): scribe took ~4 minutes to write a 26KB report at `xhigh`
- After (with shim): scribe should finish faster at `medium`
- Distiller similar

If round-times drop noticeably, shim is working. If not, either (a) sonnet doesn't gate on effort (expected), or (b) verify step 3 first.
