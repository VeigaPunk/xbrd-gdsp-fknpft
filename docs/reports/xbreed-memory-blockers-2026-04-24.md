# xbreed-memory substrate blockers — 2026-04-24
**Surfaced by:** mgd-agents-memlift-r1 Round 1 (memory-lift experiment)
**Scope:** blockers for normal xbgst operation, NOT for the memory-lift experiment itself
**Source team:** cdx-reviewer-gaps, cdx-labrat-m1-smoke, g-connector-crosslinks

---

## B1 — LIMIT 20 silent truncation [severity: blocker, VPS-confirmed]
Hardcoded in two files:
  - `scripts/xbreed-memory:54,56`
  - `~/.claude/agents/the-judge.md:18` (Phase-0 read hook)

Empirical evidence from cdx-labrat-m1-smoke adversarial probe on VPS:
  21 rows inserted, 20 returned — S9 silently dropped, round-1 rows gone.
  `SELECT COUNT(*) FROM findings` returned 21; `xbreed-memory get` returned 20 lines.
  Row for axis_id S9 absent from get output despite confirmed INSERT success.

Production state: `data/xbreed.db` on VPS has 19 rows live as of 2026-04-24T08:57Z.
**One more `xbreed-memory put` to the active mission triggers silent data loss on next judge resume.**

---

## B2 — No UPSERT / no UNIQUE INDEX [severity: blocker]
Schema: `data/xbreed-schema.sql` findings table has no UNIQUE constraint on `(mission, round, axis_id)`.
Wrapper: `scripts/xbreed-memory` `put` unconditionally `INSERT INTO` — no idempotency.

Empirical evidence from cdx-labrat-m1-smoke M1 smoke:
  Two identical rows created for same `(mission, round, axis_id)` from one tmux send-keys + one plain SSH put.
  `get` returned duplicate lines — same axis_id appeared twice in output.

Fix (reviewer-recommended, one-liner each):
  - schema: `CREATE UNIQUE INDEX findings_key ON findings(mission, round, axis_id);`
  - wrapper: `INSERT INTO` → `INSERT OR REPLACE INTO`

B1 and B2 compound: duplicate rows consume the LIMIT 20 budget, doubling B1's severity.

---

## B3 — FINDINGS_LIMIT drift risk [severity: high, design]
Connector-flagged second-order: independent hardcoded LIMIT 20 in two locations will drift over time.
If one is tuned (say to 30) and the other stays at 20, Phase-0 judge spawn context and live `get` calls
return different retrieval depths. Judge reasons about findings absent from its own context.

Fix: extract to `XBREED_FINDINGS_LIMIT` env var, default=20 (Quiroga sparse-coding budget, per
`forgetting-machine.md` line 33 — ~1-2% MTL activation ceiling).

Both locations reference the same var:
  - `scripts/xbreed-memory:54,56`
  - `the-judge.md:18` Phase-0 hook

---

## Recommended fix sequence (next session)
1. B2 first (UNIQUE INDEX + INSERT OR REPLACE) — safe on 19-row table, fixes idempotency
2. B3 second (extract FINDINGS_LIMIT constant, default=20) — prevents drift
3. B1 third (evaluate actual row count + round cadence, set env override if truncation imminent)

---

## NOT in scope of this report
- Staging tier (`.xbreed/mailbox/judge/proposals/`)
- Concept-cell LIKE-blending on reads
- `[cue-retrieved]` annotation
- Two-threshold architecture
- Topic-file hierarchy

Those are tracked separately in the multi-agent-memory-extension sketch (Stream C).

---

## Evidence lineage
- cdx-reviewer-gaps initial + supplementary + gate reviews
- cdx-labrat-m1-smoke M1 smoke + LIMIT adversarial probe (VPS, direct stdout capture)
- g-connector-crosslinks cross-axis FINDINGS_LIMIT drift analysis
- Primary source correction: ccs-scout-anthropic found code.claude.com/docs/en/memory —
  Anthropic pattern is filesystem+markdown, not API; these B1-B3 blockers are xbrd-specific
  substrate issues orthogonal to the Anthropic memory-lift experiment.
