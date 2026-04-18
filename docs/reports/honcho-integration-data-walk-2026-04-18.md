# Data Walk — Honcho v3 Memory Integration with the-judge

**Mission:** honcho-judge-0418
**Date:** 2026-04-18
**Posture:** wwkd Phase 0 (data walk BEFORE plan)
**Scope:** MVP integration of Honcho v3 memory with `the-judge` orchestrator only. Scribe/planner/distiller deferred per user directive ("use sparingly").

## What I looked at

### Honcho v3 API (external)

Source: `https://docs.honcho.dev/llms.txt` (endpoint index), plus targeted fetches of quickstart, peer.chat reference, and conclusions reference.

**Base URL:** `https://api.honcho.dev`

**Auth:** `Authorization: Bearer <API_KEY>` (HTTPBearer security scheme)

**Primitives:** workspace → peer → session → message → (async) representation; plus conclusions (direct semantic-searchable facts).

**Endpoints relevant to MVP:**

| Op | Method + Path | Body shape |
|---|---|---|
| Get-or-create workspace | `POST /v3/workspaces` (shape inferred — to be verified by cdx-labrat-api) | `{id, metadata?}` |
| Get-or-create peer | `POST /v3/workspaces/{ws}/peers` | `{id, metadata?}` |
| Get-or-create session | `POST /v3/workspaces/{ws}/sessions` | `{id, metadata?}` |
| Write conclusion(s) | `POST /v3/workspaces/{ws}/conclusions` | `{conclusions:[{content, observer_id, observed_id, session_id?}]}` — batch up to 100, `content` 1–65535 chars |
| Peer chat (read) | `POST /v3/workspaces/{ws}/peers/{peer}/chat` | `{query, session_id?, target?, stream?, reasoning_level?}` — query 1–10000 chars, `reasoning_level ∈ {minimal, low, medium, high, max}` default `low` |
| Get representation | `GET /v3/workspaces/{ws}/peers/{peer}/representation` (scoped subset retrieval) | n/a |
| Query conclusions | `POST /v3/workspaces/{ws}/conclusions/query` (semantic search with `top_k`) | `{query, top_k?}` |

**Pre-flight (2026-04-18 06:57 UTC):**

```bash
$ curl -sS -w 'HTTP %{http_code}\n' -H "Authorization: Bearer $HONCHO_API_KEY" \
    "https://api.honcho.dev/v3/workspaces?page=1&size=1"
HTTP 405
body: {"detail":"Method Not Allowed"}
```

Interpretation: auth handshake valid (not 401/403); GET not supported on that path (workspace creation is POST-only). Endpoint-shape discovery deferred to `cdx-labrat-api` probe.

### xbreed codebase (internal)

**Judge surface:** `~/.claude/agents/the-judge.md` (115 lines) — posture, sub-role dispatch table, drafting protocol, godspeed mode, handoff template. The judge runs in the CC main session. It does NOT go through `xbreed ask`/`src/ask.rs` — that path is for cross-model delegation (gemini/codex), not for the orchestrator itself.

**Integration target:** bash wrapper script in `scripts/`, not Rust. Matches the existing pattern of `scripts/xask` (228 LoC bash wrapper shelling to `xbreed ask` + native tools).

**Existing scripts:** `install-commands.sh`, `bench-*.sh`, `verify-docs.sh`, `xask`. Aesthetic = one-file bash, explicit flags, `set -u` discipline.

