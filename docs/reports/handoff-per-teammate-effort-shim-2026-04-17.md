# Handoff — Per-teammate effort propagation shim

**Mission:** make `effort:` frontmatter (or a mapped tier) operative per-teammate, closing the teammate-mode noop documented in `feedback_teammate_mode_effort_caveat.md` and R3 known-gap #3.
**Source data:** teammate-benchmark 2026-04-17 — top 2 proposals at 19/20: `cco-probe-opus-high` + `cco-bench-opus-a`. Full proposals at `data/proposals/cco-probe-opus-high.md` and `data/proposals/cco-bench-opus-a.md`.
**Status:** designed, not implemented. Pick up next session.

---

## TL;DR

Ship a Rust `claude` shim on PATH that reads the spawning teammate's name, maps name → effort tier, sets `CLAUDE_CODE_EFFORT_LEVEL` before `exec`ing the real `claude` binary. Opt-in via `xbreed effort-shim install`. Bypassable by uninstalling the symlink.

**Do NOT start coding** until M0 (the load-bearing probe) returns positive. The whole design collapses if CC doesn't expose the teammate name through one of three cascade tiers.

---

## The load-bearing unknown (M0 — MUST answer first)

Every shim proposal across 13 teammates assumed CC makes the teammate name discoverable inside the spawned pane. **No proposal verified it empirically.** Three possible sources, in priority order:

1. **Env var:** `$CLAUDE_AGENT_NAME` / `$CLAUDE_TEAMMATE_NAME` set by CC before `exec`
2. **tmux pane title:** `tmux display-message -p '#{pane_title}'` returns the teammate name
3. **argv:** `cat /proc/$$/cmdline | tr '\0' ' '` contains `--agent-name <X>` / `--teammate <X>`

If **all three fail**, the shim has no signal and the design degrades to session-wide `CLAUDE_CODE_EFFORT_LEVEL` (current R3-shipped workaround, no change). **Do not build the shim in that case.**

### Probe recipe (M0 — ~10 min, no code changes)

```bash
# In this repo, spawn one teammate (can use a throwaway team) and give it this task:
```

Teammate brief:
```
You are `ccs-probe-cascade` on team `probe-teammate-name`. Run these three
bash commands and SendMessage the literal output of all three to team-lead:

  1. env | grep -iE 'agent|teammate|claude_' | sort
  2. tmux display-message -p '#{pane_title}' 2>/dev/null
  3. tr '\0' ' ' < /proc/$$/cmdline; echo
```

Decision tree from the output:
- **Env contains your teammate name** (e.g. `CLAUDE_AGENT_NAME=ccs-probe-cascade`): **cascade tier 1 works**. Simplest shim possible. Ship.
- **tmux pane title is your teammate name**: **cascade tier 2 works**. Requires tmux; matches xbreed's teammateMode. Ship.
- **argv contains `--agent-name ccs-probe-cascade` or similar**: **cascade tier 3 works**. Parse argv. Ship.
- **None of the above**: **ABORT.** Document as follow-up in R4 ceiling-honesty fold. File a CC feature request.

Commit the probe output as `docs/reports/teammate-name-probe-2026-04-XX.md` regardless of result — the negative case is valuable.

---

## Design (assumes M0 positive)

Merges `cco-bench-opus-a`'s 3-tier resolution cascade + `cco-probe-opus-high`'s per-pane settings-dir fallback for future-proofing.

### New files

| file | purpose | LoC |
|---|---|---|
| `src/effort_shim.rs` | Core: name resolution, tier lookup, env injection, exec | ~100 |
| `src/bin/xbreed-claude.rs` | Binary entry point; just calls `effort_shim::main()` | ~10 |
| `src/effort_manifest.rs` | Walks `templates/agents/*.md`, parses YAML frontmatter `effort:`, emits the name-prefix → tier map | ~80 |
| `scripts/xbreed-effort-shim-install.sh` | Idempotent install: symlink, PATH check, diagnosis | ~30 |
| `tests/effort_shim.rs` | Integration tests — prefix match, cascade, guards | ~150 |
| `config/effort-map.yaml` | Fallback map (user-editable) if frontmatter insufficient | ~30 |

### Files to modify

| file | change | LoC delta |
|---|---|---|
| `src/lib.rs` | `pub mod effort_shim; pub mod effort_manifest;` | +2 |
| `src/cli.rs` | Add `effort-shim {install,uninstall,verify,doctor}` subcommand | +35 |
| `src/main.rs` | argv[0] basename dispatch: if called as `claude`, enter shim mode | +15 |
| `src/sync.rs` | Call `effort_manifest::emit_map` in sync path (opt-in flag) | +15 |
| `Cargo.toml` | Add `[[bin]] xbreed-claude` stanza; `which = "6"` if needed | +5 |
| `commands/references/xbreed-shared.md` | §Session Effort Configuration — flip ceiling paragraph from "not reachable" to "reachable via `xbreed effort-shim install`"; keep session-wide env var as fallback | +25 |
| `feedback_teammate_mode_effort_caveat.md` | Update to mark per-teammate effort as REACHABLE via shim, with the M0-probe result as evidence | rewrite |

