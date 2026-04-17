PROPOSAL: Per-Teammate Effort Propagation via `~/.local/bin/claude` Wrapper

---

## (a) Design Sketch

**Core mechanism:** A `claude` wrapper at `~/.local/bin/claude` intercepts every teammate pane spawn, reads the teammate name from argv, maps name-prefix → effort tier, sets `CLAUDE_CODE_EFFORT_LEVEL` before exec-ing the real binary.

```bash
#!/bin/bash
# ~/.local/bin/claude — xbreed per-teammate effort injector
# Written/updated by `xbreed sync` before TeamCreate; reads effort-map file.

REAL_CLAUDE="${XBREED_REAL_CLAUDE:-$HOME/.cargo/bin/claude}"
EFFORT_MAP="${XBREED_EFFORT_MAP:-/tmp/xbreed-effort-map.json}"

# --- Parse --teammate-id from CC argv (CC tmux-mode passes this flag) ---
TEAMMATE_ID=""
NEXT=0
for arg in "$@"; do
  [[ $NEXT -eq 1 ]] && TEAMMATE_ID="$arg" && NEXT=0 && continue
  [[ "$arg" == "--teammate-id" ]] && NEXT=1
done

# --- Effort lookup (file-map wins; prefix fallback if map absent) ---
if [[ -n "$TEAMMATE_ID" && -z "$CLAUDE_CODE_EFFORT_LEVEL" ]]; then
  if [[ -f "$EFFORT_MAP" ]]; then
    EFFORT=$(jq -r --arg k "$TEAMMATE_ID" '.[$k] // empty' "$EFFORT_MAP" 2>/dev/null)
  fi
  # Prefix fallback if map miss
  if [[ -z "$EFFORT" ]]; then
    case "$TEAMMATE_ID" in
      cco-*the-judge*|cco-*judge*) EFFORT=xhigh ;;
      cco-*)                        EFFORT=high ;;
      ccs-*distiller*|ccs-*scribe*|ccs-*simplifier*) EFFORT=medium ;;
      # ccs-* general: leave unset → inherits session default
    esac
  fi
  [[ -n "$EFFORT" ]] && export CLAUDE_CODE_EFFORT_LEVEL="$EFFORT"
fi

exec "$REAL_CLAUDE" "$@"
```

**xbreed integration — two touch points:**

1. **`xbreed sync`** generates the wrapper script at `~/.local/bin/claude` and sets `XBREED_REAL_CLAUDE` in `claude-settings.json` env block (pointing to `~/.cargo/bin/claude`). One-time setup.

2. **`xbreed team create`** (or judge pre-dispatch) accepts a `--effort-map <json-file>` flag and writes `/tmp/xbreed-effort-map.json` keyed on teammate name → effort string:
   ```json
   {
     "cco-critic-r1": "high",
     "ccs-distiller": "medium",
     "cco-the-judge": "xhigh"
   }
   ```
   The prefix fallback in the wrapper handles cases where the map is stale or incomplete.

**Rust sketch (`src/effort.rs`, new ~80 LoC):**
```rust
pub fn write_effort_map(teammates: &[(&str, &str)], out: &Path) -> Result<()> {
    let map: serde_json::Map<_,_> = teammates.iter()
        .map(|(name, effort)| (name.to_string(), json!(effort)))
        .collect();
    std::fs::write(out, serde_json::to_string_pretty(&map)?)?;
    Ok(())
}

pub fn derive_effort_from_name(name: &str) -> Option<&'static str> {
    if name.contains("the-judge") { return Some("xhigh"); }
    match name.split('-').next()? {
        "cco" => Some("high"),
        "ccs" if name.contains("distiller") 
              || name.contains("scribe") 
              || name.contains("simplifier") => Some("medium"),
        _ => None,
    }
}
```

---

## (b) Failure Modes

| Failure | Detection | Rollback |
|---|---|---|
| **CC does not pass `--teammate-id` in tmux-spawn argv** (empirical unknown — R3 Gap 3 analogous) | Wrapper fires but `TEAMMATE_ID` stays empty; `CLAUDE_CODE_EFFORT_LEVEL` unset → session default. Silent regression to current state, no crash. | None needed; no worse than today. Probe: `ps aux | grep claude` inside a pane during spawn. |
| **`~/.local/bin/claude` shadows wrong binary** | `which claude` in pane should return wrapper path; `$XBREED_REAL_CLAUDE` missing → exec falls to `$HOME/.cargo/bin/claude` hardcoded fallback. `set -e` + `exec` means silent failure = exec error logged to pane. | `rm ~/.local/bin/claude` restores prior behavior. |
| **Stale effort map from prior session** | Map at `/tmp/xbreed-effort-map.json` has old teammate names; lookups miss → prefix fallback activates. Effort may be wrong but not absent. | `xbreed sync --reset-effort-map` deletes `/tmp/xbreed-effort-map.json`. Prefix fallback then governs. |
| **`jq` absent on PATH** | Wrapper silently skips map lookup (`2>/dev/null`); prefix fallback covers standard roles. | Embed a minimal JSON key-lookup in pure bash as fallback (`python3 -c "import json,sys; ..."`) or bundle a minimal lookup in Rust-generated wrapper. |
| **Effort var set by parent shell** | `[[ -z "$CLAUDE_CODE_EFFORT_LEVEL" ]]` guard prevents overwrite — parent intent preserved. | Correct by design. |

---

## (c) Implementation Cost

**Files touched:**
- `src/effort.rs` — new (~80 LoC): `write_effort_map`, `derive_effort_from_name`
- `src/sync.rs` — extend `materialize_claude_settings` to add `XBREED_REAL_CLAUDE` to env block (~10 LoC)
- `src/cli.rs` — add `--effort-map` option to `team create` subcommand (~15 LoC)
- `src/lib.rs` — pub re-export `effort` module (~3 LoC)
- `scripts/xbreed-sync-wrapper` — bash template rendered by `xbreed sync` to `~/.local/bin/claude` (~25 lines)
- `tests/effort.rs` — new: `derive_effort_from_name` unit tests, `write_effort_map` round-trip (~60 LoC)

**LoC delta:** ~170 new, ~25 modified. All within Rust + bash, no new deps.

**New tests (6):**
1. `derive_effort_xhigh_judge` — `cco-the-judge` → `xhigh`
2. `derive_effort_high_cco` — `cco-critic-r1` → `high`
3. `derive_effort_medium_distiller` — `ccs-distiller` → `medium`
4. `derive_effort_none_ccs_general` — `ccs-scout-r1` → `None` (inherits session)
5. `write_effort_map_roundtrip` — writes + reads JSON, asserts key lookup
6. `derive_effort_prefix_fallback` — unknown name → `None`, no panic

**Critical empirical prerequisite:** confirm CC passes `--teammate-id` in tmux-mode argv before shipping. Probe: `ps aux | grep claude` or `cat /proc/<pid>/cmdline` inside a live teammate pane. If argv is absent, the file-map still works but requires a second signal (env var from CC, or a pre-exec hook). Document as Gap 4 if unconfirmed.

**Tier classification:** Runtime-tier (wrapper intercepts subprocess spawn). Not Build/CI-tier — wrapper can be bypassed by invoking real claude directly. Ceiling must be documented per enforcement-tier discipline.