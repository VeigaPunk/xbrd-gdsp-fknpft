# xbreed harness mission — R1 report

**Mission:** `/xbgst /wwkd | godspeed` against `docs/reports/xbreed-harness-charter-2026-04-17.md`
**Team:** `xbrd-harness-0417` (crashed mid-R2 of prior session, recovered via on-disk inbox + task state)
**Date:** 2026-04-17
**Scribe:** team-lead (in-session, post-crash recovery) — R2 scribe will be dispatched fresh
**Audit_hash:** `850e17e10ff0e1dc7cadb3ee16c0a41da25794f0f8cb787ca1d028cb23242cb6` [recomputed MATCH]

---

## Mission scope (3 charter items)

1. **Item 1 (axis E) — Effort-tier override precedence.** `~/.claude/settings.json` `effortLevel: "xhigh"` global beats per-agent `effort:` frontmatter (sonnet medium). Charter hypothesis: protocol-tier leak; promote to runtime.
2. **Item 2 (axis T) — xask as native CC tool.** User directive: NOT MCP. Layer-1 gate works; mid-session xask invocation ritual-only. Promote to same tier as Read/Write/Grep.
3. **Item 3 (axis B) — True-concurrent batch dispatch.** Prior-session N=6 Agent batch: 1 succeeded, 5 failed with `"Failed to create teammate pane: no space for new pane"`. Pre-spawn cap-check or concurrent-alloc semantics needed.

---

## Phase 0 — Planner (cco-planner-harness, opus + wwkd Layer 0)

Data-walk completed. Primary-source findings:
- `~/.claude/settings.json:45` — `effortLevel: "xhigh"` global default (charter hypothesized high; actual is xhigh)
- `CLAUDE_CODE_EFFORT_LEVEL` env var — **absent** (printenv check)
- `code.claude.com/docs/en/{model-config,sub-agents}` — precedence chain: `env > frontmatter effort > session effortLevel > model default`
- `scripts/xask:84-95` — ThinkingBudget is user-space in our repo (not vendor-locked)
- Plugin.json grep: 220 files, **zero `tools` key hits** — no user-space native tool registration surface

Skeleton emitted; specialists dispatched.

---

## Phase 2 — Teammate proposals (6 received, 1 absent)

| Teammate | Axis | Role | Finding summary |
|---|---|---|---|
| `cco-planner-harness` | skeleton | the-planner | Tier map: Item 1 repro-gated; Item 2 out-of-scope (documented ceiling); Item 3 partial user-space reachable |
| `ccs-labrat-effort-introspect` | E | labrat | Reported null — teammate could not mechanically observe own effective effort; accepted as epistemic ceiling (later contested by critic) |
| `ccs-scout-cc-tool` | T | scout | Null-pass on user-space native-tool registration — confirmed MCP-only per CC docs |
| `ccs-labrat-pane-cap` | B | labrat | Empirical: 9 panes in 46-row window (heights 45,28,14,22,11,5,2,1,1); 10th fails. Formula `WIN_H − (N−1) ≥ MIN_ROWS=8` |
| `ccs-connector-tmux-pane-alloc` | B/cross-axis | connector | in-process lever exists via `teammateMode` but silent-no-op on returned-peer DMs (src/sync.rs:19-23) + no /resume restore (CC docs paste-cache:419) |
| `ccs-reviewer-xbreed-shared` | X | reviewer | Doc-drift review; proposed fold into xbreed-shared.md:92 comma list rather than promoting out-of-scope items to named classes |
| `cco-critic-harness-ceiling` | X | critic (opus + heuer Layer 0) | **Self-correction arc** — initially challenged planner Item 1 overclaim → hardened via labrat null → RETRACTED & REOPENED after planner primary-source doc verification |
| `cdx-executor-brief-directive` | S | executor | **No proposal received** — S-axis absent from synthesis |

