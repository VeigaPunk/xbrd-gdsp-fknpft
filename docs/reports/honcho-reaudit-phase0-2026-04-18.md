# Phase 0 — Honcho Re-Audit Data Walk

**Mission:** honcho-reaudit-0418
**Date:** 2026-04-18
**Posture:** wwkd Phase 0 (data walk BEFORE xbgst dispatch — hard gate per handout `docs/handouts/honcho-memory-reaudit-handout.md`)
**Question the next round must close:** *Is Honcho actually a good idea to implement for memory — anywhere in the xbreed stack, in any role? Was R2's blanket rejection informed enough to stand?*
**Authority of this artifact:** primary-source reads only. No inference beyond what files say.

---

## 1. What I looked at

### 1.1 Prior-mission record (in-repo)

- `docs/handouts/honcho-memory-reaudit-handout.md` — the brief itself; §2.3 gates me to read the upstream code directly before concluding anything.
- `docs/reports/honcho-integration-data-walk-2026-04-18.md` — `honcho-judge-0418` original Phase 0 (R0 API surface mapped from docs only; `cdx-labrat-api` was the live prober).
- `docs/reports/honcho-judge-r1r2r3-2026-04-18.md` — consolidated H2-done-right report; Schema v2 11-field envelope spec; rejected alternatives; surveyed optimization routes.
- `docs/reports/honcho-stress-0418-r1-2026-04-18.md` — 6 confirmed wrapper bugs + 1 architectural RETHINK + 3 fix-directions; the Q-axis empirical probe against `/conclusions/query` hadn't run yet.
- `docs/reports/reviews/honcho-stress-0418-r1-review.md` — synth-reviewer accept-with-corrections on R1 (demoted m10 to fix-direction; flagged m1 historical-universal overreach; audit_hash reproduced).
- `docs/reports/honcho-stress-0418-r2-2026-04-18.md` — RETHINK pivot; 10 Pareto moves; Honcho rejected on two grounds (trendsetter principle + empirical mission-scope gap); SQLite-WAL chosen as substrate.

### 1.2 Substrate + integration state

- `scripts/xbreed-memory` (55 LoC) — current bash wrapper over `data/xbreed.db`; put/get subcommands; SQL-escaped via `sed "s/'/''/g"`; exit-code taxonomy 0/1/2; busy_timeout=5000 per-connection.
- `data/xbreed-schema.sql` (14 LoC) — WAL journal mode; 5-field `findings` table (mission, round, axis_id, observable, direction with `+|-|0` check); `idx_mission_round` index.
- `~/.claude/agents/the-judge.md:18` — Phase-0 hook reads the substrate with inline `sqlite3 -readonly` (latest-mission lookup via `ORDER BY round DESC, rowid DESC LIMIT 1`, then top-20 findings). Hard rule on the hook: non-empty stdout → inject context; empty → silent skip. **Honcho fallback removed** (R2 advisor option-b).

### 1.3 Honcho upstream (primary-source deep-read)

Read in full or substantively:
- `honcho/CLAUDE.md` — overview + architecture + three-agent taxonomy (Deriver / Dialectic / Dreamer) + project structure.
- `honcho/src/routers/conclusions.py` (155 lines) — `/conclusions` (create), `/conclusions/list` (paginated filter), `/conclusions/query` (semantic search), `/conclusions/{id}` (delete).
- `honcho/src/routers/peers.py` (456 lines) — `/peers/*` incl. `/chat`, `/representation`, `/context`, `/search`, `/card`.
- `honcho/src/schemas/api.py` (698 lines) — Pydantic request/response schemas for the public API contract.
- `honcho/src/utils/filter.py` (601 lines) — the advanced-filter DSL (AND/OR/NOT, comparison ops, JSONB contains).
- `honcho/src/crud/document.py` (1112 lines) — `query_documents`, `create_observations`, `fetch_documents_by_ids`, `_query_documents_pgvector`, external-vector-store dispatch, duplicate detection, soft-delete reconciliation.
- `honcho/src/crud/representation.py` (506 lines) — `RepresentationManager.get_working_representation` (three-way blend: semantic + most_derived + recent).
- `honcho/src/deriver/consumer.py` (371 lines) — queue dispatch for representation / summary / dream / deletion / reconciler task types.
- `honcho/src/deriver/deriver.py` (first 120 lines) — `process_representation_tasks_batch`: single LLM call (`minimal_deriver_prompt`) extracts explicit + deductive observations from messages, saves to per-collection documents.
- `honcho/src/dialectic/chat.py` (141 lines) — `agentic_chat` / `agentic_chat_stream`: construct `DialecticAgent` with observer/observed/session + peer cards, `agent.answer(query)` returns NL string.

