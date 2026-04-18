# Mission honcho-judge-0418 — Consolidated Report (R1+R2+R3)

**Date:** 2026-04-18
**Mission:** Integrate Honcho v3 memory with the-judge orchestrator (MVP)
**Protocol:** xbgst (Godspeed Pareto + cross-model delegation) + wwkd (What Would Karpathy Do)
**Framing:** User's 2026-04-18 directive — *"We can change the shape and features of Honcho. We are trendsetters. They are incumbent. If it's on V3, means it's old. We can make it minimal, optimal, perfect. We derive the Pareto frontier; they targeted it."*

---

## Executive summary

Over 3 rounds and 15+ teammates, the mission converged on an **H2-done-right** architecture: use Honcho as pure HTTP+auth+storage infrastructure, own the entire semantic schema in `conclusion.content` (Schema v2), bypass all Honcho "smart" endpoints (`/chat`, `/representation`, async messages). The wrapper (`scripts/honcho`, 54 LoC bash+curl) round-trips through workspace/peer/session/conclusions-write/conclusions-query/conclusions-delete primitives only. MEMORY.md remains **unconditional** and **additive-only**; Honcho is never compensatory.

**Status:**
- R1+R2 artifacts committed (`133d552`)
- R3 judge-direct patches landed (`the-judge.md` Posture bullet + `timeout 5` compound-latency fix)
- R3 executor (`cdx-executor-r3`) + labrat M2-gate (`cdx-labrat-m2gate`) in-flight for mission_id injection + end-to-end verify
- Round 4 optional (closure + final DRAFT)

---

## Mission arc

**Round 1 (orientation):** Name the axes, verify the API surface, stub the wrapper. 8 axes + 8 moves. No implementation yet beyond the wrapper stub (5 wire bugs flagged for R2).

**Round 2 (reframe):** User's "trendsetter" directive added the **T axis (Transcend)**. Critic + revenger did adversarial-design + reverse-engineering analysis, converged on keeping Honcho primitives as storage infra while owning the schema. 8 moves + 2 resolved CONFLICTs + 1 open (mission_id write-path).

**Round 3 (implementation):** Judge directly patched `the-judge.md`. Executor rebinding `scripts/honcho` for mission_id injection + JSON-escape hardening. Labrat running M2 overfit-one gate. Connector found one real regression (compound Phase 0 latency) — fixed via outer `timeout 5`.

---

## Round 1: axes + moves

**Hash:** `950824844f340c7d0293e2db13e7b47c538964f128f324a4cb6bd367fc3450fe` (cross-verified)

| axis | observable | teammate | move |
|---|---|---|---|
| A — API correctness | HTTP 200/201 from live probe | cdx-labrat-api | 5 endpoints verified; workspace `xbreed-probe-0418` created 2026-04-18T10:02:13Z |
| I — Integration point | file:line seams in the-judge.md | cdx-reviewer-integration | 3 seams: Phase 0 READ, post-COMPILE WRITE, final DRAFT write |
| S — Skeleton simplicity | LoC + runtime deps | ccs-simplifier-skeleton | 48 LoC bash+curl, 1 dep (curl); Python/TS SDKs rejected |
| D — Data schema | observer/observed/content per artifact | g-scout-schema | Conclusions = durable; Messages = ephemeral |
| R — Recall plan | wwkd milestones w/ gates | ccs-planner-recall | M1(curl toy) → M2(wrapper round-trip) → M3(judge.md patch) |
| W — Workspace hygiene | workspace_id resolution rule | cdx-reviewer-workspace | SSH regex rule retracted; hardcode `xbreed-judge` |
| F — Failure mode | degradation path table | cdx-labrat-failure | 5s cap + exit 0/1/2 + retry-once + N=1 on 401 (static key) |
| X — Cross-axis | 5-surface clash-map | g-connector-honcho | Honcho=soft observations; MEMORY.md=hard rules unconditional |

**R1 verdict:** All 8 axes moved off baseline → Round 2 mandatory.

---

## Round 2: axes + moves + reframe

**Hash:** `699659744547d839cbaa97b04086f719180e1bdad2db0ad9e7b92a6deddd9f55`

New axis (user reframe): **T — Transcend** (↑ axis-native expressiveness; reshape Honcho primitives vs adopt-as-is).

