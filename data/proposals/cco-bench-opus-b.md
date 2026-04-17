PROPOSAL: Per-teammate effort propagation via PATH-shim wrapper (`xbreed-claude`) — reachable, runtime-tier, bypassable by unsetting PATH shim.

## (a) Design sketch

**Insight.** `src/sync.rs:29` pins `teammateMode: "tmux"`, so each teammate spawns as its own `claude` process inside a tmux pane, inheriting parent env (R3 `/proc/$PPID/environ` evidence). `CLAUDE_CODE_EFFORT_LEVEL` is read at session init. If we set that env **per-pane before `claude` execs**, we get per-teammate effort in a reachable way.

**Mechanism.** Install an `xbreed-claude` bash shim in `~/.local/bin/` (or any dir placed ahead of real `claude` on PATH via `settings.env.PATH`). CC launches teammates by running `claude <args>` in the new pane; the shim intercepts:

```bash
#!/usr/bin/env bash
# xbreed-claude — per-teammate CLAUDE_CODE_EFFORT_LEVEL injector
set -euo pipefail

# Teammate name discovery cascade:
#   1. XBREED_AGENT_NAME (set by future sync.rs env plumbing, if reachable)
#   2. tmux pane title (CC sets pane_title to agent name in tmux mode — verify empirically)
#   3. argv scan for --agent-name / --teammate-name
#   4. fallback: inherit outer CLAUDE_CODE_EFFORT_LEVEL unchanged
AGENT="${XBREED_AGENT_NAME:-}"
if [[ -z "$AGENT" && -n "${TMUX_PANE:-}" ]]; then
  AGENT="$(tmux display-message -t "$TMUX_PANE" -p '#{pane_title}' 2>/dev/null || true)"
fi
# argv scan omitted for brevity

MAP="${XBREED_EFFORT_MAP:-$HOME/.claude/xbreed-effort-map.json}"
if [[ -n "$AGENT" && -r "$MAP" ]]; then
  TIER="$(jq -r --arg n "$AGENT" '.[$n] // empty' "$MAP")"
  [[ -n "$TIER" ]] && export CLAUDE_CODE_EFFORT_LEVEL="$TIER"
fi

REAL="${XBREED_CLAUDE_REAL:-$(PATH="${PATH//$HOME\/.local\/bin:/}" command -v claude)}"
exec "$REAL" "$@"
```

**Sync integration.** Extend `src/sync.rs::materialize_claude_settings` to (i) emit `"env": { ..., "PATH": "$HOME/.local/bin:$PATH" }` (or use `command.PATH` override if CC respects it); (ii) generate `~/.claude/xbreed-effort-map.json` from `templates/agents/*.md` frontmatter `effort:` values keyed by filename stem. New function `write_effort_map(agents_dir, out_path)` that parses YAML frontmatter and emits `{ "cco-critic-*": "high", "ccs-distiller": "medium", ... }` with glob expansion handled at shim lookup time (longest-prefix match).

**Why this tier.** Runtime-tier hardening with documented ceiling — the shim wraps the subprocess xbreed/CC launches; bypassable by removing the PATH prefix or unsetting `XBREED_EFFORT_MAP`. Build/CI-tier is unreachable because CC's teammate-spawn shell command isn't xbreed-owned.

## (b) Failure modes

1. **Empirical unknown: does CC set pane_title to agent name?** If no, cascade step 2 is dead and we fall to argv/env. Rollback: `XBREED_EFFORT_MAP` unset → shim is transparent. Detection: ship with `XBREED_SHIM_DEBUG=1` trace to `/tmp/xbreed-shim-$$.log`; run one xbgst round and grep for resolved names.
2. **Wrong real-claude resolution.** PATH-scrub heuristic misses a non-standard install location. Mitigation: `XBREED_CLAUDE_REAL` explicit override read first; `xbreed doctor` subcommand prints resolved real path.
3. **Shim shadows user's manual `claude` invocation.** The shim is transparent when `XBREED_AGENT_NAME`/pane-title absent, so manual use works unchanged. If it misbehaves, remove `~/.local/bin` from PATH.
4. **jq missing.** Shim degrades to pass-through (warn to stderr once). Detection: `cargo test` integration test invokes shim with a fixture map and asserts env gets set; CI gate fails if jq missing.
5. **Effort map drift from agent templates.** `xbreed sync` regenerates unconditionally; stale map still valid because CC falls back to session default. Detection: `cargo test` asserts every `templates/agents/*.md` with `effort:` has a map entry.
6. **Rollback path.** `rm ~/.local/bin/xbreed-claude && rm ~/.claude/xbreed-effort-map.json && xbreed sync --no-effort-shim` reverts to session-wide env var (current R3-shipped workaround).

## (c) Implementation cost

- **New files:** `scripts/xbreed-claude` (~60 LoC bash), `tests/effort_shim.rs` (~80 LoC — spawns shim with fixture map+env, asserts `CLAUDE_CODE_EFFORT_LEVEL` exported).
- **Modified files:** `src/sync.rs` (+~70 LoC: `write_effort_map` + frontmatter parser + shim-install helper; +2 tests); `src/cli.rs` (+~10 LoC: `--no-effort-shim` flag on sync); `commands/references/xbreed-shared.md` §Session Effort Configuration (+~25 lines documenting shim + ceiling).
- **New deps:** none (serde_yaml already in stack for YAML frontmatter; jq is OS-level, not cargo).
- **LoC delta:** +~245 LoC (including tests + doc), -0.
- **Test count:** +4 (2 unit in sync.rs, 2 integration in tests/effort_shim.rs).
- **Effort:** ~3 hrs solo including empirical pane-title probe.

Blocker to ship: one empirical probe to confirm CC sets pane_title or passes agent name in argv. If both absent, this design degrades to session-wide (no improvement over R3 workaround) — proceed only after probe returns positive.