### 1.4 Wiki anchors (memory-management corpus)

- `anthropic-effective-context-engineering.md` — context-rot; the three levers (altitude, minimal tool overlap, curated examples); just-in-time retrieval; three long-horizon techniques (compaction, structured note-taking, sub-agent arch).
- `effective-harnesses-long-running-agents.md` — initializer vs coding-agent split; the 200+ feature list with `passes`-booleans as irreversible durable-state control primitive; one-feature-at-a-time structural defense against one-shotting.
- `weng-llm-agents.md` — canonical 3-pillar taxonomy (Planning + Memory + Tool Use); human memory ↔ agent mapping (sensory / short-term context / long-term vector store); MIPS as retrieval primitive.
- `anthropic-managed-agents.md` — pets-vs-cattle decoupling; **"the session guarantees durability and interrogability; the harness owns arbitrary context management"** (load-bearing for this audit); session-log as durable-context-object outside window.
- `chase-long-horizon-agents.md` — model/framework/harness taxonomy; traces-as-source-of-truth; file-system-pilled; "memory is what lets the system reduce developer iteration cost — not a feature for users, a feature for builders."
- `karpathy/builder-philosophy.md` — NOT-READABLE (file does not exist at that path in wiki; karpathy branch has `karpathy-autoresearch.md` + `karpathy-recipe-neural-networks.md` but no `builder-philosophy.md`). The handout reference is stale or the file was renamed. Substitute: wwkd SKILL body (core principles 1, 6, 9) already loaded.

### 1.5 User / durable memory

- `~/.claude/projects/.../memory/user_trendsetter_principle.md` — verbatim: *"xbreed's runtime + xask protocol + Pareto-walk discipline are load-bearing. External tools get evaluated by one test: does it serve us as-is, or does it force us to adapt?"* — hard-gate. Adaptation red-flag list enumerated: (a) client-side capability patch, (b) forced schema shape, (c) new runtime deps, (d) persona behavior changes, (e) contract-quirk learning. Any of these → REJECT.
- `MEMORY.md` index — auto-loaded; all other memory entries present in context (see session prelude).

---

## 2. What surprised me (primary-source divergences)

### 2.1 R2 m16 "MOOT" verdict is primary-source REFUTABLE (but only partially)

**What R2 claimed:** `/conclusions/query` has no server-side mission scoping; all records across missions returned regardless of query string; endpoint-swap is moot once Honcho is rejected.

**What the code actually says:**

- `ConclusionQuery` schema (`schemas/api.py:457-476`): `query: str`, `top_k`, `distance`, `filters: dict[str, Any] | None` — the `filters` field is the server-side scoping surface and it is FULLY wired.
- `/conclusions/query` handler (`routers/conclusions.py:88-124`): requires `observer`/`observer_id` + `observed`/`observed_id` (ValidationException at `:109-112`), then calls `crud.query_documents(..., filters=body.filters, ...)` at `:114-123` passing the rest of the filter dict through.
- `query_documents` (`crud/document.py:312`): filters are applied on both paths — pgvector (`:303` via `apply_filter`) and external vector store (`:230-234` hardcodes `["level", "session_name"]` pre-filter; `:270` `fetch_documents_by_ids` REAPPLIES full filter post-fetch so even on the external-store path, the returned set IS server-side filtered; the vector-store pre-filter is a performance optimization, not a correctness gate).
- `apply_filter` DSL (`utils/filter.py:59-106`): AND/OR/NOT, comparison operators (gte/lte/gt/lt/ne/in/contains/icontains), wildcard, JSONB metadata contains. Document-class column mapping (`:50-56`): `session_id` (→ `session_name`), `workspace_id`, `observer_id`, `observed_id`, `metadata` (→ `internal_metadata`).

