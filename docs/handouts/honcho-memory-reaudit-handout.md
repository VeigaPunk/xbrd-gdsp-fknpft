# Handout — Honcho memory re-audit (next-session brief)

**For:** next Claude Code session operating on this repo as `the-judge`
**Prepared by:** prior session, mission `honcho-stress-0418` (commits `c687d22` → `2d30b8f`)
**Posture for next session:** `/xbgst /wwkd | godspeed` — but Phase 0 data walk FIRST (hard gate before dispatch)

---

## 1. What this handout exists to answer

**The core question:** *Is Honcho actually a good idea to implement for memory — anywhere in the xbreed stack, in any role?*

The prior mission pivoted to SQLite as the primary persistence substrate after the **trendsetter principle** (`~/.claude/projects/.../memory/user_trendsetter_principle.md`) was applied as a hard-gate: tools adapt to us, never the reverse. Honcho v3 was rejected on two independent grounds — (1) the principle itself (client-side mission filter = adaptation), and (2) empirical API gap (`/conclusions/query` has no server-side mission scoping; `/representation` silently drops the `queries` filter).

**The question to re-examine:** was that rejection a blanket dismissal that missed a narrower valid use case? Specifically:

- Does Honcho earn a role as a **sidecar** (NL-human-readable audit layer, not authoritative recall)?
- Does Honcho earn a role under a **different integration shape** (e.g., write-only append, or read-only across a one-way sync from our SQLite)?
- Are there **Honcho methods / endpoints / patterns we didn't probe** during the stress-test that would change the verdict? `/peers/{peer}/chat`, `/peers/{peer}/representation`, `/conclusions/list`, the async-reasoning pipeline via `/messages`, the `deriver` step, `session`-scoped vs `peer`-scoped retrieval semantics — what did we actually map vs what did we assume?
- Do the **wiki's memory-management references** surface patterns we haven't considered — ones where Honcho's specific shape (HTTP, peer-scoped, deriver-based) is actually the right fit?

Trendsetter principle still applies. The question isn't *"let's accommodate Honcho"* — it's *"did we dismiss Honcho from a position of thin information, and would deeper research change the verdict?"* If Honcho still loses after a thorough audit, that's fine; the audit closes the question.

---

## 2. Phase 0 — WWKD data walk (hard gate BEFORE xbgst dispatch)

Load the `wwkd` skill on first turn. Phase 0 enumerates actual state before any plan is written. No axes, no teammates, no dispatch until Phase 0 has produced a **data-walk artifact** at `docs/reports/honcho-reaudit-phase0-<DATE>.md`.

### 2.1 In-repo artifacts to read (chronological order)

- `docs/reports/honcho-integration-data-walk-2026-04-18.md` — original Phase 0 from `honcho-judge-0418` (the data walk BEFORE the implementation attempt)
- `docs/reports/honcho-judge-r1r2r3-2026-04-18.md` — consolidated prior-mission report: Schema v2 spec (11 fields), the "H2-done-right" framing, rejected alternatives, API surface mapped at that time
- `docs/reports/honcho-stress-0418-r1-2026-04-18.md` — stress-test Round 1: 6 confirmed bugs + architectural RETHINK
- `docs/reports/reviews/honcho-stress-0418-r1-review.md` — synth-reviewer's independent 2nd opinion on R1 synthesis
- `docs/reports/honcho-stress-0418-r2-2026-04-18.md` — stress-test Round 2: trendsetter gate applied, SQLite picked, 10 Pareto moves, Honcho rejected on 2 grounds

### 2.2 Current wrapper + substrate state

- `scripts/xbreed-memory` — 50-LoC bash wrapper over `data/xbreed.db` (SQLite WAL, busy_timeout=5000). Put/get subcommands per the exit-code contract. Read this to understand what "we currently have" looks like.
- `data/xbreed-schema.sql` — schema v1-minimal (5 fields: mission, round, axis_id, observable, direction). This is the shape we've committed to; any Honcho role must coexist with or complement it, not replace it.
- `~/.claude/agents/the-judge.md` line 18 — the Phase-0 hook that READS the substrate on judge bootup. Currently SQLite-only; no Honcho fallback.

