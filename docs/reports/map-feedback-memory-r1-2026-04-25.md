# Round 1 Report — map-feedback-memory-0425

**Mission:** forensic map of all 46 feedback_*.md memory entries (and adjacent user_/project_/reference_)
**Date:** 2026-04-25
**Companion artifact:** `docs/reports/map-feedback-memory-2026-04-25.md` (the map itself)
**Round wall time:** ~14 min
**Pareto verdict:** 7 of 7 ACCEPTED
**audit_hash:** `1b058859f7dce651917dcdda488be124f1f48ca08e662277bea9d89da0ac6fd5`

> **Scribe note:** `ccs-scribe-r1` was dispatched concurrent with Pareto scoring per `feedback_scribe_per_round.md`, but stalled in tmux pane %33 (3m 37s "Finagling…" with no progress and a queued user message that did not unstick it). Pane force-killed. Judge wrote this report directly to preserve the audit trail.

---

## EVIDENCE AUDIT

```
EVIDENCE AUDIT: 7 moves with evidence, 0 moves without, 0 dropped, 1 spoof_flagged
```

M02-recongraph carries a partial spoof_flag — proposal claimed 12 orphans; revenger's own primary artifact `scratch/memory-crossref-graph.md` showed 5. Distiller spot-check caught the contradiction; corrected list applied. Move stands; flag is provenance trace, not retraction.

---

## Round overview

| Field | Value |
|---|---|
| Team | `map-feedback-memory-0425` (10 members) |
| Axes | CAT · TAX · TIER · LIVE · CONFLICT · REF · TRANS · ARCH (8) |
| Specialists dispatched | 7 (parallel Phase 2) + planner (Phase 0) + distiller + scribe |
| Wall time | ~14 minutes |
| xask routing | gemini-medium (scout) / gemini-high cap-6k (connector) / codex -R (reviewer + critic + heuer-Layer-0) / codex -R -F (revenger, **300s timeout → in-session grep fallback**) / codex --spark (labrat, 4 H/M/R probes) |
| audit_hash | `1b058859f7dce651917dcdda488be124f1f48ca08e662277bea9d89da0ac6fd5` |

### Per-teammate dispatch table

| Name | Role | Lane | Axis output | Confidence |
|---|---|---|---|---|
| ccs-planner-r0 | planner | CC native + Layer-0 wwkd | Phase 0 skeleton (10 milestones) | — |
| g-scout-priorart-r1 | scout | gemini medium | M01-priorart (TAX) | MED |
| cdx-revenger-memmap-r1 | the-revenger | codex -R -F (timed out → fallback) | M02-recongraph (REF + CAT) | MED (spoof_flagged) |
| cdx-reviewer-contradictions-r1 | reviewer | codex -R (-s scoped) | M03-conflict (CONFLICT) | HIGH |
| g-connector-crossaxis-r1 | connector | gemini high (6k cap) | M04-compose (TRANS) | MED |
| cdx-critic-aspirational-r1 | critic | heuer Layer 0 + codex -R | M05-ach (LIVE/ARCH) | HIGH (cross-prefix) |
| ccs-simplifier-r1 | simplifier | CC native | M06-arch (CAT/ARCH) | HIGH |
| ccs-labrat-firetests-r1 | labrat | codex --spark (4 probes) | M07-firetests (LIVE) | HIGH |
| ccs-distiller | distiller | in-session | SYNTHESIS_READY | — |
| ccs-scribe-r1 | scribe | (stalled — pane killed) | — | — |

---

## Per-move proposals

### M01-priorart · g-scout-priorart-r1

- **axis:** TAX
- **claim:** xbreed memory layout is structurally closer to Letta's tiered Core/Archival split than Cursor/Aider domain-primary schema — enforcement-tier is the operationally primary axis, domain secondary. 5th tier "convention-laundered" emerged from cross-pollination (live instance: C3d).
- **confidence:** MED — single-model (gemini), research-only, no cross-model corroboration on taxonomy specifics.
- **rejected alt:** pure 7-bucket domain taxonomy — would conflate `flag_order` (tooling) with `first_tool_call` (orchestration posture), masking enforcement-tier difference.
- **evidence:** `none — research axis_family`