| move_id | axis | teammate | claim |
|---|---|---|---|
| m9  | T-critic     | cdx-critic-honchoshape    | ACH: H1 rejected (no metadata on conclusions), H2-sidecar-conditional, H3 native recommended. Later narrowed: chat() non-determinism applies ONLY to survivor paths. |
| m10 | T-revenger   | cdx-revenger-xbgstnative  | KEEP {workspace, peer, conclusions-write, conclusions-query, conclusions-delete}; SHIM {session→mission_scope, list→exact_sweep}; ABANDON {messages, chat, representation}. Schema v2. |
| m11 | S-exec       | cdx-executor-wrapper-fix  | scripts/honcho 54 LoC; 6 bugs fixed (incl peer-init discovery); M1 gate PASSES empirically. |
| m12 | C2           | cdx-reviewer-memorymd     | MEMORY.md Posture bullet between lines 17-18 (relocated from line 78). |
| m13 | Async        | cdx-labrat-asyncqueue     | /representation 572ms sync; /chat 1898ms + confabulates; /conclusions/query top_k raw. |
| m14 | Scope        | g-scout-missionscope      | /conclusions/query has NO native filter (gemini partially hallucinated). Client-side content-parse. C6 BLOCKER. |
| m15 | F2-stale     | ccs-simplifier-stalesweep | Judge-local supersedure: old_id in scope, POST new → DELETE old. ~4 LoC. |
| m16 | X-R2         | g-connector-r2            | No regressions; MEMORY.md Posture placement; survivor-routing invariant = only via POST /conclusions. |

**R2 verdict:** All 8 moves accept; T-axis convergence (critic + revenger both land on schema-owned approach); 2 CONFLICTs resolved, 1 open (m14 write-path).

---

## Round 3: implementation

**In-flight.** Pattern: judge directly patches low-risk text/config; subagents handle empirical verification + bash/curl engineering.

| change | agent | status |
|---|---|---|
| the-judge.md Posture bullet (MEMORY.md unconditional) | judge (direct Edit) | LANDED |
| the-judge.md Posture bullet: outer `timeout 5` | judge (direct Edit) | LANDED (per g-connector-r3 finding) |
| scripts/honcho mission_id injection + JSON escape fix | cdx-executor-r3 | IN-FLIGHT |
| M2 overfit-one gate empirical verify | cdx-labrat-m2gate | BLOCKED on executor-r3 |
| R3 invariant audit | g-connector-r3 | POSTED (1 regression found + fixed) |
| Consolidated scribe report | judge-direct (this file) | LANDED |

---

## CONFLICTS

| id | description | status | resolution |
|---|---|---|---|
| C1 | workspace_id derivation | RESOLVED | Caller-chosen string; hardcode `xbreed-judge` |
| C2 | MEMORY.md unconditional at HONCHO_OK=0 | RESOLVED | Posture:17-18 bullet |
| C3 | 401 threshold N=1 vs N=2 | RESOLVED | N=1 (static key, no rotation mechanism) |
| C4 | H3 (reject-Honcho) vs H2-done-right | RESOLVED | H2 threads needle via session_id→mission_id; H3 reserved R3 fallback |
| C5 | /representation (m13) vs ABANDON /representation (m10) | RESOLVED | Coexist by purpose — /conclusions/query primary, /representation NL sidecar, /conclusions/list for sweep |
| C6 | scripts/honcho put raw $VAL — no mission_id injection | OPEN | R3 executor task (cdx-executor-r3) |

---

## Rejected alternatives surveyed

- **Python SDK `honcho-ai`** — added dep + pip install + 300ms cold-start + second file; rejected for curl-only
- **TypeScript SDK `@honcho-ai/sdk`** — node_modules + build step + 500ms startup; rejected outright
- **Writing DRAFT survivors as Conclusions** (m4) — would persist pre-Pareto hypotheses; rejected for Messages tier
- **Honcho-primary authority inversion** (m8, cdx-reviewer-workspace proposed, connector rejected) — non-deterministic store cannot own authoritative records
- **SSH URL regex derivation** (m6) — solves non-problem; workspace_id is caller-chosen
- **N=1→N=2→N=1 threshold churn** (m7) — static key, retry adds no value
- **H1 adopt-as-is** (m9 critic ACH) — ontology mismatch with Pareto walk
- **H2-sidecar-as-SOR** (m9) — collapses to H1 under determinism-killing chat()
- **Per-teammate peers** (m10) — fragments top_k pool, multiplies init overhead
- **/chat for Pareto survivors** (m13) — probabilistic, confabulates on sparse data (empirical: 1898ms + "context does not include conclusions" despite data present)
- **/representation as canonical Pareto path** (m10) — NL inference layer over raw conclusions; lossy for exact recall
- **supersedes: tag on Honcho conclusion** (m15) — no metadata field exists on conclusions
- **Workspace-per-mission isolation** (g-connector-r2) — breaks cross-mission recall invariant
- **Explicit sed re-read of MEMORY.md** (m12) — contradicts auto-load guarantee + 240-line truncation risk

---

## Optimization routes surveyed