### 2.3 Honcho upstream (cloned, gitignored)

- `honcho/` in this repo — local clone of Honcho v3 upstream. **Read the deriver code, the `/peers/{peer}/chat` reasoning pipeline, and the `/conclusions/query` filter implementation directly** to verify / refute the R2 empirical finding that server-side mission scoping is absent. Prior mission only tested the API surface; the deeper question is whether the underlying engine supports what we need and just doesn't expose it.
- `~/.config/xbreed/honcho.env` — API key + base URL (mode 600, outside repo). Still valid if live probing is needed.
- `https://docs.honcho.dev/` — official docs. Prior mission hit the top-level surface; deeper sections (changelog, SDK reference, deriver internals) may contain patterns we didn't survey.
- `https://github.com/plastic-labs/honcho` — upstream repo. Commit `9676526` is where our prior mission anchored the `ConclusionCreate.content` str-type verification.

### 2.4 Wiki anchors — memory management for long-running agents

The wiki (`~/wikillm/llm-wiki/wiki/`) has a curated set of references on agent memory and context engineering. Phase 0 MUST include at least these:

- `ai/agents-and-engineering/anthropic-effective-context-engineering.md` — Anthropic's own patterns on context curation (likely directly relevant to what "memory" means here)
- `ai/agents-and-engineering/effective-harnesses-long-running-agents.md` — long-horizon agent harnesses; memory is the load-bearing concern
- `ai/agents-and-engineering/chase-long-horizon-agents.md` — long-horizon-specific patterns
- `ai/agents-and-engineering/weng-llm-agents.md` — Lilian Weng's foundational LLM-agent memory taxonomy (working/episodic/semantic memory, retrieval patterns)
- `ai/agents-and-engineering/anthropic-managed-agents.md` — managed-agent memory lifecycle
- `ai/agents-and-engineering/anthropic-multi-agent-research.md` — if multi-agent memory shape differs from single-agent
- `ai/agents-and-engineering/building-effective-agents.md` — if it touches persistence
- `karpathy/builder-philosophy.md` — the code-is-ephemeral / knowledge-is-permanent principle (informs the substrate question at architectural level)

Phase 0 data walk should include: what patterns does the wiki name for agent memory? Which of those map to SQLite? Which to HTTP-memory-as-a-service? Which don't map to either? **Is the `the-judge` role, as orchestrator of xbreed Pareto rounds, structurally more like a stateless reducer (needs minimal recall) or a stateful agent (needs rich episodic memory)?** The wiki's framing may sharpen the question.

### 2.5 Prior auto-memory (cross-session context)

- `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/user_trendsetter_principle.md` — user's principle as hard-gate
- `~/.claude/projects/.../memory/MEMORY.md` — index of all feedback/project/reference memories (auto-loaded at session start)
- Prior mission's open questions from `honcho-judge-r1r2r3-2026-04-18.md` §"Open questions for next mission" — some deferred items may now be in scope

### 2.6 Phase 0 deliverable

Before ANY xbgst dispatch, produce `docs/reports/honcho-reaudit-phase0-<YYYY-MM-DD>.md` containing:

- **What I looked at** — every file / URL actually read, not "I'll look at X"
- **What surprised me** — spec-vs-reality divergences, incomplete prior-mission probes, wiki patterns prior mission didn't consider
- **Honcho methods inventory** — actual list of endpoints + what each does + which were probed during `honcho-stress-0418` vs which weren't
- **Adapted position** — given everything we've built (SQLite substrate + xbreed-memory wrapper + 5-field schema), what role *could* Honcho plausibly play? Write out the candidate roles explicitly so the xbgst round has axes to score against.
- **Open questions for the team** — exactly what the R1 Phase 0 artifact did; one line per axis the xbgst round will cover.