**Distiller:** `ccs-distiller-harness` (sonnet medium) collapsed 11 raw proposals → 5 unique moves. Spoof-checks: 10/10 local citations VERIFIED via literal-substring grep; 5 external URL citations flagged `evidence_unverified` (per schema).

---

## Critic's epistemic arc (documented as protocol win)

The `cco-critic-harness-ceiling` run produced four sequential verdicts on Item 1:

1. **Initial (10:19Z):** "Item 1 ceiling is overclaimed" — flip-gate defined, challenged planner
2. **Hardened (10:22Z):** "AXIS X FINAL: ceilings hardened" — accepted labrat null as empirical closure
3. **Closed (10:23Z):** "AXIS X CLOSED" — partial overclaim on Item 3, Item 1 ceiling stands
4. **RETRACTED (10:25Z):** "AXIS X RETRACTION — Item 1 REOPENED" — after planner primary-source docs contradicted the null-as-ceiling synthesis

The critic's self-flagged methodology error: *"accepting a peer's null-result as ceiling-hardened without pressing on measurement is the inverse of `feedback_critic_hallucination.md` trap."* This is Heuer ACH working as designed — pressing on methodology disambiguated doc-inference from empirical-ceiling.

**Documented as protocol win.** Memory `feedback_critic_hallucination.md` guarded against fabricating "already fixed" claims; this run caught the *other* direction (prematurely accepting null-as-ceiling). Both failure modes now have prior-art examples.

---

## EVIDENCE AUDIT (R1 — pre-Pareto)

```
EVIDENCE AUDIT: 5 moves with evidence, 0 moves without, 0 dropped, 0 spoof_flagged
```

audit_hash `850e17e10ff0e1dc7cadb3ee16c0a41da25794f0f8cb787ca1d028cb23242cb6` recomputed match with distiller's committed hash — round auditable, no SPOOF_SUSPECT. Judge did not inspect SOURCE_MAP before provisional scoring (blinding protocol honored via hash-commit ordering).

---

## Pareto filter (R1)

### ACCEPTED (strict improvers)

#### M-B1 — Preflight dynamic pane-cap check (axis B, Item 3) — HIGH confidence

- **Claim:** Cap is DYNAMIC, not static. Formula: `practical_cap ≈ WIN_H − (N−1)` until `min_pane_height = MIN_ROWS (=8)`. Pre-spawn rejection at `team_size > WIN_H − (N−1)`. In-process mode exists as user-space lever (src/sync.rs:20 `teammateMode: "tmux"` is hardcoded) **but is orchestration-destructive** — `/resume` drops teammates (CC docs paste-cache:419); SendMessage to returned peers = silent no-op (src/sync.rs:19-23). Not recommended.
- **Evidence (primary-source verified):**
  - `~/.claude/hooks/adaptive-panes.sh:23` `MIN_ROWS=8` [literal-substring confirmed]
  - Labrat empirical: 9 panes in 46-row window, heights `[45,28,14,22,11,5,2,1,1]`, 10th fails [direct tmux observation]
  - `~/.claude/paste-cache/062eba943d9b5665.txt:419` "No session resumption with in-process teammates" [literal-substring confirmed]
  - `src/sync.rs:29` `teammateMode=tmux` [literal-substring confirmed]
- **Tier:** Runtime-tier (preflight cap check in xbreed binary or Bash pre-spawn hook) with documented ceiling (Anthropic-side: graceful queue-on-full inside split-pane allocator is out-of-scope)
- **R2 action:** dispatch executor for implementation + failing-test→passing-test cycle

#### M-T1 — Fold xask-native-tool into out-of-scope comma list (axis T, Item 2) — HIGH confidence

