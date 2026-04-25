# Map — Feedback Memory Layer (46 entries)

**Mission:** `map-feedback-memory-0425` · forensic map of all `feedback_*.md` (and adjacent `user_/project_/reference_`) memory files
**Date:** 2026-04-25
**Method:** `/xbgst | godspeed` · 1 round · 8 axes · 10 teammates · 7 of 7 moves accepted
**Companion:** yesterday's `map-flawless-why-2026-04-24.md` (4636e7c) — covered memory in §M as one of 8 axes; this map deepens the layer.
**audit_hash:** `1b058859f7dce651917dcdda488be124f1f48ca08e662277bea9d89da0ac6fd5`

---

## TL;DR

1. **The memory layer is a two-tier system masquerading as one.** Tier A (3–7 entries, hard) carries actual enforcement (settings.json deny array + scripts/xask + 1 test sentinel); Tier B (~40 entries, soft) is operator-habit protocol. Memory readers (humans + agents) treat all 46 as if Tier A — that gap is the core finding.
2. **Priority inversion** — counter-intuitive: docs-cited convention-laundered entries (Tier B, **31/46 = 67%**) are a HARDER remediation target than zero-ref orphans (Tier C, **12/46 = 26%**) because the docs-cited ones create false enforcement impressions, while orphans are invisible.
3. **CAT drift confirmed**: `feedback_agents_canonical_source.md` and `feedback_no_claude_md_overhead.md` are on disk but **NOT in `MEMORY.md` index**. Empirically (per labrat contra-indication probe): they don't fire. CC's loader is index-only, not directory-scan.
4. **C3d is the live Goodhart-decoy**: `feedback_no_schedule_suggestions.md` HARD-bans /schedule pitches but the `schedule` skill manifest contains "ALSO OFFER PROACTIVELY: end your reply with one-line offer". **Same trigger, no mechanical arbiter.** The ban is convention-laundered; the skill wins by salience every session.
5. **Code-surface ratio is 1:15** — only 3 of 46 memories self-enforce via compiled Rust (all OAuth cluster → `src/ask.rs`). 43 have zero code anchor. This sets the realistic remediation workload baseline.
6. **GUARD vs INERT distinction**: 2 OAuth memories enforce ABSENCE of removed features (tombstone-unsafe). The "structurally impossible" trigger condition that defines Tombstone candidacy must require defeating settings.json deny OR xask parse gate — not just "the feature is gone."

---

## §CAT — Catalog Completeness

> **Source:** M02-recongraph (cdx-revenger-memmap-r1) + M06-arch (ccs-simplifier-r1) + Phase 0 baseline
> **Linchpin:** `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/MEMORY.md` (44 indexed) vs on-disk (46 *.md files)

```
$ comm -23 <(ls memory/*.md | xargs -n1 basename | grep -v MEMORY.md | sort) \
           <(grep -oP '\(.*?\)' memory/MEMORY.md | tr -d '()' | sort)
feedback_agents_canonical_source.md     # has 9 inbound repo refs (CAT drift, NOT structurally orphaned)
feedback_no_claude_md_overhead.md       # zero inbound refs (true orphan + CAT drift)
```

**Index drift = 2 entries on disk but missing from MEMORY.md.** No phantom links (every indexed entry exists on disk).

**Empirical load behavior** (labrat contra-indication probe, M07 + M05 H3 upgrade):
- `feedback_no_claude_md_overhead.md` says "stop writing behavioral rules into CLAUDE.md"
- The injected project CLAUDE.md DOES contain behavioral rules
- The conflict never surfaces in session context → memory never fires
- **CC loader is index-only.** Files outside MEMORY.md are dark nodes for fresh sessions, regardless of disk presence.

Stale index summary (Phase 0 finding): `feedback_recompile_on_change.md` index blurb says "cargo build + cp" but file body says `make install` preferred. Content layer is current; index summary lags.

---

## §TAX — Taxonomy (Enforcement-Tier Primary)

> **Source:** M01-priorart (g-scout-priorart-r1) — research lens on prior art
> **Linchpin:** Letta Core/Archival/Tombstone vs Cursor/Aider domain-first

