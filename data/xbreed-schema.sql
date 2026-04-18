-- xbreed persistence substrate — R3 M1 (Mission honcho-stress-0418)
-- Schema v1-minimal per R2 L-axis: 5 fields
PRAGMA journal_mode=WAL;

CREATE TABLE IF NOT EXISTS findings (
  mission     TEXT NOT NULL,
  round       INTEGER NOT NULL,
  axis_id     TEXT NOT NULL,
  observable  TEXT NOT NULL,
  direction   TEXT NOT NULL CHECK(direction IN ('+','-','0'))
);

CREATE INDEX IF NOT EXISTS idx_mission_round ON findings(mission, round DESC);