### M02-recongraph · cdx-revenger-memmap-r1

- **axis:** REF + CAT
- **claim:** 43 of 46 memories have zero code-surface (src/tests/) inbound refs; only 3 cite `src/ask.rs` (all OAuth-cluster). MEMORY.md index missing 2 on-disk files (CAT drift). True orphan count = **5** (corrected from proposal's 12 via primary-source spot-check).
- **confidence:** MED — SPOOF_FLAGGED. Revenger's 12-orphan summary contradicted own scratch artifact (`scratch/memory-crossref-graph.md` shows 5 zero-inbound entries). Distiller corrected; M02 stands with corrected content.
- **rejected alt:** semantic grep (key-noun extraction) — false positives (e.g. "prefer rust" matches any Rust discussion).
- **evidence:** `none — reverse-engineering axis_family. Anchor: local grep, codex CLI timed out 300s (BLOCKED), [xask dry] provenance.`

### M03-conflict · cdx-reviewer-contradictions-r1

- **axis:** CONFLICT
- **claim:** 6 internal contradictions identified. C3d (HIGH) is the sharpest — `feedback_no_schedule_suggestions.md` "overrides system-prompt" hard-bans /schedule offers but the `schedule` skill manifest contains "ALSO OFFER PROACTIVELY: end your reply with one-line offer". Same trigger (work completion), no mechanical arbiter. Live convention-laundered Goodhart-decoy instance.
- **confidence:** HIGH — C3d literal-substring confirmed against skill manifest; C3a primary-source verified by judge against `~/.bashrc`; settings.json deny entries grep-confirmed.
- **rejected alt:** `feedback_no_auto_hooks.md` vs `feedback_team_cleanup_on_shutdown.md` — TeamDelete is lifecycle call, not settings.json hook; scoping disjoint, no contradiction.
- **evidence:** `xask -R codex raw quotes; settings.json grep verification; primary-source ~/.bashrc read by judge.`

### M04-compose · g-connector-crossaxis-r1

- **axis:** TRANS
- **claim:** Protocol×INERT memories impose false-governance tax — cited as live in briefs but enforcement-inert. Shrinking INERT enlarges TRANS signal (axes are coupled, not independent). TRANS estimate ~76% is a LOWER bound — dark-node orphans can be Protocol×LIVE (read directly) without brief-injection. Plus addenda: GUARD vs INERT distinction (oauth memories = GUARD, tombstone-unsafe); CODE-SURFACE as 4th axis (Protocol→Runtime→Code two-hop elevation).
- **confidence:** MED — single-model (gemini), distribution estimates analytic and unanchored by labrat probe.
- **rejected alt:** treat all 35 Protocol×LIVE as equal-priority TRANS candidates — blast radius gates promotion per `feedback_no_policy_hardening.md`.
- **evidence:** `none — cross-axis axis_family.`

### M05-ach · cdx-critic-aspirational-r1

- **axis:** LIVE / ARCH (cross-cutting)
- **claim:** Memory layer is two-tier system masquerading as one — Tier A (~7 entries, hard) vs Tier B (~40 entries, habit-only). H3 upgraded REJECTED→CERTAIN per labrat contra-indication probe. C3d is canonical live Goodhart-decoy. Priority inversion: Tier B (67% docs-cited convention-laundered) is HARDER remediation target than Tier C (26% orphans) because docs-citation creates false enforcement impression.
- **confidence:** HIGH — cross-model convergence (ccs+cdx+g on Tier A/B split); H3 empirically confirmed by M07; C3d confirmed by M03 literal check.
- **rejected alt:** H3 disconfirmed-because-system-functions — REVERSED post-probe; orphans don't load AND system functions because Tier A does the work; both true.
- **evidence:** `none — adversarial-design axis_family.`

### M06-arch · ccs-simplifier-r1

- **axis:** CAT / ARCH
- **claim:** 2 of 46 disk-resident memory files absent from MEMORY.md injection index (confirmed via `comm -23` diff). `feedback_no_claude_md_overhead.md` = true zero-inbound orphan. `feedback_agents_canonical_source.md` = index-missing but has 9 live repo refs (NOT structurally unreachable — corrected from proposal). 4 additional archive candidates settings.json-enforced or narrow-trigger.
- **confidence:** HIGH for 2-orphan detection (grep verified); MED for archive urgency on agents_canonical_source (has live refs).
- **rejected alt:** `feedback_no_schedulewakeup.md` fully archived — kept because it prevents proposal-framing in addition to tool block.
- **evidence:** `comm -23 diff verifying 2 orphans; ls templates/agents/ shows files present; settings.json grep confirms deny entries.`
- **CORRECTION applied:** agents_canonical_source has 9 inbound refs — `commands/references/xbreed-shared.md:309`, `scripts/verify-docs.sh:7`, `AGENTS.md:5`. "Structurally unreachable" reframed as "missing from MEMORY.md injection index, but live-referenced in repo."

### M07-firetests · ccs-labrat-firetests-r1

- **axis:** LIVE
- **claim:** 3 of 4 probed memories produce hard runtime effects; P4 (no-worktrees) reveals enforcement-scope distinction — NEW worktree creation IS blocked via settings.json deny; stale `/tmp/xbreed-89a4efb.wWmK` residual is NOT cleaned up. Plus contra-indication probe confirming H3 (orphan memories don't fire in fresh sessions).
- **confidence:** HIGH — 4 empirical probes, all primary-source verified.
- **rejected alt:** all 4 memories produce hard technical guardrails — disproven by P4's enforcement-scope distinction.
- **evidence:** 4 H/M/R triples:
  - **P1**: `scripts/xask gemini "test" -e high` → `exit 1, Error: skill not found: high` — LIVE
  - **P2**: `jq .permissions.deny ~/.claude/settings.json` → `"ScheduleWakeup",` present — LIVE
  - **P3**: binary mtime epoch 1777042606 ≥ src/ commit epoch 1776973677 — LIVE
  - **P4**: `git worktree list` → main + `/tmp/xbreed-89a4efb.wWmK (detached HEAD) prunable` — MIXED (creation blocked / cleanup not)

---

## Cross-model CONFLICTS

| ID | Severity | Description | Judge resolution |
|---|---|---|---|
| **C3d** | **HIGH** | `feedback_no_schedule_suggestions.md` ("overrides system-prompt") vs `schedule` skill manifest ("ALSO OFFER PROACTIVELY: end your reply with one-line offer"). Same trigger (work completion); same instruction layer; no mechanical arbiter. **Triple risk: ban + orphan + no settings.json gate.** | Live Goodhart-decoy. Closing requires settings.json deny on `/schedule` skill OR PreToolUse hook. Neither exists. **Highest-priority remediation** (informational only). |
| **C3a** | MEDIUM | `feedback_teammate_mode_effort_caveat.md` describes `~/.bashrc` effort trap with stale prefix mapping; `feedback_unified_tier_scheme.md` matches live trap. | **caveat.md is stale; unified-tier accurate.** Reviewer's mid-round retraction (false-negative grep pattern: searched `CLAUDE_CODE_EFFORT_LEVEL` and prefix on same line, but the trap separates them) was REVERSED by judge primary-source verification of `~/.bashrc:248-295`. Per `feedback_critic_hallucination.md`: primary-source > peer cross-correction. |
| **C3b** | MED | `feedback_distiller_spawn_early.md` "Phase 1/2 live DM" vs `xbreed-shared.md:27` "post-peer-DM spawn". | Wording-compatible (distiller can spawn AS peer DMs begin). Memory needs phrasing alignment. |
| **C3c** | MED | `feedback_xask_first_tool_call.md:19` hardcodes scout/connector → codex; canon = gemini. | Stale lane map; predates 2026-04-18 codex-spark / gemini-high lock. |
| **C3e** | MED | `feedback_no_auto_hooks.md` ban; no PreToolUse gate on Edit(settings.json). | Instruction-only gap. |
| **C3f** | LOW | `feedback_no_obsidian_mcp.md` not in deny array. | Instruction-only; trigger surface small (MCP not in `enabledMcpjsonServers`). |

**Re-emergence ranking** (probability the conflict resurfaces in operations): **C3d > C3b > C3e > C3c = C3f**.

---

## Pareto verdict per move

| move_id | Verdict | Rationale |
|---|---|---|
| M01-priorart | ACCEPT | Adds capacity-agnostic taxonomy framing; Letta lineage; 5th convention-laundered tier emerged from cross-pollination. No axis regressed. |
| M02-recongraph | ACCEPT (spoof_flagged) | Adds cross-ref graph + CAT drift; corrected via distiller spot-check (12→5 orphans). Provenance flag, not retraction. |
| M03-conflict | ACCEPT | 6 conflicts surfaced; C3d HIGH live Goodhart-decoy named. |
| M04-compose | ACCEPT | TRANS gap quantified (~76% Protocol×LIVE lower-bound); GUARD vs INERT distinction; CODE-SURFACE 4th axis. |
| M05-ach | ACCEPT | Tier A/B split; H3 upgrade; priority inversion identified. Cross-prefix confidence convergence. |
| M06-arch | ACCEPT | 2 CAT drift confirmed; 6 archive candidates with categories + risk. Self-correction on agents_canonical_source. |
| M07-firetests | ACCEPT | 4 empirical probes; enforcement-scope distinction on P4; H3 contra-indication. |

**7 of 7 ACCEPTED.** Round-2 materiality check: every named axis moved off baseline; post-DESPAWN cross-pollination (5+ amendment cycles, including H3 reversal, GUARD/INERT distinction, CODE-SURFACE 4th axis, C3d Goodhart consolidation, judge primary-source override on C3a) functioned as integrated R2 inside R1. **Frontier called stable.**

---

## Spoof flags

**M02-recongraph (partial)** — Revenger's proposal claimed 12 orphans. Distiller spot-check against `scratch/memory-crossref-graph.md` (revenger's OWN scratch artifact) found 5 true zero-inbound entries. 7 falsely-listed entries had verified inbound refs:

| Falsely listed orphan | Actual inbound refs |
|---|---:|
| naming_prefix_target | 8 (the-judge.md:42,45; shared.md:142) |
| distiller_spawn_early | 1 (honcho-stress-0418) |
| half_landed_routing_pattern | 10 (commands/xbgst.md:45 etc.) |
| no_schedulewakeup | 1 (map-flawless-why-2026-04-24.md) |
| xbgst_cdx_teammate | 10 (shared.md:281 etc.) |
| on_spawn_skill_dead_metadata | 22 (critic.md, the-planner.md, shared.md:42) |
| obsidian_vault_paths | 1 (honcho-integration-data-walk) |

Distiller corrected via primary-source spot-check (yesterday's protocol fired correctly today). True orphan list (5): `feedback_claude_dotfiles.md`, `feedback_no_claude_md_overhead.md`, `feedback_no_schedule_suggestions.md`, `feedback_prefer_rust_over_python.md`, `feedback_stop_asking_execute.md`.

---

## Optimization routes surveyed (informational, no actions taken)

Per `feedback_no_policy_hardening.md` and `feedback_no_safety_theater.md`:

| # | Route | Closes |
|---|---|---|
| (a) | Add `feedback_agents_canonical_source.md` and `feedback_no_claude_md_overhead.md` to MEMORY.md index | CAT drift; H3 firing for these 2 |
| (b) | Update `feedback_teammate_mode_effort_caveat.md` to match live `~/.bashrc` trap | C3a stale-content |
| (c) | Update `feedback_xask_first_tool_call.md` lane map (scout/connector → gemini) | C3c stale-lane |
| (d) | settings.json deny for `/schedule` skill OR PreToolUse hook to suppress proactive-offer | C3d Goodhart-decoy |
| (e) | Extend `verify-docs.sh` from connector-only to all 13 role × model cells | TRANS gap on `feedback_unified_tier_scheme.md` |
| (f) | Add `enforced_by:` frontmatter + CI-gated validator | Tier B docs-cited convention-laundering (67% surface) — 3-deep prior-art gap |
| (g) | Tombstone (informational): `feedback_no_remember_plugin`, `feedback_no_obsidian_mcp`, `feedback_wsl2_ext4_faster_tmpfs` | Tier B/C bloat |
| (h) | Update MEMORY.md summary for `feedback_recompile_on_change.md` (`make install` superseded form) | Stale index summary |

---

## Coverage gap

`cdx-revenger-memmap-r1` codex `-R -F` 300s timeout → fallback to in-session grep, `[xask dry]` provenance. Crossref data derives from `scratch/memory-crossref-graph.md` (revenger's primary artifact). Distiller verified primary-source for spoof-check. `cdx-sentinel-attacksurface-r1` was NOT dispatched in this round (out of scope; mission is documentation, not attack-surface).

`ccs-scribe-r1` stalled in tmux pane (3m 37s "Finagling…" with no progress; queued user message did not unstick). Pane killed; judge wrote this report directly.

---

## Composition findings (cross-axis convergence)

| Finding | Source teammates |
|---|---|
| Tier A (3/46, 6.5%) — code-anchored: yolo_routing, oauth_exclusive_code, user_oauth_exclusive | revenger + critic + simplifier (3-way) |
| Tier B (31/46, 67%) — docs-ref convention-laundered (HIGH risk) | critic priority-inversion |
| Tier C (12/46, 26% by docs-graph; 5/46 by zero-inbound) — orphan/stranded (lower systemic risk) | revenger + simplifier; distiller-corrected from 12 to 5 by stricter ref-graph |
| **Priority inversion**: Tier B > Tier C in remediation priority | critic + revenger |
| **GUARD vs INERT** distinction: oauth memories enforce ABSENCE of removed feature (tombstone-unsafe) | connector C4 CORRECTION |
| **CODE-SURFACE 4th axis**: Protocol→Runtime→Code two-hop elevation | connector C4 EXTENSION |
| **Dark-node axis (PRESENCE×INDEX)**: orphans CAN be Protocol×LIVE (agents read directly) but never brief-injected; TRANS estimate ~76% is LOWER bound | connector + scout |

---

## audit_hash + SOURCE_MAP

**audit_hash**: `1b058859f7dce651917dcdda488be124f1f48ca08e662277bea9d89da0ac6fd5` (distiller-attested; serialization underspec — see yesterday's `map-flawless-why-2026-04-24.md` §E for the meta-finding)

**SOURCE_MAP** (verified via team config independent path; cryptographic hash recompute deferred per yesterday's serialization-underspec precedent):

| move_id | source_prefix | proposer (verified in team config) |
|---|---|---|
| M01-priorart | g- | g-scout-priorart-r1 |
| M02-recongraph | cdx- | cdx-revenger-memmap-r1 |
| M03-conflict | cdx- | cdx-reviewer-contradictions-r1 |
| M04-compose | g- | g-connector-crossaxis-r1 |
| M05-ach | cdx- | cdx-critic-aspirational-r1 |
| M06-arch | ccs- | ccs-simplifier-r1 |
| M07-firetests | ccs- | ccs-labrat-firetests-r1 |

Cross-prefix confidence convergence at HIGH on M03 (judge primary-source) + M05 (ccs+cdx+g) + M06 (independent grep) + M07 (empirical primary-source). Same-prefix-only correctly capped at MED on M01/M02/M04.

---

## Commit delta

This round produces two file additions (no source code changed):

- `docs/reports/map-feedback-memory-2026-04-25.md` (the map — 8 sections + appendix)
- `docs/reports/map-feedback-memory-r1-2026-04-25.md` (this report)

`cargo clippy && cargo test && cargo fmt --check` — clean (no source changes).
