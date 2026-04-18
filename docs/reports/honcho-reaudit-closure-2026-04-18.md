# Honcho Re-Audit — Closure

**Mission:** honcho-reaudit-0418
**Date:** 2026-04-18
**Status:** CLOSED for this round. R1-NL-prose shape DEFERRED pending H3 observability. R5 DURABLY KILLED.
**Mandate:** handout §7 — "if the verdict is 'no role', document WHY with deeper grounding than the prior mission had, so this question is closed and not re-opened next time."

---

## 1. Verdict

**R0 (no Honcho integration) is the frontier for this round.** No Honcho wrapper is to be built. The `~/.claude/agents/the-judge.md:18` Phase 0 hook remains SQLite-only. The H3 PARALLEL timing verdict governs what happens next: ship 1-2 real xbgst missions on the current SQLite substrate, instrument trigger detection, re-evaluate only if observability data reveals recall gaps.

**R1-NL-prose is DEFERRED, not killed.** Round 1 miscategorized one of the "three independent kills" on R1; the primary-source correction surfaced in Round 2 reduces the gate-blockers on R1 to a single contract-quirk semantic mismatch (K1b). R1 remains a targeted re-audit candidate if and only if the H3 ship arm produces empirical recall-gap evidence that SQLite + FTS5 + auto-memory + scribe-reports cannot close.

**R5 is DURABLY KILLED** on two independent primary-source kills (K2 metadata dead path + K3 probabilistic dialectic). R2, R3, R4 remain rejected from Phase 0 §4. No reopen path exists for any of these four without structural upstream change in Honcho itself.

**This closure is more durable than mission `honcho-stress-0418` R2's rejection** because it stands on code-grounded kills (primary-source file:line in Honcho upstream) and distinguishes role-specific kills from blanket dismissal. The R1/R5 split is the load-bearing refinement.

---

## 2. Role-by-role kill table

| Role | Shape | Status | Primary kill | Source |
|---|---|---|---|---|
| R0 | no integration | **PASS (round verdict)** | n/a | trivial |
| R1 | NL audit sidecar (write-only to Honcho; ad-hoc cross-mission recall; SQLite remains Phase-0 authority) | **DEFERRED** | K1b contract-quirk: Honcho sessions are ephemeral; we need durable mission namespace → gate (e) | `schemas/api.py:479-509` shows `session_id: str \| None` native; ephemeral semantics per Honcho docs |
| R2 | write-only mirror, nothing reads | **FAIL spirit** | adoption theater: no workflow value | T-axis R1/m6 + X-axis m9 |
| R3 | `/messages` + deriver async pipeline | **FAIL spirit** | probabilistic contract quirk → gate (e) | `dialectic/chat.py:20-78` (tool-loop agent by construction); deriver single-LLM-call non-determinism |
| R4 | peer_card mission state | **FAIL spirit** | schema coercion: peer_card is single-list, overwrite-per-set → gate (b) | R1 M-axis m7 + T-axis per-role verdict |
| R5 | Phase-0 semantic enrichment | **KILLED** | K2 + K3 both apply (see §3) | see §3 |

### Why R1 is deferred, not killed

Round 1's three-kill framework for R1 was:
- Kill-1: session_id NULL at write → red-flag (a) client-side capability patch
- Kill-2: `ConclusionCreate` has no metadata field → content.icontains forced → red-flag (b)
- Kill-3: Phase-0 auto-memory feedback loop → red-flag (e) contract-quirk

