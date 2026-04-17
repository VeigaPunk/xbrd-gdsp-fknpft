# flows-doc-sync R1 — docs/command-flows.md rewrite + handoff empirical append

**Mission:** Update `docs/command-flows.md` to match 2026-04-17 state (3-level OAuth gemini cascade, 3-lane codex dispatch with `-R`, 14 xbreed agents, mandatory connector every round). Append empirical-verification section to `docs/reports/handoff-unified-tier-scheme-2026-04-17.md`.

**Session:** /xbgst /wwkd | godspeed — 2026-04-17
**Commit:** uncommitted WIP at report time (staged: `README.md`, `docs/command-flows.md`, `docs/reports/handoff-unified-tier-scheme-2026-04-17.md`; unstaged unrelated: `src/mailbox.rs`)
**Team:** flows-doc-sync-0417 (5 teammates: ccs-executor-flows, g-connector-docparity, cdx-reviewer-parity, ccs-scribe-r1, ccs-distiller)
**Distiller audit hash:** `ae1c772c6de6992cad4d5ea7db070007771c378eee716107b02ba46be06d374c`

## Phase 0 — WWKD data-walk

Handoff sanity-check block (`docs/reports/handoff-unified-tier-scheme-2026-04-17.md:186-208`) executed before dispatch. Reconstructed results from judge broadcast + executor's appended empirical section:

| Check | Expected per handoff | Actual | Verdict |
|---|---|---|---|
| xbreed agent count | `13` (handoff line ~189 grep expectation) | `14` (`ls ~/.claude/agents/*.md \| grep -vE 'musketeer\|puppeteer' \| wc -l`) | **DRIFT** — handoff prose line 138 lists 14 names; resolution = rewrite lands `14`, excludes musketeer+puppeteer as user-invoked |
| Codex model constants (3) | gpt-5.4-mini, gpt-5.4, gpt-5.3-codex-spark | match `src/ask.rs:112/119/127` | GREEN |
| `-R` / `--review` flag | present in CLI | match `src/cli.rs:60` | GREEN |
| Gemini OAuth cascade | 3-level | match `gemini_auth_chain()` `src/ask.rs:212-228` | GREEN |
| API-key-path removal | 0 hits | `0` "API key from .env.local" + `0` "GeminiKeys" in `src/` | GREEN |
| Canonical agent dir | `~/.claude/agents/` (post-commit 324402d) | `templates/` removed; CLAUDE.md:34 updated mid-run | GREEN |

**Key drift found:** Handoff's own grep expectation line (`13`) contradicted its prose line 138 (14 named agents). Actual `wc -l` = **14**. Resolution landed in rewrite: `14 xbreed-managed definitions; excludes the-musketeer + the-puppeteer (user-invoked, not xbreed-orchestrated)`.

## Round 1 proposals (per teammate)

### ccs-executor-flows (axes A1/A4/A5)
- **Move title:** M1-rewrite + handoff empirical append
- **Axis targeted:** A1 (command-flows.md accuracy), A4 (handoff verification append), A5 (table of agents)
- **Proposed edit:** Full rewrite of `docs/command-flows.md` to match 2026-04-17 state; append §Empirical verification to handoff with all 6 sanity checks logged verbatim.
- **Evidence:** diff on disk; `cargo clippy && cargo test` green; 6/6 handoff sanity checks logged in appended section; `14 xbreed-managed definitions` landed at line 399 with parenthetical exclusion note.
- **Rejected alternatives:** Splitting timing table into mini-lane vs full-lane rows — rejected because no fresh empirical data available for a split; added prose caveat instead.

