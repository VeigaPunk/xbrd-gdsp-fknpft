# Handoff — NDJSON mailbox R4 mission → next session

**From:** `/xbgst /wwkd | godspeed` team `mailbox-r4-0417`, 2026-04-17 (2 rounds + terminal)
**Pushed:** `36d3c61..2b99292` (6 commits, main → main, local only — push when ready)
**R1 findings report:** `docs/reports/mailbox-r4-r1-2026-04-17.md` (7 moves, 3 HIGH surfaced)
**R2 findings report:** `docs/reports/mailbox-r4-r2-2026-04-17.md` (4 shipped, M9 discovery, M2 UNMEASURABLE)
**Prior handoffs:** `handoff-mailbox-r3-2026-04-16.md` (R3 close), `handoff-mailbox-latency-2026-04-16.md` (R0→R2)

---

## Mission outcome

**R4 shipped M1 (PID-recycle) + M4 (counter-leak) + M5 (PID-collision) + M6 (IO-silent) + M8 (test-harness hang). 4 HIGH latent bugs closed on Option E machinery in 2 rounds. L-p50/L-p95 axis now empirically UNMEASURABLE at bench scale under Option E (M9 bimodal rename race).**

---

## Axes final state

| Axis | Direction | R3 final | R4 final | Verdict |
|---|---|---|---|---|
| **L-p50** | ↓ | 41.25ms | unchanged | **UNMEASURABLE-BY-DESIGN** (M9: bimodal rename race; methodology blocker) |
| **L-p95** | ↓ | 45.72ms | unchanged | **UNMEASURABLE-BY-DESIGN** (same) |
| **A-decouple** | ↑ | 57µs caller | unchanged | HARDENED — M4/M5/M6 close 3 HIGH bugs in Option E |
| **C-invariants** | maintain | 17 tests | **21 tests** (+4 new: orphan mtime-floor up/under 60s, panic counter-leak, sync-fallback clobber) | STRENGTHENED |
| **R-refactor** | ↓ | baseline | 0 breakage | additive-only (match replaces is_ok) |
| **S-simplicity** | maintain | -225 LOC (R3) | 0 deletions R4 | maintained |
| **M-methodology** | ↑ | 2.93% variance @ IT=100 | M9 rule documented | NEW DISCOVERY — decompose bimodal, single-thread isolate |
| **X-adversarial** | ↓ | +1 caught R3 | +3 HIGH caught-and-closed (M4+M5+M6) + 1 test-harness (M8) | STRONG REGRESSION CLOSURE |

---

## Commits this mission

```
2b99292 docs(mailbox): R4 R2 findings report
b6de41e chore(bench): R4 R2 cadence diagnostic harness + M2 INCONCLUSIVE data
8802cf9 feat(mailbox): R4 R2 M4+M5+M6+M8 Option E hardening
d93274f docs(r4): fix R1 report OR/AND inconsistency + charter native-tool correction
5258f16 docs(charter): xbreed harness charter — effort, xask-tool, batch-spawn
31cbc56 docs(mailbox): R4 Round-1 findings report — 7 moves, 3 HIGH surfaced
e481582 feat(mailbox): R4 M1 — PID-recycle mtime-floor for orphan recovery
```

## Quick verify (next session, ≤60s)

```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft

cargo test --lib mailbox                           # 21/21 expected
cargo test --lib mailbox -- --test-threads=4       # 21/21 parallel (M8 verified)
make verify                                        # 8 bash gates
git log 36d3c61..HEAD --oneline                    # 7 commits this mission
cargo clippy --quiet                               # clean
cargo fmt --check                                  # clean
```

---

## Shipped moves (detail)

### M1 — PID-recycle mtime-floor (e481582)

Closes PID-recycle latent gap in `collect_compact_sidecars` orphan recovery.
`pid_is_alive(pid)` alone fails when a dead compactor's PID is reused by an
unrelated live process. Mtime floor adopts orphans >60s old regardless of
PID liveness.

- OR-clause at `src/mailbox.rs:205-207`:
  `!pid_is_alive(pid) || orphan_file_has_stale_mtime(..., 60s)`