Round 2 reclassification (primary-source verified):
- **Kill-1 is a wrapper-bug misclassification.** `schemas/api.py:485` confirms `session_id: str | None` IS a native `ConclusionCreate` field. `crud/document.py:770, 782` persists it to `Document.session_name`. `utils/filter.py:50-56` provides the native filter mapping. `document.py:303` and `document.py:270` apply the filter on both pgvector and external vector-store paths. The 13 NULL records in production Honcho are an **old wrapper bug** — the wrapper never passed `session_id` on write. Fixing the wrapper to pass `session_id` is **using a native API field**, not client-side capability patching. Kill-1 does not apply.
- **Kill-2 does not apply to R1-NL-prose.** R1's design is prose-only content queried by session_id. It does not need axis_id/round/direction for server-side filtering. Kill-2 is a kill for R5 (semantic enrichment with structure), not R1.
- **Kill-3 does not apply to R1.** Phase 0 §4 explicitly frames R1 as "a queryable append-only audit index, **never** a read path for Phase-0 hook." The feedback loop concern is about R5's Phase-0 injection path; R1 has no Phase-0 injection.

What survives for R1: **Kill-1b** — Honcho sessions are ephemeral (their docs frame sessions as grouping-per-interaction), but we need durable mission namespace. This is a contract-quirk gate (e): we must learn-around the semantic mismatch. It is softer than Kill-1 (no client-side patch required; just awareness). It is enough to defer R1 but not enough to kill it durably.

### Why R5 is durably killed (and why K3 does NOT kill R1)

R5's Phase-0 semantic-enrichment path depends on BOTH:
- **K2: `ConclusionCreate` has no user-supplied metadata field.** `schemas/api.py:479-509` — fields are `content, observer_id, observed_id, session_id`. `crud/document.py:781` hardcodes `internal_metadata={}` for manually-created conclusions. Richer fields (axis_id, round, direction) cannot be stored as server-filterable metadata. They can only live in `content` with `icontains` filtering, which trips red-flag (b) schema coercion.
- **K3: `/chat` is a probabilistic tool-using NL agent.** `dialectic/chat.py:20-78` shows `DialecticAgent.answer(query)` runs a tool-calling loop over `search_memory / get_recent_observations / get_observation_context` etc. Non-determinism + confabulation on sparse data = red-flag (e). R5's read-side enrichment depends on this path.

Both kills survive any wrapper-level fix. Both kills are independent. Either alone is sufficient. R5 does not come back without upstream Honcho schema + API redesign.

**K3 does NOT kill R1.** R1's read path is `POST /conclusions/query` (synchronous, deterministic, filter-DSL-based at `routers/conclusions.py:88-124`), not the dialectic `/chat` endpoint. The probabilistic contract quirk that kills R5 does not bite R1 at all. This distinction was load-bearing in the Round 2 reclassification.

---

## 3. Re-open triggers

If any of these fire, re-audit the Honcho question with the evidence the trigger produced. Otherwise, closure stands.

| Trigger | Priority | Observable | Threshold | Detector | Action | Automation |
|---|---|---|---|---|---|---|
| **C3** upstream adds metadata field | **PRIMARY (evidence-driven)** | `ConclusionCreate` schema at `honcho/src/schemas/api.py` | any non-empty user metadata field lands in the schema | scout probing changelog / upstream releases | K2 collapses → R5 becomes re-audit candidate; re-dispatch `/xbgst /wwkd` | human (changelog) |
| **C1 / t1** real recall miss | secondary (operational) | prior finding in `data/xbreed.db` not surfaced by Phase-0 hook but relevant to mission | 1 confirmed miss after hook-fix + recall_audit in place; min-N = 2 missions | scribe `recall_audit` section (H3 M3); `scripts/xbreed-memory grep` audit helper | **First** re-dispatch `/wwkd` to fix the hook query (rowid-DESC silent-failure on wrong-but-non-null result). **Then** re-arm t1 detection. Only after t1 fires with hook fixed: `/xbgst` if SQLite floor insufficient | **BLOCKED until hook fixed** (current rowid-DESC cannot detect wrong-but-non-null) |
| **C2** MEMORY.md size explosion | secondary (maintenance) | line count of `~/.claude/projects/.../memory/MEMORY.md` | >84 lines (2× current baseline ~42) | automated `wc -l` check | re-audit auto-memory schema; re-dispatch `/xbgst` if structural overhaul needed | fully automated |
| **C4** user principle revision | secondary (governance) | `user_trendsetter_principle.md` edits | explicit rewrite of adaptation red-flag list | user | re-audit with new gate constraints | human |
| **t3** `icontains` violation | **zero-tolerance architectural** | any `findings.observable` containing `axis_id`, `"round"`, `round=`, `"direction"`, `direction=` encoded | 1 occurrence | `scripts/xbreed-memory redflags` (automated) | STOP extension work; open issue with explicit **blast-radius** statement (icontains contamination degrades SQL + FTS5 + any future semantic layer simultaneously); `/wwkd` before further implementation | fully automated |
| **t1+t4** compound "local surfaces exhausted" | compound gate for R5 re-evaluation | t1 fires (post-hook-fix) AND t4 residual observed in same mission | both conditions in one mission; min-N = 2 | semi-automated (t1 post-fix) + user (t4 proxy) | **Graduated escalation**: (1) evaluate `SQLite FTS5` virtual table on `findings.observable` first (per A6 connector rank d > e); (2) only if FTS5 repeatedly fails on same pattern → open Honcho R5 re-evaluation. **Three sequential gates, not one** | compound |