**Verdict:** `session_id` is a **native, first-class, server-side filter key on `/conclusions/query`**. If an xbgst mission is the `session_id`, then `POST /v3/workspaces/{ws}/conclusions/query` with body `{"query":"...","filters":{"observer_id":"the-judge","observed_id":"the-judge","session_id":"honcho-reaudit-0418"}}` performs mission-scoped semantic search server-side with zero client patching.

**BUT (a real gap remains):** `ConclusionCreate` (`schemas/api.py:479-509`) has fields `content, observer_id, observed_id, session_id` only. **There is no user-supplied `metadata` field on conclusion creation.** `create_observations` (`crud/document.py:769,781`) hard-codes `internal_metadata={}` for user-created conclusions — the metadata column exists but is not addressable from the API for manual creates. So richer dimensions (axis_id, round, direction, confidence, audit_hash) **cannot be stored as server-filterable metadata**; they can only live inside the `content` string. Filtering them would require `content.icontains(...)` (which is supported) or parsing the string client-side (which trips the trendsetter gate).

### 2.2 `/representation`'s "silently drops queries" claim is semantically trivial — wrong field name

R2 and R1 both observed: `/peers/{peer}/representation` silently drops the `queries` filter. Primary source: `PeerRepresentationGet` (`schemas/api.py:175-208`) has NO `queries` field. The correct field is `search_query: str | None` (singular, not a list). Pydantic silently ignores unknown input fields. This isn't a capability gap — it's a field-name mismatch in the probe. The `search_query` field is fully wired through to `_query_documents_semantic` → `crud.query_documents` with the same filter DSL behind it. The wrapper that R2 probed sent `{"queries":["..."]}` and the correct shape is `{"search_query":"...","search_top_k":N,"search_max_distance":0.X}`. Observation unchanged for our purposes: the wrapper needs rebuilding regardless; the R2-derived finding "representation is a blind dump" is tool-error, not server gap.

### 2.3 The deriver pipeline is thinner than advertised

`honcho/CLAUDE.md` describes a three-agent taxonomy (Deriver with 6 tools + Dialectic with 9 tools + Dreamer). The actual in-clone code is simpler: `deriver/deriver.py:process_representation_tasks_batch` is a single `honcho_llm_call` with a minimal prompt that returns explicit + deductive observations from messages. The async-reasoning pipeline's *architectural* claim (separate Deriver agent with tool-use) exists in the `CLAUDE.md` prose but not visibly in the shipped code (the clone is an older/simpler checkpoint; the `agent/` subdirectory referenced in `CLAUDE.md` does not exist in this clone). For our purposes: the *cost* of driving memory through deriver-pipeline is one LLM call per message batch, **plus** embedding generation and pgvector/external-store writes. The async-pipeline's smart-ingest path is real but bypassable — we already bypass it by using `/conclusions` direct create (which skips the deriver entirely per `crud/document.py:688-891`).

### 2.4 Dialectic is a tool-using NL agent — the 1898ms cost is credible

`dialectic/chat.py:20-78` — `agentic_chat` constructs a `DialecticAgent` with observer/observed/session + peer cards, calls `agent.answer(query)`. The agent presumably runs a tool-calling loop (search_memory / get_recent_observations / etc. per `CLAUDE.md`). A full tool-loop latency of ~2s is consistent with what R2 empirically measured. This endpoint is probabilistic by construction; it's the "intent" surface, not the "store" surface.

### 2.5 Managed-Agents pattern fits this decision cleanly