### g-connector-docparity (axis A3 cross-ref consistency)
- **Move title:** M2a-agent-count + M2b-routing-drift + M2c-readme-stale + M2d-codex-profile + M2e-memory-gap
- **Axis targeted:** A3 (cross-file consistency xbreed/command-flows/handoff/templates/memory)
- **Proposed edits (5 findings):**
  1. **[HIGH] M2a** — Handoff internal miscount 13 vs 14; ground-truth ls=14. Rewrite must use 14. **ACCEPTED.**
  2. **[HIGH] M2b** — `~/.claude/agents/the-judge.md:25/32/34` (originally cited `templates/agents/the-judge.md:23/30/32`, relocated post-commit 324402d) still cite `xask --effort high codex` for reviewer/sentinel/critic rows; xbreed-shared.md §Axis→Profile lines 76/83/85 say `xask -R codex`. Handoff only updated the-revenger. **OUT-OF-SCOPE — follow-up PR.**
  3. **[MED] M2c** — `README.md:135` "8 agents" stale. **Judge cleanup accepted** → README.md:126 "8 agents" → "14 xbreed-managed".
  4. **[LOW] M2d** — `~/.codex/config.toml [profiles.xbreed]` = gpt-5.4 (INTENTIONAL per handoff line 60); distinguish from `src/ask.rs` `-c` override default in Model-selection section. **ACCEPTED.**
  5. **[LOW] M2e** — `memory/user_oauth_exclusive.md:11` reachable-id list missing `gpt-5.4-mini` — informational only.
