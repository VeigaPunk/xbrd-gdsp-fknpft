# test-0418-landings-xbgst — R3 Audit Trail
**Date:** 2026-04-18 | **HEAD after R3:** `d1ee0ae` | **Session:** xbgst-r3

---

## Summary

R3 implemented all R1-accepted moves, extended the Pareto frontier on the SH-axis, reconciled the E-axis parity finding as a deployment-gap (not a code bug), and completed the 6-spawn-cap RECON. 107 tests pass (was 103 at pre-walk HEAD `d61457f`). No open Pareto debt. Round closes clean.

---

## R2 Frontier Extension Moves

### SH-axis — `tests/xask_full_flag.sh` (g-connector-r2 proposal)
- **Status:** LANDED, GREEN
- **Commit:** `d1ee0ae`
- **Evidence:** `make verify` passes; new test registered in Makefile verify target.
- **Coverage gap closed:** full-flag dispatch path (`-s`, `-e`, `-o` all present) was untested at integration layer.

### E-axis parity — deployment-gap reconcile (cdx-labrat-edges-r2, reclassified VALID)
- **Status:** RESOLVED — not a code bug, deployment-gap
- **Root cause:** commit `4339240` (R1 executor) modified `scripts/xask` but did not run `make install`. PATH binary at `~/.local/bin/xask` was stale. cdx-labrat probe 4 hit "unknown flag -F" against the stale binary, not against the repo copy.
- **Resolution:** `make install` run; `~/.local/bin/xask` now canonical. No source change required.
- **Meta-lesson:** Three agents (critic, connector, judge) independently primary-source-verified `scripts/xask` and all concluded "not a bug." All three were reading the right repo file — none were checking the PATH file. Future walk protocol: when multi-agent consensus says "not a bug" but a labrat probe fails, diff `which xask` vs `scripts/xask` before closing.
- **Memory updated:** `feedback_recompile_on_change` — added `make install` / PATH-vs-repo diff-check note.

### C2-axis ceiling — silent-truncation observability (ccs-critic-trendfit)
- **Status:** INFORMATIONAL — no Pareto move this round
- **Finding:** All 4 lanes (gemini, codex, claude, ask) silently truncate long outputs with no structured signal. Affects observability for orchestrators parsing JSON output.
- **Deferral reason:** Requires live-model probe (out-of-scope for static-walk round). Logged for future dedicated probe session.

---

## R3 Executor Work

### cdx-executor-r2-moves — R1 accepted moves landing
**Commit:** `4d1ff76`

Moves landed:
| Move | Axis | Description |
|------|------|-------------|
| R1-M1 | Dispatch | `-F` flag threading: flag order guard added to `scripts/xask:37` dispatch path |
| R1-C1 | Coverage | Missing arg-validation test for positional-before-flag error path |
| R1-D1 | Coverage | Mailbox drain-path strict-improver: `drain_target` rename(2) contract |
| R2-strict-1 | Regression | Mailbox regression: fd-cache invalidation on compact |
| R2-strict-2 | Regression | Mailbox regression: stale-inode guard on async compact path |

**Gate:**
```bash
cargo test 2>&1 | tail -3
```
Expected: `test result: ok. 107 passed; 0 failed`
Actual: `test result: ok. 107 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

### cdx-executor-shaxis — SH-axis test + Makefile registration
**Commit:** `d1ee0ae`

- `tests/xask_full_flag.sh` created: exercises `-s medium -e high -o /tmp/out.txt gemini "probe"` dispatch path end-to-end (mocked model response).
- Makefile `verify` target updated to include `bash tests/xask_full_flag.sh`.

---

## cdx-revenger-spawnagent-cap RECON

**Verdict:** 6-spawn-cap does not exist.

**Primary artifact:** `docs/reports/recon-spawnagent-cap-2026-04-18.md`

**Method:** cdx-revenger grepped Claude Code source, agent SDK docs, MCP spec, and xbreed src for any spawn/agent-count ceiling. No hard cap found at 6 or any fixed integer. Observed "6" in prior sessions was coincidence of round size, not enforced limit.

**User directive resolution:** "tamper with 6 spawn_agent cap" — no tamper required, no wrapper needed. Resolution: update orchestrator prompt template to request N=12 when wider swarms needed. Trendsetter verdict: CONSUMER (we adapt our prompt, tool adapts to us as-is).

---

## cdx-labrat-noninteractive-walltime — 10-probe matrix

**Scope:** Measure xbreed wrapper tax vs direct codex invocation; flag-variant spread.

**Results:**

| Probe | Variant | Wall time |
|-------|---------|-----------|
| 1 | direct `codex` | 4.69s |
| 2 | `xask codex` (baseline) | 6.24s |
| 3 | `xask --json codex` | 6.31s |
| 4 | `xask -F codex` (stale bin) | ERROR — unknown flag |
| 5 | `xask --output-last-message codex` | 6.58s |
| 6 | `xask --spark codex` | 6.89s |
| 7 | `xask -R codex` | 7.02s |
| 8 | `xask -e high codex` | 6.44s |
| 9 | `xask -s medium codex` | 6.38s |
| 10 | `xask --json --output-last-message codex` | 6.61s |

**Conclusions:**
- xbreed wrapper tax: ~1.55s (shell spawn + policy check + arg rewrite)
- Flag-variant spread: ≤0.9s — no outlier flag combination
- Probe 4 ERROR reconciled as deployment-gap (stale `~/.local/bin/xask`), not a code defect
- Ship decision: M02a/M02b proceed without latency concern; 1.55s tax acceptable for policy gate value

---

## Critic CONFLICTS — Resolved

### Conflict A: "-F alone untested" (sentinel vs connector)
- **Connector claim:** `-F` flag untested at argv construction layer.
- **Reviewer counter-cite:** `ask.rs:621` — flag is parsed and threaded correctly; unit test exists for argv assembly.
- **Resolution:** Reviewer correct. The real gap was dispatch-layer arg-order (R1-M1), not argv construction. Connector retracted. Talk-past on terminology, not a real contradiction.

### Conflict B: drain_target TOCTOU (sentinel claim)
- **Sentinel claim:** `drain_target` rename in mailbox drain path is a TOCTOU window.
- **Resolution:** Retracted. `rename(2)` is atomic on Linux ext4. Mailbox directory is owner-only (0700). No race window exists between check and rename. Sentinel's model was POSIX-generic; repo's Linux-specific guarantees close the window.

---

## Commits This Walk

| SHA | Message |
|-----|---------|
| `d1ee0ae` | feat(xbgst-r3): SH-axis test + 6-cap RECON + deployment-gap reconcile |
| `4d1ff76` | test(xbgst-r2): close 4 coverage gaps from R1 Pareto + strict-improver scope |
| `f7f6ad6` | docs(xbgst-r1): test-0418-landings audit trail |
| `d61457f` | docs(session-handout): 2026-04-18 recap for next session |

---

## Links
- R1 report: `docs/reports/test-0418-landings-xbgst-r1-2026-04-18.md` (committed `f7f6ad6`)
- RECON artifact: `docs/reports/recon-spawnagent-cap-2026-04-18.md`
- Noninteractive leverage plan: `docs/reports/plan-noninteractive-leverage-0418.md`
- Next: no scheduled R4 — walk complete, all Pareto debt closed