The Anthropic Managed Agents post (`anthropic-managed-agents.md`) names the decoupling: **"session guarantees durability and interrogability; harness owns arbitrary context management."** This maps directly onto the question: SQLite (authoritative, durable, structurally queryable) is the session-log layer; Honcho's deriver/dialectic would be the harness-layer context management (semantic recall, NL synthesis). They are not substitutes — they are *complementary* layers if we choose to run both. The architectural question is whether the harness layer earns its place at xbreed's scale.

### 2.6 The "scribe as durable-state owner" pattern is already in-house

`effective-harnesses-long-running-agents.md` feature-list-with-`passes`-boolean and `chase-long-horizon-agents.md` traces-as-source-of-truth both point to: **durable, irreversibly-monotonic, append-only state as the control primitive.** xbreed's `docs/reports/*.md` scribe artifacts already play that role — one commit per round, one markdown per mission, git log as the linear history. Our SQLite `findings` table is the structured mirror of the same data, not a substitute. Neither needs Honcho to do its job.

---

## 3. Honcho methods inventory — probed vs. unprobed

Mapping every relevant endpoint against what prior missions touched.

| Endpoint | Method | Purpose | Probed in honcho-judge-0418? | Probed in honcho-stress-0418? | Re-audit coverage |
|---|---|---|---|---|---|
| `POST /v3/workspaces/{ws}/peers` | get-or-create peer | wrapper init | YES (R1 cdx-labrat-api) | YES | primary-source verified |
| `POST /v3/workspaces/{ws}/sessions` | get-or-create session | ephemeral grouping | YES (R1) | NO direct probe | handout calls out "session-scoped vs peer-scoped retrieval semantics" as unmapped — this IS the mission-scoping answer |
| `POST /v3/workspaces/{ws}/conclusions` | batch create conclusions | direct write of pre-formed facts | YES (R2 Q labrat) | YES (R1/R2 BUG-F1) | schema limitation confirmed (no metadata) |
| `POST /v3/workspaces/{ws}/conclusions/list` | paginated filter-based list | list with filter DSL | NO | partial (R2 migration script) | **UNPROBED — the `filters` dict supports full DSL incl. `session_id`** |
| `POST /v3/workspaces/{ws}/conclusions/query` | semantic search with filters | primary scoped retrieval path | NO | YES (R2 Q labrat with filters dict, but exact shape not in report) | **R2 probe not shown sending `{filters:{session_id:...}}` — if not tested, empirical "absent" claim is inconclusive** |
| `DELETE /v3/workspaces/{ws}/conclusions/{id}` | delete conclusion | cleanup | YES (R2 migration draft) | YES | n/a to re-audit |
| `POST /v3/workspaces/{ws}/peers/{peer}/chat` | agentic NL dialectic | probabilistic NL Q&A | NO | YES (R2 labrat — 1898ms, confabulates) | mechanism read — tool-using agent; cost credible; not deterministic; rejected for authoritative paths in R2 |
| `POST /v3/workspaces/{ws}/peers/{peer}/representation` | curated subset of knowledge | NL representation render | YES (R1 — BUG-G1/G4 probe-errors) | YES | field-name mismatch explained (§2.2); not a capability gap |
| `GET /v3/workspaces/{ws}/peers/{peer}/context` | rep + peer_card combined | one-shot context fetch | NO | NO | **UNPROBED** — convenience endpoint; not load-bearing for recall decision |
| `POST /v3/workspaces/{ws}/peers/{peer}/search` | message search | NL over messages, not conclusions | NO | NO | **UNPROBED** — messages tier, not conclusions tier; orthogonal to judge-phase-0 need |
| `POST /v3/workspaces/{ws}/messages` | message ingest (deriver input) | async-pipeline entry | NO | NO | **UNPROBED** — intentionally; deriver pipeline is the bypassable smart-ingest layer (§2.3) |
| `POST /v3/workspaces/{ws}/peers/{peer}/sessions` | list sessions per peer | mission enumeration | NO | NO | **UNPROBED** — could enumerate missions via the session list if we adopt session-as-mission |
| `PUT /v3/workspaces/{ws}/peers/{peer}/card` | peer card CRUD | small structured durable doc | NO | NO | **UNPROBED** — peer_card is a list[str] attached to an observer/observed pair; candidate for small highly-structured state |
| reconciler pipeline (`src/deriver/consumer.py`) | vector-sync + cleanup | server-side housekeeping | n/a | n/a | opaque but self-maintaining — not a caller concern |
| dreamer pipeline (`src/dreamer/`) | observation consolidation | memory deduplication | NO | NO | **UNPROBED** — async; only runs on messages, not direct conclusions; orthogonal to authoritative recall |

