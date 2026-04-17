# Teammate Benchmark — Phase B (DEFERRED handoff TODO)

**Mission:** deferred continuation of `/xbgst /wwkd | godspeed` teammate benchmark.
**Status:** **NOT EXECUTED.** Handoff spec only.
**Plan:** `docs/plans/teammate-benchmark-plan-2026-04-17.md` §Deferred Phase B

---

## Why deferred

Phase B requires **serial CC-session restarts** (one per effort tier in `{low, medium, high, xhigh}`). Each restart breaks godspeed by ~5-15 minutes of session cold-start + team spawn. The user approved the C→A→B(deferred) sequencing explicitly.

**Side benefit on execution:** closes R3 known-gap #3 (`CLAUDE_CODE_EFFORT_LEVEL` is currently docs-confirmed, not restart-verified).

---

## Scope

4 serial CC sessions. Each session starts fresh with the env var set in the shell BEFORE `claude` launches. Each session spawns the Phase A team (3 sonnet + 3 opus). Total 24 teammates = 4 tiers × 6 replicas.

## Pre-session checklist (per session)

1. **Verify no running CC session:** `pgrep -f "claude$" | head`. If any, exit cleanly first.
2. **Set env var in parent shell:**
   ```bash
   export CLAUDE_CODE_EFFORT_LEVEL=low   # or medium / high / xhigh
   env | grep EFFORT
   ```
3. **Launch claude:** `claude`
4. **Inside CC, confirm env propagated to teammate pane (empirical Gap-3 close):**
   Spawn one smoke teammate, have it run `printenv | grep CLAUDE_CODE_EFFORT_LEVEL`, SendMessage to team-lead.
5. **Confirm no stale teams:** `ls ~/.claude/teams/ ~/.claude/tasks/`.

## Per-session recipe

```bash
# After launching claude with CLAUDE_CODE_EFFORT_LEVEL set:
# 1. Precheck
xbreed precheck pane-cap --team-size 6

# 2. Inside CC: dispatch via same Phase A brief template
#    (name teammates bench-<tier>-sonnet-{a,b,c} + bench-<tier>-opus-{a,b,c})

# 3. After all 6 SYNTHESIS_READY, collect:
python3 scripts/bench-locate.py ~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft \
  bench-<tier>-sonnet-a bench-<tier>-sonnet-b bench-<tier>-sonnet-c \
  bench-<tier>-opus-a bench-<tier>-opus-b bench-<tier>-opus-c

bash scripts/bench-collect.sh <6-jsonls> > data/bench-phase-b-<tier>.tsv
```

## Cross-session join

4 sessions × 6 teammates = 24 TSV rows. Join by `(model, effort_env)` to produce a 2×4 grid:

| model \ effort | low | medium | high | xhigh |
|---|---|---|---|---|
| sonnet (3× replicas) | ? | ? | ? | baseline (Phase A) |
| opus (3× replicas)   | ? | ? | ? | baseline (Phase A) |

Phase A xhigh results can serve as the xhigh cell, saving one session.

## R3 known-gap #3 closure criteria

- **If `tok/s` variance across effort tiers within a model > 30%:** env var is honored → R3 gap CLOSED.
- **If variance remains ±20% (matching Phase C no-op baseline):** env var is **also** ignored in teammate-mode path → escalate to CC bug report. Update memory `feedback_teammate_mode_effort_caveat.md` to reflect env is noop too.
- **Mixed / noisy:** inconclusive; run a 5th session with `unset CLAUDE_CODE_EFFORT_LEVEL` as a control.

## Expected runtime

- 4 × (session cold start ~30s + bench dispatch ~60s + teammate SYNTHESIS_READY ~200s + collection + shutdown ~60s) ≈ 22-25 min per session × 4 sessions = **~90-100 min total**.
- NOT godspeed. Plan accordingly.

## Commit plan

- 1 commit per session: `feat(bench): Phase B — effort=<tier> — <n> rows`
- 1 final commit after join: `docs(bench): Phase B cross-tier analysis + R3 gap-3 verdict`

## Primary source to read before execution

- `commands/references/xbreed-shared.md` §Session Effort Configuration — the canonical caveat + env workaround
- `docs/reports/xbreed-harness-r3-2026-04-17.md` §Known gaps — R3 Gap 3 statement
- `docs/reports/teammate-benchmark-phase-a-2026-04-17.md` — baseline xhigh measurements
- `docs/plans/teammate-benchmark-plan-2026-04-17.md` §M3-M5 — reuse dispatch table + metrics pipeline

## Risk

If the `CLAUDE_CODE_EFFORT_LEVEL` env var is ignored on teammate-mode spawns (same bug pattern as frontmatter), the whole 90-minute exercise produces a negative result: "env workaround is also noop". That would be valuable but expensive. Mitigation: Phase B SHOULD include the smoke-probe in the pre-session checklist — one teammate, one `printenv` call — before committing to spawning the full 6. If the smoke probe shows the env var is NOT in the teammate's environment, abort that session and document.

---

No tasks remain for Phase B in the current session. Pick up from this handoff when running the 4-session serial protocol.
