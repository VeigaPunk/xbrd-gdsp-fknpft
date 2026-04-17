PROPOSAL: Per-teammate effort propagation via `claude` wrapper + effort-rules sidecar

---

## (a) Design sketch

The root constraint: `src/sync.rs:29` hard-codes `teammateMode: "tmux"`, so each teammate is a separate `claude` process in its own tmux pane. CC propagates only `tools` + `model` into these spawns; `effort:` frontmatter is silently dropped. The session-wide `CLAUDE_CODE_EFFORT_LEVEL` env var IS inherited by all panes (R3 Gap 3 empirically partial; Probe B shows env propagates via `/proc/$PPID/environ`), but sets the same tier for all teammates — per-teammate differentiation requires something keyed on identity.

**Primary mechanism: wrapper + name-prefix rule file**

1. `xbreed sync` emits `~/.xbreed/effort-rules.json` alongside `claude-settings.json`:

```json
{
  "rules": [
    {"prefix": "cco-",           "effort": "high"},
    {"prefix": "ccs-distiller",  "effort": "medium"},
    {"prefix": "ccs-scribe",     "effort": "medium"},
    {"prefix": "ccs-simplifier", "effort": "medium"}
  ],
  "default": null
}
```

2. `scripts/claude-wrapper` (installed as `~/.local/bin/claude`, which already shadows `~/.cargo/bin/claude` per `feedback_recompile_on_change.md`):

```bash
#!/usr/bin/env bash
REAL_CLAUDE="$(command -v -p claude 2>/dev/null || echo ~/.cargo/bin/claude)"
RULES="$HOME/.xbreed/effort-rules.json"

# Identity probe: CC may set CLAUDE_CODE_TEAMMATE_NAME — empirically unverified,
# needs labrat probe. Fallback: scan args for --name.
TEAMMATE_NAME="${CLAUDE_CODE_TEAMMATE_NAME:-}"
if [[ -z "$TEAMMATE_NAME" ]]; then
  for arg in "$@"; do
    [[ "$arg" =~ ^(cco-|ccs-|cdx-|g-) ]] && TEAMMATE_NAME="$arg" && break
  done
fi

if [[ -n "$TEAMMATE_NAME" && -f "$RULES" && -z "${CLAUDE_CODE_EFFORT_LEVEL:-}" ]]; then
  TIER="$(python3 -c "
import json,sys
rules=json.load(open('$RULES'))
name='$TEAMMATE_NAME'
for r in rules['rules']:
    if name.startswith(r['prefix']):
        print(r['effort']); sys.exit(0)
print(rules.get('default') or '')
" 2>/dev/null)"
  [[ -n "$TIER" ]] && export CLAUDE_CODE_EFFORT_LEVEL="$TIER"
fi

exec "$REAL_CLAUDE" "$@"
```

3. `src/sync.rs` extended: `materialize_effort_rules(agent_defs: &[AgentDef]) -> Value` generates the rules JSON from agent definitions. `write_xbreed_config(out_dir)` calls both `write_claude_settings` and writes `effort-rules.json` to `~/.xbreed/`.

**Empirical gap (labrat probe required before shipping):** Does CC pass the teammate name as an arg or env var to the spawned `claude` process? The wrapper's fallback arg-scan works if `--name ccs-distiller` appears in argv. If CC spawns by reading agent frontmatter without CLI name passing, the wrapper gets no identity signal and fails open (no effort override, session default applies). This is the critical unknown — the probe is cheap (pstree + /proc/$PID/cmdline inside a spawned teammate).

---

## (b) Failure modes

| Failure | Detection | Rollback |
|---|---|---|
| CC doesn't pass `--name` and doesn't set `CLAUDE_CODE_TEAMMATE_NAME` | Wrapper emits no override; teammates run at session default (same as current behavior) | No-op failure: safe, no regression |
| `REAL_CLAUDE` resolution finds wrapper recursively | Infinite exec loop | Add `XBREED_WRAPPER_ACTIVE=1` guard; if set, skip wrapper logic and exec directly to `~/.cargo/bin/claude` |
| `effort-rules.json` stale after agent rename | Wrong tier applied | `xbreed sync` always regenerates rules from source-of-truth agent defs; re-sync fixes it |
| `CLAUDE_CODE_EFFORT_LEVEL` already set by user (e.g., session-wide medium run) | Wrapper correctly skips (the `-z` guard above respects outer env) | N/A |
| Python3 unavailable | Silent no-op (effort not set) | Replace python3 one-liner with jq or pure-bash JSON parse |

---

## (c) Implementation cost

**Files touched:**
- `scripts/claude-wrapper` (new) — ~35 LoC bash
- `src/sync.rs` (modified) — `materialize_effort_rules` + `write_xbreed_config` — ~40 LoC Rust
- `src/lib.rs` — re-export new public fn — ~3 LoC
- `commands/references/xbreed-shared.md` §Session Effort Configuration — wrapper install step + empirical-gap caveat — ~20 LoC doc

**New tests:**
- `tests/sync_effort_rules.rs` — verify `materialize_effort_rules` emits correct prefix→tier map for known prefixes; verify `ccs-distiller` gets medium, `cco-critic` gets high, unknown gets null — ~40 LoC

**Total delta:** ~140 LoC across 4 files + 1 new test file. Zero CC source changes. Rollback: rename/remove wrapper, `~/.local/bin/claude` falls through to `~/.cargo/bin/claude`.

**Ceiling (honest):** Runtime-tier. Bypass surface: user can set `CLAUDE_CODE_EFFORT_LEVEL` before session to override; wrapper skips if already set. Protocol-tier fallback: if the labrat probe confirms CC passes no name signal, the wrapper is dead code and the only reachable path remains brief-injection (effort token in teammate brief — behavioral analog, not CC-native effort).

**Recommended next step:** labrat probe on `CLAUDE_CODE_TEAMMATE_NAME` + `/proc/$PID/cmdline` within a live teammate. If name is in cmdline → wrapper ships. If not → close with protocol-tier brief-injection as R4 ceiling-honest doc update.