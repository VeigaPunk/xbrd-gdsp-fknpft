PROPOSAL: Per-teammate effort propagation via PATH-shim that sets `CLAUDE_CODE_EFFORT_LEVEL` per pane before `exec`ing real `claude`. Circumvents the CC-internal teammate-mode spawn path (which drops frontmatter `effort:`) by intercepting at the only user-space surface we own: the `claude` process entry.

═══ (a) DESIGN SKETCH ═══

**Mechanism.** Teammates are spawned as `claude` processes inside tmux panes. CC names each pane after the teammate (`tmux display-message -p "#{pane_title}"` → e.g. `cco-critic-foo`). Env inherits parent→pane (R3 proved `/proc/$PPID/environ` shows `CLAUDECODE=1` propagating). So: a shim binary on PATH ahead of the real `claude` can read the pane title, look up the effort tier, set `CLAUDE_CODE_EFFORT_LEVEL`, and `exec` the real binary.

**New files:**

`config/effort-map.yaml` — longest-prefix match, task-aware:
```yaml
defaults: { tier: high }
prefixes:
  "cco-the-judge":        { tier: xhigh }
  "cco-critic-":          { tier: xhigh }
  "cco-":                 { tier: high }
  "ccs-distiller":        { tier: medium }
  "ccs-simplifier":       { tier: medium }
  "ccs-scribe":           { tier: medium }
  "ccs-":                 { tier: high }
  "cdx-":                 { tier: high }
  "g-":                   { tier: high }
```

`src/effort_shim.rs` — core logic (~80 LoC):
```rust
pub fn resolve_tier(name: &str, map: &EffortMap) -> &str {
    map.prefixes.iter()
        .filter(|(k,_)| name.starts_with(k.as_str()))
        .max_by_key(|(k,_)| k.len())
        .map(|(_,v)| v.tier.as_str())
        .unwrap_or(&map.defaults.tier)
}

pub fn shim_main(argv: Vec<String>) -> ! {
    if env::var("XBREED_SHIM_RAN").is_ok() {
        exec_real_claude(argv);  // loop guard
    }
    env::set_var("XBREED_SHIM_RAN", "1");
    if let Some(name) = detect_teammate_name() {
        let map = load_map().unwrap_or_default();
        let tier = resolve_tier(&name, &map);
        if env::var("CLAUDE_CODE_EFFORT_LEVEL").is_err() {
            env::set_var("CLAUDE_CODE_EFFORT_LEVEL", tier);
        }
        log_resolution(&name, tier);
    }
    exec_real_claude(argv);
}

fn detect_teammate_name() -> Option<String> {
    if env::var("TMUX").is_err() { return None; }
    let out = Command::new("tmux").args(["display-message","-p","#{pane_title}"]).output().ok()?;
    let title = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (title.starts_with("cco-")||title.starts_with("ccs-")||title.starts_with("cdx-")||title.starts_with("g-")).then_some(title)
}
```

`src/bin/xbreed-claude.rs` — `fn main() { xbreed::effort_shim::shim_main(std::env::args().collect()) }`.

**Install** (idempotent): `scripts/install-effort-shim.sh` symlinks `~/.local/bin/claude → xbreed-claude`, resolves real claude via `command -v claude` minus the shim dir, caches path in `~/.xbreed/real-claude-path`. User PATH already has `~/.local/bin` before `~/.cargo/bin` (per `feedback_recompile_on_change.md`), so shim wins.

**sync.rs change:** none — settings.json `env:` stays shared; per-teammate override happens at shim layer, not CC config.

═══ (b) FAILURE MODES ═══

1. **Pane title != teammate name.** If CC changes naming convention or agent spawns under a generic name, all teammates fall to `defaults.tier`. *Detect:* `~/.xbreed/log/shim.log` records `name → tier` per invocation; grep for unexpected `UNKNOWN → high` entries. *Rollback:* `rm ~/.local/bin/claude`.

2. **Recursive fork.** Shim `exec`s itself if real-path resolution wrong. *Detect:* `XBREED_SHIM_RAN=1` guard passes through on second entry; integration test asserts `pstree` depth ≤2. *Rollback:* symlink removal.

3. **User exports `CLAUDE_CODE_EFFORT_LEVEL` in outer shell.** Shim respects pre-set value (code above: only sets if unset), so session-wide override still works. Trade-off: user-set value overrides per-teammate map — documented, not a bug.

4. **Non-tmux teammate path (future iTerm2 native).** `detect_teammate_name()` returns None, shim no-ops. Graceful fall-through.

5. **Config file missing/malformed.** `load_map().unwrap_or_default()` → everyone gets `defaults.tier=high`. Matches current observed behavior — no regression.

6. **Empirical not-yet-confirmed** (R3 gap #3). If CC reads `effortLevel` from `settings.json` *before* reading env (contra docs), env override is a no-op and this whole mechanism fails silently. Mitigation: shim also writes `~/.xbreed/runtime-settings/<pane_title>.json` with `{"effortLevel":"<tier>"}` and exports `CLAUDE_CONFIG_DIR=~/.xbreed/runtime-settings/<pane_title>` — gives per-pane settings dir as backup. Adds ~15 LoC.

═══ (c) IMPLEMENTATION COST ═══

- **New:** `src/effort_shim.rs` (~80 LoC), `src/bin/xbreed-claude.rs` (~10 LoC), `config/effort-map.yaml` (~25 LoC), `scripts/install-effort-shim.sh` (~30 LoC), `tests/effort_shim_test.rs` (~120 LoC).
- **Edit:** `src/lib.rs` (+1 `pub mod`), `Cargo.toml` (+4 LoC `[[bin]]` target), `commands/references/xbreed-shared.md` §Session Effort Configuration (+20 LoC — add "per-teammate reachable via shim" paragraph), `feedback_teammate_mode_effort_caveat.md` (mark partially mitigated).
- **Net LoC delta:** ~270 additions, ~30 edits.
- **New tests (≥6):** prefix longest-match correctness, defaults fallback, loop guard, argv pass-through, malformed-yaml graceful, non-tmux no-op, user-preset respected.
- **Verify loop:** `cargo clippy && cargo test && cargo fmt --check` clean; manual smoke = `echo $CLAUDE_CODE_EFFORT_LEVEL` inside a spawned teammate pane.

Ceiling-honest framing: this is **Build/CI + Runtime-tier** (shim is a real binary with tests; bypass surface = un-symlink or unset PATH). Still not Anthropic-side enforcement.