PROPOSAL: Per-Teammate Effort Propagation — `xbreed team-spawn`

---

## (a) Design Sketch

**Root cause:** `src/sync.rs:29` hard-forces `teammateMode: "tmux"`. CC's `TeamCreate` creates all panes as children of one parent process — they all inherit one env, so `CLAUDE_CODE_EFFORT_LEVEL` is session-wide or nothing.

**Key insight:** In tmux mode each teammate IS a separate OS process. If xbreed spawns those processes directly (bypassing CC's `TeamCreate` pane-creation), it controls per-process env.

**Mechanism: `xbreed team-spawn` — manual tmux spawn with per-effort env injection**

```rust
// src/team_spawn.rs  (new module)
pub struct TeammateSpec {
    pub name:    String,
    pub effort:  String,   // "low"|"medium"|"high"|"xhigh"|"max"
    pub agent:   PathBuf,  // templates/agents/<role>.md
    pub brief:   String,
}

pub fn spawn_effort_team(team_name: &str, specs: &[TeammateSpec]) -> Result<()> {
    for spec in specs {
        // 1. Write brief to per-teammate tmpdir CLAUDE.md (identity injection)
        let proj = write_teammate_project(team_name, &spec.name, &spec.brief)?;

        // 2. Spawn pane with per-effort env — before claude starts
        Command::new("tmux").args([
            "new-window", "-d", "-n", &spec.name,
            // env vars set in pane context, inherited by claude child
            &format!(
                "CLAUDE_CODE_EFFORT_LEVEL={} \
                 CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1 \
                 claude --project-dir {}",
                spec.effort,
                proj.display()
            ),
        ]).status()?;
    }
    Ok(())
}
```

**Identity injection:** Each teammate's `proj/CLAUDE.md` contains its brief + agent system prompt. This replaces the TeamCreate brief-delivery path. Team name is passed via `XBREED_TEAM=<name>` env var or a `CLAUDE.md` directive.

**SendMessage routing:** Depends on whether CC's mailbox routes by pane-name or by TeamCreate registry. Two sub-options:
- **Option A (preferred):** Also call CC's `TeamCreate` with dummy minimal specs for routing registration only — gets mailbox wired, then the manually-spawned panes (same names) take over. Race window: ~500ms between TeamCreate and pane creation.
- **Option B (fallback):** Inspect `~/.claude/teams/<team-name>/` structure, write registrations directly. Fragile against CC version changes.

**Callsite in xbreed orchestration:** Replace `Agent(TeamCreate...)` + separate `TeamCreate` call with `Bash: xbreed team-spawn --config /tmp/team-spec.yaml`.

---

## (b) Failure Modes

| Failure | How to detect | Rollback |
|---|---|---|
| **Routing break** — manually-spawned panes not registered in CC mailbox; `SendMessage` to teammate silently drops | Send a ping DM immediately post-spawn; if no ack within 5s, declare `ROUTING_FAIL` | Fall back to current `CLAUDE_CODE_EFFORT_LEVEL=<single-tier> claude` session-wide |
| **Race: TeamCreate re-creates panes** (Option A) — CC's TeamCreate creates new panes with same names, clobbering manually-spawned panes | Detect by checking pane count before/after TeamCreate; if delta > 0, clobber occurred | Use Option B (direct registry write) instead |
| **env var ignored at session-init** — CC reads effort from settings.json before env is available | Teammate's first Bash call: `printenv CLAUDE_CODE_EFFORT_LEVEL` — if absent or wrong, signal `EFFORT_MISMATCH` to judge | Re-export in pane via `tmux send-keys -t <pane> "export CLAUDE_CODE_EFFORT_LEVEL=<tier>" Enter` before claude reads effort (racy but recoverable) |
| **Version drift** — CC changes team registry format, Option B breaks | `cargo test` catches if a test reads registry format against a fixed snapshot | Drop to Option A |

**Rollback path:** `src/sync.rs` already supports session-wide `CLAUDE_CODE_EFFORT_LEVEL` as documented workaround. `team-spawn` is additive — if unused, current behavior unchanged. Git revert is a clean rollback.

---

## (c) Implementation Cost

**Files touched:**
- `src/team_spawn.rs` — new (~130 LoC): `TeammateSpec`, `spawn_effort_team`, `write_teammate_project`
- `src/cli.rs` — add `team-spawn` subcommand, parse `--config <yaml>` (~35 LoC delta)
- `src/lib.rs` — expose `team_spawn` module (~5 LoC)
- `src/sync.rs` — make `materialize_claude_settings` accept optional effort override for the non-team-spawn path (~15 LoC delta)
- `config/team-spec-schema.yaml` — new schema for the YAML config format (~30 LoC)
- `tests/team_spawn.rs` — new (~80 LoC): unit tests for `spawn_effort_team` (mock tmux), `write_teammate_project` roundtrip, CLI parsing

**Total LoC delta:** ~295 new, ~20 modified.

**Unknown requiring a labrat probe before implementation:** Does CC's `SendMessage` route via pane-name match alone (mailbox file lookup by agent name), or does it require a TeamCreate registry entry? If name-based, Option A is unnecessary; if registry-based, Option A or B is mandatory. This is the single blocking unknown — one `xask --spark codex` probe resolves it.

**Tier classification:** Runtime-tier (env set before process starts) with documented ceiling: if CC's mailbox requires TeamCreate registration, routing is Option A (racy) or Option B (fragile). Build/CI-tier enforcement requires a `cargo test` that mocks tmux + verifies env injection before claude exec.