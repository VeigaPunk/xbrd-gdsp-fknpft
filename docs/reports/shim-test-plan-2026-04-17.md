# Per-teammate effort shim — test plan (new session)

Installed 2026-04-17 via /xbgst.

## 0. What was installed

- `~/.bashrc` has a DEBUG trap block (bottom of file, marked `xbreed per-teammate effort shim`)
- No xbreed binary changes, no Rust compilation needed

## 1. Restart required

Current Claude Code session (this one) won't pick up the trap — its outer bash was started before the `.bashrc` edit. Exit this CC session and start a fresh one:

```bash
claude
```

## 2. Sanity-check the trap is installed

In the fresh session, open a terminal pane and run:

```bash
grep -c __xbreed_effort_trap ~/.bashrc
# expect: 2  (the function def + the trap line)
```

## Milestone 2 — overfit gate: `ccs-labrat-verify` (one name, bit-for-bit)

**Purpose:** prove the trap fires in CC's bash-tool subshell (not a preexisting outer shell) and the exported var propagates to the spawned teammate process, before generalising to other name classes.

### M2-0. Pre-check — confirm trap is live in THIS shell

Run this **directly in the tmux pane** (not via CC's Bash tool — child processes do not inherit traps unless `set -T` is active, so `trap -p` in a subshell shows empty even if the parent trap is live):

```bash
trap -p DEBUG
# must print: trap -- '__xbreed_effort_trap' DEBUG
# If empty: this pane never sourced ~/.bashrc.
#   Fix (preferred): reopen pane with `bash --login`, then relaunch CC.
#   Fix (last resort): export BASH_ENV=~/.bashrc — WARNING: BASH_ENV fires on ALL
#     `bash -c` calls system-wide; mitigate by ensuring ~/.bashrc has `[[ $- == *i* ]]`
#     early-bail (NOT $TMUX_PANE — that var is inherited by non-interactive children).
#     Unset BASH_ENV after this session.
# Trap must be live in the shell that executes the Agent spawn command;
# $BASH_COMMAND will contain the full CC binary path + --agent-name <name> + --team-name <team>;
# the trap glob (*--agent-name*) is path-agnostic by design.
# CLAUDE_CODE_EFFORT_LEVEL is set there and inherited by the CC child.
```

### M2-1. Failing baseline — wrong name class → `UNSET`

The trap has no `*)` default branch — an unrecognised name leaves `CLAUDE_CODE_EFFORT_LEVEL` at whatever it was before. Run this in the pane first to enforce test-order independence:

```bash
unset CLAUDE_CODE_EFFORT_LEVEL   # run directly in tmux pane before spawning
```

Then spawn with a name matching no case branch:

```
TeamCreate(team_name="shim-fail", description="fail baseline")
Agent(subagent_type="labrat", team_name="shim-fail", name="foo-bar-verify",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")
```

Expected from labrat: exactly `UNSET`.

### M2-2. Passing case — `ccs-labrat-verify` → `high` (bit-for-bit)

Again unset first to prove the trap set the value — not prior state:

```bash
unset CLAUDE_CODE_EFFORT_LEVEL   # run directly in tmux pane before spawning
```

```
TeamCreate(team_name="shim-verify", description="verify shim")
Agent(subagent_type="labrat", team_name="shim-verify", name="ccs-labrat-verify",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")
```

Expected from labrat: exactly `high`.  
Gate passes **iff**: M2-1 returned `UNSET` AND M2-2 returns `high`. Without the failing baseline, a stale outer export silently fakes a pass.

### M2-3. Arm-ordering guard — `ccs-distiller-verify` → `medium` (S3 mutation kill; SYN-01: exact-value)

Bash `case` is first-match-wins. The medium-specific arm (`ccs-distiller-*|ccs-scribe-*|ccs-simplifier-*`) must precede the general `ccs-*` high arm — a reorder silently promotes all medium teammates to high. M2-2 does not catch this. M2-3 does. SYN-01 residual: assert EXACTLY `medium`, not merely non-empty (prevents a stale `high` from a prior spawn silently satisfying a non-empty check).

```bash
unset CLAUDE_CODE_EFFORT_LEVEL   # run directly in tmux pane before spawning
```

```
TeamCreate(team_name="shim-verify-med", description="verify medium tier")
Agent(subagent_type="labrat", team_name="shim-verify-med", name="ccs-distiller-verify",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")
```

Expected from labrat: exactly `medium`.
Full gate passes **iff**: M2-1=`UNSET` AND M2-2=`high` AND M2-3=`medium`.

Name → tier mapping exercised: `ccs-distiller-verify` hits `ccs-distiller-*` medium arm before the general `ccs-*` high arm. Current install order is correct (medium-specific before general high); this gate detects future arm-reorder regressions.

### M2-4. Bracket-class probe — `g_scout_verify` (underscore) / `CCS-labrat-upper` (uppercase) → `high` (SYN-02)

Tests whether agent names with underscores or mixed-case bypass the inner `case` patterns. Current regex `[a-zA-Z0-9_-]+` captures the name, but inner arms use literal prefixes (`g-*`, `ccs-*`) which are case-sensitive and hyphen-prefixed. An underscore (`g_scout_verify`) or uppercase (`CCS-labrat-upper`) will miss all inner arms, leaving `CLAUDE_CODE_EFFORT_LEVEL` at stale value.

```bash
unset CLAUDE_CODE_EFFORT_LEVEL
```

```
TeamCreate(team_name="shim-syn02", description="bracket-class probe")
Agent(subagent_type="labrat", team_name="shim-syn02", name="g_scout_verify",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")
```

Expected (desired): `high` (because `g_*` variants should logically get high).  
Current implementation: inner `case` arm `g-*` does NOT match `g_scout_verify` (underscore vs hyphen) → `UNSET` (inner fallthrough; outer `*)` arm cleans on next cmd). Gate reveals SYN-02 gap. Fix: add `g_*|g-*` arm to inner case, or extend `ccs-*|cdx-*|g-*` to `ccs-*|cdx-*|g[-_]*`.

### M2-5. Contamination gate — `ccs-scribe-contam` → `medium`, then `noncase-contam` (NO pre-unset) → `UNSET` (SYN-09/SYN-11; requires Task A patch)

Proves the outer `*)` unset arm (FM-K fix) clears stale tier between spawns. Requires the shim patch from Task A to pass.

```
# Step 1 — spawn medium-tier agent; trap sets medium + XBREED_EFFORT_SHIM_HIT
TeamCreate(team_name="shim-contam", description="contamination gate")
Agent(subagent_type="labrat", team_name="shim-contam", name="ccs-scribe-contam",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")

# Step 2 — WITHOUT running unset manually; the *)  arm fires on the Agent() call itself
Agent(subagent_type="labrat", team_name="shim-contam", name="noncase-contam",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage the literal output to team-lead. Then despawn.")
```

Expected from ccs-scribe-contam: `medium`.  
Expected from noncase-contam: `UNSET` (outer `*)` arm fires on the second Agent() invocation, clearing `medium` before noncase-contam spawns; inner case also misses `noncase-contam` so no re-set).  
WITHOUT Task A patch: noncase-contam would return `medium` (stale contamination).

### M2-6. Reverse-branch probe — `--team-name` before `--agent-name` → `high`

Verifies the alternate-order outer pattern (`*--team-name\ *--agent-name\ *`) fires correctly.

```bash
unset CLAUDE_CODE_EFFORT_LEVEL
```

In a pane with the DEBUG trap live, run:

```bash
true --team-name myteam --agent-name ccs-labrat-rev
# Capture was set by trap; echo fires *)  arm and cleans it — use witness instead:
echo "witness: ${XBREED_EFFORT_SHIM_HIT}"
# Expected: ccs-labrat-rev:high:<BASHPID>
```

Or via spawn:

```
Agent(subagent_type="labrat", team_name="shim-m26", name="ccs-labrat-rev",
      prompt="Run: printf '%s' \"${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}\" and SendMessage to team-lead. Despawn.")
```

Expected: `high` (the launch command contains both tokens regardless of order; inner arm `ccs-*` matches).

## 3. End-to-end: spawn a teammate and check env (generalised, post-M2)

After Milestone 2 passes, validate remaining name classes via a quick one-off:

```
TeamCreate(team_name="shim-verify", description="verify shim")
Agent(subagent_type="labrat", team_name="shim-verify", name="ccs-labrat-verify",
      prompt="Run: env | grep -E '^CLAUDE_CODE_EFFORT_LEVEL|^TMUX_PANE|^XBREED_EFFORT_SHIM_HIT' and SendMessage the output to team-lead. Then DESPAWN.")
```

Expected from the labrat:

```
CLAUDE_CODE_EFFORT_LEVEL=high
TMUX_PANE=%...
XBREED_EFFORT_SHIM_HIT=ccs-labrat-verify:high:<a-pid>
```

Because `ccs-labrat-verify` matches the `ccs-*|cdx-*|g-*` case → `high`. `XBREED_EFFORT_SHIM_HIT` encodes `<agent-name>:<tier>:<BASHPID>` at trap-fire time — if the effort var was already unset by a subsequent command before the spawn, the witness confirms the trap DID fire correctly (SYN-03 discrimination gate).

Swap the name to test other branches:
- `ccs-distiller-verify` → `medium`
- `ccs-scribe-verify` → `medium`
- `ccs-simplifier-verify` → `medium`
- `cco-critic-verify` → `high`
- anything else `ccs-*` / `cdx-*` / `g-*` → `high`

## 4. Failure modes & their fingerprints

| Symptom | Cause | Fix |
|---|---|---|
| `CLAUDE_CODE_EFFORT_LEVEL` empty in labrat env | Trap didn't fire — pane bash is non-interactive and never sourced ~/.bashrc | Run `trap -p DEBUG` **in the pane directly** (not via CC Bash tool — subshells don't inherit traps); fix: `export BASH_ENV=~/.bashrc` or reopen pane with `bash --login` |
| Var is set but to the wrong tier | Name didn't match any case branch | Add branch to the case statement in ~/.bashrc |
| All teammates get the same tier | Outer session has `CLAUDE_CODE_EFFORT_LEVEL` already exported before CC starts | Unset it in outer shell: `unset CLAUDE_CODE_EFFORT_LEVEL` (trap only exports; CC reads env precedence > settings, so an outer preset sticks session-wide) |
| Trap fires but effort doesn't feel different | Only Opus 4.7 actually honors effort tier (per `LNH()` gating) — sonnet teammates see env but may not modulate | Expected for sonnet — var is set but reasoning-budget gating is opus-specific |
| Trap fires on unrelated commands (e.g. `echo "--agent-name foo"`) | FM-H: trap glob `*--agent-name*--team-name*` has no binary anchor — matches any command containing those tokens | Add CC binary prefix anchor: `*claude*--agent-name*--team-name*` (non-issue in practice if only CC spawns teammates, but fragile) |
| After adding starship/powerline/bash-preexec, trap stops firing | FM-J: prompt frameworks that call `trap ... DEBUG` internally clobber the xbreed trap silently | Keep shim block at the very END of ~/.bashrc (current install is last — safe); if frameworks are added after, move shim block below them |
| Bare `claude` (no --agent-name) inherits stale tier from prior named spawn | **FM-K (confirmed bug):** trap has no `*)` unset fallthrough — outer `case "$BASH_COMMAND"` only acts on name-matching commands; all others leave var exported in pane | Add `*) unset CLAUDE_CODE_EFFORT_LEVEL ;;` to outer case in `__xbreed_effort_trap`. Timing is safe: DEBUG fires before exec, so unset runs in parent before fork; CC child inherits nothing (correct). M2 gate's explicit `unset` pre-step works around this for testing. |

## 5. Uninstall

```bash
# Remove the block from ~/.bashrc
sed -i '/# ---- xbreed per-teammate effort shim/,/# ---- end xbreed per-teammate effort shim ----/d' ~/.bashrc
```

Then restart CC. Teammates revert to outer session's effort.

## 6. What to report back

If round-times drop noticeably, shim is working. If not, either (a) sonnet doesn't gate on effort (expected), or (b) verify step 3 first.