- **Evidence:** xask gemini --effort high confirmed findings 2+4; diverged on count (gemini echoed handoff's 13, ground-truth ls=14). `wc -l` invoked directly for count arbitration. File-line cites verified pre-drift and post-drift (templates/ → ~/.claude/agents/ relocation).
- **Rejected alternatives:** Pre-emptive `the-judge.md` fix in same PR — rejected because handoff PR template says "No code changes" and connector is analysis-only.
- **Amendment mid-round:** Path citations updated (templates/agents/ → ~/.claude/agents/; lines shifted +2) after user nuked `templates/` wholesale in commit 324402d.

### cdx-reviewer-parity (axis A2 parity verification)
- **Move title:** M3-dupe-mermaid + M3-stale-apikey (both REFUTED)
- **Axis targeted:** A2 (post-edit parity review vs src/)
- **Proposed findings:**
  1. **[BLOCKER, REFUTED]** `docs/command-flows.md:227-228` — claimed duplicate mermaid G node ("stale `xask codex` + new `xask -R codex` both present").
  2. **[BLOCKER, REFUTED]** `docs/command-flows.md:314` — claimed stale "OAuth first → API key fallback" note in /xbgst sequenceDiagram.
  3. **[VERIFIED CLEAN]** 3 codex model constants match `src/ask.rs:112/119/127`; `-R`/`--review` matches `src/cli.rs:60`; 3-level OAuth cascade matches `gemini_auth_chain()` `src/ask.rs:212-228`; 0 `API key from .env.local` hits, 0 `GeminiKeys` hits in `src/`.
- **Evidence:** (1) **REFUTED** by judge primary-source Read — L227 defines node G (`F --> G[...]`), L228 uses G as source edge (`G --> Gx`); not a duplicate. (2) **REFUTED** — `grep 'API key' docs/command-flows.md` → 0 hits; L314 reads `end` (par block close); L319 reads "Auth cascade: OAuth-only (3 levels)".
- **Rejected alternatives:** none stated.
- **Process-quality note from reviewer:** suggested the "duplicate-node defect is a copy-paste artifact of Insert-not-Edit rewrite" — but this was itself predicated on the refuted finding.

## Round 1 Pareto filter

- **EVIDENCE AUDIT:** 5 moves with evidence / 0 without / 0 dropped-for-missing-form / **2 spoof_flagged** (cdx-reviewer-parity M3 pair).
- **Survivors (ACCEPTED):**
  - **M1-rewrite** (A1/A4/A5, HIGH) — executor
  - **M2a-agent-count** (A3, HIGH, cross-model confirmed ccs+g) — executor + connector convergence
  - **M2d-codex-profile** (A3, LOW) — connector via executor addendum
  - **Judge cleanup post-synthesis** — `docs/command-flows.md:400` mermaid `templates/skills/` node retargeted to `commands/*.md` + `~/.claude/skills/` + `~/.agents/skills/`; `README.md:126` `8 agents` → `14 xbreed-managed`.
- **Regressions:** none.
- **CONFLICTS_RELAY:**
  - g-connector-docparity via xask gemini diverged from ground-truth ls on agent count (gemini echoed handoff's 13; direct `wc -l` = 14). Surfaced verbatim; judge resolved in favor of ground-truth ls.
  - cdx-reviewer-parity produced 2 hallucinated/stale-view blockers (L227-228 dupe, L314 API-key). Surfaced verbatim; judge REFUTED both via primary-source Read.

## Out-of-scope (real drift, deferred)

- **M2b-routing-drift (MED):** `~/.claude/agents/the-judge.md:25/32/34` still cite `xask --effort high codex` for reviewer/sentinel/critic; `docs/command-flows.md:133` designates these as `-R` lane. Follow-up PR will fix all three rows to `xask -R codex`. Context: user nuked `templates/` wholesale in commit 324402d mid-run; drift relocated to canonical path.
- **Stale `templates/agents/` refs sweep** across `docs/` + `memory/`: flagged as follow-up Grep pass (not executed this round).
- **M2e-memory-gap:** `memory/user_oauth_exclusive.md:11` reachable-id list missing `gpt-5.4-mini` — informational, deferred.

## Optimization routes surveyed but rejected

- **Skipping the connector:** rejected — mandatory every round per `xbreed-shared.md:67` + `the-judge.md:17`.
- **Full-scale 12-teammate spawn:** rejected — scope is mechanical doc rewrite; 5 is Pareto-efficient.
- **Pre-specifying exact rewrites in judge brief:** rejected — teammates need to discover drift themselves (this is precisely how the agent-count 13→14 miscount surfaced; judge did not pre-flag it).
- **Pre-emptive the-judge.md fix in same PR** (connector-proposed): rejected — handoff PR template says "No code changes" in analysis-only lane; routing-drift carved out to follow-up PR instead.
- **Splitting timing table into mini-lane vs full-lane rows** (executor-considered): rejected — no fresh empirical data; prose caveat chosen instead.

### Process finding (for future rounds)

**Reviewer hallucination incident:** cdx-reviewer-parity returned 2 BLOCKER claims (M3 pair), both REFUTED by primary-source Read. Pattern matches `feedback_critic_hallucination.md`. Recommendation: reviewer briefs should include a "Read current file AFTER executor's edit lands" gate, not during parallel-execution race — parallel review risks reading pre-rewrite content or confabulating line numbers.

## Next round?

**VERDICT: SKIP Round 2.**

Rationale:
- All 5 axes (A1/A2/A3/A4/A5) moved off baseline in R1.
- Mechanical doc-rewrite mission with explicit per-section handoff checklist — no deliberation surface remaining.
- Residual drift surfaced (M2b) and either fixed by judge post-synthesis (line-400, README-126) or scoped out to follow-up PR (M2b).
- No proposal pending from any teammate.
- `feedback_xbgst_keep_going.md` exit condition satisfied: Round 2 skipped only when zero axis would improve — here, all proposals either accepted or deferred-by-design; a Round 2 dispatch would invite scope creep.

## Post-Round-1 pivot (audit-trail only)

User directive 2026-04-17 mid-run: **all teammate dispatches move to sonnet medium** (supersedes opus-medium unified scheme from `feedback_unified_tier_scheme.md`). Out-of-scope for the flows-doc-sync mission but logged here for audit trail — next xbgst session should reconcile this with memory/feedback_unified_tier_scheme.md and commit an update.

---

## Links

- Plan: `docs/reports/handoff-unified-tier-scheme-2026-04-17.md` (handoff + empirical-verification append)
- Prior reports: `docs/reports/shim-wwkd-r1-*.md`, `docs/reports/shim-wwkd-r2-*.md`
- Related memory: `feedback_scribe_per_round.md`, `feedback_critic_hallucination.md`, `feedback_connector_every_round.md`, `feedback_unified_tier_scheme.md`
- Next: no R2; follow-up PR for M2b (`~/.claude/agents/the-judge.md:25/32/34` routing-drift fix)
