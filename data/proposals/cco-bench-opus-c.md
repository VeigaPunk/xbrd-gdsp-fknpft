PROPOSAL: Per-teammate effort propagation via PATH-shadowed `claude` shim

## (a) Design sketch

**Core:** CC teammate-mode inherits parent env (`CLAUDECODE=1` propagation confirmed R3). Since teammate-mode spawns `claude` subprocesses that read env at session-init, inject a per-teammate `CLAUDE_CODE_EFFORT_LEVEL` by PATH-shadowing `claude` with a wrapper that sets the env var before exec-ing the real binary.

**Components:**

1. `src/shim.rs` (new) — at `xbreed sync` time:
   - Read each referenced agent template (`~/.claude/agents/<name>.md`), parse `effort:` frontmatter
   - Emit `<team_dir>/shims/claude` (bash wrapper, 0755)
   - Emit `<team_dir>/effort-map.json` — `{"cco-critic-foo":"xhigh","ccs-distiller":"medium",...}`

2. `src/sync.rs` (modify) — inject env into `claude-settings.json`:
   ```rust
   settings["env"] = json!({
       "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1",
       "PATH": format!("{}:{}", shim_dir.display(), env::var("PATH")?),
       "XBREED_EFFORT_MAP": effort_map_path.display().to_string(),
   });
   ```

3. Shim script (bash, ~25 LoC):
   ```bash
   #!/usr/bin/env bash
   # recursion guard
   [[ -n "$XBREED_SHIM_ACTIVE" ]] && exec /usr/bin/env -u XBREED_SHIM_ACTIVE \
       "$(type -aP claude | grep -v xbreed-shims | head -1)" "$@"
   export XBREED_SHIM_ACTIVE=1

   # identity: prefer tmux pane title (CC sets it to teammate name)
   name="$(tmux display-message -p '#{pane_title}' 2>/dev/null)"
   [[ -z "$name" ]] && for a in "$@"; do
       [[ "$a" =~ ^--agent(-name)?$ ]] && next=1 && continue
       [[ "$next" == 1 ]] && name="$a" && break
   done

   if [[ -n "$name" && -r "$XBREED_EFFORT_MAP" ]]; then
       effort="$(jq -r --arg n "$name" '.[$n] // empty' "$XBREED_EFFORT_MAP")"
       [[ -n "$effort" ]] && export CLAUDE_CODE_EFFORT_LEVEL="$effort"
   fi

   exec "$(type -aP claude | grep -v xbreed-shims | head -1)" "$@"
   ```

**Resolution order inside shim:** (1) tmux pane title (CC-set); (2) `--agent` / `--agent-name` arg sniff; (3) fall through to session default. Teammate identity lookup is best-effort — on miss, silently fall through so the system degrades to R3 session-wide env workaround.

## (b) Failure modes

| Mode | Detection | Rollback |
|---|---|---|
| Shim recursion (shim resolves itself as "real claude") | `XBREED_SHIM_ACTIVE` sentinel; integration test `tests/shim_exec.rs` asserts `CLAUDE_CODE_EFFORT_LEVEL=X claude --version` completes in <2s | Unset `PATH` injection in settings.json; shim is idempotent no-op |
| tmux pane title not set → identity unknown | Shim emits stderr warning `XBREED_SHIM: no teammate name, falling through`; grep stderr in test | Falls through to session default — zero regression |
| CC changes tmux integration (pane title no longer matches teammate name) | Canary: `xbreed precheck shim-identity` walks current tmux session, asserts pane_title ∈ known teammate names from latest team config | Disable shim via `XBREED_SHIM_DISABLE=1` env; sync.rs omits PATH injection when that is set |
| Real `claude` binary symlinked into `~/.local/bin` and shadowed PATH leaks into non-teammate shells | `type -aP claude | grep -v xbreed-shims` pattern; shim only injected via settings.json `env`, scoped to CC session; does NOT leak to user's interactive shell | None needed — scoped by construction |
| `jq` not installed | Shim uses POSIX fallback (sed-extract); detected by CI check | Doc: xbreed sync fails hard if jq absent; add to install-commands.sh |
| Effort-map stale (team reconfigured mid-session) | `xbreed sync` regenerates map; existing panes keep old effort (acceptable — next round's panes pick up new map) | Re-run `xbreed sync`; new spawns inherit |

**Tier framing (per xbreed-shared.md §Enforcement Tiers):** Runtime-tier with documented ceiling. Bypass surface: any teammate that execs `/usr/bin/claude` directly (not via PATH) skips the shim. Acceptable — the shim targets CC-spawned teammates, not arbitrary processes.

## (c) Implementation cost

| Surface | Files | LoC |
|---|---|---|
| New | `src/shim.rs` | ~90 (effort-map writer + shim emit + frontmatter parse) |
| New | `templates/claude-shim.sh` | ~30 (bash, included via `include_str!`) |
| New | `tests/shim_exec.rs` | ~60 (integration: emit shim, stub claude binary in PATH, assert env prop) |
| New | `tests/shim_identity.rs` | ~40 (tmux pane-title resolution + recursion-guard test) |
| Modified | `src/sync.rs` | +25 (PATH injection + map path wiring) |
| Modified | `src/cli.rs` | +8 (optional `xbreed sync --effort-map` flag) |
| Modified | `src/lib.rs` | +1 (`pub mod shim`) |
| Unit tests | `shim.rs` inline | ~30 (frontmatter-parse, sentinel-guard, identity-resolve edge cases) |
| **Total delta** | 5 new, 3 modified | **~284 LoC, 4 new tests + 3 unit tests** |

Additional dependency: `jq` required on teammate env (already widespread; documented in install-commands.sh). No new Rust crates.

**Frontier implication:** Closes R3 known-gap #3 structurally — per-teammate effort becomes reachable from user-space via `effort:` frontmatter already present in agent templates. Makes `feedback_sonnet_effort_tiers.md`, `feedback_cco_opus_high.md`, `feedback_the_planner_wwkd.md` operative rather than aspirational.