**Total delta: ~400 LoC net add.**

### Core resolution cascade

```rust
// src/effort_shim.rs (pseudocode)
fn shim_main(argv: Vec<String>) -> ! {
    let real_claude = resolve_real_claude_path();

    // Recursion guard — critical
    if std::env::var("XBREED_SHIM_ACTIVE").is_ok() {
        exec(real_claude, argv);
    }
    std::env::set_var("XBREED_SHIM_ACTIVE", "1");

    // Honor user pre-set — session-wide CLAUDE_CODE_EFFORT_LEVEL wins
    if std::env::var("CLAUDE_CODE_EFFORT_LEVEL").is_err() {
        if let Some(name) = resolve_teammate_name() {  // 3-tier cascade
            if let Some(tier) = lookup_tier(&name) {
                std::env::set_var("CLAUDE_CODE_EFFORT_LEVEL", tier);
                log_resolution(&name, tier);
            }
        }
    }

    exec(real_claude, argv);  // never returns
}

fn resolve_teammate_name() -> Option<String> {
    // Tier 1: env (future-proof)
    if let Ok(n) = std::env::var("CLAUDE_AGENT_NAME") { return Some(n); }
    if let Ok(n) = std::env::var("CLAUDE_TEAMMATE_NAME") { return Some(n); }
    // Tier 2: tmux pane title
    if std::env::var("TMUX").is_ok() {
        if let Ok(out) = Command::new("tmux").args(["display-message","-p","#{pane_title}"]).output() {
            let title = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if title.starts_with("cco-") || title.starts_with("ccs-") || title.starts_with("cdx-") || title.starts_with("g-") {
                return Some(title);
            }
        }
    }
    // Tier 3: argv scan
    let args: Vec<_> = std::env::args().collect();
    for w in args.windows(2) {
        if w[0] == "--agent-name" || w[0] == "--teammate" || w[0] == "--teammate-id" {
            return Some(w[1].clone());
        }
    }
    None
}

fn lookup_tier(name: &str) -> Option<String> {
    let map = load_effort_map();  // from ~/.xbreed/effort-map.json (synced from agent frontmatter)
    // Longest-prefix match — specificity wins
    map.iter()
        .filter(|(k, _)| name.starts_with(k.as_str()))
        .max_by_key(|(k, _)| k.len())
        .map(|(_, v)| v.clone())
}
```

### Install flow

```bash
xbreed effort-shim install
  → builds xbreed-claude binary (cargo install --path .)
  → symlinks ~/.local/xbreed-shims/claude → ~/.local/bin/xbreed-claude
  → prints one-line PATH hint if ~/.local/xbreed-shims is not ahead of the real claude
  → runs `xbreed effort-shim doctor` to self-verify

xbreed effort-shim uninstall
  → rm ~/.local/xbreed-shims/claude
  → no state residue
```

---

## Sequenced milestones (wwkd posture)

### M0 — Load-bearing probe (no code)
**Does:** spawn one labrat, run 3-command probe, commit result.
**Gate:** probe output committed; one of the 3 cascade tiers positive OR design aborted.

### M1 — Skeleton shim that logs and passes through
**Does:** `src/effort_shim.rs` + `src/bin/xbreed-claude.rs` that:
- Resolves real claude path
- Calls `resolve_teammate_name()`
- Logs resolution to `~/.xbreed/shim.log`
- Does NOT set effort yet — just observes
- Execs real claude
**Gate:** manually PATH-prepend the shim; spawn one teammate; `grep ccs ~/.xbreed/shim.log` returns a line. No teammate behavior change.

### M2 — Overfit: one teammate gets medium effort
**Does:** hard-code a single name→tier rule in M1's skeleton (`if name == "ccs-probe-medium" → "medium"`), no manifest yet. Spawn `ccs-probe-medium` teammate; have it `printenv | grep CLAUDE_CODE_EFFORT_LEVEL` and report.
**Gate:** teammate's env shows `CLAUDE_CODE_EFFORT_LEVEL=medium`. This **closes R3 known-gap #3 empirically** — env is restart-verified to propagate into teammate processes, if the probe succeeds.

### M3 — Manifest generation from agent frontmatter
**Does:** `src/effort_manifest.rs` walks `~/.claude/agents/*.md` + `templates/agents/*.md`, parses YAML frontmatter, emits `{prefix → tier}` JSON. Replace M2's hard-coded rule with manifest lookup.
**Gate:** `xbreed sync` regenerates manifest idempotently; ccs-distiller resolves to medium via prefix match; unknown name resolves to None.

### M4 — CLI + install + doctor
**Does:** `src/cli.rs` adds `effort-shim {install,uninstall,verify,doctor}`. Install script symlinks + checks PATH ordering. Doctor reports: real-claude path, shim path, map size, last 5 log entries.
**Gate:** clean install from fresh checkout on a second machine; `xbreed effort-shim verify` passes or prints actionable fix.

