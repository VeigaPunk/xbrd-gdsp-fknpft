# shim-handoff xbgst — Round 2 Interim Findings
**Date:** 2026-04-17 | **Mission:** shim-handoff-0417 | **Session:** R2 interim | **Scribe:** ccs-scribe-r1
**Status: INTERIM** — single source (revenger decisive finding before full R2 shutdown). Full R2 report supersedes when SYNTHESIS_READY fires.

---

## 1. Mission context

R1 verdict: PATH-shim DOA (CC spawns via absolute versioned path, never consulting PATH). Round 2 was launched to resolve:
- FM#7: Does CC re-read `CLAUDE_CODE_EFFORT_LEVEL` per-spawn or once at session init?
- Probe A: EXECPATH interception viability
- Spawn mechanism: exactly how does CC create teammate panes?

---

## 2. Decisive R2 finding — FM#7 RESOLVED (code-level)

**Source:** cdx-revenger-teammate (final finding before shutdown)

**Code cite (CC 2.1.112 JS deobfuscated):**
```js
function xOH() {
  let H = process.env.CLAUDE_CODE_EFFORT_LEVEL;
  return H?.toLowerCase() === "unset" || H?.toLowerCase() === "auto" ? null : hc(H);
}
function LNH(H, $) {
  let q = s_(H).includes("opus-4-7") && !O$().unpinOpus47LaunchEffort,
      K = xp8(H), _ = xOH();
  if (_ === null) return q ? K : void 0;
  ...
}
```

**Reading:**
- `xOH()` reads `process.env.CLAUDE_CODE_EFFORT_LEVEL` **at call time** (not cached).
- `LNH()` (launch-effort-determination) is called **per CC process at its own startup**.
- Each teammate is its own CC process → **each teammate's own `process.env` matters independently**.
- FM#7b verdict: **POSITIVE** — CC re-reads the env var per-process-start, not once at session init.

**Implication:** If a teammate's process env contains `CLAUDE_CODE_EFFORT_LEVEL` at spawn time, CC honors it. The injection surface is valid — the question is HOW to get the right value into each teammate's process env BEFORE `LNH()` fires.

---

## 3. Spawn mechanism finalized

CC creates teammate panes via:
1. `tmux split-window` (creates pane running interactive bash — **sources `~/.bashrc`**)
2. `tmux send-keys <absolute-versioned-path> <argv> Enter` (fires CC binary after bashrc is sourced)

**Key:** `~/.bashrc` IS sourced at pane-creation, BEFORE CC's command fires. No `bash -lc` (loginrc not sourced). Interactive bash only.

**Implication:** A `~/.bashrc` hook that reads `$TMUX_PANE`, reverse-lookups the teammate name from `~/.claude/teams/<team>/config.json`, and exports `CLAUDE_CODE_EFFORT_LEVEL` will fire BEFORE CC's `xOH()` / `LNH()` call. The env var will be present in the pane's process.env when CC starts.

---

## 4. Updated path ranking

| Rank | Path | LoC | Status |
|------|------|-----|--------|
| 1 | **~/.bashrc hook + $TMUX_PANE** — reverse-lookup name from config.json, export CLAUDE_CODE_EFFORT_LEVEL | ~15 bash | NEW. Requires $TMUX_PANE present at bashrc source time (R2 probe needed to confirm). Zero binary, zero cargo change. |
| 2 | **OUTER PATH-shim sets CLAUDE_CODE_EXECPATH=self** — CC propagates to teammate spawns | ~20 lines | Still worth probing (EXECPATH visible in R1 probe env). Moot if Rank 1 passes. |
| 3 | **src/sync.rs emits `tmux setenv -t <pane> CLAUDE_CODE_EFFORT_LEVEL <v>`** — does tmux setenv propagate to send-keys'd commands? | ~5 lines | Unprobed. If pane-env propagates to send-keys'd process.env, this is cleanest of all (xbreed already knows name at sync time). |
| 4 | Full Rust shim binary (~400 LoC) | ~400 LoC | OBSOLETE given Rank 1-3. |

---

## 5. R2 probe requests (for next labrat dispatch)

### Probe 1 — bashrc $TMUX_PANE presence (mandatory before Rank 1 ships)