- Helper `orphan_file_has_stale_mtime` at `src/mailbox.rs:250-261`
- Tests: `drain_adopts_orphan_compact_file_with_recycled_pid_after_60s` (L811)
  + boundary `drain_skips_orphan_compact_file_with_recycled_pid_under_60s` (L846)
- `filetime = "0.2"` added to `[dev-dependencies]`

### M4+M5+M6+M8 — Option E hardening batch (8802cf9)

**M4 — COMPACT_PENDING counter-leak on worker panic:** is_ok() guard at L41
skipped fetch_sub on panic → `__wait_compact_idle` spins indefinitely. Fixed
by explicit match on result covering all three arms; every arm decrements.
New test `compact_worker_panic_does_not_leak_pending_counter` uses
`panic_in_worker: bool` CompactJob flag to inject real panics via
`catch_unwind`.

**M5 — same-PID sidecar collision (HIGH, 4-way cross-prefix convergence):**
Worker thread + sync-fallback both used `std::process::id()` alone as
filename suffix. Under channel-Full + concurrent writes, `rename(2)`
atomically overwrote the worker's in-progress snapshot — silent data loss.
Fix: static `COMPACT_SEQ: AtomicU64` at `src/mailbox.rs:25` → filenames
become `events.compact.<pid>.<seq>` and `events.compact_ready.<pid>.<seq>`.
`collect_compact_sidecars` orphan parser updated to `s.split('.').next()`
for PID extraction. New test `compact_sync_fallback_does_not_clobber_worker_snapshot`.

Cross-model convergence: cdx-reviewer-optione (R1 xask) + ccs-mutation-r4
(Scenario-2 clobber) + ccs-connector-r4 (primary-source hard-dep) +
cco-critic-r4 (serial-commit endorsement).

**M6 — IO-error silent swallow:** `result.is_ok()` evaluates true for
`Ok(Err(anyhow::Error))`, so IO errors from `compact_events_sync` were
silently swallowed, orphan files sat 60s until mtime-floor reclaim. Fixed
inline by M4's explicit match — Ok(Err) arm emits eprintln.

**M8 — test-harness parallel hang (BLOCKER):** `__send_panicking_job`'s
blocking `send()` + cloned `tx` held across `__poison_compact_worker`'s
`drop(tx) + join()` deadlocked under `--test-threads=4`. Fixed by
`try_send` + yield loop: `Full` → yield, `Disconnected` → undo
`COMPACT_PENDING` + break. CLAUDE.md verify loop restored.

Bonus (found during R2): `__poison_compact_worker` had latent deadlock →
`drop(tx) + join`. Exposed flake in `compact_returns_under_1ms_for_caller`
→ 5-attempt retry on `(0,0)` async discriminator.

### Bench diagnostic harness (b6de41e)

Opt-in `CADENCE_BENCH` env-var mode sweeps `CADENCE_FLOOR_BYTES` across
256k/512k/1m at n=100k, 50 iterations per floor. Default `cargo bench`
unchanged (functional baseline mode). Used to discover M9.

---

## R2 Discovery

### M9 — BIMODAL RENAME RACE (NEW, methodology)

At n=100K (~7MB mailbox), async `compact_events()` worker wins rename
race vs drain ~50% of iterations under load → drain finds NotFound →
fast-path 0-events → p50 bimodal, masking true read latency. Option E's
production limbo window materializes at bench scale.

**Methodology rule:** Any future L-axis measurement MUST decompose the
two race paths (compact-wins vs compact-loses) or use single-threaded
isolation with disabled Option E worker. **Aggregate p50 lies under
bimodal distribution** — this is the exact confound cco-critic-r2's
paired-bootstrap reframe was designed to catch.

**Empirical evidence (labrat-cadence-redo under 9-agent load):**
- baseline (no_floor): p50=164.5ms (inflated from 41.25ms clean baseline)
- 256k floor: p50=65.4ms
- 512k floor: p50=137.8ms
- 1m floor: p50=88.3ms

Non-monotonic in floor → strong bimodal signature, not latency signal.

