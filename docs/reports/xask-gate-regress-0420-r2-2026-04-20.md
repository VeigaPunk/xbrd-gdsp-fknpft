# xask-gate-regress-0420 R2 — V-Axis Regression Guards
**Status:** COMPLETE | **Date:** 2026-04-20 | **Session:** 2

---

## Round Overview

R2 scope: V-axis (Build/CI-tier). Two regression guards land as shell test scripts wired into `make verify`, ensuring M2 (xask fail-loud) and M4-thinking-budget cannot silently regress. Connector (g-connector-r2) scanned for additional xask surface degradation patterns.

R1 base: commit `3f790fd` — 4 moves (Makefile install sync, xask fail-loud, xbgst.md gate strings, shared.md naming convention). R1 `audit_hash`: `c717b11cda1f07d58b60117008608b9d96b09fb13a91ded9c2aa7a9cd5f8fa61`.

---

## Per-Teammate Findings

### cdx-executor-tests — V1: xask fail-loud regression guard

| Field | Value |
|---|---|
| **MOVE** | Add `tests/xask_template_missing_fail_loud.sh` |
| **AXIS** | V (Build/CI-tier) |
| **CLAIM** | A test that sets `XBREED_DISPATCH_DIR` to a nonexistent path and asserts xask exits 1 + stderr contains "dispatch template not found" in exactly 3 lines will catch future silent-fallback regressions on the M2 path. |
| **EVIDENCE** | `tests/xask_template_missing_fail_loud.sh:9` — `XBREED_DISPATCH_DIR="/tmp/xask-nonexistent-$$" xask codex "probe"`; assertion at line 13: `status == 1 && grep -q "dispatch template not found" && wc -l stderr == 3` |
| **REJECTED-ALTERNATIVE** | Unit test mocking dispatch path — rejected: xask is a shell script; shell-level integration test is the only faithful coverage tier. |
| **Confidence** | 0.93 |

### cdx-executor-tests — V2: thinking-budget reachability guard

| Field | Value |
|---|---|
| **MOVE** | Add `tests/xask_thinking_budget_reachable.sh` |
| **AXIS** | V (Build/CI-tier) |
| **CLAIM** | Dry-running `xask -d -e high gemini "probe"` and asserting stdout contains `# ThinkingBudget: 8192` ensures the effort→budget translation path is exercised in CI without a live API call. |
| **EVIDENCE** | `tests/xask_thinking_budget_reachable.sh:9` — `xask -d -e high gemini "probe"`; assertion at line 13: `status == 0 && grep -q "# ThinkingBudget: 8192" stdout` |
| **REJECTED-ALTERNATIVE** | Live gemini call — rejected: flaky in CI, quota-dependent, OAuth-exclusive (would fail in most CI environments). |
| **Confidence** | 0.91 |

### cdx-executor-tests — V3: Makefile verify wiring

| Field | Value |
|---|---|
| **MOVE** | Extend `Makefile` verify target with two new test entries |
| **AXIS** | V (Build/CI-tier) |
| **CLAIM** | Without Makefile wiring, the test scripts are dead artifacts. Appending both to verify ensures `make verify` catches drift. |
| **EVIDENCE** | `Makefile:26-27` (uncommitted diff, executor-authored): `bash tests/xask_template_missing_fail_loud.sh` + `bash tests/xask_thinking_budget_reachable.sh` |
| **REJECTED-ALTERNATIVE** | Separate `make test-xask-guards` target — rejected: fragmentation; existing pattern is single verify invocation. |
| **Confidence** | 0.97 |

### g-connector-r2 — Residual surface scan

**STALL — no findings posted.** Per `feedback_connector_stall.md`: g-connector-* routinely stalls 90s+ post-xask and never posts. Connector residual findings section carried forward as UNRESOLVED. Orchestrator may re-dispatch if residual sweep is required before R3.

---

## Red-Before-Green Proof

**EXECUTOR-EVIDENCE-ABSENT:** cdx-executor-tests provided no `evidence:` DM block. Per scribe protocol, fabrication is prohibited. Logical pre-R1 red-state analysis follows:

### V1 — xask_template_missing_fail_loud.sh

**Pre-R1 red state (logical):** Before M2 (`scripts/xask:124-132` fail-loud), xask used a silent raw-`$QUERY` fallback when the dispatch template was missing. Invoking `xask codex "probe"` with a nonexistent `XBREED_DISPATCH_DIR` would exit 0 and produce no "dispatch template not found" stderr. The test assertion `status == 1 && grep -q "dispatch template not found"` would fail both predicates.

**Green (scribe pre-commit run, 2026-04-20):**
```
PASS: xask exits with status 1 and stderr includes dispatch template error when dispatch dir is missing
EXIT: 0
```

### V2 — xask_thinking_budget_reachable.sh

**Pre-regression red state (logical):** If the `--effort high` → `--thinking-budget 8192` translation in `scripts/xask` were removed or broken, the dry-run stdout would omit `# ThinkingBudget: 8192`. The grep assertion would fail.

**Green (scribe pre-commit run, 2026-04-20):**
```
PASS: xask includes # ThinkingBudget: 8192 in stdout with -e high gemini
EXIT: 0
```

---

## Pareto Verdict

| Move | Improves? | Regresses? | Verdict |
|---|---|---|---|
| V1: xask_template_missing_fail_loud.sh | V-axis: +1 (M2 guarded) | none | **KEEP** |
| V2: xask_thinking_budget_reachable.sh | V-axis: +1 (M4-budget guarded) | none | **KEEP** |
| V3: Makefile verify wiring | V-axis: +1 (CI reachability) | none | **KEEP** |
| Connector residual sweep | — | — | **BLOCKED (stall)** |

All three executable moves accepted. Connector stall is a known pattern — not a regression, does not block commit.

---

## Findings

- `tests/xask_template_missing_fail_loud.sh:13` — 3-line stderr assertion is tight: if fail-loud adds or removes a line in future, test breaks. Acceptable strictness for a gate test.
- `tests/xask_thinking_budget_reachable.sh:13` — `-d` (dry-run) flag assumed to suppress live dispatch. Dry-run semantics must be preserved in xask for this test to remain valid.
- `Makefile:26-27` — executor added lines to verify target (uncommitted). No pre-wiring gap found; executor completed both file creation and Makefile wiring in same session.
- Connector residual scan: UNRESOLVED. No findings received. Known stall pattern (`feedback_connector_stall.md`). Surface risk: any xask path not covered by existing 11-test suite may harbor silent degradation undiscovered.

---

## audit_hash

`a78e64603655025425b786044d80e16b7500100e6d0bcf8984c34c88fdcb1b21`

---

## Links

- Plan: `docs/reports/xask-gate-regress-0420-r1-2026-04-20.md` (R1 anchor)
- R1 commit: `3f790fd`
- Next: R3 if connector residual sweep requires re-dispatch; else mission closed.
