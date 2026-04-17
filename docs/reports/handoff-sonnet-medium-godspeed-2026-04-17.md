# Handoff — Sonnet-medium pivot + godspeed purity + godspeed-mode skill

**Date:** 2026-04-17
**Commits this session (`81adb58..05a6619`, ahead of baseline by 5):**

| SHA | Title |
|---|---|
| `324402d` | `chore: remove templates/ — ~/.claude/agents/ is canonical source` |
| `128e724` | `feat(routing+docs): sonnet-medium pivot + godspeed purity + command-flows sync` |
| `9c381db` | `docs(shared): reference user-scoped godspeed skill backing the \| godspeed marker` |
| `718a709` | `docs(shared): split godspeed into two skills — universal posture + orchestrator/planner spec` |
| `05a6619` | `docs(shared): mark godspeed-mode upstream as READ-ONLY, never touched from our side` |

All pushed to `origin/main`. Branch clean on repo side.

---

## Mission for the next session

**Run full-capability bench tests of the new orchestration stack, with telemetry, and characterize how fast the sonnet-medium + godspeed-mode loop actually is.**

The stack as of `05a6619` is the first version where every moving part is aligned:

- Every teammate is `sonnet · medium` (frontmatter + DEBUG trap both agree)
- The judge stays `opus 4.7 · xhigh`
- Every dispatch appends ` | godspeed` (` | godspeed-impl` for executor) — the purest-form marker, no preamble
- The judge and the-planner load the full `godspeed-mode` skill (Pareto-walking at velocity) on first turn when godspeed-framed; every other teammate just reads the 4-line `godspeed` posture
- Templates/ is gone — no ambiguity about which agent file is canonical
- Mandatory connector on every Pareto round (structural, not optional)

We have not measured it. The last benchmark set in the repo (`data/bench-phase-c.paths`, `docs/reports/teammate-benchmark-summary-2026-04-17.md`) was under the opus-medium scheme. Post-pivot numbers should look materially different on wall time and throughput; the question is how much, and whether axis-improvement-per-round holds up.

---

## State of the world — everything touched this session

### In-repo (committed, on `origin/main`)

| File | Change |
|---|---|
| `AGENTS.md` | Model column uniformly `sonnet · medium`; `the-judge` marked as orchestrator exception; `cco-` prefix reserved for judge; 14 agents listed. |
| `CLAUDE.md`, `CODEX.md` | Project-structure list updated — `templates/` entries removed; `~/.claude/agents/` named as canonical. |
| `README.md` | Agent roster count `8` → `14 xbreed-managed` (excludes musketeer/puppeteer); install instructions no longer reference `templates/`; `git show 0ac5571:templates/` cited as historical recovery path. |
| `commands/references/xbreed-shared.md` | §Godspeed Mode — Purest Form rewritten; §Skill split documents the 2-skill scheme; Axis → Profile Mapping table all `sonnet · medium`; §Sonnet-medium unified scheme paragraph supersedes the opus-medium one; templates/ refs removed; `~/.claude/agents/` named as canonical. |
| `commands/xbgst.md` | Step-4 bullet-3 godspeed preamble removed in favor of ` | godspeed` suffix directive; axis-profile mapping collapsed to sonnet-medium uniform; executor lane uses ` | godspeed-impl`. |
| `docs/command-flows.md` | Full rewrite per prior handoff checklist: 3-level OAuth-only gemini cascade; 3-lane codex dispatch with `-R`; 14-agent count node; new §Model selection section; sonnet-medium tier map; per-teammate effort DEBUG-trap note; mermaid reviewer arrows retargeted to `xask -R codex`; connector flagged MANDATORY in `/xgs` and `/xbgst` sequences. |
| `docs/reports/handoff-unified-tier-scheme-2026-04-17.md` | `## Empirical verification during execution` section appended — records the 13→14 miscount + all 6 sanity-check outputs verbatim. |
| `docs/reports/flows-doc-sync-r1-2026-04-17.md` | NEW — scribe R1 report for the flows-doc-sync xbgst run. |
| `scripts/xask` | `TEMPLATE_DIR` now env-overridable (`XBREED_DISPATCH_DIR`); fallback-to-raw-`$QUERY` documented; safe when templates/ absent. |
| `scripts/verify-docs.sh` | `AGENTS_DIR` now points at `~/.claude/agents/` (override: `XBREED_AGENTS_DIR`); stale `templates/skills/xbgst/SKILL.md` reference removed (`commands/xbgst.md` is canonical). |
| `tests/axis_family_schema_check.sh` | Same repoint to `~/.claude/agents/` via `XBREED_AGENTS_DIR`. |
| `templates/` | **Deleted** — 25 files removed (14 agents + 7 skills + 4 dispatch). Historical snapshot at commit `0ac5571`. |