---

## 3. xbgst / godspeed dispatch (after Phase 0 gate passes)

Once the Phase-0 artifact is written and committed, dispatch xbgst/godspeed with axes named against the **candidate-Honcho-roles** surface, not against "fix the old bugs." The prior mission already covered bug surface; this audit is about **role scoping**.

### 3.1 Candidate axes (examples, not prescriptive)

- **R** — candidate role inventory (ACH: sidecar / audit-only / write-only / read-only-one-way / no-role). Heuer Layer-0 critic.
- **M** — Honcho method deep-dive: endpoints not probed (`/messages`, `/chat`, `deriver` pipeline, session vs peer scoping). Revenger + labrat.
- **W** — wiki-pattern mapping: which memory-management patterns from the wiki map to Honcho's specific shape vs SQLite's shape. Scout.
- **T** — trendsetter-gate re-verification: does any candidate role pass the hard-gate, or do they all still fail? Critic.
- **S** — substrate-integration: if a role passes, how does it cohabit with SQLite-first? One-way sync? Async derive-then-write? Reviewer.
- **X** — cross-axis connector (MANDATORY). gpt-5.4 via xask -R codex since gemini quota may still be exhausted — verify canary first.
- **C** — adversarial challenge: is this audit premature (should we ship R3's SQLite for a few real missions first and re-audit when we have usage data)? Critic + heuer.
- **U** — user-workflow-preservation: the prior mission went judge-direct on scribe/synth-reviewer for R2+R3; this round should honor the user's original `scribe + cdx-reviewer-diffs` concurrent pattern. Scribe.

Cap: ≤8 proposer axes + distiller + scribe + synth-reviewer concurrent. Connector is MANDATORY every round.

### 3.2 Trendsetter principle as standing gate

Every axis scores through the hard-gate. Any candidate role that requires client-side patching of a Honcho capability gap is disqualified regardless of other merits. The gate is not re-evaluable within this round — it's durable user preference persisted in memory.

### 3.3 Pattern-preservation the prior mission skipped

Previous session went judge-direct for R2+R3 scribe/synth-reviewer (documented in `docs/reports/honcho-stress-0418-r2-2026-04-18.md` §authoring note). The user's original directive was `scribe per round + cdx-reviewer-diffs concurrent`. Honor that pattern in this round — spawn both, don't collapse into advisor audit by default. Advisor call between rounds is still the user's explicit ask for memory-quality audit; add it to that protocol.

---

## 4. Known state + constraints

- **SQLite substrate is operational.** `data/xbreed.db` exists (gitignored); `scripts/xbreed-memory` put/get works; the-judge.md:18 hook reads from it. The seeded findings from R3 will surface to your Phase-0 hook on bootup — that's a *feature* of the prior mission, not a constraint to work around.
- **Honcho wrapper code removed** (`scripts/honcho`, `scripts/honcho-migrate-r2.sh` gone at commit `e351539`). If this audit concludes a Honcho role is worth adopting, the wrapper needs to be rebuilt — don't just resurrect the old 68-LoC HTTP wrapper, which had 6+ bugs documented in R1.
- **`xbreed-judge` workspace is stale** (9 records, 5 broken). Untouched — no active reader. If Honcho gets a role, deal with this as part of that role's migration story.
- **Gemini quota** — at end of prior session, 2h16m cooldown was in effect. Verify health canary before dispatching g-lane teammates: `xask --effort low gemini "ping" 2>&1 | grep -qv "RESOURCE_EXHAUSTED\|429"`. If still 429, connector goes on gpt-5.4 via `xask -R codex` (the user explicitly overrode the memory-locked `g-connector-r1 gemini-high` rule in the prior mission; use the same override if needed).
- **tmux pane cap** — prior session tripped it once (22+ idle panes). Shutdown path: `SendMessage shutdown_request` individually (broadcast `*` doesn't accept structured), wait for `shutdown_approved`, then `TeamDelete` as MANDATORY final step (per updated `feedback_team_cleanup_on_shutdown.md`). Force path: `tmux kill-pane` + `xbreed-cleanup <team> --force` + `TeamDelete`.

---

## 5. Do-not-repeat (lessons from prior session)

- **Distiller + scribe context gap.** Spawned agents do NOT inherit the main-session conversation transcript. If the distiller is told "synthesize the proposals in the inbox," it will not see them via team-broadcast; judge must forward verbatim as a single SendMessage payload. R2 hit this twice (distiller first pass degraded; scribe hallucinated teammate names). Mitigation options enumerated in `docs/reports/honcho-stress-0418-r1-2026-04-18.md` §11.5 — pick one upfront.
- **Reviewer scope violations.** A reviewer with xask -R codex can trigger the executor to *apply* edits, not just propose them. Prior mission's `cdx-reviewer-contract-r2` rewrote `scripts/honcho` and `~/.claude/agents/the-judge.md` in place. Brief reviewer teammates with explicit *"propose diff, do NOT apply"* to prevent this.
- **Line-count citation errors** (spoof_flagged in R2). Executors claimed their artifact sizes without `wc -l`-ing them. Distiller spoof-check caught it; keep the check armed.
- **Trendsetter gate comes from the user mid-round.** Be prepared to re-weight Pareto scoring post-proposal when a durable principle lands as a hard-gate. The `user_principle_weighting` section in the R2 synthesis is the template.

---

## 6. Artifact inventory (read-only, do not modify without cause)

| path | role |
|---|---|
| `docs/reports/honcho-integration-data-walk-2026-04-18.md` | original data walk (prior mission) |
| `docs/reports/honcho-judge-r1r2r3-2026-04-18.md` | original mission consolidated report |
| `docs/reports/honcho-stress-0418-r1-2026-04-18.md` | stress-test R1 |
| `docs/reports/honcho-stress-0418-r2-2026-04-18.md` | stress-test R2 (RETHINK pivot) |
| `docs/reports/reviews/honcho-stress-0418-r1-review.md` | synth-reviewer R1 |
| `scripts/xbreed-memory` | current substrate wrapper |
| `data/xbreed-schema.sql` | v1-minimal 5-field schema |
| `data/xbreed.db` | live db (gitignored; seeded from prior session) |
| `~/.claude/agents/the-judge.md` line 18 | the-judge Phase-0 hook (SQLite-only) |
| `honcho/` | upstream Honcho clone (gitignored) |
| `~/.config/xbreed/honcho.env` | Honcho API key (live, mode 600) |
| `~/wikillm/llm-wiki/wiki/ai/agents-and-engineering/` | memory-management reference corpus |
| `~/.claude/projects/.../memory/user_trendsetter_principle.md` | the hard-gate principle |

---

## 7. Success condition for this next session

**Phase 0 artifact** + **xbgst round producing a Pareto-filtered verdict** on *whether Honcho earns a role in the xbreed memory stack, and if so, which specific role under what integration shape*. The verdict must survive the trendsetter hard-gate.

If the verdict is "no role" — document *why* with deeper grounding than the prior mission had (specifically, wiki-pattern mapping + Honcho method deep-dive), so this question is closed and not re-opened next time.

If the verdict is "specific role X under shape Y" — produce a minimal implementation plan (wwkd milestone-serial) that preserves the SQLite substrate as primary and adds Honcho only in the named role.

Both outcomes are acceptable. The outcome to *avoid* is another round that handwaves on surface-level observations without the wiki + upstream-code grounding Phase 0 is gated on.

---

*Handout prepared 2026-04-18 as closure of mission `honcho-stress-0418`. Hand to next session as the Phase-0 gate + xbgst dispatch brief.*
