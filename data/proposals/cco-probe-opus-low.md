PROPOSAL: per-teammate effort propagation via PATH-shim on `claude` binary

## (a) Design sketch

**Mechanism:** intercept `claude` invocations from tmux panes with a bash shim that sets `CLAUDE_CODE_EFFORT_LEVEL` per-teammate before `exec`-ing the real binary. Uses the only reachable lever given CC's teammate-mode exposes only `tools+model` to spawn args: env-var injection at process boundary.

**Flow:**
1. `xbreed team-init <team> --effort-map '{"cco-critic":"xhigh","ccs-distiller":"medium"}'` → writes `$HOME/.xbreed/teams/<team>/effort-map.json` + symlinks `$HOME/.xbreed/active-team/` → team dir.
2. `xbreed shim install` → writes `$HOME/.xbreed/bin/claude` (shim), records real-claude path (`command -v claude` before PATH mutation) in `$HOME/.xbreed/shim.conf`.
3. Orchestrator exports `PATH="$HOME/.xbreed/bin:$PATH"` BEFORE first `TeamCreate`. tmux panes inherit PATH; their `$SHELL -c "claude ..."` invocations hit the shim first.
4. Shim identifies teammate via `tmux display-message -t "$TMUX_PANE" -p '#{pane_title}'` (CC sets pane title to teammate name in tmux mode — verify empirically; fallback listed in (b)).
5. Shim looks up name → effort tier in active-team effort-map.json, sets env, execs real claude.

**Shim (pseudocode, ~40 lines bash):**
```bash
#!/usr/bin/env bash
set -eu
REAL=$(cat "$HOME/.xbreed/shim.conf" 2>/dev/null) || REAL=/usr/bin/claude
[ -n "${XBREED_SHIM_DISABLE:-}" ] && exec "$REAL" "$@"
MAP="$HOME/.xbreed/active-team/effort-map.json"
if [ -n "${TMUX_PANE:-}" ] && [ -f "$MAP" ]; then
  name=$(tmux display-message -t "$TMUX_PANE" -p '#{pane_title}' 2>/dev/null || true)
  [ -z "$name" ] && name=$(tmux show-environment -t "$TMUX_PANE" XBREED_TEAMMATE 2>/dev/null | cut -d= -f2 || true)
  tier=$(python3 -c "import json,sys;d=json.load(open('$MAP'));print(d.get('$name',''))" 2>/dev/null || true)
  [ -n "$tier" ] && export CLAUDE_CODE_EFFORT_LEVEL="$tier"
fi
exec "$REAL" "$@"
```

**Rust wiring (`src/shim.rs`, new):**
- `install_shim(home) -> Result<PathBuf>` — writes `include_str!("../scripts/claude-shim.sh")` to `~/.xbreed/bin/claude`, chmod 0755, records real path.
- `uninstall_shim(home) -> Result<()>` — rm shim + conf (rollback path, see (b)).
- `write_effort_map(team, map: HashMap<String,String>)` — JSON dump + atomic rename.
- CLI subcommands: `xbreed shim {install,uninstall,status}`, `xbreed team effort <team> <name>=<tier>...`.

## (b) Failure modes + detection + rollback

| Failure | Detection | Rollback |
|---|---|---|
| Pane title != teammate name (CC changes convention) | Shim logs lookup misses to `~/.xbreed/shim.log`; CI test asserts at least one hit per team after TeamCreate | Add secondary lookup: `tmux show-environment XBREED_TEAMMATE` set by orchestrator via `tmux set-environment -t <pane>` before claude starts — but this races process start (risk) |
| Shim breaks passthrough (exec failure, real-claude path stale) | `claude --version` integration test on every release; shim honors `XBREED_SHIM_DISABLE=1` bypass | `xbreed shim uninstall` removes file — PATH resolves to real claude again; worst case `rm ~/.xbreed/bin/claude` manually |
| `jq`/`python3` missing on host | Prefer pure-bash grep+cut parsing (map file already simple `"name":"tier"`); CI lint asserts no hard deps | Swap to `awk` fallback in shim template |
| CC bypasses PATH (absolute path to claude) | Verify via `pstree`/`ps` on test team: does teammate process cmdline show `/home/.../bin/claude` or `claude`? If absolute, shim never fires | **Hard ceiling** — would require patching CC or MCP-side injection; documented as limit, matching R3 ceiling-honesty pattern |
| Multiple concurrent xbreed teams clash on `active-team` symlink | Detection: second TeamCreate overwrites first's active-team → wrong map applies to orphan pane | Scope lookup by tmux session id: `effort-map-$(tmux display-message -p '#{session_id}').json`; reject concurrent active-team if symlink exists |
| Effort tier propagated but CC ignores env var at mid-session teammate spawn (only read at session init) | Run R3 Gap 3 probe: start session w/ shim, have teammate `printenv CLAUDE_CODE_EFFORT_LEVEL` | If CC reads env only at session-init → per-teammate scheme fails; fallback to session-wide CLAUDE_CODE_EFFORT_LEVEL (current state), ceiling documented |

**Critical detection step before shipping:** must close R3 Gap 3 empirically. If CC ignores env var in teammate subprocess (reads only at parent session init), entire shim scheme is dead — fails gracefully (no-op, passthrough intact) but delivers no per-teammate effort. This is the load-bearing assumption.

**Rollback (clean):** `xbreed shim uninstall` + unset PATH prefix. Zero residue in CC / tmux / settings.json. Shim is orthogonal to xbreed's existing sync.rs emit path — no changes to `materialize_claude_settings`, no teammateMode mutation, leaves R3 doc path intact as documented fallback.

## (c) Implementation cost

**Files touched:**
- NEW: `src/shim.rs` (~180 LoC)
- NEW: `scripts/claude-shim.sh` (~40 LoC, embedded via `include_str!`)
- NEW: `tests/shim_integration.rs` (~120 LoC)
- MOD: `src/lib.rs` (+1 mod line)
- MOD: `src/cli.rs` + `src/main.rs` (+~50 LoC for subcommands)
- MOD: `commands/references/xbreed-shared.md` §Session Effort Configuration (replace "not reachable" with shim path + Gap-3 empirical prerequisite note)

**LoC delta:** ~400 lines net add (incl. tests + docs).

**New tests (all Build/CI-tier):**
1. `shim_template_is_valid_bash` — `bash -n` on embedded template
2. `effort_map_roundtrip_json` — serde round-trip of `HashMap<String,Tier>`
3. `install_writes_shim_with_0755` — chmod assertion
4. `uninstall_is_idempotent` — repeated calls = no error
5. `active_team_symlink_refuses_double_init` — concurrent-team safety
6. Integration: spawn subprocess with `TMUX_PANE` mock + fake tmux binary on PATH, assert `CLAUDE_CODE_EFFORT_LEVEL` set in env passed to fake-claude

**Effort ceiling honesty:** this is runtime-tier hardening with documented ceiling per §Enforcement Tiers — shim bypassable by `XBREED_SHIM_DISABLE=1` or PATH override, and subject to CC's env-var-read-timing (Gap 3 prerequisite). Build/CI tests bound the shim's own correctness; they don't prove CC honors per-subprocess env.

Ship Gap-3 probe FIRST (one session restart with `CLAUDE_CODE_EFFORT_LEVEL=medium claude` + teammate printenv). If env propagates to teammate subprocess AND CC re-reads it at teammate init → shim viable. If not → don't build it; escalate to "Anthropic-side exposure" wishlist.