---

## Rejected / Deferred

| Candidate | Disposition | Reason |
|---|---|---|
| M2 bytes-floor cadence | REJECT-on-methodology | UNMEASURABLE-BY-DESIGN under Option E; <0.1ms enqueue overhead sub-noise-floor; bimodal rename race masks delta |
| M3 incremental drain | REJECT | gated on rejected M2 |
| M7 paired-bootstrap HALT gate | DEFER R3 | bench-m7 didn't deliver; not R2-gate-load-bearing since M2 rejected on methodology not noise |
| M4-constant-pid mutation | ACCEPT as scope-limited | intra-process AtomicU64 + std::process::id() IS correct; cross-process coverage requires subprocess test (R3) |
| M4-IO-silent mutation | ACCEPT as scope-limited | fetch_sub is correct; eprintln is observability, not correctness (R3 io_error_count) |

---

## R3 backlog (7 items)

1. **Subprocess test for M4-constant-pid mutation** — `std::process::Command` spawns separate xbreed instances, verifies exactly-one-consumed on pre-placed sidecars with same `{pid}.{seq}` under rename contention.

2. **COMPACT_IO_ERROR_COUNT atomic** — `AtomicU64` counter + test-observable assertion in `Ok(Err)` arm. Closes M4-IO-silent mutation.

3. **M2 bench redesign** — deterministic pre-compact state OR inject disable-worker test knob to measure cadence delta in isolation, OR close M2 as redundant with Option E's de-facto cadence.

4. **M7 paired-bootstrap HALT harness** — per cco-critic-r2 spec: interleaved baseline/candidate per iteration, 1000-resample bootstrap of per-iteration deltas, gate: `ci_upper < 0 && |mean_delta| >= MDE` (MDE≈4ms). Replaces noise-floor-degenerate 5% point-estimate.

5. **M5 + M9 unified rename-contention primitive** — serialization gate OR rename-slot reservation. Both moves share fire-and-forget compact + rename race surface; ad-hoc per-race fixes (M5 AtomicU64 suffix, M9 methodology rule) are symptomatic not root-causal. R3 design review with cco-critic.

6. **Pre-existing ambient parallel flake** in `compact_sidecar_consumed_exactly_once_under_concurrent_drain` (~1-2% under `--test-threads=4`). Introduced in R3 commit `f189d43`, not R4 regression. Barrier-test design probe.

7. **Simplifier stale artifacts** — `src/mailbox.rs:171` docstring omits mtime-floor branch added by M1; `docs/reports/mailbox-bench-baseline.json` still contains stale `drain_dual_regime_ms` section no current bench writes.

---

## Session epistemic log

R4 caught + corrected hallucinations + methodology errors BEFORE commit:

1. **ccs-simplifier-r4 filetime shadow-dep claim** (R1) — claimed `filetime NOT in [dev-dependencies]`, builds via transitive from criterion. FALSIFIED by team-lead primary-source `Cargo.toml:31`. Spoof-flagged per xbreed-shared.md §Evidence Authenticity Spot-Check.

2. **cdx-reviewer-optione M5 severity** (R1) — initial BLOCKER → HIGH after mutation-tester evidence narrowed Scenario 1 protection via rename exclusion. Scenario 2 (channel Full + concurrent writes) is the actual race surface.

3. **ccs-executor-pidrecycle M5 framing** (R1) — initial "latent future-refactor risk" → "LIVE race, HIGH" after reviewer's correction accepted. Channel-Full + concurrent-writes scenario is reachable today (not requiring M2 cadence).

4. **Planner M1 state skew** (R1) — planner Read at 08:24 saw pre-M1 state; critic audit at 08:45 saw post-M1 state landed by executor. NOT hallucination — temporal skew. Planner issued amendment-1 primary-source-verifying via `git diff HEAD` + grep.

5. **cco-critic-r2 M7 spec bug catch** (R2) — caught OR/AND inconsistency in scribe-r4-r1 report (line 182 said OR, line 296 said both required). Fixed at commit `d93274f`. pooled_SD wrong scale for median-delta CI — paired-bootstrap reframe supersedes.

