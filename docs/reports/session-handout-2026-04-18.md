# Session Handout — 2026-04-18

> Prepared for next session. Captures what landed, what's pending, and what to do next.

## TL;DR

Six missions closed this session, four commits landed, three durable memories updated. Main deliverables: xask `-F/--full` escape hatch for the-revenger RECON (Path B), xask usage-format α content patch (closes drift), mailbox drain race fix (pre-existing bug), and three closure docs (Bun / xask-format / non-interactive leverage). Two unresolved decision points carry forward.

---

## Commits this session (newest first)

| SHA | Subject | Scope |
|---|---|---|
| `7b17e70` | `docs(noninteractive-leverage): closure — Tier-1 ship α, Tier-3 fix prereqs, Tier-2 blocked` | docs/reports/noninteractive-leverage-0418-closure-2026-04-18.md |
| `f105a48` | `fix(mailbox): drain_target rename no longer matches compact_ready prefix + non-blocking panic-job send` | src/mailbox.rs (2 fixes applied by gemini during codex-swarm-mini mission, kept after verify-loop) |
| `4339240` | `feat(ask): Path B — keep -R mini migration + add -F/--full escape hatch` | src/cli.rs, src/main.rs, src/ask.rs, scripts/xask, tests/ask_with_loadout.rs, commands/references/xbreed-shared.md, AGENTS.md |
| `89a4efb` | `docs(xask): α content patch — close drift at L3/L8/L86 + protocol synopsis + mini-lane doc sync` | scripts/xask, docs/xask-protocol.md |
| `839b5cc` | `docs(xask-usage-format): closure — Variant B REJECTED, α/β paths named` | docs/reports/xask-usage-format-0418-closure-2026-04-18.md |
| `ff8a96e` | `docs(bun-fit-audit): closure — 4-axis REJECT, guard.rs bypass + trendsetter disqualify` | docs/reports/bun-fit-audit-0418-closure-2026-04-18.md |

Verify loop after every src change: `cargo clippy && cargo test && cargo fmt --check` — all green at `4339240` (72 ask tests + 11 ask_with_loadout integration tests + 7 guard + 10 precheck + 10 mailbox + 1 sync + 2 timeout_reap = 103 tests passing).

Binary installed at `~/.local/bin/xbreed` per `feedback_recompile_on_change`.

---

## What changed in the codebase (functional)

### `src/ask.rs` — 4 codex lanes (was 3)

```rust
// Retired: CODEX_DEFAULT_MODEL (was ambiguous after mini migration)
pub const CODEX_MINI_MODEL: &str = "gpt-5.4-mini";   // default + review lanes
pub const CODEX_FULL_MODEL: &str = "gpt-5.4";        // NEW: -R -F escape hatch
pub const CODEX_SPARK_MODEL: &str = "gpt-5.3-codex-spark"; // spark lane unchanged

// build_codex_ask_with_loadout signature added `full: bool` parameter
// dispatch() signature added `full: bool` parameter
// Review branch logic: review && full → CODEX_FULL_MODEL; else → CODEX_MINI_MODEL
```

Lane map (user directive 2026-04-18):
- `xask --spk codex` → `gpt-5.3-codex-spark` + reasoning=low (labrat probes)
- `xask codex` → `gpt-5.4-mini` + fast_mode + reasoning=high (default)
- `xask -R codex` → `gpt-5.4-mini` + fast_mode + xhigh-from-config (review default — reviewer/critic/sentinel)
- `xask -R -F codex` → **`gpt-5.4` full** + fast_mode + xhigh-from-config (ESCAPE HATCH — the-revenger RECON, 1.05M context)

### `scripts/xask` — `-F/--full` flag + drift-closed comments

Added `-F/--full` flag parsing, FULL_FLAGS forwarding, updated Usage/Flags/lanes comments, updated error-path echo. Path α drift fix: added `[-o <file>]` and `[--json]` to compact-form usage, fixed `<boundary>` → `<scope>` placeholder.

### `src/mailbox.rs` — two fixes

1. `collect_compact_sidecars`: drain_target renamed to `{stem}.drained_by.{pid}.{suffix}` so it no longer matches `compact_ready.` prefix (closes infinite rename loop under concurrent drain).
2. `__send_panicking_job`: try_send + yield loop instead of blocking send (unblocks M4 poison test).