- **Claim:** No user-space non-MCP native-tool-registration surface exists in CC 2.1.112. User directive explicitly rejects MCP. Extend xbreed-shared.md:92 "Out of scope" paragraph as flat comma-list addition. **Do not promote to a named class** — repeats the R3 A4' heuer-ACH overclaim pattern already rejected (xbreed-shared.md:96–98).
- **Evidence:**
  - xbreed-shared.md:92 out-of-scope paragraph [literal-substring confirmed]
  - xbreed-shared.md:96–98 "Why three tiers" rejection rationale [literal-substring confirmed]
  - Plugin.json grep: 220 files, ZERO "tools" key hits [local filesystem probe]
  - git 827c179 "feat(ask-resilience-r3): W3 ghost-leak fully hardened + include_str! SSoT + 3-tier enforcement model" [git log confirmed]
  - Critic adversarial hardening: flip-gate unanswered; scout null-pass stood under ACH pressure
- **Tier:** Build/CI-tier (documentation add to xbreed-shared.md; include_str! SSoT binding via existing R3 A2' mechanism keeps the comma-list authoritative)
- **R2 action:** docs-only edit — roll into R1 commit alongside the scribe report

### DEFERRED to R2 (not regressions, open questions)

#### M-E1-AMENDED — Item 1 effort-precedence: REPRO GATE OPEN (axis E) — LOW confidence

- **Claim:** Cannot close as Out-of-scope without clean CC 2.1.112 repro (no env var, mechanically-observed effective effort, not assumed). Three competing hypotheses:
  - **(a)** CC 2.1.112 bug: docs say frontmatter wins, observed behavior shows session overrides — regression
  - **(b)** Agent-tool teammate-mode spawn path ≠ subagent-delegation spawn path; frontmatter not honored in teammate-mode specifically
  - **(c)** Labrat methodology error: labrat reported null based on assumed effective effort, not mechanically observed effort
- **Evidence:**
  - `templates/agents/distiller.md:6` `effort=medium` [literal-substring confirmed — frontmatter already set]
  - `~/.claude/settings.json:45` `effortLevel=xhigh` [literal-substring confirmed]
  - `CLAUDE_CODE_EFFORT_LEVEL` env: absent [printenv check]
  - CC docs precedence chain [`evidence_unverified: external web docs` — planner-cited]
  - Labrat null-result [methodology contested by critic self-correction]
- **R2 action:** dispatch `ccs-labrat-effort-mechobs-r2` with **mechanical observation protocol** — task: "Can a CC 2.1.112 teammate mechanically observe its own effective effort tier from within the process?" If yes → run repro and determine hypothesis (a)/(b)/(c). If no → ceiling is **epistemic-not-ergonomic** (fix reachable via frontmatter trust-docs; verification upstream). Distinct verdict from "Out-of-scope" — worth naming in final DRAFT.

#### M-BE1 — Effort-weighted pane accounting (axis B×E cross-axis) — MED (same-model capped)

- **Claim:** Raw pane-count gate underestimates pool pressure. xhigh-effort panes hold longer → sustained drain. Preflight formula should weight by effort tier, not raw headcount.
- **Evidence:** Structural argument only — no cross-model measurement; same-model cap enforced (distiller rule) because both supporting teammates were `ccs-` prefix
- **R2 action:** dispatch `cdx-labrat-crossmodel-be1-r2` (spark codex) for cross-model validation — promotes to `high` if supported; stays MED or drops if not

#### M-D1 — Future labrat-N batch exemption (axis D forward) — LOW

- **Claim:** Future missions with concurrent multi-labrat swarms should run empirical batches before gate directives land, OR the M-B1 gate should be scoped to exempt labrat-N spawns under cap margin.
- **Evidence:** `none — forward recommendation` (schema-exempt)
- **R2 action:** none this round — capture in R2 charter addendum; executor implementing M-B1 should note the exemption vector

### RETRACTED / COLLAPSED (distiller dedup)

- T×B coupling (connector): RETRACTED — scout confirmed T ⊥ B
- `gemini --effort` vendor-lock framing (connector): RETRACTED — category error; xask:89-95 ThinkingBudget is user-space
- Static `cap=6` ceiling (labrat-pane-cap v1): SUPERSEDED by empirical dynamic formula
- in-process switch as Item 3 fix (connector v1, critic v1): REJECTED — orchestration-destructive

---

## CONFLICTS (R1)

**CONFLICT 1 — in-process DM viability (B-axis):**
- Source A (`src/sync.rs:19-23`, VERIFIED): SendMessage to returned peers = silent no-op; DM critique loop collapses
- Source B (labrat empirical): in-process DMs work intra-session for active peers
- **Resolution:** Scope-limited — both correct in different scopes (active vs. returned peers). `/resume` limitation (CC docs:419 VERIFIED) makes in-process non-viable for multi-round xbreed regardless. Both sources reject in-process as the fix → M-B1 wins. Conflict does not affect move recommendation.

**CONFLICT 2 — E-axis surface definition (RESOLVED by team-lead):**
- labrat: CC spawn effort (Agent tool, null result) — **is** charter Item 1
- connector: xask downstream effort (`scripts/xask:84-95`, user-space) — **different surface**, off-axis
- **Resolution applied:** axes distinct; labrat wins on charter Item 1; connector's xask finding is a separate surface (not a move for this mission)

---

## S-axis gap (acknowledged)

`cdx-executor-brief-directive` (S — scope-discipline) produced no proposal. Distiller flagged absence. Low-impact: the charter's scope was already narrow and three other axes (B, E, T) covered the substantive moves. No R2 redispatch warranted.

---

## R2 dispatch charter

**Rationale:** Round-2-always-runs invariant (xbreed-shared.md:250). R1 improved axes off pre-walk baseline (M-B1 accepted at HIGH; M-T1 accepted at HIGH; M-E1 evolved from premature-closure to repro-gated). R2 scope:

1. **Close M-E1 via mechanical-observation protocol** (`ccs-labrat-effort-mechobs-r2`)
2. **Cross-model validate M-BE1** (`cdx-labrat-crossmodel-be1-r2`, via `xask --spark codex`)
3. **Implement M-B1** (`cdx-executor-preflight-cap-r2`, red-before-green TDD, |godspeed-impl)
4. **Execute M-T1 doc fold** (rolled into R1 commit — 1-line addendum to xbreed-shared.md:92 out-of-scope paragraph)
5. **R2 distiller + scribe** spawned after proposals land (per `feedback_scribe_per_round.md`)

**Exit condition:** R2 either closes M-E1 repro and executes M-B1 (frontier moves → R3) or produces zero axis improvements (frontier halts → final DRAFT + cleanup).

---

## Commit plan (R1)

Single commit staged:
- `docs/reports/xbreed-harness-r1-2026-04-17.md` (this report)
- `docs/reports/xbreed-harness-charter-2026-04-17.md` (unchanged — already committed)
- (pending M-T1 doc fold into xbreed-shared.md — optional R1 inclusion; alternatively rolled into R2 commit alongside M-B1 implementation)

Provisional: M-T1 included in R1 commit as accepted doc change; M-B1 + M-E1 arrive in R2 commit.

---

## Provenance

- **Prior-session crash:** session `42ece512-0254-4b57-b466-e8454e358609` (parent) crashed after distiller SYNTHESIS_READY (10:25:47Z) + critic AMENDMENT (10:26:42Z), before scribe/commit. 8 orphan processes force-killed via `xbreed-cleanup --force` at recovery start.
- **Recovery evidence:** team-lead inbox contained 621 lines (70+ messages); distiller messages had full synthesis payload; audit_hash recomputed match confirmed no tampering.
- **Scribe discipline:** `feedback_scribe_per_round.md` mandates dispatched-scribe-per-round for clean-context R1 reports. Post-crash recovery is a known exception — judge writes in-session to preserve the recovered state durably before R2 starts.
