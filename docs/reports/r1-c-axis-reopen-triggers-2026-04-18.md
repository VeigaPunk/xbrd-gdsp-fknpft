# R1 C-axis reopen triggers

**Date:** 2026-04-18
**Purpose:** operationalize the four concrete conditions that re-open the "does xbreed need a second memory source?" question after real missions.
**Rule:** these are post-mission triggers, not speculative architecture prompts. A trigger only counts if it is observable in the shipped surfaces: SQLite memory, the-judge Phase-0 hook, or committed report artifacts.

---

## Monitoring points

- **Phase-0 floor recall:** `~/.claude/agents/the-judge.md:18` is the live hook. It reads only the latest mission and only the top 20 rows, then injects non-empty stdout into judge context.
- **SQLite inspection surface:** `scripts/xbreed-memory:20-21` binds the DB path, `:51-60` exposes the current Phase-0 floor (`get`, `latest-mission`), and `:62-79` exposes the audit helpers (`grep`, `redflags`).
- **Report-corpus precedent for missed nuance:** `docs/reports/test-brief-2026-04-16.md:233-260` shows a second pass adding three genuine gaps the main brief missed; `docs/reports/handoff-harness-0417-2026-04-17.md:135-138` records advisor catching three gaps the judge missed.
- **Report-corpus precedent for memory correction:** `docs/reports/shim-handoff-xbgst-r1-2026-04-17.md:191-197`, `:291-297`, and `:327` show an explicit "flagged for memory correction" pattern and the threshold for routing to judge + memory update.
- **Report-corpus precedent for Phase-0 memory quality mattering:** `docs/reports/honcho-stress-0418-r2-2026-04-18.md:43`, `:109`, `:191`, `:206`, `:219` show that polluted or mis-shaped Phase-0 memory changed the design and forced hook surgery.
- **Hard gate against structure-in-text:** `docs/reports/honcho-reaudit-phase0-2026-04-18.md:147-150` explicitly allows only NL prose in a sidecar and says the role fails if we must embed `axis_id` / `round` / `direction` for filtering.

---

## Trigger matrix

### t1 — real recall miss

**OBSERVABLE**
The relevant prior finding existed in SQLite before mission start, but the live judge hook would not have surfaced it because of the current retrieval shape at `~/.claude/agents/the-judge.md:18` (latest-mission only, top-20 cap). Verify with `scripts/xbreed-memory get` / `grep` at `scripts/xbreed-memory:51-79`, then compare against the later accepted finding in `docs/reports/`.

**THRESHOLD**
Fire on **1 confirmed miss**. This is a floor-recall failure, not a ranking-quality complaint. If a later report cites a prior finding that was already in SQLite and the Phase-0 hook could not have surfaced it, the current floor is insufficient.

**DETECTOR**
`the-judge` during postmortem, or an automated post-mission audit using `xbreed-memory`; `scribe` may also notice when drafting the report if the accepted finding clearly predated the mission.

**ACTION**
1. Open an issue against the Phase-0 retrieval shape.
2. Re-dispatch `/wwkd` to redesign the hook/query contract.
3. Re-dispatch `/xbgst` only if the miss suggests the SQLite floor is fundamentally insufficient and a second memory source is back in play.

**TEST**
After mission `N`, run:

```bash
scripts/xbreed-memory latest-mission
scripts/xbreed-memory get <prior-mission>
scripts/xbreed-memory grep "<accepted finding phrase>" <prior-mission>
rg -n "<accepted finding phrase>" docs/reports/
```

If `grep` proves the finding was already in SQLite but the hook at `the-judge.md:18` would not have returned it, `t1` fires.

### t2 — scribe-coverage gap

**OBSERVABLE**
A later secondary summary, review, or advisor addendum adds a consequential nuance that is absent from the original scribe artifact in `docs/reports/`. Precedent already exists: `docs/reports/test-brief-2026-04-16.md:233-260` and `docs/reports/handoff-harness-0417-2026-04-17.md:135-138`.