### Tests — 4 new

- `codex_ask_review_default_uses_mini_model` (renamed from `codex_ask_review_lane_uses_full_model`)
- `codex_ask_review_full_flag_uses_full_model` (NEW — -R -F verifies CODEX_FULL_MODEL)
- `codex_ask_full_without_review_is_noop` (NEW — safety: --full alone stays on mini)
- `ask_codex_review_full_flag_routes_to_full_model` (NEW integration test — 3 cases: -R --full → full, -R → mini, --full → mini)

---

## Durable memory updates

| Memory file | Change |
|---|---|
| `feedback_unified_tier_scheme.md` | 2026-04-18 revision supersedes 2026-04-17 `-R → full gpt-5.4`. Now documents 4 codex lanes including `-R -F` escape hatch. CODEX_FULL_MODEL / CODEX_MINI_MODEL split described. |
| `feedback_connector_gemini_high.md` | Renamed "LOCKED no codex fallback" language → explicit "gemini-high primary, sonnet in-session fallback" naming. Applied to connector.md, the-judge.md, xbreed-shared.md. |
| `feedback_xask_flag_order.md` | **NEW** — flags-before-positional discipline, cites scripts/xask:37 strict `while $1 == -*` loop. Incident history: bun-fit AM, codex-swarm-mini PM, non-interactive labrat Probe 2 — 3 occurrences in one day. |
| `reference_gemini_fanout_skill.md` | Scope narrowed to mutation-tester only. Labrat swarms migrated to codex-spark with same "Orchestrate 10 parallel probes" pattern. |
| `MEMORY.md` index | Updated entries for the above; added xask_flag_order entry. |

---

## Other doc updates

- `~/.claude/agents/the-judge.md` — sub-role table updated (reviewer mentions -F, the-revenger defaults to -R -F for RECON, Codex labrat swarm replaces Gemini labrat swarm)
- `~/.claude/agents/labrat.md` — Codex-spark is sole labrat channel (Gemini labrat paths removed)
- `~/.claude/agents/connector.md` — Delegation bullet names sonnet in-session fallback explicitly
- `~/.claude/agents/simplifier.md` — Deletion-safety uses Codex labrat swarm
- `commands/references/xbreed-shared.md` — Layer-1 gate strings + axis-profile routing table updated for -F escape hatch

---

## Pending decisions carried forward

### 1. `codex-swarm-mini-0418` D1 — user chose Path B, implemented. CLOSED.

### 2. `noninteractive-leverage-0418` Tier-3 fixes (**blocks Tier-1 stdin-chain**)

**guard.rs stdin blind spot** — `evaluate_from_json` inspects Bash args only, not piped stdin content. A `codex --json | gemini -` pipeline bypasses policy.yaml `deny_bash_patterns`. Must be addressed before any stdin-chain leverage ships. Options:
  - (a) add stdin content evaluation to `evaluate_from_json`
  - (b) document stdin as unguarded channel and scope stdin-chain usage to trusted content only

**D2 grandchild-orphan leak** (carry-over from codex-swarm-mini-0418) — `execute_with_timeout::child.kill` only kills direct child; grandchildren reparent to PID 1. Fix: `Command::process_group(0)` before spawn + `libc::kill(-(pgid as i32), SIGKILL)` on timeout. Hardens non-swarm timeouts as a side-effect.

### 3. `noninteractive-leverage-0418` Tier-1 `-o` orchestration primitive

Planner M02b: add a reader in `dispatch()` that optionally captures `-o FILE` content post-completion for downstream routing. Zero adaptation — xbreed consumer of already-emitted codex output. Low-risk α ship.

### 4. `noninteractive-leverage-0418` Tier-2 reassessment

- **Gemini `--acp` daemon (M06)**: concurrency behavior undocumented. Separate audit round needed before adoption.
- **Stream-json stdout-passthrough (M02a)**: safe to ship as stdout-only consumer; DO NOT route to mailbox during execution (fd-cache constraint).
- **CONFLICT-2 (codex `features.multi_agent` orchestration)**: labrat M02 from prior mission proved shell-tool nesting works (14.8s wall). Whether `features.multi_agent` adds xbreed-driveable orchestration remains open. Dedicated labrat probe if user wants to exhaust the surface.

