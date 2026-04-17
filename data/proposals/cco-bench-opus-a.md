PROPOSAL: Per-teammate effort via PATH-prepended `claude` shim + manifest lookup

## (a) Design sketch

**Intervention surface:** CC's tmux teammateMode spawns each teammate as its own `claude` process inside a tmux pane. That `claude` invocation is a PATH lookup — so a shim placed earlier on PATH intercepts every teammate spawn and can inject `CLAUDE_CODE_EFFORT_LEVEL` BEFORE the real binary reads session config.

**Three components:**

1. **Manifest generator** (`src/effort_manifest.rs`, new ~80 LoC) — reads `effort:` field from `templates/agents/*.md` YAML frontmatter (already present per roster) and emits `~/.config/xbreed/effort-manifest.yaml`:
   ```yaml
   default: xhigh              # session fallback
   agents:
     cco-critic-*:   xhigh     # glob match on agent name
     cco-*:          high
     the-judge:      xhigh
     ccs-distiller:  medium
     ccs-simplifier: medium
     ccs-scribe:     medium
   ```
   Called from `xbreed sync` (extends existing `src/sync.rs` path).

2. **Shim binary** (`src/bin/xbreed-claude-shim.rs`, new ~120 LoC — Rust, not bash, for reliable arg-passing):
   ```rust
   fn main() -> Result<()> {
       let real_claude = which::which_in("claude", Some(real_path_env()), ".")?;
       if env::var("CLAUDECODE").is_ok() && env::var("CLAUDE_CODE_EFFORT_LEVEL").is_err() {
           // We're inside a teammate spawn (CLAUDECODE=1 + teams flag present)
           if let Some(agent_name) = resolve_agent_name() {  // 3-tier: env → tmux pane title → argv scan
               let effort = manifest_lookup(&agent_name)?;   // glob match with specificity order
               env::set_var("CLAUDE_CODE_EFFORT_LEVEL", effort);
           }
       }
       // exec (not spawn) — zero process overhead, preserves PID semantics for CC's pane tracking
       Err(Command::new(real_claude).args(env::args_os().skip(1)).exec().into())
   }
   ```
   Agent-name resolution cascade:
   - **Tier 1:** `$CLAUDE_AGENT_NAME` if CC sets it (probe-confirmed unknown as of R3 — treat as best-case).
   - **Tier 2:** `tmux display-message -p '#T'` on `$TMUX_PANE` — CC tmux teammateMode sets pane title to agent name.
   - **Tier 3:** scan argv for `--teammate-name <x>` if CC passes it.
   - All three fail → no override, teammate inherits session default (graceful degradation).

3. **PATH-prepend install** (`src/cli.rs` new subcommand `xbreed effort-shim install`, ~30 LoC): symlinks `~/.local/xbreed-shims/claude → $(which xbreed-claude-shim)` and instructs user to prepend `~/.local/xbreed-shims` to PATH in shellrc. Opt-in — not auto-installed.

## (b) Failure modes

| Failure | Detect | Rollback |
|---|---|---|
| CC changes tmux-pane-title convention | shim fallback cascade exhausts → no override → session-default applies | Benign — logs `[xbreed-shim] no agent name resolved` to stderr |
| Manifest glob matches wrong agent (e.g. `cco-critic-foo` matches `cco-*` before `cco-critic-*`) | Order matters; specificity sort at parse time. Unit test on ambiguous names | Fix glob order; no shipped state corruption |
| Non-teammate `claude` invocation (user runs `claude` interactively) | Shim guards on `$CLAUDECODE` presence. Interactive `claude` has no `CLAUDECODE` → shim no-ops + execs | User sees no change |
| `CLAUDE_CODE_EFFORT_LEVEL` already set (user opted for session-wide override) | Shim honors pre-set env — does not overwrite | Explicit: user's env wins |
| Shim binary missing from PATH after `cargo install` | `xbreed effort-shim verify` subcommand checks PATH order + symlink target + prints diagnosis | Uninstall: remove symlink, prepend revert |
| Shim itself segfaults / panics | Process death; tmux pane shows crash, teammate never boots | Systematic: integration test spawns a noop teammate through shim + asserts boot |
| CC authors formally document per-teammate `effort:` in future release | Detect via version-pin check on startup (read `claude --version`) | Shim becomes inert or is uninstalled via `xbreed effort-shim uninstall` |

**Ceiling-honesty:** This is **runtime-tier with documented ceiling** (per §Enforcement Tiers). The shim wraps every `claude` invocation on PATH, but any process that calls real-claude by absolute path (`/usr/local/bin/claude`) bypasses it. Not reachable: Build/CI tier — we cannot fail `cargo test` if a teammate launches without an effort override, since CC itself is the launcher.

## (c) Implementation cost

| File | Δ LoC | Kind |
|---|---|---|
| `src/effort_manifest.rs` | +80 | new |
| `src/bin/xbreed-claude-shim.rs` | +120 | new |
| `src/cli.rs` | +35 | subcommand `effort-shim {install,uninstall,verify}` + `sync` call-site |
| `src/sync.rs` | +15 | wire manifest generation into existing sync path |
| `src/lib.rs` | +3 | mod declarations |
| `Cargo.toml` | +2 | add `which` crate; add second `[[bin]]` stanza for shim |
| `tests/effort_shim.rs` | +150 | new integration suite |
| `templates/agents/*.md` | 0 | read-only — frontmatter already has `effort:` |
| `commands/references/xbreed-shared.md` | +25 | §Per-Teammate Effort (replaces the "not reachable" caveat with the new opt-in path + retain session-wide workaround as fallback) |
| **Total** | **~430 LoC** | 6 files touched, 2 new binaries, 1 new test file |

**New tests (3 integration + 4 unit):**
1. `manifest_generator_parses_frontmatter` — given fixture templates, expected YAML.
2. `manifest_lookup_specificity` — `cco-critic-x` resolves to `cco-critic-*` entry before `cco-*`.
3. `shim_noop_when_no_claudecode` — interactive invocation passthrough.
4. `shim_sets_effort_when_claudecode_and_name` — end-to-end via a mock `claude` stub.
5. `shim_honors_preset_env` — `CLAUDE_CODE_EFFORT_LEVEL` already set → unchanged.
6. `shim_falls_back_gracefully_on_resolve_failure` — no TTY, no env, no tmux → no override, exit 0.
7. `effort_shim_verify_detects_path_ordering` — `which claude` returns non-shim → verify exits 1.

**Build-time cost:** one additional `[[bin]]` target adds ~3s to release build (shim is small). Total delta ≈ 430 LoC across 6 files, 7 new tests, no schema migration, opt-in via explicit `xbreed effort-shim install`.