### Out-of-repo (user-scope; not tracked in xbrd-gdsp-fknpft)

| Path | Change |
|---|---|
| `~/.claude/agents/{connector,critic,executor,labrat,mutation-tester,reviewer,scout,scribe,sentinel,simplifier,the-planner,the-revenger}.md` | Frontmatter `model: opus` → `sonnet`; `effort: high|medium` → `medium`. 12 files. |
| `~/.claude/agents/the-judge.md` | Stays `model: opus` + `effort: xhigh`. Sub-role dispatch table: reviewer/sentinel/critic cites updated from `xask --effort high codex` → `xask -R codex` (closes the M2b drift from flows-doc-sync R1). Posture bullet added: load `godspeed-mode` via `Skill()` when session is godspeed-framed. `cco-` prefix description updated (reserved for judge only). |
| `~/.claude/agents/the-planner.md` | Frontmatter pivoted to sonnet-medium; header body updated from "Opus 4.7 high" → "Sonnet · medium (2026-04-17 pivot)". Layer-0 section extended: second tool call = `Skill("godspeed-mode")` when godspeed-framed. |
| `~/.claude/agents/distiller.md` | Already sonnet-medium; unchanged. |
| `~/.bashrc` `__xbreed_effort_trap` (lines ~228-278) | Collapsed from 3 branches to 2: `*the-judge*` → `xhigh`; `cco-|ccs-|cdx-|g-` → `medium`; unmapped → NOMATCH. Former `*critic*|*connector*|*planner*` → `high` branch removed. |
| `~/.claude/skills/godspeed/SKILL.md` | Universal 4-line posture, no preamble, trigger = "godspeed" / `--with godspeed` / `\| godspeed`. Content is exactly what the user specified. |
| `~/.claude/skills/godspeed-mode/` (NEW dir) | `SKILL.md` (full orchestrator/planner behavioral directive) + symlinks `directive.md` / `filter.md` / `velocity.md` / `codex-AGENTS.md` → `/home/vhpnk/godspeed-mode/`. |
| `/home/vhpnk/godspeed-mode/` (NEW clone) | Read-only clone of `git@github.com:VeigaPunk/godspeed-mode.git`. **Never edit / commit / push against this upstream**; `git pull` is fine (one-way consumer). |
| `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/` | `feedback_unified_tier_scheme.md` rewritten (sonnet-medium + godspeed purity); `feedback_agents_canonical_source.md` updated for templates/ removal; `reference_godspeed_skills.md` added (skill split + clone path); `feedback_godspeed_repo_readonly.md` added (hard rule); `MEMORY.md` index synced. |

### Verification snapshot (2026-04-17 15:40 UTC-3, pre-bench)

```bash
cargo clippy --all-targets -- -D warnings   # clean, 0 warnings
cargo test                                  # all suites pass (N = current suite count)
cargo fmt --check                           # clean
bash scripts/verify-docs.sh                 # OK — canonical connector routing consistent
bash tests/axis_family_schema_check.sh      # PASS — enum drift mutation caught + restored
xask -d -R codex "probe" | grep REVIEW      # REVIEW: true
```

All six sanity checks from the prior handoff still green. None broken by the pivot.

---

## Bench-test plan for next session

**Goal:** measure the full-capability stack (sonnet-medium teammates + ` | godspeed` purity + judge/planner godspeed-mode skill + mandatory connector + Pareto filter with evidence schema) under realistic missions, with telemetry capable of answering the question "is this materially faster and at least as directionally-correct as the pre-pivot opus-medium version?"

### Bench Mission A — small scoped doc-rewrite (reproducible baseline)

- **Scope:** something like this session's `flows-doc-sync` — a doc rewrite with explicit per-section checklist. Pick a different target doc so caching doesn't muddy timing (e.g. a rewrite of `docs/swarm-test-flow.md` or `docs/xask-protocol.md`).
- **Invocation:** `/xbgst /wwkd <prompt> | godspeed`
- **Expected team size:** 4-6 (executor / connector / reviewer / scribe / distiller + maybe scout).
- **Rounds expected:** 1 (mechanical scope). Round 2 should NO-OP per exit condition.
- **Measures:** team wall time from TeamCreate → final DRAFT; per-teammate first-byte latency (SendMessage arrival time); Pareto survivors / spoof_flagged ratio; hallucination rate (connector + judge spot-checks vs raw teammate output).

