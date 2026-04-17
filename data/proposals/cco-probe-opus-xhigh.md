PROPOSAL: Per-teammate effort propagation via PATH-shadowed `claude` wrapper + xbreed-emitted effort map. `cco-probe-opus-xhigh`, axis bench/solo.

**Root-cause framing.** CC teammate-mode ultimately `exec`s `claude` inside a fresh tmux pane (sync.rs forces `teammateMode: "tmux"`). That exec is a PATH lookup against the teammate pane's environment. R3-E1 verified env vars inherit parent→teammate (`CLAUDECODE=1` propagated). So the *per-process env at exec time* is the reachable surface: if we can set `CLAUDE_CODE_EFFORT_LEVEL` differently per teammate on that single exec, CC's documented precedence (env > settings.json > frontmatter) gives us per-teammate effort — the exact axis teammate-mode drops.

**(a) Design sketch.**

1. `xbreed sync` gains `--install-effort-wrapper` (default on). Two new artifacts:
   - `~/.xbreed/bin/claude` — shim script (template `include_str!`d from `src/wrapper_sh.rs`).
   - `~/.xbreed/effort-map.json` — `{teammate-name: effort-tier}` emitted by walking `~/.claude/agents/*.md` + any `templates/agents/` override, parsing YAML frontmatter for `effort:`.

2. Shim logic (bash, ~40 lines):
   ```bash
   #!/usr/bin/env bash
   [[ -n "$XBREED_WRAP" ]] && exec "$XBREED_REAL_CLAUDE" "$@"   # recursion guard
   export XBREED_WRAP=1
   name=""
   for ((i=1; i<=$#; i++)); do
     case "${!i}" in
       --agent|--teammate|--agent-name)
         j=$((i+1)); name="${!j}"; break;;
       --agent=*|--teammate=*) name="${!i#*=}"; break;;
     esac
   done
   if [[ -n "$name" ]] && tier=$(jq -er --arg n "$name" '.[$n]' ~/.xbreed/effort-map.json 2>/dev/null); then
     export CLAUDE_CODE_EFFORT_LEVEL="$tier"
   fi
   : "${XBREED_REAL_CLAUDE:=$(command -v -a claude | grep -v "^$HOME/.xbreed/bin/" | head -1)}"
   printf '%s\t%s\t%s\t%s\n' "$(date -Is)" "${name:-?}" "${CLAUDE_CODE_EFFORT_LEVEL:-inherit}" "$*" >> ~/.xbreed/wrapper.log
   exec "$XBREED_REAL_CLAUDE" "$@"
   ```

3. sync writes shim + map; `xbreed precheck effort-wrap` verifies `~/.xbreed/bin` precedes real `claude` on PATH (warns with one-line rc-export hint if not — user still owns PATH setup; no auto-rc-edit, per `feedback_no_safety_theater.md`).

4. Unknown-teammate fallthrough = inherit session default. No behaviour change for anyone not listed in the map.

**(b) Failure modes.**

| # | Break | Detect | Rollback |
|---|---|---|---|
| 1 | CC teammate-spawn argv lacks a recognizable `--teammate`/`--agent` flag (format drift across CC releases) | `wrapper.log` column 2 = `?` across all rows after a sync | `rm ~/.xbreed/bin/claude`; env-inheritance path is lossless — teammates just resume session-wide default |
| 2 | Real `claude` resolution wrong (npm vs. fnm vs. brew) → shim execs itself-or-nothing | recursion guard trips → `stderr` banner + exit 127 immediately on first spawn | `PATH` without `~/.xbreed/bin` (user unsets or removes shim) |
| 3 | effort-map stale vs. agent frontmatter (sync not rerun after template edit) | `precheck effort-wrap` compares `mtime(effort-map.json)` vs. newest agent template | Re-run `xbreed sync`; map is rebuilt idempotently |
| 4 | `CLAUDE_CODE_EFFORT_LEVEL` leaks to tools CC shells out to (bash heredocs, git hooks) | it already does under the documented env-var workaround — baseline parity | n/a |
| 5 | CC starts reading effort from a non-env source (future version) | R3-style probe: teammate brief includes `printenv \| grep EFFORT` — if set but behaviour unchanged, log a regression note | env-var path stops being operative; shim becomes no-op, same as today's ceiling |
| 6 | User has symlink `~/.local/bin/claude → real-claude` earlier in PATH | `precheck effort-wrap` prints resolved ordering | documented in precheck output |

**(c) Implementation cost.**

- `src/sync.rs` — +25 LoC (call into new module, thread install flag). Existing tests intact.
- `src/effort_map.rs` — new, ~110 LoC (glob `~/.claude/agents/*.md`, `serde_yaml::from_str` frontmatter extract, emit map). `serde_yaml` already in Cargo.toml.
- `src/wrapper_sh.rs` — new, ~20 LoC wrapping `include_str!("../scripts/claude-effort-wrapper.sh")`.
- `scripts/claude-effort-wrapper.sh` — new, ~45 lines bash.
- `src/precheck.rs` — +35 LoC for `effort-wrap` subcommand (PATH ordering check + map-staleness check).
- `src/main.rs` — +8 LoC clap subcommand wiring.
- `commands/references/xbreed-shared.md` — replace §Session Effort Configuration ceiling paragraph with "OPERATIVE: per-teammate via xbreed sync `--install-effort-wrapper`" + keep the env-var fallback for bare-`claude` invocations.
- Tests: `tests/effort_map_integration.rs` — 6 new tests (frontmatter variants, missing `effort:`, map idempotency, recursion guard via subprocess, PATH precheck pass/fail, unknown-teammate fallthrough).

**Total LoC delta ≈ 255 Rust + 45 bash + 90 test = ~390.** Net-new deps: none. Affects sync path only; guard/dispatch paths untouched. R3-E1 env-inheritance evidence is load-bearing — shim's correctness reduces to "can the shim set env before `exec`." That's standard POSIX.