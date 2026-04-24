-- xbreed persistence substrate — R3 M1 (Mission honcho-stress-0418)
-- Schema v1.1: 5 fields + UNIQUE key on (mission, round, axis_id) for idempotent writes.
-- v1.1 added 2026-04-24 (B2 fix from xbreed-memory-blockers-2026-04-24.md).
PRAGMA journal_mode=WAL;

CREATE TABLE IF NOT EXISTS findings (
  mission     TEXT NOT NULL,
  round       INTEGER NOT NULL,
  axis_id     TEXT NOT NULL,
  observable  TEXT NOT NULL,
  direction   TEXT NOT NULL CHECK(direction IN ('+','-','0'))
);

CREATE INDEX IF NOT EXISTS idx_mission_round ON findings(mission, round DESC);

-- UNIQUE key pairs with wrapper's `INSERT OR REPLACE INTO findings` to give
-- Quiroga-style GNW-ignition semantics: duplicate proposals for the same
-- (mission, round, axis_id) do NOT consume the sparse-coding budget
-- (XBREED_FINDINGS_LIMIT, default 20); they overwrite in place.
CREATE UNIQUE INDEX IF NOT EXISTS idx_findings_key ON findings(mission, round, axis_id);