**Existing persistence surfaces (5 — Honcho will be #6):**

1. CC auto-memory: `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/` (MEMORY.md index + typed files: user/feedback/project/reference)
2. Obsidian vault: `/home/vhpnk/claudevlt/` (user-curated, agent-read-only per feedback_no_obsidian_mcp)
3. Per-round scribe reports: `docs/reports/*.md` (auditable trail, one per round per mission)
4. Git log + commit messages (one commit per round per `feedback_commit_per_round`)
5. (this file) wwkd data-walk artifacts

The connector axis (`g-connector-honcho`) is mapping the clash risks.

### Judge dispatch lifecycle (inferred from the-judge.md)

The xbgst round has these emittable phases:

1. **Phase 0** — name axes (direction + observable per axis), up to 8
2. **Phase 1** — assign deterministic teammate names
3. **Phase 2** — spawn with peer roster + xask gate + `| godspeed`
4. **Phase 3** — distiller synthesis → SYNTHESIS_READY + audit_hash → Pareto filter → round summary
5. **Exit** — true zero-axis-improvement OR 4-round cap → final DRAFT → auto-cleanup

**Candidate Honcho integration seams (to be narrowed by `cdx-reviewer-integration`):**

- (a) Phase 0 pre-axis query — judge calls `peer("the-judge").chat("recent xbgst missions and learned axis patterns for <topic>")` to ground axis selection
- (b) Post-SYNTHESIS_READY write — judge writes one conclusion per round capturing survivors + axis-observable state
- (c) Final DRAFT write — judge writes one "mission outcome" conclusion at auto-cleanup

User directive "use sparingly" → likely 2 of these, not 3. `cdx-reviewer-integration` picks.

## What surprised me

1. **Conclusions endpoint is direct + immediate** — not all Honcho writes go through the async reasoning pipeline. `POST /conclusions` accepts pre-formed structured facts. This maps *cleanly* onto xbgst's SYNTHESIS_READY block: the judge already produces a structured, reviewed, consensus-capped payload; conclusions is the natural persistence target. Async reasoning (messages → representation) is for *inferred* knowledge; conclusions is for *authoritative* ones.
2. **Workspace/peer are top-level + stable; sessions are ephemeral** — cross-mission peer reuse is the design intent. Our `the-judge` is a peer that persists across missions; each xbgst run is a session.
3. **HTTP 405 on GET `/v3/workspaces`** — the shape isn't purely REST-CRUD; get-or-create semantics are POST-first. Specialized helpers rather than resource-style listing.
4. **API-key auth format** is vanilla Bearer, despite the surrounding subscription/SDK framing — means a curl wrapper is viable with zero SDK deps.

## Spec ↔ reality divergences

- **llms.txt vs api-reference pages:** the llms index lists endpoints like `GET /v3/workspaces/list` via `get-all-workspaces.md`, but the pre-flight `GET /v3/workspaces?page=1&size=1` returned 405. The list endpoint may live at a different path; `cdx-labrat-api` will nail it down.
- **SDK-first framing vs direct HTTP:** quickstart strongly recommends SDKs. Our wrapper will be direct HTTP anyway (YAGNI on a language runtime dep). Trade-off accepted: if Honcho changes SDK semantics, wrapper may need updating; matches existing `scripts/xask` risk posture.

## Known constraints

- User is solo-dev; OAuth-everywhere (`user_oauth_exclusive.md`) — but Honcho is a separate service with its own API key (not OAuth). Key lives at `~/.config/xbreed/honcho.env`, mode 600, sourced by wrapper.
- No secrets in briefs (logged in team config) — teammates source the env file rather than get the key embedded.
- `feedback_no_safety_theater`: skip menu-style options, execute.
- `feedback_commit_per_round`: every xbgst round commits. Honcho work will follow same rule.
- The user's quote: "harmonizes perfectly with our protocol, especially if we assign it to the scribe, the judge, the planner, and the distiller — to use sparingly". MVP = judge only.

## Open questions for the team (being answered by Round 1 teammates)

- **A:** exact endpoint shape for workspace/peer/session get-or-create (cdx-labrat-api)
- **I:** exact file:line seams in the-judge.md for the Honcho hooks (cdx-reviewer-integration)
- **S:** scripts/honcho wrapper LoC target + subcommand set (ccs-simplifier-skeleton)
- **D:** which xbgst artifacts become conclusions vs get dropped (g-scout-schema)
- **R:** wwkd milestone sequence with bash gates (ccs-planner-recall)
- **W:** workspace_id resolution rule + peer naming (cdx-reviewer-workspace)
- **F:** failure-mode table + timeout budget (cdx-labrat-failure)
- **X:** clash map across the 5 existing persistence surfaces (g-connector-honcho)

Findings land as SendMessage replies; distiller synthesizes; judge applies Pareto filter; ccs-scribe-r1 writes the round report; commit; Round 2 unless zero-axis-improvement.