**THRESHOLD**
Fire on **1 mission** if the missed nuance changes routing, hard-gate interpretation, or memory updates. Otherwise fire on **2 independent missions** with omitted but consequential nuance. One stylistic omission is not enough; one load-bearing omission is.

**DETECTOR**
`scribe`, `reviewer`, `the-judge`, or the user during the report/review pass.

**ACTION**
1. Open an issue against report coverage and the scribe template.
2. Re-dispatch `/wwkd` if the fix is likely protocol-only.
3. Re-dispatch `/xbgst` if the pattern suggests that NL sidecar summarization should be reconsidered as a memory layer.

**TEST**
For each real mission, compare the scribe artifact with exactly one secondary pass:

```bash
rg -n "## Findings|## Out-of-scope|## Links" docs/reports/<scribe-report>.md
rg -n "gaps|missed|addendum|advisor flagged|memory correction" docs/reports/
```

If the secondary pass adds a new consequential nuance not already captured by the scribe artifact, count it toward `t2`.

### t3 — icontains violation

**OBSERVABLE**
Any future content-text blob or `observable` field contains structural encoding such as `axis_id`, `"round"`, `round=`, `"direction"`, or `direction=`. SQLite-side detection is now explicit at `scripts/xbreed-memory:71-79`. The design-level prohibition is explicit at `docs/reports/honcho-reaudit-phase0-2026-04-18.md:147-150`.

**THRESHOLD**
Fire on **any single occurrence**. This is a structural red flag, not an error-budget metric.

**DETECTOR**
Automated (`scripts/xbreed-memory redflags`), plus user or reviewer by direct inspection.

**ACTION**
1. Stop the extension work immediately.
2. Open an issue describing the structure-in-text violation.
3. Re-dispatch `/wwkd` before any further implementation.
4. Re-dispatch `/xbgst` if the violating design was already being proposed as a new memory shape.

**TEST**

```bash
scripts/xbreed-memory redflags
rg -n 'axis_id|"round"|round=|"direction"|direction=' docs/reports/
```

Any hit is a `t3` fire.

### t4 — auto-memory drift

**OBSERVABLE**
The report corpus explicitly contradicts or qualifies prior auto-memory assumptions strongly enough that a memory correction is required. The precedent pattern is already present in `docs/reports/shim-handoff-xbgst-r1-2026-04-17.md:191-197`, `:291-297`, `:327`. The hierarchy constraint remains `~/.claude/agents/the-judge.md:18`: persistence is additive only and may not silently replace `MEMORY.md`.

**THRESHOLD**
Fire on **1 hard-gate contradiction** that requires a memory correction, or on **2 softer memory-correction events within 3 real missions**. This makes it testable after either one severe miss or a small run of accumulated drift.

**DETECTOR**
User, `the-judge`, or `scribe` during report closeout.

**ACTION**
1. Open an issue against the offending memory entry or memory policy.
2. Re-dispatch `/wwkd` to define the correction path.
3. Re-dispatch `/xbgst` if repeated drift implies a missing second memory source, not just a bad memory entry.

**TEST**

```bash
rg -n 'memory correction|memory update if confirmed|needs correction|contradicts the .* assumption' docs/reports/
```

A hard-gate contradiction fires `t4` immediately. Otherwise count corrections across the last 3 real missions and fire when the count reaches 2.

---

## Minimal policy

- `t1` is about **floor recall**. One real miss is enough.
- `t2` is about **scribe coverage quality**. One consequential miss or two softer misses reopen it.
- `t3` is about **structural discipline**. One hit is enough.
- `t4` is about **memory drift over time**. One hard contradiction or two softer corrections reopen it.

---

## Verification

The SQLite-side detectors in this report are exercised by `tests/xbreed_memory_audit.sh`, which validates the DB override plus the `latest-mission`, `grep`, and `redflags` helpers against a temp database.