### Bench Mission B — open-ended code refactor (stresses Pareto walk)

- **Scope:** something where axes are real and can regress — e.g. "add telemetry hooks to `src/ask.rs` without widening the auth surface AND without adding new dependencies" (antimetabole shape — perfect for the symmetric Pareto filter).
- **Invocation:** `/xbgst <prompt> | godspeed`
- **Expected team size:** 8-10 (connector + scout + reviewer + executor + labrat + critic + sentinel + scribe + distiller + maybe mutation-tester).
- **Rounds expected:** 2-3 before saturation.
- **Measures:** same as A + cross-model divergence rate (gemini vs codex findings that land CONFLICTS block); axis-improvement-per-round (materiality rule — is each round's frontier actually moving observables, or is prose drifting?); evidence-schema compliance (drops vs spoof_flagged).

### Bench Mission C — reverse-engineering probe (stresses the-revenger + godspeed-mode for planner)

- **Scope:** "characterize the CLAUDE_CODE_SUBAGENT_MODEL override path — when does it apply, when is it NOMATCH, what env is inherited into the spawned teammate's process?"
- **Invocation:** `/xbgst <prompt> | godspeed`
- **Expected team size:** the-planner + the-revenger + labrat swarm + scribe.
- **Rounds expected:** 2 (planner Phase-0 data-walk → specialists).
- **Measures:** same as above + planner's first-turn loads (wwkd + godspeed-mode, confirmed via ShimHit env var per `feedback_teammate_mode_effort_caveat.md`); `/proc/$PID/environ` inspection of spawned teammates to verify `CLAUDE_CODE_EFFORT_LEVEL=medium` is actually propagated.

### Telemetry hooks to prepare FIRST (Phase 0 data-walk on the bench setup itself)

Before running any mission, stand up the telemetry capture so the first run's numbers aren't lost:

1. **Per-command wall time logger** — wrap the `Agent()` dispatch points so each spawn records `t_spawn`, `t_first_byte`, `t_final_msg`, `t_despawn`. A cheap stub: `/tmp/xbgst-bench-<ts>/per-teammate.jsonl` appended from a bash hook around the `Bash(Agent(...))` invocation. Pareto filter: `cheap (stub > full wrapper) / parallel-able (per-teammate independent) / auto-recording`.
2. **Round telemetry** — at each `SYNTHESIS_READY` and each Pareto compile, append a row: `round_id / survivors / dropped_evidence / spoof_flagged / axes_improved_since_last`. Single append-only file per mission.
3. **Effort-shim witness** — rely on the existing `XBREED_EFFORT_SHIM_HIT` env var (`feedback_teammate_mode_effort_caveat.md`); add a `grep` on the teammate's initial stderr/stdout to confirm `agent:medium:<pid>` is emitted.
4. **Skill-load witness** — confirm the-judge and the-planner actually call `Skill("godspeed-mode")` on first turn via the conversation transcript `grep` (not just that it's in the posture — that it fires).
5. **Hallucination counter** — every time the judge spot-check refutes a teammate's content-state claim (like the 2 reviewer hallucinations this session), log a row: `move_id / teammate / claim / judge_verdict / evidence`. Compare rate pre/post sonnet-medium pivot.

The velocity-half directive from `godspeed-mode/velocity.md` applies literally here: **build the cheapest possible stub of the expensive component first, build the variant catalog (the three bench missions) second, build the parallel runner third, auto-recording fourth.** Don't optimize the telemetry pipeline before the first run.

### Comparison baseline

Prior xbgst runs under opus-medium scheme:
- `docs/reports/flows-doc-sync-r1-2026-04-17.md` — this session, sonnet-medium partial (teammates were already sonnet, but the pivot hadn't landed in shared.md yet)
- `docs/reports/xbreed-harness-r1-2026-04-17.md` — pre-pivot baseline
- `docs/reports/xbgst-wwkd-r2-0416.md` — opus-medium full

Diff these against the new runs on: wall time per round, token usage (OAuth quota burn), axis-improvement-per-round, hallucination rate.

**Open question:** can sonnet-medium + godspeed-mode match opus-medium's directional correctness while being ≥2x faster? That's the central hypothesis — no pre-specified goal on the bench itself (let the frontier walk), but if the answer is "no, sonnet-medium loses directional correctness on Mission B", we flip specific roles back. If "yes," the pivot pays.

---

## Known caveats / don't-fix-in-the-bench-session

- `src/mailbox.rs` still has an uncommitted mod from before this session — pre-existing WIP, not ours.
- `test.sh`, `test2.sh`, `data/bench-phase-c.paths`, older report files — all user WIP, untracked, leave alone.
- The upstream `VeigaPunk/godspeed-mode` repo is **read-only from our side**. If a bench finding suggests the spec is wrong or outdated, record it in OUR repo / memory — do not edit, commit, or push to godspeed-mode. See `feedback_godspeed_repo_readonly.md`.
- `the-planner.md` currently has BOTH `on_spawn_skill: wwkd` (frontmatter) AND a posture bullet saying "second tool call = `Skill("godspeed-mode")` when godspeed-framed". If CC can only honor one `on_spawn_skill`, wwkd wins (per frontmatter precedence); the godspeed-mode load happens via the posture bullet as an explicit second `Skill()` call. Verify this propagates correctly during Mission C.
- The cdx-reviewer in this session hallucinated 2 "blockers" (duplicate mermaid node + stale API-key fallback) that grep refuted. Memory already records `feedback_critic_hallucination.md` — the bench should characterize this rate on sonnet-medium explicitly, not treat it as a one-off.

---

## Quick sanity checks before first bench mission

```bash
# 1. Agent roster is 14 sonnet-medium (+ 1 opus-xhigh judge)
for f in ~/.claude/agents/*.md; do
  test_name=$(basename "$f" .md)
  [[ "$test_name" == "the-musketeer" || "$test_name" == "the-puppeteer" ]] && continue
  grep -E '^(model|effort):' "$f" | tr '\n' ' '; echo " $test_name"
done
# Expect: 13 lines of "model: sonnet effort: medium <name>" + 1 "model: opus effort: xhigh the-judge"

# 2. Godspeed skills are discoverable
ls ~/.claude/skills/godspeed/SKILL.md ~/.claude/skills/godspeed-mode/SKILL.md

# 3. Godspeed-mode spec files are live (symlinks resolve)
ls -L ~/.claude/skills/godspeed-mode/{directive.md,filter.md,velocity.md}

# 4. Upstream clone is clean + read-only intent respected
git -C /home/vhpnk/godspeed-mode status   # expect: clean, untracked: none
git -C /home/vhpnk/godspeed-mode log -1   # see upstream HEAD

# 5. DEBUG trap is 2-branch (judge xhigh / prefix medium)
sed -n '228,278p' ~/.bashrc | grep -cE '\*(the-judge|critic|connector|planner)\*|cco-\*|\*\)' # expect: judge + prefix + catchall = 3 case patterns (no critic/connector/planner branch)

# 6. xbreed verify-loop green
cargo clippy --all-targets -- -D warnings && cargo test && make verify-docs && bash tests/axis_family_schema_check.sh
```

If any drift, inspect before starting Mission A — a misconfigured stack invalidates the bench numbers.

---

## Reference — memory files live as of this handoff

- `feedback_unified_tier_scheme.md` — canonical sonnet-medium scheme + godspeed purity
- `feedback_agents_canonical_source.md` — ~/.claude/agents/ is canonical, templates/ gone
- `reference_godspeed_skills.md` — skill split (godspeed vs godspeed-mode) + clone path
- `feedback_godspeed_repo_readonly.md` — hard rule: never edit / commit / push upstream
- `feedback_connector_every_round.md` — mandatory connector still in force
- `feedback_connector_gemini_high.md` — connector LOCKED to gemini high still valid
- `feedback_teammate_mode_effort_caveat.md` — DEBUG trap witness mechanism for bench telemetry
- `feedback_critic_hallucination.md` — primary-source verify before acting on content-state claims
- `feedback_no_hold_protocol.md` — auto-shutdown + TeamDelete after final DRAFT (bench mission cleanup)
- `feedback_commit_per_round.md` — commit per round for auditable bench trail

---

## PR-message template (if the bench mission opens a PR)

```
bench(xbgst): sonnet-medium + godspeed-mode capability characterization

Measures the post-2026-04-17-pivot orchestration stack against pre-pivot
opus-medium runs. Three missions (A: scoped doc-rewrite / B: open-ended
refactor / C: reverse-engineering probe). Telemetry captured:
per-teammate wall time, round-level survivor/drop/spoof counts,
effort-shim witness verification, skill-load confirmation,
hallucination rate.

Findings: <fill in from actual numbers>.
```

Handoff complete. Next session: pick Mission A, build the velocity-half stub, and run.
