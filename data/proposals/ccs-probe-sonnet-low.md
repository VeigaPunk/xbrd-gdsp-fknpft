PROPOSAL: Per-Teammate Effort Propagation — claude-shim + effort-map

**Context:** `src/sync.rs:29` hard-forces `teammateMode: "tmux"` — every teammate is a separate CC process in its own tmux pane. CC propagates only `tools` + `model` on spawn; `effort:` frontmatter is silently ignored (R3 known-gap #3, xbreed-shared.md §Session Effort Configuration). `CLAUDE_CODE_EFFORT_LEVEL` is read at CC session init — so the only reachable per-teammate vector is injecting the env var *before* the teammate's `claude` process starts.

---

**(a) Design Sketch**

**Mechanism: effort-map file + claude shim (runtime-tier)**

**Step 0 — Map write (Rust, before TeamCreate):**
```rust
// src/effort.rs (NEW ~40 LoC)
pub enum EffortTier { Low, Medium, High, XHigh }
impl EffortTier {
    pub fn as_str(&self) -> &'static str {
        match self { Low=>"low", Medium=>"medium", High=>"high", XHigh=>"xhigh" }
    }
}
pub fn write_effort_map(team: &str, roster: &[(&str, EffortTier)]) -> Result<()> {
    let dir = Path::new("/tmp/xbreed-effort-map").join(team);
    fs::create_dir_all(&dir)?;
    for (name, tier) in roster {
        fs::write(dir.join(name), tier.as_str())?;
    }
    Ok(())
}
pub fn cleanup_effort_map(team: &str) -> Result<()> {
    let dir = Path::new("/tmp/xbreed-effort-map").join(team);
    if dir.exists() { fs::remove_dir_all(dir)?; }
    Ok(())
}
```

**Step 1 — claude shim (bash, installed at `~/.local/bin/claude`, shadows `~/.cargo/bin/claude`):**
```bash
#!/usr/bin/env bash
# scripts/claude-shim (~20 LoC)
TEAM="${XBREED_TEAM:-}"
PANE_TITLE=$(tmux display-message -p '#{pane_title}' 2>/dev/null || echo "")
MAP="/tmp/xbreed-effort-map/${TEAM}/${PANE_TITLE}"
if [[ -n "$TEAM" && -n "$PANE_TITLE" && -f "$MAP" ]]; then
    export CLAUDE_CODE_EFFORT_LEVEL=$(cat "$MAP")
    echo "[effort-shim] ${PANE_TITLE} → ${CLAUDE_CODE_EFFORT_LEVEL}" \
         >> /tmp/xbreed-effort-shim.log
fi
exec ~/.cargo/bin/claude "$@"
```

**Step 2 — Orchestrator exports XBREED_TEAM before TeamCreate:**
Parent session: `export XBREED_TEAM=<team-name>` — inherited by all tmux panes CC spawns.

**Step 3 — Cleanup on TeamDelete:**
Call `cleanup_effort_map(team)` (or add to `xbreed precheck pane-cap` sweeper).

**Key assumption:** CC sets the tmux pane title to the teammate name string when spawning in `teammateMode: "tmux"`. Needs labrat verification (`tmux list-panes -F '#{pane_title}'` from a live team session) before shipping.

---

**(b) Failure Modes**

| Failure | Detection | Rollback |
|---|---|---|
| CC does NOT set pane title to teammate name | Shim log shows empty PANE_TITLE; effort not injected | No regression — falls back to session default (xhigh). Label labrat probe as prerequisite gate |
| Shim exec-path bug breaks all CC sessions | `claude --version` fails; shim always falls through to exec | `rm ~/.local/bin/claude` — one file, instant |
| Stale map from prior team (same teammate name) | Team-namespaced path `/tmp/xbreed-effort-map/<team>/<name>` prevents cross-team collision if team names are unique (UUID-suffixed in xbreed convention) | `cleanup_effort_map` on TeamDelete |
| `CLAUDE_CODE_EFFORT_LEVEL` not honored at init (R3 Gap 3 unresolved) | R3 follow-up probe: spawn teammate, check `printenv | grep CLAUDE_CODE_EFFORT_LEVEL` | If unverified, mechanism is a no-op — no regression, same current state |
| Race: CC spawns teammate before map write completes | Shim log shows MAP miss; effort unset | Make `write_effort_map` a hard pre-condition before any TeamCreate call; enforce in orchestrator brief |

**Ceiling:** Runtime-tier only. Bypassed by: removing shim, skipping XBREED_TEAM export, or calling `~/.cargo/bin/claude` directly. Cannot reach build/CI-tier without CC exposing teammate-spawn args to hooks (not documented in CC 2.1.112).

---

**(c) Implementation Cost**

| File | Status | ΔLoC |
|---|---|---|
| `src/effort.rs` | NEW | ~40 |
| `src/lib.rs` | MOD (pub mod effort) | ~3 |
| `scripts/claude-shim` | NEW | ~20 |
| `tests/effort_map.rs` | NEW (map write/read/cleanup unit tests) | ~30 |

**Total:** ~93 LoC, 4 files (2 new Rust, 1 new bash, 1 new test).

**Installation step (protocol-tier):** `cp scripts/claude-shim ~/.local/bin/claude && chmod +x ~/.local/bin/claude` — documented in README or `xbreed sync` output.

**Prerequisite labrat probe (should gate shipment):** Spawn one teammate, read `tmux list-panes -F '#{pane_title}'` from parent, confirm CC sets pane_title = teammate name. If that probe fails, the entire mechanism is blocked and the design falls back to a "separate master session per effort tier" approach (architecturally heavier, requires cross-session DM infrastructure).

**Naming prefix derivation (optional enhancement, zero extra LoC):** If the labrat probe confirms pane title = teammate name, the shim can derive effort purely from naming prefix (`cco-` → xhigh, `ccs-` → medium) without the effort-map file, eliminating the map write/cleanup entirely. Tradeoff: less flexible (locks effort to naming prefix convention), but simpler and no pre-TeamCreate coordination needed.