### t4 — named residual, NOT a trigger

**t4 auto-memory drift is an accepted residual of R0, not a re-open condition.** Systematic bias in `memory/project_*.md` is undetectable without a second memory source — the exact capability whose absence t4 would justify re-opening. This is epistemic circularity, not operational signal. Honcho does not solve t4 either: deriver has its own accumulation bias with no reconciliation primitive against auto-memory. Monitoring is human-only: periodic review of `memory/*.md` + `rg "memory correction" docs/reports/` for hard contradictions. Documented here as governance statement so future sessions don't mistake it for an automation gap.

### Notes on trigger quality (post A3 critique + distiller final addendum)

- **t1 is blocked** until the Phase-0 hook query is fixed. Current `~/.claude/agents/the-judge.md:18` orders by `round DESC, rowid DESC LIMIT 1`, which silently surfaces wrong-but-non-null results when insert-order diverges from wall-clock. The hook must move to `date DESC` or explicit mission-name filtering before t1 can detect misses reliably. Until then, t1 is in place as a design gate but cannot fire empirically.
- **t3 is the only fully-automated, zero-tolerance trigger.** Its action includes a mandatory blast-radius statement because `icontains` contamination is upstream of all recall mechanisms: once structured data is encoded inside prose content, SQL substring search, FTS5 lexical search, and any semantic layer all degrade simultaneously. Detection is cheap; remediation is expensive.
- **t1+t4 compound path** is the R5 re-evaluation gate. R5 does NOT reopen on t1 alone. The gate order is: fix hook → arm t1 → observe t1 fire → observe t4 residual in same mission → evaluate FTS5 → FTS5 fails repeatedly → *then* Honcho R5 re-audit. This matches A6 connector's substrate-ranking (`auto-memory (b) > FTS5 (d) > Honcho R5 (e)`).

---

## 4. Ship plan (H3 PARALLEL arm — per A5 m16)

The closure is not "do nothing." It is "ship forward on SQLite while the audit stays armed." The plan:

| Milestone | Does | Gate |
|---|---|---|
| **M1 skeleton (toy)** | Run `/xbgst "pick a color for the team logo"` end-to-end to verify the full infrastructure: spawn → Phase 0 hook reads SQLite → proposers post → distiller synthesizes → `xbreed-memory put` writes findings → scribe commits → TeamDelete | `git log --oneline -5` shows scribe commit; `sqlite3 data/xbreed.db "SELECT COUNT(*) FROM findings WHERE mission='logo-color-0418'"` returns ≥ 5 |
| **M2 overfit one real mission** | Run `/xbgst "audit src/mailbox.rs for any R3 shim regressions the dirty-state review missed"` (mission id: `mailbox-shim-audit-0418`) — chosen because mailbox.rs is already dirty in git status and R3 shim commits are fresh | First-turn judge output shows M1 findings injected by Phase-0 hook; SQLite ≥ 5 findings for M2 mission |
| **M3 observability** | Mandatory `recall_audit` section in every scribe report: list Phase 0 findings referenced; list Phase 0 findings present but unused; one-line verdict (useful / noise / absent) | `grep -l recall_audit docs/reports/logo-color-0418-*.md docs/reports/mailbox-shim-audit-0418-*.md` yields ≥ 2 matches |
| **M4 polish** | Update `CLAUDE.md` Architecture section to document current state: SQLite-authoritative, Honcho-deferred, closure pointer, SQLite FTS5 as contingency path | `grep -q "SQLite-authoritative" CLAUDE.md` and `grep -q "FTS5" CLAUDE.md` |

Dependencies: M1 → M2 → M3 → M4. M1 is the skeleton verification; M2 is the overfit-one real case; M3 is the observability layer that produces C1 trigger data; M4 is the polish that documents the closure in `CLAUDE.md`.

Out of scope: any Honcho code, any schema change, any wrapper resurrection. Those are the re-audit work, not ship work.

---

## 5. Durability statement

This closure is more durable than the prior mission `honcho-stress-0418` R2 rejection because:

1. **Two independent primary-source kills for R5** (K2 `schemas/api.py:479-509`, K3 `dialectic/chat.py:20-78`) versus the prior mission's single-path dismissal. Either kill alone suffices; both apply.
2. **Kill-1 reclassification** from "red-flag (a) client-side patch" to "wrapper-bug-fixable-via-native-field-use" is itself a Phase-0 data-walk artifact that forbids the prior mission's analytical shortcut. Future audits start from the corrected framework, not the misread one.
3. **Role-by-role kill table** (§2) distinguishes R1 deferral from R5 death. Prior mission used blanket "reject Honcho." Precision matters because C3 (upstream metadata field) and H3 observability signals are different triggers with different consequences — blanket rejection would re-open the whole question on any signal.
4. **Re-open triggers are named, thresholded, and operationalized** (§3 + `docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md`). The A2 trigger-detection tooling (`scripts/xbreed-memory redflags` + `grep` + `latest-mission`) makes t3 and C1 monitoring automatable. Prior mission had no detector surface.
5. **Cross-surface semantic-recall-gap analysis** (A6 connector m17) ranked `(a) recall-not-needed` as the top answer: the Phase-0 hook's structural 20-row injection is sufficient at current scale; no gap demands filling. If a gap appears, `SQLite FTS5` (zero new deps, `sqlite3 v3.45.1` confirmed) is the first contingency, R1-NL-prose sidecar is the second, full re-audit is the third. Graduated response replaces binary yes/no.

---

## 6. Archival: shape-for-record

These are preserved explicitly so a future re-audit does not redo the work.

### Unprobed surface ranked (R1 M-axis m8)

| rank | surface | score | status |
|---|---|---|---|
| 1 | `POST /conclusions/list` | 9/10 | PROBED in R2 A4: returns 0 under `session_id` filter (session_name=null workspace-wide); no asymmetry with `/conclusions/query`; confirms session-scoping in principle but not in this workspace's data |
| 2 | `POST /peers/{peer}/sessions` | 6/10 | unprobed — mission enumeration surface; supports session-as-mission framing behind R1/R5 |
| 3 | `PUT /peers/{peer}/card` | 4/10 | unprobed — peer_card single-list, overwrite-per-set; this is exactly R4, already rejected |
| 4 | `GET /peers/{peer}/context` | 3/10 | unprobed — `peers.py:409` hardcodes `session_name=None` → endpoint always global, never mission-scoped; not useful for mission-scoped recall |
| 5 | `POST /messages` + deriver | 2/10 | unprobed by design — R3 path, already rejected on gate (e) |
| 6 | dreamer async consolidation | 1/10 | unprobed — depends on deriver pipeline (R3), cascade-disqualified |