Two dominant prior-art taxonomies in agent-system memory:
- **Domain-first** (Cursor `.cursorrules`, Aider `CONVENTIONS.md`, ChatGPT memory): groups by codebase/topic mental model. Useful for finding rules, but groups same-domain rules with different enforcement strengths.
- **Enforcement-tier-first** (Letta/MemGPT Core/Archival, OpenAI Assistants v2 instructions): forced by architecture (Core always-injected vs Archival RAG-retrieved). Maps cleanly to xbreed because xbreed's 46 entries describe behavioral postures + tool-invocation rules, NOT codebase domains.

**Recommended primary axis** (with secondary domain tag):

| Tier | Definition | xbreed example |
|---|---|---|
| **Always-Active** | Loaded every session via index; trigger fires structurally | feedback_xask_flag_order.md |
| **Conditional** | Trigger condition can occur but firing requires reader habit | feedback_critic_hallucination.md |
| **Inert** | Trigger condition structurally impossible (settings.json deny / xask gate / removed code) | feedback_no_remember_plugin.md |
| **Tombstone** | Inert AND superseded — purely historical | (none currently — see GUARD distinction below) |
| **Convention-Laundered** | Looks enforced (memory says "hard rule") but no mechanical anchor; conflicts with another directive on same trigger | **feedback_no_schedule_suggestions.md** (C3d) |

The 5th tier (Convention-Laundered) emerged from cross-pollination between scout, critic, and reviewer. **No surveyed agent system has an explicit binary hard/soft tag for behavioral rules** — this is a novel framing for xbreed.

Domain buckets (secondary tag, for navigation):
- xask transport (flag_order, first_tool_call, yolo_routing, oauth_exclusive_code)
- godspeed posture (godspeed_repo_readonly, xbgst_keep_going, naming_prefix_target, no_hold_protocol)
- orchestration discipline (connector_every_round, scribe_per_round, commit_per_round, distiller_spawn_early, team_cleanup_on_shutdown, half_landed_routing_pattern)
- behavior (no_safety_theater, stop_asking_execute, no_schedule_suggestions, claude_dotfiles, prefer_rust_over_python)
- OAuth (user_oauth_exclusive, oauth_exclusive_code, oauth_not_api)
- tooling (no_obsidian_mcp, no_remember_plugin, no_schedulewakeup, reviewer_brief_propose_only, mutation_tester_cleanup, no_worktrees, no_auto_hooks, recompile_on_change)
- mailbox-internals (mailbox_cpu_layer_closed, mailbox_fd_cache_constraint, wsl2_ext4_faster_tmpfs)

---

## §TIER — Enforcement Distribution

> **Source:** M02-recongraph (cdx-revenger-memmap-r1) + M05-ach (cdx-critic-aspirational-r1) cross-model convergence
> **Linchpin:** `~/.claude/settings.json` deny array + `scripts/xask:37` flag-order + `tests/ask_with_loadout.rs` sentinel