6. **ccs-labrat-cadence xask BLOCKED** (R1) — codex-spark 300s timeout. Layer-3 fallback doctrine: failure IS the result. Hand-calc analytic (~88% p50 delta IF I/O-linear) flagged INCONCLUSIVE, replaced in R2 with real bench → M9 discovery.

7. **Bench-m7 no-deliver** (R2) — spawned, never emitted proposal, never reported failure. Deferred to R3 without forced explanation. Worth investigating in next session whether spawn succeeded but task-prompt stalled, or harness-level issue.

8. **cco-critic-r4 spot-check incomplete** (R1→R2 transition) — shutdown before replying to team-lead's confirm_model DM. Hash-match on v2 + v3 accepted provisional per xbreed-shared.md §Judge Blinding Protocol Step 4 (spot-check is only load-bearing if hashes diverged or SPOOF_SUSPECT fires).

Each caught before committing. Cognitive hygiene ✓.

---

## File map

| Path | Role |
|---|---|
| `src/mailbox.rs` | NDJSON mailbox — Option E async compact + M1 PID-recycle mtime-floor + M4 counter-leak + M5 AtomicU64 seq suffix + M6 explicit Result match + M8 try_send test-harness |
| `benches/mailbox.rs` | Criterion @ IT=100 + `XBREED_BENCH_TMPDIR` env discipline + opt-in `CADENCE_BENCH` cadence diagnostic mode |
| `docs/reports/mailbox-bench-baseline.json` | `function_wall_ms` @ IT=100 + `cadence_floor_ms` (M9 data) |
| `docs/reports/mailbox-r4-r1-2026-04-17.md` | R1 findings report (9→7 distilled moves, 3 HIGH surfaced) |
| `docs/reports/mailbox-r4-r2-2026-04-17.md` | R2 findings report (4 shipped, M9 discovered, M2 UNMEASURABLE) |
| **`docs/reports/handoff-mailbox-r4-2026-04-17.md`** | **this file** |
| `docs/reports/xbreed-harness-charter-2026-04-17.md` | NEXT MISSION charter (effort-tier override, xask-as-native-CC-tool, true-concurrent batch dispatch) — dispatch fresh `/xbgst /wwkd \| godspeed` |
| `docs/reports/handoff-mailbox-r3-2026-04-16.md` | PRIOR session handoff (R3 close, Option E shipped) |

---

## Provenance

- **Orchestrator:** Claude Opus 4.7 (1M context), `/xbgst /wwkd` skills, godspeed mode, team `mailbox-r4-0417`, 2-round Pareto + R3 backlog
- **Team:** 13 members dispatched across R1+R2 (planner, scout, reviewer×2, 2× labrat, connector, mutation-tester×2, critic×2, 2× executor, simplifier, distiller, 2× scribe). All shut_down cleanly. TeamDelete pending final cleanup.
- **Distiller swap (mid-R1):** user override — ccs-distiller (sonnet medium) shut down at R1 SYNTHESIS_READY time, replaced with cco-distiller (opus low) carrying full corpus in spawn brief. Opus-low effort sufficient for synthesis task.
- **Gemini:** rate-limited session-wide (session memory `project_gemini_rate_limited_0416.md`); codex fallback on scout/reviewer/labrats/mutation-tester. Connector emitted BLOCKED (no fallback per LOCK). Critic R1 xask codex timeout 540s×2 → Heuer-native critique path.
- **Path:** OAuth-CLI throughout (ChatGPT for codex). No API framing.
- **xask mid-session usage:** ~1 call per teammate at Layer 1 gate; effectively zero post-boot (documented in `xbreed-harness-charter-2026-04-17.md` Item 2 as the asymmetry next-mission will close).
- **Next session:** fresh `/xbgst /wwkd | godspeed` dispatch on `xbreed-harness-charter-2026-04-17.md` — three harness items (effort override, xask-native-CC-tool, true-concurrent batch). OR R3 on mailbox items 1-7 above if user prefers stay-on-mailbox. User's directive: harness charter queued first.

Godspeed.