### R1/R5 integration diff sketches (R1 S-axis m10)

Preserved in the round reports (`docs/reports/honcho-reaudit-r1-2026-04-18.md` §8 shape-for-record and `docs/reports/honcho-reaudit-r2-2026-04-18.md`). If C3 fires, the wwkd re-audit should start from those sketches + the A6 `SQLite FTS5` contingency analysis, not from first principles.

---

## 7. References

- **Phase 0 data walk:** `docs/reports/honcho-reaudit-phase0-2026-04-18.md` (commit `a2495da`)
- **Round 1 scribe:** `docs/reports/honcho-reaudit-r1-2026-04-18.md` (commit `f1dffc1`), synthesis content-fingerprint `sha256:c8b474d114ed65203ca75ec3dad63ef79b0a1a8e8c35c66880b25bba2ee64dc1` (not independently reproducible — no SOURCE_MAP published)
- **Round 1 synth-review:** `docs/reports/reviews/honcho-reaudit-r1-review.md` (commit `c5b3477`) — the BLOCKER finding that surfaced Kill-1 reclassification
- **Round 2 scribe:** `docs/reports/honcho-reaudit-r2-2026-04-18.md` (pending commit), synthesis content-fingerprint `sha256:9e5adca4b35a2106601344da2174be61e09171aca71da7d8f471f147cbecfe0d` (not independently reproducible — no SOURCE_MAP published)
- **Round 2 synth-review:** `docs/reports/reviews/honcho-reaudit-r2-review.md` (pending commit) — verdict: accept-with-corrections; all 4 m21 primary-source claims independently verified; flagged 1898ms latency claim as inherited Phase-0 unverified-for-R1/R2 (closure doc does not propagate it)
- **Re-open triggers (operationalized):** `docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md`
- **Trigger detection tooling:** `scripts/xbreed-memory` (added `latest-mission`, `grep`, `redflags` subcommands + `XBREED_MEMORY_DB` override) + `tests/xbreed_memory_audit.sh`
- **Prior-mission context:** `docs/reports/honcho-stress-0418-r2-2026-04-18.md` (R2 pivot that chose SQLite), `docs/reports/honcho-judge-r1r2r3-2026-04-18.md` (original integration report)
- **User principle:** `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/user_trendsetter_principle.md`
- **Honcho upstream (gitignored clone):** `honcho/src/schemas/api.py:479-509`, `honcho/src/crud/document.py:770,781,782,303,270`, `honcho/src/utils/filter.py:50-56`, `honcho/src/dialectic/chat.py:20-78`, `honcho/src/routers/peers.py:409` (global-scope hardcode)

---

## 8. Forward pointer — next action

**Ship M1 of the H3 plan.** Do not touch Honcho. Do not build integration code. Do not resurrect the wrapper. Run one toy mission, verify the SQLite loop end-to-end, commit the scribe report, move to M2. The audit is closed for now; the observability layer that could reopen it is the thing to build, not another re-audit.

If C3 fires (upstream Honcho adds metadata field) or C1 fires (real recall miss after M2/M3) or t3 fires (`icontains` violation): re-dispatch `/xbgst /wwkd` with this closure as the Phase-0 entry point. Otherwise, this question is closed.

---

*Authoritative closure — judge-direct DRAFT (claude-opus-4-7, xhigh). Based on Round 1 synthesis (audit_hash c8b474…ee64dc1), Round 2 corrected synthesis (audit_hash 9e5adca…f147cbecfe0d), and primary-source re-reads of Honcho upstream. Supersedes `honcho-stress-0418` R2's blanket Honcho rejection with the role-specific kill framework of §2.*