Three enforcement tiers exist in this repo (per yesterday's `map-flawless-why` §S, applied to memory layer):

| Tier | Reach | xbreed memory examples | Count |
|---|---|---|---:|
| **Build/CI** | rustc + cargo test + verify-docs.sh | (none specifically — verify-docs.sh enforces routing, not memory directives) | 0 |
| **Runtime** | settings.json deny / scripts/xask / src/ask.rs | feedback_no_worktrees (deny: EnterWorktree, ExitWorktree, Bash(git worktree:*)) · feedback_no_schedulewakeup (deny: ScheduleWakeup) · feedback_godspeed_repo_readonly (deny: Edit(godspeed-mode/**)) · feedback_no_remember_plugin (enabledPlugins: false) · feedback_xask_flag_order (xask:37 flag loop) · feedback_yolo_routing (src/ask.rs argv) · feedback_oauth_exclusive_code (src/ask.rs key-path removed) | **~7** |
| **Protocol** | Memory directive only (operator/agent reads brief) | All other 39 entries | **~39** |

### Code-surface ratio (M02 corrected by distiller spoof-check)

Per primary-source `scratch/memory-crossref-graph.md`:
- **3 of 46 memories cite `src/` or `tests/`** — all OAuth cluster: `feedback_yolo_routing.md`, `feedback_oauth_exclusive_code.md`, `user_oauth_exclusive.md`
- **31 of 46 docs-only cited** — referenced in `docs/reports/` or `commands/` but no code anchor
- **5 of 46 zero-inbound orphans** (corrected from 12 — see §REF)
- Remaining 7 cite mixed surfaces (memory + code + docs)

**Code-surface : memory-surface ratio = ~1:15.** Only 6.5% of the memory layer reaches the highest enforcement tier (compiled Rust + tests).

### Two-hop transcription elevation

| Hop | Direction | Cost | Already done |
|---|---|---|---|
| Protocol → Runtime | settings.json deny / xask gate addition | Low (config edit) | Worktrees, ScheduleWakeup, godspeed-readonly, remember plugin |
| Runtime → Code | Compiled Rust + test sentinel | Higher (code change) | OAuth cluster only (3 entries) |

The remaining transcription gap is dominated by Protocol → Runtime opportunities for high-blast rules (e.g., `feedback_unified_tier_scheme.md` could extend `verify-docs.sh` to check all 13 role × model cells). See §TRANS.

---

## §LIVE — Empirical Activation State

> **Source:** M07-firetests (ccs-labrat-firetests-r1, 4 H/M/R probes) + M05-ach H3 contra-indication (cdx-critic-aspirational-r1)
> **Linchpin:** trigger-the-rule-and-observe approach + falsifiability probe (does conflict surface when expected?)

### 4 fire-test probes (2026-04-25)

| Probe | Memory | Trigger | Result | Verdict |
|---|---|---|---|---|
| **P1** | feedback_xask_flag_order.md | `scripts/xask gemini "test" -e high` | exit 1, stderr `Error: skill not found: high` | **LIVE (certain)** — hard runtime error |
| **P2** | feedback_no_schedulewakeup.md | `jq .permissions.deny ~/.claude/settings.json \| grep -i schedule` | `"ScheduleWakeup",` present in deny | **LIVE (strong)** — permissions-level |
| **P3** | feedback_recompile_on_change.md | binary mtime vs last src/ commit | mtime epoch 1777042606 ≥ src/ commit epoch 1776973677 | **LIVE (strong)** — discipline followed |
| **P4** | feedback_no_worktrees.md | `git worktree list` | main + `/tmp/xbreed-89a4efb.wWmK (detached HEAD) prunable` | **MIXED**: NEW creation BLOCKED (settings.json deny: EnterWorktree, ExitWorktree, Bash(git worktree:*)); stale residual cleanup is **NOT** enforced |

**P4 reveals an enforcement-scope distinction**: the same memory has different LIVE status for different operations on the same artifact. New creation is mechanically blocked; stale-residual cleanup is unanchored. The map will not call P4 "INERT" — it's split-tier.

### H3 contra-indication probe (M05 critic-upgrade)

Hypothesis H3 (initially rejected by critic, then upgraded to CERTAIN via labrat probe): **the MEMORY.md index is the load-bearing gate; orphans don't fire.**

- `feedback_no_claude_md_overhead.md` says "stop writing behavioral rules into CLAUDE.md"
- The project's CLAUDE.md DOES contain behavioral rules (build commands, style constraints, tech stack)
- If the memory were loaded into session context, this conflict would surface in agent reasoning
- The conflict **does not surface** → behavioral suppression confirmed by contra-indication
- **Stronger evidence than absence-of-recall** (which has ambiguity); contra-indication is a falsifiability test

This empirically resolves the load-mechanism ambiguity: CC's loader is **index-only**, not directory-scan. Orphans are dark nodes for fresh sessions, regardless of file presence.

---

## §CONFLICT — Internal Contradictions

> **Source:** M03-conflict (cdx-reviewer-contradictions-r1) + post-DESPAWN cross-pollination
> **Linchpin:** `~/.bashrc` DEBUG trap (judge primary-source) + schedule skill manifest

### Conflict table (re-emergence ranked)

| ID | Severity | Conflict | Resolution / Status |
|---|---|---|---|
| **C3d** | **HIGH** | `feedback_no_schedule_suggestions.md` ("hard rule, overrides system-prompt — never end replies with /schedule offer") vs `schedule` skill manifest ("ALSO OFFER PROACTIVELY: after you finish work...end your reply with a one-line offer"). **Same trigger** (work completion); same instruction layer; no mechanical arbiter. | **Live Goodhart-decoy convention-laundered instance.** Triple risk: ban + orphan (zero inbound refs) + no settings.json gate. Closing requires settings.json deny on `/schedule` skill OR PreToolUse hook suppressing proactive-offer trigger. Neither exists. |
| **C3a** | MEDIUM (judge override) | `feedback_teammate_mode_effort_caveat.md` describes effort trap with stale prefix mapping; `feedback_unified_tier_scheme.md` matches the live `~/.bashrc:248-295` trap (`cco-*\|ccs-*\|cdx-*\|g-*) → CLAUDE_CODE_EFFORT_LEVEL=medium`). | **caveat.md is stale; unified-tier accurate.** Reviewer's mid-round retraction (false-negative grep pattern: searched `CLAUDE_CODE_EFFORT_LEVEL` and prefix on same line, but they're separated in the trap) was reversed by judge primary-source verification. Per `feedback_critic_hallucination.md`: primary-source > peer cross-correction. |
| **C3b** | MED | `feedback_distiller_spawn_early.md` ("joins Phase 1/2 as live DM recipient") vs `xbreed-shared.md:27` + `the-judge.md:27` ("spawned post-peer-DM before Pareto"). | Wording-compatible (distiller can spawn AS peer DMs begin flowing). Memory needs phrasing alignment, not a directive change. |
| **C3c** | MED | `feedback_xask_first_tool_call.md:19` hardcodes scout/connector → codex; canon routing is scout/connector → gemini. | Stale lane map (memory predates 2026-04-18 codex-spark + gemini-high lock). |
| **C3e** | MED | `feedback_no_auto_hooks.md` bans Edit(settings.json) without explicit ask; no PreToolUse gate exists on settings.json edit. | Instruction-only; no mechanical guard. Operationally fine if all agents read this memory. |
| **C3f** | LOW | `feedback_no_obsidian_mcp.md` ban; obsidian-mcp not in settings.json deny array (or `enabledPlugins: false`). | Instruction-only. The MCP server is also not in `enabledMcpjsonServers` so the trigger surface is small. |

**Re-emergence ranking** (probability the conflict resurfaces in operations): **C3d > C3b > C3e > C3c = C3f**. C3d wins because it's same-trigger collision with no arbiter; C3b is wording but fires every distiller dispatch; C3e and below depend on rare conditions.

### Verified Tier A bans (no contradiction)

| Memory | Mechanical anchor |
|---|---|
| feedback_no_worktrees.md | settings.json deny: `EnterWorktree`, `ExitWorktree`, `Bash(git worktree:*)` |
| feedback_no_schedulewakeup.md | settings.json deny: `ScheduleWakeup` |
| feedback_godspeed_repo_readonly.md | settings.json deny: `Edit/Write(godspeed-mode/**)` |
| feedback_no_remember_plugin.md | settings.json: `"remember@claude-plugins-official": false` + gitignored |

The right bans WERE anchored. The wrong ones (C3d, C3e, C3f) weren't. The actionable gap is narrow and specific.

---

## §REF — Cross-Reference Graph

> **Source:** M02-recongraph (cdx-revenger-memmap-r1) — codex `-R -F` 300s timeout → fallback to in-session grep, marked `[xask dry]`
> **Linchpin:** `scratch/memory-crossref-graph.md` (revenger's primary artifact)

### Spoof correction

The proposal claimed 12 orphans. Distiller spot-check against the same revenger's own scratch artifact found 5. **Distiller filter caught the contradiction; primary-source correction applied.** This is the second time in two days the spot-check class fires on a revenger-generated artifact (yesterday: connector self-flagged hallucinated specifics on c725c2a; today: orphan-list overclaim).

### True orphan list (5)

Memories with **zero inbound refs** anywhere in the repo or `~/.claude/agents/`:

1. `feedback_claude_dotfiles.md` — Per-project `.claude/` commit pattern; no downstream consumer found
2. `feedback_no_claude_md_overhead.md` — CLAUDE.md ban (also CAT drift); H3 contra-indication confirmed it doesn't fire
3. `feedback_no_schedule_suggestions.md` — /schedule ban (also C3d HIGH conflict); active LIVE rule despite zero refs
4. `feedback_prefer_rust_over_python.md` — Rust default (mtime 2026-04-24, recent — no time to accumulate refs)
5. `feedback_stop_asking_execute.md` — execute-after-Pareto-consensus (mtime 2026-04-12, oldest)

**Orphan status is NOT age-correlated** — orphans span the full 2026-04-12 → 2026-04-24 creation window. "Wait for refs to accumulate" is invalid deferral.

### Top-cited memories (by inbound count)

| Memory | Inbound refs | Top sources |
|---|---:|---|
| reference_godspeed_skills.md | 35 | the-planner.md, xbreed-shared.md, multiple agent files |
| feedback_teammate_mode_effort_caveat.md | 29 | docs/reports/ × many |
| project_on_spawn_skill_dead_metadata.md | 22 | critic.md, the-planner.md, shared.md:42 |
| feedback_critic_hallucination.md | 17 | docs/reports/ across multiple missions |
| feedback_connector_gemini_high.md | 13 | shared.md:39 + multiple handoffs |
| user_trendsetter_principle.md | 12 | docs/handouts/ + reports |
| feedback_scribe_per_round.md | 11 | docs/reports/ × many |
| project_mailbox_fd_cache_constraint.md | 11 | docs/reports/ × bench |
| feedback_half_landed_routing_pattern.md | 10 | commands/xbgst.md:45, xbreed.md, xbt.md, xgs.md |
| feedback_xbgst_cdx_teammate.md | 10 | shared.md:281 |
| feedback_unified_tier_scheme.md | 9 | docs/reports/ + handoffs |
| feedback_agents_canonical_source.md | 9 | shared.md:309, verify-docs.sh:7, AGENTS.md:5 (CAT drift but heavily referenced) |

### Code-anchored memories (3 of 46)

| Memory | Code citations |
|---|---|
| feedback_yolo_routing.md | src/ask.rs:57, 451 + tests/ask_with_loadout.rs:192 |
| feedback_oauth_exclusive_code.md | src/ask.rs:11 |
| user_oauth_exclusive.md | src/ask.rs:11, 160 |

All three are OAuth-cluster — `src/ask.rs` is the single code-surface landing point for the entire memory corpus.

---

## §TRANS — Memory → Runtime Transcription Gap

> **Source:** M04-compose (g-connector-crossaxis-r1) + M05 critic priority-inversion + connector C4 EXTENSION (CODE-SURFACE 4th axis)
> **Linchpin:** `feedback_no_policy_hardening.md` — blast-radius gates promotion; only multiplicative-cost failures warrant Runtime/Code elevation

### TIER × LIVE composition matrix

| TIER × LIVE | Coupling | Decoupling-on-rewrite |
|---|---|---|
| **Protocol × INERT** | False-safety tax: cited as live in briefs but trigger structurally impossible; masks legible TRANS surface | Tombstone (delete or demote) IF settings.json deny / xask gate is the actual enforcement |
| **Protocol × LIVE** | TRANS gap: relies on brief injection; breaks silently when briefs drift | Promote high-blast rules to Runtime (settings.json) or Build/CI (verify-docs.sh) |
| **Runtime × LIVE** | Self-coupling: flag-order failure breaks ALL downstream routing at once | Already mitigated at scripts/xask:37 |
| **Code × LIVE** | Highest-trust: compiled enforcement, build-fails on drift | Model for remaining high-blast promotions |

### GUARD vs INERT distinction (connector C4 CORRECTION)

A memory can describe a removed feature in two structurally different ways:

- **INERT**: trigger condition unreachable; rule has no purpose anymore (e.g., a rule about a tool that's been removed AND has no negative-test coverage)
- **GUARD**: rule enforces the absence of a removed feature; tombstoning would remove the "this was intentionally deleted" signal that prevents re-introduction

Examples:
- `user_oauth_exclusive.md` + `feedback_oauth_exclusive_code.md` → **GUARD** (negative-space enforcement against re-adding API-key path; verified via `tests/ask_with_loadout.rs`)
- `feedback_no_remember_plugin.md` → **INERT** (settings.json + gitignore are the actual enforcement; memory adds redundant narrative)

**Tombstone-safe set is smaller than naive INERT classification suggests.** Cleanup must distinguish.

### Transcription candidates (informational, blast-radius gated)

| Memory | Current tier | Promotion target | Blast radius if violated | Cost |
|---|---|---|---|---|
| feedback_unified_tier_scheme.md | Protocol × LIVE | Build/CI: extend verify-docs.sh to all 13 role × model cells | Whole-table effort-tier corruption (c725c2a class) | Low (script extension) |
| feedback_xbgst_keep_going.md | Protocol × LIVE | Build/CI: include_str! content-sentinel for Round-2 clause in shared.md | Premature halt at R1 → unresolved frontiers | Low (test addition) |
| feedback_no_schedule_suggestions.md (C3d) | Convention-Laundered | Runtime: settings.json deny on /schedule skill OR PreToolUse hook | Every work-completion turn produces wrong-class suggestion | Low (config edit) |
| feedback_critic_hallucination.md | Protocol × LIVE | (no good promotion path) | Hallucinated content claims propagate | — (stays Protocol; structural test impractical) |

**Note**: All transcription proposals are **informational** per `feedback_no_policy_hardening.md`. The user decides — this map does not auto-harden.

### enforced_by frontmatter (3-deep prior-art gap, scout finding)

A novel mechanism would be:

```yaml
---
name: <memory>
enforced_by: settings.json#deny.ScheduleWakeup
# or: scripts/xask:37 / src/ask.rs:160 / tests/ask_with_loadout.rs:192
# or: protocol-only
---
```

With a **CI-gated validator** that cross-refs each `enforced_by` value against its claimed anchor:
- Settings.json claim → grep deny array
- Script claim → grep file:line
- Code claim → grep src/ + tests/
- Protocol-only → leave as-is

The validator MUST be in CI (same step as `cargo clippy && cargo test`) — a validator outside required CI is itself convention-laundered by the same recursive logic. **Three-deep prior-art gap**: no system surveyed has the tag, the validator, OR the CI-integration; xbreed would be first to combine all three.

(Informational. No frontmatter or validator added in this run.)

---

## §ARCH — Archive Candidates (informational)

> **Source:** M06-arch (ccs-simplifier-r1) + M05 critic priority-inversion
> **Linchpin:** `feedback_no_safety_theater.md` — informational sweep, no deletions executed

### Priority-inversion synthesis

| Tier | Population | Risk class | Remediation priority |
|---|---:|---|---|
| Tier A (code-anchored) | 3 (6.5%) | Lowest — already enforced at compile time | None |
| Tier B (docs-ref convention-laundered) | 31 (67%) | **HIGHEST** — wide readability creates false enforcement impression | Highest priority |
| Tier C (zero-ref orphan) | 12 (26%, was 5 by stricter ref-graph) | Urgent but lower systemic risk — invisible, no agent reads them as authoritative | Lower priority than Tier B |

Counter-intuitive: orphans LOOK more urgent (broken, invisible) but Tier B is the harder target because its discoverability actively masks the enforcement gap. **The 31 docs-ref entries are the Goodhart-decoy surface in production.**

### Archive candidates by category (no deletions executed)

| Memory | Category | Rationale | Risk-of-archive |
|---|---|---|---|
| feedback_agents_canonical_source.md | Stale-fact + Index-missing | templates/ deleted claim contradicted by f3882aa restore + ls present + 9 inbound refs | Low — fix index entry instead of archive |
| feedback_no_claude_md_overhead.md | Index-missing + true orphan + LIVE conflict | Not in MEMORY.md; conflicts with project CLAUDE.md; doesn't fire | **Medium — fix not delete** (rule is high-value if loaded) |
| feedback_no_remember_plugin.md | Banned-and-dead | settings.json `enabledPlugins: false` + gitignored — settings is SSoT | Low |
| feedback_no_schedulewakeup.md | Banned-and-dead | settings.json deny — but prevents proposal-framing in addition to tool block | Low-medium |
| feedback_no_obsidian_mcp.md | Operationally inert | Not in settings.json at all; trigger requires explicit settings edit | Low |
| feedback_wsl2_ext4_faster_tmpfs.md | Operationally inert | Mailbox CPU-layer closed (project_mailbox_cpu_layer_closed); narrow trigger | Low — factual note, harmless to retain |

### NOT archive candidates (clarification)

- **OAuth cluster** (`user_oauth_exclusive.md`, `feedback_oauth_exclusive_code.md`): GUARD class, NOT INERT. Tombstone-unsafe.
- **project_mailbox_fd_cache_constraint.md**: cited 11 times; architectural constraint still active (compact_events_sync at mailbox.rs:46). NOT a Tombstone candidate despite topic-narrow framing.
- **5 true orphans**: `feedback_claude_dotfiles`, `feedback_no_claude_md_overhead`, `feedback_no_schedule_suggestions`, `feedback_prefer_rust_over_python`, `feedback_stop_asking_execute`. Index-missing of #2, conflict-active of #3, recent of #4 — each has a different reason to keep alive (post-fix).

---

## §X — Synthesis: Why the Memory Layer "Works"

The xbreed memory layer functions because of asymmetric trust:

1. **Tier A (3–7 entries)** carries the load-bearing enforcement — settings.json deny + scripts/xask + src/ask.rs OAuth gate. These are unfalsifiable by reading the memory file alone; they're enforced by the binary or harness regardless of memory content.
2. **Tier B (~40 entries)** is operator-habit protocol — agents and humans read briefs, recall directives, and act accordingly. The memory layer is the **operator's externalized working set**, not a runtime surface.
3. **The MEMORY.md index is the load-bearing gate** for getting a memory into session context (H3 confirmed). Files outside the index are dark nodes — they exist on disk but never fire.

The system **works** because the right rules were anchored:
- High-blast rules (worktrees, ScheduleWakeup, godspeed-readonly, OAuth path, remember plugin, xask flag-order) ARE in settings.json or scripts/xask
- Low-blast rules (style preferences, posture directives) are habit-only — appropriate Protocol-tier scope

The system **fails silently** when:
- A new ban lands in MEMORY.md without a settings.json or hook anchor (C3d Goodhart-decoy)
- Two memories describe the same runtime artifact with conflicting content and reader pulls the stale one (C3a, partially mitigated because reader can verify against ~/.bashrc)
- A memory is written to disk without index entry (CAT drift; doesn't fire)
- A docs-cited Tier B rule contradicts another Tier B rule on the same trigger with no arbiter (Tier B convention-laundering)

**The map's load-bearing finding**: the right work to harden the memory layer is **not** archiving orphans (Tier C). It's **promoting high-blast Tier B rules to Runtime** (extend verify-docs.sh, add settings.json denies for confirmed conflicts, add CI-gated `enforced_by` validator if novel protocol surface is desired). The orphans are dead code — annoying but not load-bearing in either direction.

---

## §Z — Optimization Routes Surveyed (informational)

Per `feedback_no_policy_hardening.md` and `feedback_no_safety_theater.md`, listed for completeness; **no actions taken**, user decides:

| # | Route | Closes | Cost | Notes |
|---|---|---|---|---|
| (a) | Add `feedback_agents_canonical_source.md` + `feedback_no_claude_md_overhead.md` to MEMORY.md index | CAT drift; H3 firing for these 2 | Trivial (2 lines in MEMORY.md) | The 2nd is a behavioral rule worth firing; the 1st is stale and would need content fix first |
| (b) | Update `feedback_teammate_mode_effort_caveat.md` to match `~/.bashrc` trap (`cco-*\|ccs-*\|cdx-*\|g-*) → medium`) | C3a stale-content | Trivial | `feedback_unified_tier_scheme.md` already has the right map — could collapse caveat into a "see unified-tier" pointer |
| (c) | Update `feedback_xask_first_tool_call.md` lane map (scout/connector → gemini) | C3c stale-lane | Trivial | The actual lane map lives in shared.md L36-46 |
| (d) | settings.json deny for `/schedule` skill OR PreToolUse hook to suppress proactive-offer | C3d Goodhart-decoy | Low (config edit) | Highest-priority remediation — fires every work-completion turn |
| (e) | Extend `verify-docs.sh` from connector-only to all 13 role × model cells | TRANS gap on `feedback_unified_tier_scheme.md`; whole-table drift surface | Low (script extension) | Inherits the c725c2a-class threat model from yesterday's map §S |
| (f) | Add `enforced_by:` frontmatter + CI-gated validator | TRANS gap on docs-cited Tier B (67% of memory) | Medium (validator script + CI integration) | 3-deep prior-art gap; novel mechanism if pursued |
| (g) | Tombstone (informational): `feedback_no_remember_plugin`, `feedback_no_obsidian_mcp`, `feedback_wsl2_ext4_faster_tmpfs` | Tier B/C bloat | Trivial (delete files + remove from index) | Low risk — settings or topology already enforces |
| (h) | Update MEMORY.md summary for `feedback_recompile_on_change.md` to reflect `make install` superseding form | Stale index summary | Trivial | Phase 0 finding |

---

## §0 — What This Map Is NOT

- **Not a remediation plan.** Optimization routes (§Z) are informational; user decides actions.
- **Not a recommendation to harden.** Per `feedback_no_safety_theater.md`, system is calibrated for solo dev; pre-flight ceremony is anti-value.
- **Not a critique of memory accretion.** 46 entries across 6+ mission windows is the natural growth pattern; the asymmetric trust model (Tier A small, Tier B large) is appropriate engineering trade-off.
- **Not a final word on Tier B prioritization.** Specific Tier-B-to-Runtime promotions need their own blast-radius analysis per rule. This map names the surface; user picks targets.

---

## Appendix A — Round 1 trace

- **Team**: `map-feedback-memory-0425`, 10 members, ~14 min wall time
- **Phase 0**: `ccs-planner-r0` (sonnet · WWKD-loaded) — 10-milestone skeleton with M09 archive-only-Protocol gate
- **Phase 1 axes**: CAT/TAX/TIER/LIVE/CONFLICT/REF/TRANS/ARCH (8 axes)
- **Phase 2 specialists** (7 dispatched in parallel):
  - `g-scout-priorart-r1` (gemini medium) → M01-priorart
  - `cdx-revenger-memmap-r1` (codex -R -F, 300s timeout → fallback) → M02-recongraph
  - `cdx-reviewer-contradictions-r1` (codex -R, scoped) → M03-conflict
  - `g-connector-crossaxis-r1` (gemini high, 6k cap) → M04-compose
  - `cdx-critic-aspirational-r1` (heuer-planning + codex -R) → M05-ach
  - `ccs-simplifier-r1` (CC native) → M06-arch
  - `ccs-labrat-firetests-r1` (codex --spark, 4 H/M/R) → M07-firetests
- **Phase 3 distiller**: `ccs-distiller` (sonnet · in-session) — caught M02 12-orphan overclaim via primary-source spot-check; preserved C3a judge-override
- **Phase 4 scribe**: `ccs-scribe-r1` (sonnet · medium · filter-exempt) → round report at `docs/reports/map-feedback-memory-r1-2026-04-25.md`
- **Pareto verdict**: 7 of 7 ACCEPTED; integrated R2 inside R1 via post-DESPAWN cross-pollination
- **Spoof corrections**: M02 (12 orphans → 5), agents_canonical_source ("structurally unreachable" → "missing from index, has 9 live repo refs")
- **Judge override**: C3a MEDIUM (reversed reviewer's retraction-to-low based on labrat false-negative grep)

## Appendix B — Memory mtime distribution

(For §ARCH chronological context — orphan status NOT age-correlated)

| Date | Memory files (orphans flagged) |
|---|---|
| 2026-04-12 | feedback_claude_dotfiles ⓞ, feedback_distiller_spawn_early, feedback_stop_asking_execute ⓞ |
| 2026-04-13 | feedback_no_policy_hardening, feedback_commit_per_round, feedback_no_hold_protocol, feedback_xbgst_keep_going, feedback_connector_stall |
| 2026-04-14 | feedback_no_worktrees, feedback_no_safety_theater, user_oauth_exclusive |
| 2026-04-16 | feedback_no_obsidian_mcp, reference_obsidian_vault_paths, feedback_naming_prefix_target, feedback_no_remember_plugin, feedback_no_schedulewakeup, feedback_yolo_routing, feedback_xbgst_cdx_teammate, feedback_oauth_not_api, feedback_scribe_per_round, feedback_critic_hallucination, feedback_wsl2_ext4_faster_tmpfs, project_mailbox_fd_cache_constraint, project_mailbox_cpu_layer_closed |
| 2026-04-17 | feedback_teammate_mode_effort_caveat, feedback_connector_every_round, feedback_agents_canonical_source ⊘, feedback_godspeed_repo_readonly, reference_godspeed_skills, feedback_xask_first_tool_call |
| 2026-04-18 | feedback_team_cleanup_on_shutdown, user_trendsetter_principle, feedback_reviewer_brief_propose_only, reference_gemini_fanout_skill, feedback_connector_gemini_high, feedback_xask_flag_order, feedback_recompile_on_change |
| 2026-04-19 | feedback_oauth_exclusive_code, feedback_unified_tier_scheme, feedback_no_claude_md_overhead ⓞ⊘ |
| 2026-04-20 | project_on_spawn_skill_dead_metadata, feedback_half_landed_routing_pattern |
| 2026-04-23 | feedback_no_auto_hooks, feedback_mutation_tester_cleanup |
| 2026-04-24 | feedback_prefer_rust_over_python ⓞ, feedback_no_schedule_suggestions ⓞ |

Legend: ⓞ = zero-inbound orphan (5 total); ⊘ = missing from MEMORY.md index (2 total). C3d-active despite ⓞ status (no_schedule_suggestions has zero refs but is a HIGH-severity LIVE conflict).

---

*This document was produced by the system mapping its own memory layer. It does not propose changes — it makes the trust model legible so the operator can decide which Tier B rules deserve Runtime promotion and which orphans deserve archival.*
