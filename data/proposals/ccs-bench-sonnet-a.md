PROPOSAL: Per-Teammate Effort Propagation via PATH-Intercepting Claude Wrapper

---

## (a) Design Sketch

**Mechanism: tmux pane-title identification + PATH-shadowing wrapper**

CC spawns each teammate as a tmux pane. When it creates the pane, the window/pane title is likely set to the teammate name (e.g., `ccs-distiller`, `cco-critic-r1`). A wrapper script placed earlier in PATH intercepts the `claude` invocation, reads the current pane title, maps it to an effort tier, and exports `CLAUDE_CODE_EFFORT_LEVEL` before exec'ing the real binary.

**New Rust additions in `src/sync.rs` (or new `src/effort.rs`):**

```rust
pub struct EffortRule {
    pub name_prefix: String,  // e.g. "cco-" or "ccs-distiller"
    pub tier: String,          // "xhigh", "high", "medium"
}

/// Writes effort-map.json + xbreed-claude-wrapper.sh to out_dir.
pub fn write_effort_wrapper(out_dir: &Path, rules: &[EffortRule]) -> Result<PathBuf> {
    // 1. Serialize rules as { prefix → tier } JSON
    let map: IndexMap<&str, &str> = rules.iter()
        .map(|r| (r.name_prefix.as_str(), r.tier.as_str()))
        .collect();
    let map_path = out_dir.join("effort-map.json");
    std::fs::write(&map_path, serde_json::to_string_pretty(&map)?)?;

    // 2. Write bash wrapper
    let wrapper = format!(r#"#!/usr/bin/env bash
EFFORT_MAP="{map}"
if [[ -f "$EFFORT_MAP" && -n "$TMUX_PANE" ]]; then
    TITLE=$(tmux display-message -p '#{{pane_title}}' 2>/dev/null || echo "")
    while IFS=$'\t' read -r pfx tier; do
        [[ "$TITLE" == "$pfx"* ]] && export CLAUDE_CODE_EFFORT_LEVEL="$tier" && break
    done < <(jq -r 'to_entries[]|[.key,.value]|@tsv' "$EFFORT_MAP" 2>/dev/null)
fi
exec "$(command -v claude.real || command -v /usr/local/bin/claude.real)" "$@"
"#, map = map_path.display());

    let wrapper_path = out_dir.join("xbreed-claude-wrapper.sh");
    std::fs::write(&wrapper_path, &wrapper)?;
    std::fs::set_permissions(&wrapper_path,
        std::os::unix::fs::PermissionsExt::from_mode(0o755))?;
    Ok(wrapper_path)
}
```

**Deploy step** (one-time, user-run):
```bash
mv $(which claude) $(dirname $(which claude))/claude.real
ln -s ~/.config/xbreed/generated/xbreed-claude-wrapper.sh ~/.local/bin/claude
```

**Default effort-rule set** (encodes current aspirational memories):
```
cco-           → xhigh
ccs-distiller  → medium
ccs-scribe     → medium
ccs-simplifier → medium
ccs-           → high   # all other sonnet teammates
g-             → high   # gemini-targeted teammates
```

Prefix matching is longest-match: `ccs-distiller` beats `ccs-` for `ccs-distiller-r1`.

---

## (b) Failure Modes

**F1 — Pane title not set to teammate name** (LINCHPIN, unverified)
- CC may not set tmux pane/window title to teammate name.
- Detection: `tmux display-message -p '#{pane_title}'` inside a teammate pane returns something other than the teammate's name.
- Rollback: wrapper falls through silently (no `CLAUDE_CODE_EFFORT_LEVEL` export) → session default applies. **No regression vs current behavior.** Labrat probe required before committing to this approach.

**F2 — Race: pane title set after claude starts**
- tmux window title may be set post-spawn, not pre-exec.
- Detection: probe shows title is empty or "bash" at wrapper invocation time.
- Mitigation: add 50ms sleep before title read (fragile). Better: use arg-parsing fallback — if CC passes `--agent-id ccs-distiller` or similar, parse args directly.

**F3 — Prefix collision**
- `ccs-` prefix matches both `ccs-distiller` (want medium) and `ccs-critic` (want high).
- Mitigation: rule table is sorted longest-prefix-first; exact name match wins.
- Detection: cargo test `test_longest_prefix_wins`.

**F4 — Stale effort-map from prior session**
- Different team config uses same generated dir.
- Mitigation: `xbreed sync` always overwrites `effort-map.json`; add mtime guard in wrapper (`[[ $(($(date +%s) - $(stat -c %Y "$EFFORT_MAP"))) -gt 3600 ]] && unset map`).

**F5 — `claude.real` path resolution fails**
- `command -v claude.real` returns empty if user skips deploy step.
- Detection: wrapper exits with non-zero; effort injection never runs.
- Mitigation: wrapper emits stderr warning if real binary not found.

---

## (c) Implementation Cost

| Item | Files | LoC delta |
|---|---|---|
| `EffortRule` struct + `write_effort_wrapper()` | `src/sync.rs` (or new `src/effort.rs`) | +75 |
| `xbreed sync --effort-rules` CLI flag | `src/cli.rs` | +20 |
| Wrapper script template | inline in Rust (no separate file) | 0 |
| Unit tests | `tests/effort_wrapper.rs` (new) | +60 |
| **Total** | 3 files | **+155 LoC** |

**New tests (3):**
1. `write_effort_wrapper_generates_executable_script` — assert wrapper is created, is chmod +x, contains map path.
2. `longest_prefix_match_wins` — `ccs-distiller-r1` matches `ccs-distiller` over `ccs-`.
3. `no_tmux_pane_no_export` — simulate absent `$TMUX_PANE`, assert no effort line in output.

**Pre-implementation labrat gate (mandatory before writing a line):**
```bash
# Inside a running teammate pane:
tmux display-message -p '#{pane_title}'
# If output = teammate name → F1 is clear, proceed.
# If output = "bash" or session name → F1 blocks, pivot to arg-parsing fallback.
```

Tier: **Runtime-tier** (bypass: skip the wrapper symlink, use raw `claude`). Ceiling: contingent on F1 pane-title hypothesis — empirical verification is the gating precondition.