**Question:** Is `$TMUX_PANE` set in the pane's bash environment at `~/.bashrc` source time — BEFORE CC's `send-keys` command fires?

**Method:**
```bash
# Add to ~/.bashrc (temporarily):
echo "bashrc sourced TMUX_PANE=$TMUX_PANE at $(date)" >> /tmp/bashrc-tmuxpane-log

# Spawn one teammate. After spawn, check:
cat /tmp/bashrc-tmuxpane-log
```

**Expected:** log entry with non-empty `TMUX_PANE` value (e.g. `%267`) — confirms Rank 1 is viable.

**Gate:** if `TMUX_PANE` is populated in the log, Rank 1 is proven and bashrc hook can be written. If empty or missing, tmux split-window starts bash before pane ID is assigned — need to probe `tmux setenv` path (Rank 3) instead.

### Probe 2 — CLAUDE_CODE_EXECPATH interception (Rank 2; run concurrently)

**Question:** If outer shell sets `CLAUDE_CODE_EXECPATH=/path/to/shim`, does CC use that path when spawning teammate processes, or does it use the hardcoded versioned path regardless?

**Method:** xask codex re: CC source — does CC read `CLAUDE_CODE_EXECPATH` for teammate spawn binary resolution, or is it write-once diagnostics-only?

**Alternative:** `export CLAUDE_CODE_EXECPATH=/tmp/shim-probe.sh` (script that `echo`s and execs real path), spawn teammate, check if probe fires.

### Probe 3 — tmux setenv propagation (Rank 3; optional)

**Question:** If `tmux setenv -t <pane> CLAUDE_CODE_EFFORT_LEVEL medium` is run before `tmux send-keys` fires in that pane, does the CC process inherit the env var?

**Context:** `src/sync.rs` (xbreed's spawn path) fires before CC's send-keys. This would be the cleanest architecture (xbreed knows the teammate name, tier lookup at sync time, zero user-side ~/.bashrc modification).

---

## 6. Load-bearing unknowns (post-R2 interim state)

| Unknown | Status | Next action |
|---------|--------|-------------|
| FM#7b — CC re-reads env per-spawn? | **RESOLVED: YES** (code-level: `xOH()` reads process.env at call time; `LNH()` called per-process-start) | Archive. |
| $TMUX_PANE at bashrc source time | **OPEN** — Rank 1 viability gate | R2/R3 labrat: bashrc log probe |
| CLAUDE_CODE_EXECPATH interception | **OPEN** — Rank 2 probe | R2/R3 labrat or xask codex |
| tmux setenv propagation | **OPEN** — Rank 3 probe | Optional; most elegant if positive |
| CONFLICT C — CLAUDE_CODE_EFFORT_LEVEL non-propagation (reviewer addendum) | **PARTIALLY ADDRESSED** — FM#7b resolves the per-spawn read; CONFLICT C is about env chain from OUTER lead process. If env is set in the pane's OWN bash (via bashrc hook), chain propagation is irrelevant. | Rank 1 probe closes this implicitly. |

---

## 7. Interim verdict

FM#7 is resolved definitively: each teammate's `process.env.CLAUDE_CODE_EFFORT_LEVEL` is read independently at its own startup. The injection mechanism is valid; the open question is delivery.

Rank 1 (bashrc + $TMUX_PANE) is the cheapest path and should be probed first. ~15 lines bash, zero binary, no cargo changes, no PATH manipulation. If `$TMUX_PANE` is present at bashrc source time, this is the implementation.

R1 handoff milestones M1–M6 should be held pending Rank 1 probe result. If Rank 1 passes, the entire handoff plan is replaced by a ~15-line bash snippet + config.json reverse-lookup.

---

## 8. Out-of-scope

- Full R2 SYNTHESIS_READY synthesis (distiller not yet run; this is single-source interim)
- Rank 1 implementation (pending probe gate)
- Memory entry `feedback_teammate_mode_effort_caveat.md` update (pending Rank 1 confirmation)
- R1 handoff formal rescoping (judge owns plan revisions)

---

## 9. Links

- R1 report: `docs/reports/shim-handoff-xbgst-r1-2026-04-17.md`
- Plan/handoff: `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md`
- M0 probe: `docs/reports/teammate-name-probe-2026-04-17.md`
- Next: Full R2 report (post-SYNTHESIS_READY) supersedes this interim