**Bottom-line unprobed surface:** (a) `session_id` as a filter key on `/conclusions/query` (the crux of §2.1), (b) `conclusions/list` filter DSL (same DSL, synchronous, paginated), (c) `/peers/{peer}/context` one-shot, (d) `/peers/{peer}/sessions` for mission enumeration, (e) peer_card as small structured durable doc, (f) the deriver-async-messages path (intentionally skipped — bypassable).

**Lowest-risk falsifier for R2's empirical claim:** one curl:

```bash
curl -sS -X POST "$HONCHO_BASE_URL/v3/workspaces/xbreed-judge/conclusions/query" \
  -H "Authorization: Bearer $HONCHO_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"query":"round 1 survivors","top_k":10,"filters":{"observer_id":"the-judge","observed_id":"the-judge","session_id":"honcho-stress-0418"}}'
```

If this returns only records whose stored `session_id` is `honcho-stress-0418` → R2's empirical claim is FALSIFIED; session-as-mission works server-side. If it returns cross-session records or 422s → R2 stands; the filter key is documented but not applied, and we're back to client-side patching.

---

## 4. Adapted position — candidate Honcho roles (enumerated, not ranked)

Given what we have now (SQLite-WAL substrate operational, 5-field schema, xbreed-memory wrapper, R3 commits all landed, Honcho wrapper removed, the-judge.md:18 hook SQLite-only), the question is: does a **narrower** Honcho role pass the trendsetter hard-gate?

Candidate roles, with hard-gate pre-check:

### Role R0 — "No role"
**Shape:** SQLite remains sole memory substrate. Honcho layer stays out entirely. Close the question.
**Trendsetter gate:** passes trivially (no integration).
**What's lost:** semantic retrieval; NL summarization of prior missions; cross-mission pattern recall via vector similarity.
**What's preserved:** determinism; single-process; zero network dep; scribe-reports + auto-memory cover most recall needs; trendsetter principle fully honored.
**When this is right:** if the xbgst round concludes that the harness layer (semantic / NL) does not justify an additional moving part.

### Role R1 — "Sidecar: NL audit-only, read-only"
**Shape:** scribe writes a one-line NL summary of each round's survivors as a Honcho conclusion (observer/observed = `the-judge`, `session_id` = mission_id, `content` = NL summary string). Reads go through `/conclusions/query` with `filters.session_id` for cross-mission semantic recall. **SQLite remains authoritative**; Honcho is a queryable append-only audit index, never a read path for Phase-0 hook.
**Trendsetter gate:** passes IF §2.1 falsifier confirms session-as-mission scoping works AND we only store `content=<NL prose>` (no schema reshape on our side).
**Fails gate:** if we need to embed axis_id/round/direction for filtering beyond session and have to post-parse `content` strings.
**What's gained:** semantic cross-mission recall ("what did we learn last time we hit a rowid-DESC tiebreak issue?"); NL summaries queryable by similarity.
**What's added:** network dep; one HTTP call per round-close; API key dependency; second durable store to keep in-sync (or explicitly accept as lossy mirror).