- **5-surface persistence clash-map:** auto-memory (hard rules, unconditional) / scribe (chronological audit) / Honcho (probabilistic observations) / obsidian (user-only, read-only) / git (code history)
- **Wrapper aesthetic:** bash+curl vs Python SDK vs TS SDK vs MCP server → bash+curl wins on LoC + dep count + startup latency
- **Write seam placement:** SYNTHESIS_READY vs post-COMPILE vs final DRAFT → post-COMPILE + final DRAFT (pre-Pareto writes prohibited — hypotheses not survivors)
- **Failure mode:** silent-skip vs retry-exponential vs fail-loud → retry-once + loud-on-401 (bounded ceiling, deterministic)
- **Phase 0 read path:** /chat (1898ms + confabulates) vs /representation (572ms formatted NL) vs /conclusions/query (top_k raw) → endpoint-per-purpose (three endpoints, three roles)
- **ACH H1/H2/H3:** H2-done-right operative via session_id→mission_id mapping; H3 reserved fallback if H2 operational fails

---

## Schema v2 (authoritative)

```json
conclusion.content (JSON string) = {
  "mission":       "<mission_id>",
  "round":         N,
  "axis_id":       "<A|I|S|D|R|W|F|X|T|C2|...>",
  "observable":    "<one-line-description>",
  "direction":     "+|-|0",
  "confidence":    0.0-1.0,
  "audit_hash":    "sha256:<hex>",
  "supersedes":    "<prior_conclusion_id>|null",
  "survivor_of":   ["<axis>", ...],
  "pareto_status": "candidate|survivor|dominated|retracted",
  "blinded_set":   "<round_hash>|null"
}
```

**Rationale:** `survivor_of` + `pareto_status` make the entire Pareto walk reconstructible from content blobs alone — no external ledger required. `blinded_set` enables round-level audit without a second lookup. Honcho carries none of these natively; the content-JSON envelope is the only viable encoding.

---

## Files changed

### R1+R2 (commit `133d552`)
- `docs/reports/honcho-integration-data-walk-2026-04-18.md` (Phase 0 data walk, 165 lines)
- `scripts/honcho` (54 LoC bash+curl wrapper)
- `.gitignore` (honcho/ clone excluded)

### R3 (pending commit)
- `~/.claude/agents/the-judge.md` — Posture bullet insertion (lines 17-18) + outer `timeout 5` in the Honcho invocation
- `scripts/honcho` — mission_id injection + JSON-escape fix (cdx-executor-r3 in-flight)

### External config
- `~/.config/xbreed/honcho.env` — exports `HONCHO_API_KEY`, `HONCHO_BASE_URL`, `HONCHO_WORKSPACE=xbreed-judge` (mode 600, outside repo)

---

## User's "trendsetter" reframe (verbatim, 2026-04-18)

> *"We can also change the shape and features of Honcho. We can use their infra and adapt their concepts to our own methods and improve on it. We are trendsetters. They are Incumbent already. If it's on V3, means it's old. We can make it minimal, optimal, perfect. We derive the pareto frontier; They targeted it."*

This reframe landed mid-Round-2 and added the T axis. Without it, R2 would have accepted H1 (adopt-as-is) by default. The Pareto filter's "improves at least one axis" rule required a new axis to force the reshape question into the scoring matrix.

---

## Open questions for next mission

1. **Scribe/planner/distiller Honcho integration** (user: "sparingly"). Judge-first MVP is this mission's scope; other roles deferred.
2. **Static key rotation mechanism** — R1 m7 deferred; not needed until user adopts key rotation.
3. **Stale-conclusion sweep — wrapper vs judge-native** — currently judge-native per m15-revised; could be `honcho sweep` subcommand if cross-role need emerges.
4. **Round 4 (optional)** — closure round: commit R3 executor work, final DRAFT with AXES FINAL STATE, auto-cleanup.
5. **Scope extension to cross-project** — current hardcode `xbreed-judge` single workspace; if multi-project need emerges, per-repo workspace derivation (NOT per-mission) is the path.

---

## Audit trail

| commit/artifact | hash / id | contents |
|---|---|---|
| R1 SYNTHESIS_READY | `950824844f340c7d0293e2db13e7b47c538964f128f324a4cb6bd367fc3450fe` | 8 moves m1-m8 |
| R2 SYNTHESIS_READY | `699659744547d839cbaa97b04086f719180e1bdad2db0ad9e7b92a6deddd9f55` | 8 moves m9-m16 |
| R1+R2 commit | `133d552` | data-walk + scripts/honcho + .gitignore |
| Honcho workspace | `xbreed-probe-0418` (live) | created 2026-04-18T10:02:13Z via cdx-labrat-api probe |
| Honcho workspace | `xbreed-judge` (live) | created during scripts/honcho first invocation |

---

*Report written by judge directly (ccs-scribe-r2 spawn blocked on transient tmux pane-geometry constraint). Model: claude-opus-4-7. Facts sourced from team inbox transcripts, live git log, verified commit contents, and direct Read of scripts/honcho + the-judge.md.*
