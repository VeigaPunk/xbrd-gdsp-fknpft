# Plan — Drift-defense net: all 14 SSoT lanes × 4 copy surfaces
**Session:** drift-net-0607 | **Dispatched by:** the-judge (team-lead) | **Date:** 2026-06-07

---

## Phase 0 — State map

### Exists
- `scripts/verify-docs.sh` — checks connector lane only (xask --effort [a-z]+ [a-z]+) in
  {AGENTS.md, ~/.claude/agents/the-judge.md, ~/.claude/agents/connector.md, commands/xbgst.md}
- `tests/mirror_drift_mutation.sh` — mutation proof for connector×AGENTS.md only
- `make verify` wires mirror_drift_mutation.sh + 9 other shell tests (none cover routing)
- templates/agents/ — 14 role files in-repo, all with delegation lines

### Missing
- Drift check for 8 remaining xask lanes (reviewer, scout, labrat, executor, the-revenger,
  sentinel, critic, mutation-tester) across 4 copy surfaces
- Drift check covering templates/agents/<role>.md delegation lines (not touched by verify-docs.sh)
- Drift check covering templates/agents/the-judge.md (repo copy, not ~/.claude/agents/)
- Mutation-test proof that new checks catch drift (existing mutation test only covers connector×AGENTS.md)

### Risk
- commands/xbgst.md has TWO sub-tables (Phase-1 bullet list + Phase-2 verbatim gate table) —
  grep must find canonical in BOTH; a single `grep -F` over the whole file catches both at once
  (simplest path, sufficient for drift; no line-range scoping needed)