### Role R2 — "Sidecar: scribe-to-Honcho one-way mirror"
**Shape:** same as R1, but we treat Honcho purely as a backup / discoverable NL index; nothing in xbreed reads from it. Human reads via Honcho console or occasional ad-hoc query for prior-art discovery. the-judge never calls it.
**Trendsetter gate:** passes trivially — one-way write, zero integration on the read side.
**Usefulness:** questionable. If nothing in xbreed reads it, the-judge gets no benefit; user gets a queryable index they could also get from grepping `docs/reports/`.
**Net:** this role is hard to justify unless the user specifically wants an ad-hoc semantic NL index for their own use outside the agent loop.

### Role R3 — "Write-only audit trail, deriver-fed"
**Shape:** use `/messages` + deriver pipeline to let Honcho auto-extract observations from scribe reports. We POST the scribe markdown as one `MessageCreate`, Honcho's deriver extracts observations asynchronously into its own knowledge graph, and we can later `/chat` it ("what patterns emerged across all missions involving mailbox.rs?").
**Trendsetter gate:** passes on the write side (native shape); on the read side, `/chat` is probabilistic and already rejected in R2 for authoritative paths.
**Usefulness:** high-ceiling (real semantic cross-mission inference), low-floor (probabilistic, 1898ms, confabulates on sparse data). Only justifies itself if cross-mission pattern-mining becomes a recurring human need.
**Cost:** server-side LLM calls via deriver (Honcho's provider, not our OAuth); embedding generation; dependency on Honcho's reasoning stability.

### Role R4 — "Peer-card as structured durable doc"
**Shape:** use the peer_card (`list[str]` attached to observer/observed pair) as a small structured durable state — analogous to the `passes`-boolean feature list in `effective-harnesses-long-running-agents.md`. xbreed writes a per-mission peer_card update on round close (`list[str]` of axis-direction strings).
**Trendsetter gate:** ambiguous — peer_card is native Honcho shape; using it as mission-state-carrier is semantic rebinding, similar to session_id-as-mission. Passes if no client-side post-processing is needed.
**Usefulness:** limited — peer_card is a single list, not a history; it would be overwritten per-mission, losing the append-only invariant. Could be kept as "latest-mission state" only.
**Net:** weaker than R1; peer_card is a poor fit for mission-history.

### Role R5 — "Semantic enrichment of SQLite rows"
**Shape:** keep SQLite authoritative. On the-judge Phase 0, after pulling the 20 structural findings from SQLite, fire a best-effort `/conclusions/query` with the current mission description as the query string + `filters.session_id` set to recent missions, merge top-K NL results into the prompt as "related prior observations." Hard-gated: if Honcho returns nothing/errors/times out, skip silently.
**Trendsetter gate:** passes on the same basis as R1 (session-as-mission mapping, no client patching).
**Usefulness:** directly benefits Phase 0 context quality with relevant prior-mission prose. Failure-isolated: SQLite is the floor, Honcho is pure upside.
**Cost:** one additional HTTP call per Phase 0 (~500ms semantic path). Requires the write path from R1 to have populated content over time.

---

## 5. Trendsetter-gate standing matrix

Applied to each candidate role; any ✗ disqualifies.

| Role | (a) client-side capability patch | (b) forced schema shape | (c) new runtime deps | (d) persona behavior change | (e) contract-quirk learning |
|---|---|---|---|---|---|
| R0 no-role       | n/a ✓ | n/a ✓ | n/a ✓ | n/a ✓ | n/a ✓ |
| R1 NL sidecar    | ✓ (if §2.1 falsifier confirms) | ✓ (content=prose, session_id=mission native fields) | ✗ curl dep (already have) but no SDK | ✓ | ✓ tolerable |
| R2 one-way mirror | ✓ | ✓ | same as R1 | ✓ | ✓ |
| R3 deriver-fed   | ✓ write side / ✗ read side (probabilistic) | ✓ | same | ✓ | ✗ non-determinism is a contract quirk |
| R4 peer-card     | ✓ | semantic rebind; ambiguous | same | ✓ | ✓ |
| R5 semantic enrich | ✓ (if §2.1 falsifier confirms) | ✓ | same | ✓ | ✓ |

**R0, R1, R2, R4, R5 are gate-passing candidates.** R3 trips the gate on (e) for read side — the `/chat` probabilistic confabulation is the exact class of contract quirk the principle guards against.

---

## 6. Open questions for the xbgst round (one per proposed axis)

These are the axis-level scoring questions the team should take to verdict. **Every axis must score the question THROUGH the trendsetter gate.** Passing the gate is binary; it is not a tradeoff dimension.

- **F (falsifier) — empirical:** Does the curl from §3 (`{"filters":{"session_id":"honcho-stress-0418"}}`) return session-scoped results, or does it return cross-session records / 422? This single probe binary-decides whether R2's empirical ground #2 stands. → labrat.
- **R (role-inventory) — critic ACH:** Across {R0, R1, R2, R4, R5}, which roles score + on information-gain-per-cost AFTER the trendsetter gate? Include Heuer Layer-0 competing-hypotheses matrix. → critic (heuer L0 + codex -R).
- **W (wiki-mapping) — scout:** Which patterns from the wiki memory-management corpus (managed-agents session/harness split; Chase model/framework/harness; Weng long-term-memory-as-vector-store; context-engineering's three levers) map to Honcho's specific shape vs SQLite's shape? Does the load-bearing split (durable session log vs harness-owned context management) argue for keeping both? → scout (gemini medium, codex fallback on 429).
- **M (methods-deep-dive) — revenger:** For the unprobed surface in §3 table (`/conclusions/list` filter DSL, `/peers/{peer}/context`, peer_card, messages+deriver, dreamer), which are truly relevant to the judge orchestrator role vs which are orthogonal? Map the minimum probe-set. → revenger (codex -R).
- **S (substrate-integration) — reviewer:** If a role passes the gate (candidates: R1 / R5), what is the integration shape? Write-seam placement (post-SYNTHESIS_READY vs final DRAFT); read-seam placement (Phase 0 only, or post-Pareto-filter augment); failure-isolation (how does Honcho unavailability affect xbgst round flow); data-sync policy (accept lossy mirror vs enforce consistency). → reviewer (codex -R, "propose diff, do NOT apply").
- **C (challenge) — critic adversarial:** Is this audit premature? R3 SQLite substrate just landed; we have zero real-mission usage data. Should we ship 3-5 real missions first, collect observable pain (what recall failures? what scribe/auto-memory gaps?), and re-audit with data rather than re-auditing from theory? → critic (heuer L0 + codex -R).
- **T (trendsetter-gate audit) — critic:** Re-verify the gate itself against each candidate role, with attention to the nuance that session_id-as-mission is semantic rebinding vs schema patching. Does any candidate slip through the gate on a technicality that still violates the principle's spirit? → critic (heuer L0 + codex -R).
- **X (connector) — MANDATORY every round:** Cross-axis pattern — how do the surviving roles interact with the other 4 persistence surfaces (auto-memory, scribe reports, obsidian, git)? Is there a 6-surface clash that emerges only when Honcho is re-introduced in a narrower role? → connector (**gemini-high LOCKED** per `feedback_connector_gemini_high.md`; codex -R override ONLY if gemini 429 canary fails).
- **U (user-workflow-preservation) — structural:** Honor scribe-per-round + cdx-reviewer-diffs concurrent pattern (prior R2/R3 went judge-direct, which the memo flagged as a deviation). Advisor audit between rounds remains as user's explicit ask. Brief reviewers with "propose diff, do NOT apply" to prevent the R2 scope-violation pattern. → orchestration, not a scored axis.

---

## 7. Verdict this round is NOT permitted to pre-empt

Phase 0's job is orientation, not conclusion. The xbgst round that follows will Pareto-filter the candidate roles under the trendsetter gate. Two outcomes are pre-approved:

1. **"No role" (R0).** If gate-passing candidates fail to earn their place against R0 under Pareto scoring, document WHY with deeper grounding than R2 had — specifically, wiki-pattern mapping + the §3 unprobed-surface closure + the §2.1 falsifier outcome. Closes the question definitively.
2. **"Specific role X under shape Y".** If a candidate passes the gate AND earns a Pareto improvement under scoring, produce a minimal wwkd milestone-serial plan. SQLite substrate remains primary. Honcho role is additive only. Integration shape must be failure-isolated (SQLite can't regress on Honcho outage).

Anything else — "maybe", "pending more research", "implement and see" — is a Phase-0 failure and should route back to a second data walk.

---

## 8. Known constraints carried into the xbgst round

- **Gemini quota:** as of prior session close, 2h16m cooldown still pending. Canary first: `xask --effort low gemini "ping" 2>&1 | grep -qv "RESOURCE_EXHAUSTED\|429"`. If 429, X-axis connector falls back to `xask -R codex` (gpt-5.4) per R2 override precedent.
- **tmux pane cap:** R2 tripped it at 22+ panes. Graceful path: `SendMessage shutdown_request` individually (broadcast `*` doesn't accept structured payloads), wait for `shutdown_approved`, then `TeamDelete`. Force: `tmux kill-pane` + `xbreed-cleanup <team> --force` + `TeamDelete`. TeamDelete MANDATORY in both paths.
- **Pattern-preservation:** scribe per round (`ccs-scribe-r{N}`) + `cdx-reviewer-diffs-r{N}` concurrent. Advisor audit between rounds per user's explicit ask. Reviewer briefs must carry *"propose diff, do NOT apply"* to prevent the R2 executor-via-reviewer scope-violation.
- **Distiller + scribe context-gap:** spawned sub-agents do NOT inherit main-session transcript. Judge forwards all verbatim proposals in the distiller brief as a single payload; scribe briefs carry enough teammate-name roster to prevent hallucination.
- **Honcho wrapper is REMOVED** (`scripts/honcho` and `scripts/honcho-migrate-r2.sh` deleted at R3 commit `e351539`). If R1/R5 lands, the wrapper rebuilds from scratch at the wwkd-plan stage; do NOT resurrect the old 68-LoC wrapper that had 6+ documented bugs.
- **Live state:**
  - `xbreed-judge` Honcho workspace — 9 records, 5 broken, 3 valid, 1 stray (unchanged since R1).
  - `xbreed-stress-0418` Honcho workspace — 3 `the-judge`-peer records (R2 Q labrat probes) + 4 probe-peer records (R1 scribe probe); isolated.
  - `data/xbreed.db` — seeded from R3; the-judge Phase 0 hook reads from it on session start.
  - If R1/R5 lands, decide in the wwkd plan: migrate the 3 valid old records to a new mission-namespaced `session_id`, or write them off as pre-gate debris.

---

## 9. Audit trail

| item | value |
|---|---|
| prior R1 commit | `c687d22` |
| prior R2 commit | `d161e6d` / `2d30b8f` (R3 M5 + handout) |
| handout | `docs/handouts/honcho-memory-reaudit-handout.md` |
| this artifact | `docs/reports/honcho-reaudit-phase0-2026-04-18.md` |
| honcho clone | `honcho/` (gitignored; read at its current HEAD in the clone) |
| key upstream files read | `src/routers/conclusions.py`, `src/routers/peers.py`, `src/schemas/api.py`, `src/utils/filter.py`, `src/crud/document.py`, `src/crud/representation.py`, `src/deriver/consumer.py`, `src/deriver/deriver.py` (first 120 lines), `src/dialectic/chat.py` |
| wiki anchors consulted | anthropic-effective-context-engineering, effective-harnesses-long-running-agents, weng-llm-agents, anthropic-managed-agents, chase-long-horizon-agents |
| wiki file NOT found | `karpathy/builder-philosophy.md` — handout reference is stale or file was renamed |
| author | judge-direct (claude-opus-4-7, xhigh) |

---

*Phase 0 complete. xbgst dispatch may proceed AFTER this artifact is committed (handout hard-gate). Axis framing in §6 is the input to the round; trendsetter gate is the hard-gate every axis scores through; the §3 falsifier probe is the single empirical move that pivots R2's ground #2.*