### 5. `xask-usage-format-0418` Path β (deferred)

Adopt Mutation 7 refined (`[options]` synopsis + tabular Options block + `[type]` annotations) — **contingent on `--help` handler landing** since β's "Run 'xask --help'" line would be a broken promise without it. Sequencing: α → β → --help, not either/or.

---

## Untracked files — next-session triage

```
mcp_server
mcp_server.ts
docs/reports/godspeed-routing-swarm-analysis.md
.tmp/
data/bench-phase-c.paths
docs/reports/shim-critique-a3-negcoverage-2026-04-17.md
test.sh
test2.sh
```

- **`mcp_server` + `mcp_server.ts`** — scope-creep from gemini's yolo-mode over-reach during codex-swarm-mini-0418. Out of scope. Likely should be nuked or moved to a scratch dir.
- **`docs/reports/godspeed-routing-swarm-analysis.md`** — created by gemini during the same over-reach. Review before decide (may have useful content).
- **`.tmp/`**, **`data/bench-phase-c.paths`**, **`shim-critique-a3-negcoverage-2026-04-17.md`**, **`test.sh`**, **`test2.sh`** — pre-existing untracked artifacts from earlier sessions. Needs user triage (likely gitignore or nuke).

---

## Protocol-tier discipline reminders

**Flag-order**: xask flags MUST precede positionals. Third-occurrence bug today — memory entry `feedback_xask_flag_order.md` is now in MEMORY.md index for auto-load. Pre-dispatch self-check: grep briefs for `codex "` / `gemini "` and reorder any flag tokens after.

**Yolo-mode scope**: gemini `--approval-mode yolo` + codex `--sandbox danger-full-access` intentionally give write access. Non-reviewer lanes (connector, scout, etc.) are NOT scoped to propose-only by default. When writing briefs for yolo-enabled teammates on analysis-only missions, add explicit "do NOT apply edits" clause.

**Trendsetter principle**: external CLI capabilities not in xbreed's runtime as-is = disqualifying. But adding xbreed-side CONSUMERS of native CLI output is NOT adaptation — it's xbreed-layer extension. Distinguishes "wire through a native feature" (OK) from "patch around a missing feature" (REJECT).

---

## Recommended next-session sequence

1. **Triage untracked files** — decide fate of `mcp_server*` + pre-existing test.sh etc.
2. **Fix Tier-3** (blocks Tier-1 stdin-chain):
   - guard.rs stdin blind spot
   - D2 grandchild-orphan in execute_with_timeout
3. **Ship α Tier-1**:
   - M02b `-o FILE` reader in dispatch() for post-completion orchestration consumer
   - (Optional) reuse shell-tool nested spark swarm from codex-swarm-mini-0418 (already empirically viable)
4. **Then β evaluation**:
   - xask usage-format `[options]` + Options block + `--help` handler (bundle — neither ships alone)
   - Stream-json stdout-passthrough consumer
5. **Defer / escalate**:
   - Gemini `--acp` daemon audit (separate round)
   - CONFLICT-2 `features.multi_agent` xbreed-reachability (labrat probe if desired)

---

## Reference index

**Session closure docs** (all in `docs/reports/`):
- `honcho-reaudit-closure-2026-04-18.md` (closed pre-session)
- `bun-fit-audit-0418-closure-2026-04-18.md` (this session — Bun REJECT)
- `xask-usage-format-0418-closure-2026-04-18.md` (this session — Variant B REJECT, α/β named)
- `codex-swarm-mini-0418-closure-2026-04-18.md` — **not written this session** (the mission was halted by user pivot; decision deferred then resolved implicitly via Path B commit `4339240`. A retroactive closure doc would be cleanup-debt worth considering next session.)
- `noninteractive-leverage-0418-closure-2026-04-18.md` (this session — Tier-1/2/3 classification)

**Planner artifacts**:
- `docs/reports/plan-noninteractive-leverage-0418.md` (6-milestone plan)

**Memory index**: `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/MEMORY.md`