- mutation-tester has DUAL canonical patterns in SSoT — need to extract both from row and verify both
- sentinel has DUAL canonical patterns — same treatment as mutation-tester
- connector deliberate NO-`--gs` invariant must not be silently bypassed — fixed-string grep
  on the canonical (which lacks --gs) will fire if someone adds --gs (the string won't match)
- templates/agents/ vs ~/.claude/agents/ — task explicitly scopes to repo templates/ surface;
  existing verify-docs.sh continues to cover ~/.claude/agents/; no overlap needed

---

## WWKD

1. **What:** scripts/verify-routing.sh covering all 9 xask lanes × 4 copy surfaces
   (AGENTS.md, templates/agents/the-judge.md, commands/xbgst.md, templates/agents/<role>.md),
   model/tier-agnostic (canonical extracted from SSoT at runtime, not hardcoded),
   wired into `make verify` with mutation-test proof. Success boundary: `make verify` exits 0;
   injecting any wrong xask flag into any surface causes `verify-routing.sh` to emit DRIFT
   and exit non-zero.

2. **Why:** 68695ff (gemini→codex swap) drifted silently in 13 lanes; repaired today in 50fa08b.
   Drift accumulates because verify-docs.sh covers only one lane out of 14 and misses the
   --gpt55 and --spark flag families entirely. The next model pivot will drift again without
   a wider net.

3. **Assumptions/Risks:**
   - ASSUMPTION: canonical xask invocations are backtick-delimited in the SSoT table — confirmed
     by data walk. Extraction: `grep -F "| \`<role>\`" SSOT | grep -oE '\`xask [^\`]+\`' | head -1`
   - ASSUMPTION: fixed-string grep on the canonical (without trailing arg placeholder) works as
     a universal substring match across all format variants — confirmed: `xask --gpt55 --gs -e low codex`
     appears literally in all 4 surface formats
   - RISK: mutation-tester and sentinel have multiple canonical patterns per SSoT row — executor
     must handle multi-pattern extraction (grep -oE on row returns multiple matches; loop over them)
   - RISK: if SSoT table is restructured (row format changes), extraction regex breaks silently —
     mitigated by the "FAIL: cannot extract canonical" guard pattern from verify-docs.sh
   - REJECTED ALTERNATIVE: one script per lane (14 scripts). Rejected: maintenance burden scales
     linearly, format-parsing logic is identical, a parameterized loop in ~100 lines replaces 14×~50-line scripts

4. **How:** (milestone order, each runnable in isolation)
   - M01: Overfit one lane (reviewer) → verify-routing.sh handles reviewer across all 4 surfaces
   - M02: Generalize to all 9 xask lanes → loop over lane-map, each lane checks all surfaces
   - M03: Mutation-test proof (reviewer + scout) → tests/routing_drift_mutation.sh 4-step cycle
   - M04: Wire into make verify → Makefile adds 2 lines; `make verify` exits 0

5. **Escalation points:**
   - Is commands/xbgst.md scoped correctly? Current task text says "Phase-1/gate tables" — both
     Phase-1 bullet list (lines 68-74) and Phase-2 gate table (lines 86-98) carry canonical strings.
     Plan covers both via single file-level grep. If judge requires section-isolated checks, M02
     needs line-range scoping added (non-blocking flag, not a hard stop).
   - Should verify-routing.sh ALSO cover ~/.claude/agents/<role>.md (the canonical agent files)?
     Excluded from plan per task scope ("templates/agents/<role>.md delegation lines"). Escalate
     to judge if coverage should widen to ~/.claude/agents/ surface.

---

## Milestones

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M01 | Overfit reviewer lane | `bash scripts/verify-routing.sh 2>&1` | `OK: reviewer routing consistent` (or DRIFT on broken repo) | executor |
| M02 | Generalize all 9 xask lanes | `bash scripts/verify-routing.sh 2>&1` | `OK: all 9 lane(s) routing consistent` + exit 0 | executor |
| M03 | Mutation-test proof | `bash tests/routing_drift_mutation.sh` | `PASS: verify-routing.sh catches reviewer drift in templates/agents/reviewer.md` | executor |
| M04 | Wire into make verify | `make verify 2>&1 \| tail -20` | All steps print OK, exit 0 | executor |

---

## Implementation notes per milestone

### M01 — Overfit reviewer lane (scripts/verify-routing.sh)

```bash
#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"

# Extraction: backtick-delimited xask invocations from SSoT row for the role.
# grep -F matches the literal role name in the table; grep -oE extracts all backtick-xask hits.
extract_canonical() {
  local role="$1"
  grep -F "| \`${role}\`" "$SSOT" | grep -oE '\`xask [^`]+\`' | sed "s/\`//g"
}

check_lane() {
  local role="$1"; local canonical="$2"; local file="$3"
  if [[ ! -f "$file" ]]; then printf "MISSING: %s\n" "$file" >&2; return 1; fi
  if ! grep -qF "$canonical" "$file"; then
    actual=$(grep -oE 'xask [^`|"]+' "$file" | grep -i "${role}" | head -1 || true)
    printf "DRIFT: %s\n  expected: %s\n  actual: %s\n" "$file" "$canonical" "${actual:-<not found>}"
    return 1
  fi
}
```

Surfaces for reviewer:
- AGENTS.md (delegation bias column)
- templates/agents/the-judge.md (dispatch table)
- commands/xbgst.md (Phase-1 list + Phase-2 gate table — single grep covers both)
- templates/agents/reviewer.md (prose delegation line)

### M02 — Generalize (lane-map loop)

Declare lane-to-surface mapping:
```
LANES=(scout reviewer labrat executor connector the-revenger sentinel critic mutation-tester)
# For each lane: extract from SSoT, check AGENTS.md + templates/agents/the-judge.md
#   + commands/xbgst.md + templates/agents/${role_file}.md
# role_file: the-revenger → the-revenger, mutation-tester → mutation-tester
```

Multi-pattern lanes (sentinel, mutation-tester): loop over all `extract_canonical` outputs — each
canonical string must appear in each surface. Use `while IFS= read -r canonical; do ... done` pattern.

### M03 — Mutation test (tests/routing_drift_mutation.sh)

Follow mirror_drift_mutation.sh pattern exactly:
1. Baseline: `bash scripts/verify-routing.sh` exits 0
2. Mutate: sed-replace canonical in templates/agents/reviewer.md (flip `--gpt55` → `--effort medium`)
3. DRIFT check: `bash scripts/verify-routing.sh` exits non-zero AND emits DRIFT string
4. Restore + re-confirm baseline
5. EXIT trap restores file if script aborts

Second mutation case (scout in AGENTS.md): same 4-step cycle, different lane + surface.
Covers both the `--gpt55` flag family AND the `--effort medium --gs` family.

### M04 — Makefile wiring

Add after existing shell tests in verify target:
```makefile
@echo "verify: scripts/verify-routing.sh"; bash scripts/verify-routing.sh
@echo "verify: tests/routing_drift_mutation.sh"; bash tests/routing_drift_mutation.sh
```

---

## Dependencies

- M01 → M02 (M02 extends M01's script; run M01 first to confirm single-lane baseline)
- M02 → M03 (mutation test needs the complete multi-lane verifier to be meaningful)
- M02 → M04 (Makefile wires the completed verifier)
- M03 → M04 (Makefile wires the completed mutation test)
- M01/M02/M03 are independent of existing verify-docs.sh (no shared state, no conflicts)

evidence: none — planning artifact