### M5 — Test suite
**Does:** `tests/effort_shim.rs` — 7 tests:
1. `resolve_tier_longest_prefix_wins`
2. `shim_noop_when_claude_code_effort_level_preset`
3. `shim_recursion_guard`
4. `shim_falls_through_gracefully_on_resolve_miss`
5. `manifest_regenerates_idempotently`
6. `effort_shim_verify_detects_wrong_path_ordering`
7. `non_teammate_name_passes_through` (e.g. user running `claude` interactively)
**Gate:** `cargo test effort_shim` all green; `cargo clippy && cargo fmt --check` clean.

### M6 — Docs + memory updates + ship
**Does:**
- `commands/references/xbreed-shared.md` §Session Effort Configuration — flip ceiling paragraph
- `feedback_teammate_mode_effort_caveat.md` — mark resolved; new memory entry pointing at the shim
- `docs/reports/per-teammate-effort-shim-2026-04-XX.md` — completion report
**Gate:** `xbreed effort-shim install` is the ship path; one clean xbgst round with mixed-tier teammates shows each at its intended tier in logs.

---

## Failure modes (from the benchmark proposals, deduped)

| # | Mode | Detect | Rollback |
|---|---|---|---|
| 1 | **M0 blocker:** CC exposes no teammate name via env/tmux/argv | Probe returns negative | Abort shim; document as R4 ceiling |
| 2 | Recursion: shim exec's itself | `XBREED_SHIM_ACTIVE=1` guard trips | None needed — guard handles |
| 3 | Wrong real-claude resolution | `xbreed effort-shim doctor` prints resolved path; mismatch visible | Set `XBREED_CLAUDE_REAL` env explicitly |
| 4 | Stale manifest vs agent templates | `doctor` compares mtimes | `xbreed sync` regenerates |
| 5 | User pre-sets `CLAUDE_CODE_EFFORT_LEVEL` | Shim honors pre-set (early return) | Correct by design |
| 6 | CC changes name-passing convention between versions | Shim cascade falls through to None → session default | Update cascade code; no data loss |
| 7 | `CLAUDE_CODE_EFFORT_LEVEL` env read by CC only at process-start, not on teammate spawn | M2 gate will catch this: if printenv shows the var but effort feels unchanged, the env-var path itself is noop for teammate-mode too | Escalate to CC team; ship nothing |
| 8 | Shim binary missing from PATH after `cargo install` | `doctor` detects | Reinstall via `xbreed effort-shim install` |

**Failure mode 7 is the second load-bearing unknown.** M2 is specifically designed to catch it — if the env var gets SET but teammate behavior doesn't change (e.g., teammate output volume / variance same as xhigh baseline), the whole scheme collapses. M2 is the go/no-go gate for M3+.

---

## Ceiling (honest framing)

**Tier:** Runtime-tier hardening with documented ceiling.
**Bypass surfaces:**
- User removes shim symlink → real claude, session default applies
- User invokes `/absolute/path/to/claude` → PATH shim not consulted
- `XBREED_SHIM_ACTIVE=1` set in outer env → shim no-ops
- Map miss (unknown teammate name) → session default applies (zero regression)

**Not Build/CI-tier** because CC is the launcher; we can't fail `cargo test` if CC spawns a teammate without the shim.
**Not Protocol-tier** because the enforcement is in the binary, not in agent briefs.

If future Anthropic-side CC exposes per-teammate `effort:` in teammate-mode spawn args directly, the shim becomes inert — `xbreed effort-shim uninstall` is the clean removal.

---

## Reading order for next session

1. **This doc** (handoff)
2. **The top-2 proposals (the actual design source):**
   - `data/proposals/cco-probe-opus-high.md`
   - `data/proposals/cco-bench-opus-a.md`
3. **Context files:**
   - `src/sync.rs` — where teammateMode is forced (understand current pipeline)
   - `commands/references/xbreed-shared.md` §Session Effort Configuration — current ceiling statement to revise
   - `docs/reports/xbreed-harness-r3-2026-04-17.md` — R3 Gap 3 context
4. **Benchmark data (if second-opinion scoring wanted):**
   - `data/bench-quality.json` — all 13 proposal scores
   - `docs/reports/teammate-benchmark-summary-2026-04-17.md` — synthesis + caveats

---

## First command on next session

```bash
# 1. Confirm no stale teams blocking
ls ~/.claude/teams/ ~/.claude/tasks/
# 2. Precheck capacity
xbreed precheck pane-cap --team-size 1
# 3. Spawn M0 labrat with the 3-command probe (see §The load-bearing unknown above)
# 4. Commit probe result regardless of outcome
# 5. Branch on result:
#    - positive → start M1 skeleton
#    - negative → write R4 ceiling-honesty fold, close mission
```

Mission pickup point: `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md` (this file).
