# Handoff — NDJSON mailbox R3 mission → next session

**From:** `/xbgst /wwkd | godspeed` team `mailbox-r3-0416`, 2026-04-16 (2 rounds + implementation)
**Pushed:** `139601e..50d1803` (6 commits, main → main, local only — push when ready)
**R1 findings report:** `docs/reports/mailbox-r3-r1-2026-04-16.md` (9 moves, 4 rejections, CONTRADICTION-3 resolved)
**Prior handoff:** `docs/reports/handoff-mailbox-latency-2026-04-16.md` (R0→R2 mission)

## Mission outcome

**R3 shipped M1 methodology + M2 Option E + C-invariants coverage + S-simplicity deletion. CPU-parse layer empirically closed across 4 arms.**

Per prior handoff §R3+ candidates (Option E idle-thread compact, serde_json_borrow/Cow, simd-json per-line, H12 persistent fd, H10 substrate reopen), the R3 session processed every candidate through empirical probes, rejected all CPU-axis options on measurement, and landed Option E with critic-mandated hardening.

## Axes final state

| Axis | Direction | R0 baseline | R3 final | Verdict |
|---|---|---|---|---|
| **L-p50** | ↓ | 42.2ms drain @ n=100k | unchanged | CLOSED — CPU-parse empirically exhausted |
| **L-p95** | ↓ | ~44ms drain @ n=100k | unchanged | CLOSED — same |
| **A-decouple** | ↑ | 24.5ms sync caller | **57µs async caller (430×)** | SHIPPED (opt-in `compact_async`) |
| **C-invariants** | maintain | 13 tests | **17 tests** | 2 sidecar + 2 orphan-recovery + 3 Option E tests added |
| **R-refactor** | ↓ | baseline | 0 breakage | additive-only (compact_events unchanged) |
| **S-simplicity** | maintain | +225 mmap LOC | **−225 LOC + 1 dep** | mmap cluster + memmap2 dropped |
| **M-methodology** | ↑ | ±20ms @ IT=25 | 2.93% variance @ IT=100 | ITERATIONS 25→100 + TMPDIR |
| **X-adversarial** | ↓ | baseline | +1 caught, mitigated | orphan-recovery gap surfaced + closed |

## Commits this session

```
8216eb4 style(mailbox): cargo fmt on orphan recovery block
24b1538 fix(mailbox): R3 Gate 4 — orphan recovery for crashed compactors
f563165 docs(handoff): mailbox-r3-0416 mission complete — R0→R3 summary (this file, pre-Gate-4)
50d1803 chore(mailbox): R3 follow-up — Cargo.lock memmap2 drop + baseline JSON refresh
335aae4 perf(mailbox): R3 M2 Option E — async compact worker, 430× caller latency reduction
b9893e3 docs(mailbox): R3 round-1 findings report — 9 moves, M2 Option E accepted w/ critic conditions
e598400 perf(bench): R3 M1 methodology — ITERATIONS 25→100, explicit TMPDIR
f189d43 test(mailbox): R3 M-tests — cover concurrent-drain ownership + .tmp sidecar filter
27c7e97 perf(mailbox): R3 M-deletion — drop mmap cluster (REJECTED R2, ~225 LOC + 1 dep)
```

**Gate 4 audit note:** Post-ship distiller audit caught Gate 4 (orphan recovery) missing from 335aae4. Landed in 24b1538 (+93 LOC to `collect_compact_sidecars` + 2 tests). Final test count: **17/17 mailbox tests green**. All four Option E gates confirmed landed.

## Quick verify (next session, ≤60s)

```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft

make verify                            # 8 bash gates
cargo test --lib mailbox               # 15/15 expected
git log 139601e..HEAD --oneline        # 6 commits this mission
jq '.function_wall_ms.compact_events.\"n=100000\"' docs/reports/mailbox-bench-baseline.json
# Expected: {"p50_ms": 23.55, "p95_ms": 31.34}  (ITERATIONS=100, 2.93% variance)
```

## Shipped moves (detail)

### move_005 — S-simplicity deletion (27c7e97)

mmap cluster + dual-regime bench + memmap2 dep all dropped. R2 verdicts (REJECT on both cold+warm cache regimes) meant the code was dead by empirical closure, not hypothetical.

- Removed from `src/mailbox.rs`: `drain_events_mmap` pub fn, `parse_drain_file_mmap`, 3 mmap tests
- Removed from `benches/mailbox.rs`: `RegimeStats`, `collect_drain_dual_regime_ms`, `REGIME_SIZE`
- Removed from `Cargo.toml`: `memmap2 = "0.9"`
- Net: -225 LOC, -1 dep

### move_001 + move_002 — C-invariants coverage (f189d43)

Distiller surfaced 4 mutations that survived the R2 test suite — all in `collect_compact_sidecars`:
- M1: read-before-rename (rename-order flip) — SURVIVED
- M4: `.tmp` filter removed — SURVIVED
- E: copy+delete replaces rename (atomicity removed) — SURVIVED
- F: `starts_with(sidecar_prefix)` always-true (accepts all files) — SURVIVED

New tests:
- `compact_sidecar_consumed_exactly_once_under_concurrent_drain` — kills M1 + E + F via 2-thread drain with pre-placed sidecar + foreign file
- `drain_skips_compact_ready_tmp_sidecars` — kills M4 via explicit `.tmp` placement

### M1 — M-methodology upgrade (e598400)

- `ITERATIONS` 25 → 100 (R1 noted ±20ms variance at 25 blocked all L-axis gates)
- Explicit `TMPDIR` discipline via `XBREED_BENCH_TMPDIR` env var (defaults `/tmp`, WSL2 ext4 substrate assumption documented)
- Verified: 2.93% variance across 3 consecutive runs @ n=100k (under 5% gate)

### move_003 — M2 Option E async compact (335aae4)

Opt-in `compact_async` fn alongside sync `compact_events` (API-preserving). Caller cost 24.5ms → 57µs = 430× reduction. Critic-reframed as **CLI-ergonomics axis**, not hot-path latency — `compact_events` has exactly one production call site (`src/main.rs:96` CLI subcommand handler).

**Four gates applied (per cco-critic-r3-arch FINAL + cco-critic-r2-arch late finding):**
1. `sync_channel(1)` cap=1 — prevents pid-rename collision on concurrent same-process compacts
2. `catch_unwind` around worker body — prevents silent panic-drop of future compaction
3. Opt-in `compact_async` fn — sync `compact_events` preserved; no caller ripple
4. **Orphan-recovery in `collect_compact_sidecars`** — adopts `events.compact.<dead_pid>` files whose owning PID is dead. Closes pre-existing latent bug that Option E would make load-bearing (async window amplifies panic/exit-between-rename-and-sidecar-publish into common case).

## Rejected (empirically durable)

| Candidate | Arm | Result | Root cause |
|---|---|---|---|
| M3-simd | per-line simd-json | 0.57× (slower) | AlignedVec private API + mutable-buffer tax; +129MB alloc delta |
| M3-alt | hand-rolled byte-scan | 0.53× + correctness kill | SIMD setup dominates at ≤200B payloads; escaped-quote handling broken |
| M4b | Cow/BorrowedEvent | 0.38× (slower) | serde_json escape processing forces materialization regardless of lifetime |
| M4a | SmolStr narrow fields | 0.89× | Inline-branch overhead > allocator fast-path at this call frequency |
| M5 H12 | persistent fd | invariant amplification | TOCCTOU on drain/compact rename; amplifies documented best-effort window by orders of magnitude |

## Durable negative (preserve across sessions)

**Parse + alloc are fused in `serde_json` via escape-sequence processing.** Type-level surgery (Cow borrow, SmolStr inline, hand-rolled parser, simd-json) cannot separate them at xbreed's payload sizes. Four empirical probes confirm. Future sessions should not re-probe CPU-layer until the data format or batching strategy changes. The L-p50/L-p95 frontier is **I/O-axis only** from here.

## R4 candidates (deferred, not automatic)

From planner addendum-9 second-order framing — these require their own planning + charter:

1. **Compact cadence policy.** Option E unlocks aggressive compact-every-N-writes or compact-at-threshold. Reduces bytes at drain time (current drain reads full mailbox = dominant I/O cost at high event count). Compound I/O win stacked on M2's caller-latency win. **Expected:** material L-p50/p95 improvement if live-mailbox size drops.
2. **Rename elimination.** drain currently does `rename(path, .drain)` before read. Could read-in-place if writers use file-locking, but R1 rejected this (reintroduces lock contention). Worth re-examining only if drain-time file size is the binding constraint.
3. **Incremental drain.** Read N most recent events per call, defer older to background compact. Pairs with Option E naturally (async compact handles the deferred tail).
4. **Write batching.** `BufWriter` accumulating events with periodic flush. Changes `write_event` durability semantics (currently per-event `O_APPEND` → next fsync visible). Out of scope for compact-latency mission; separate charter.
5. **Reviewer/mutation-tester pass on M2 Option E.** Optional — the worker + channel + orphan-recovery machinery didn't get a dedicated review round. If R4 opens with fresh eyes on `compact_async`, worth a pass.

## Session epistemic log

R2 session caught hallucinations / methodology misreads BEFORE commit:

1. **Planner's "40ms = alloc-bound" cost decomposition (addendum-4 → retracted addendum-7):** decomposition framed drain wall as parse~20ms + alloc~40ms + other~2ms; M4b empirical REJECT (0.38×) falsified the separability. Retraction: parse + alloc are fused in serde_json escape processing.
2. **cdx-reviewer-refactor's initial "guaranteed categorical loss" severity on move_006:** reviewer self-corrected after primary-source read of `src/mailbox.rs:51-56` — mailbox is documented best-effort; H12 "amplifies documented acceptable tradeoff by orders of magnitude," not categorical violation.
3. **cco-critic-r3-arch's `move_009` "ROI≈0" framing (late addendum):** critic themselves updated to "ACCEPT with conditions" once A<C<B<D blast-radius ranking placed A (Option E) as lowest-risk. Framing: legitimate CLI-ergonomics win, not server hot-path.
4. **ccs-mutation-r3 EventSmol compile-warning flag:** distiller spoof-checked — grep for `EventSmol`/`SmolStr`/`smol_str` in `src/mailbox.rs` → NO MATCHES. Warning was from rejected labrat probe working tree, not production code. Discarded.
5. **cdx-reviewer's ndjson_bench.rs L59 compile error:** distiller cargo-check confirmed clean. Error was artifact of working-tree state during mid-R1 experiments; `src/bin/*.rs` probe scaffolds cleaned up at session end.
6. **cco-critic-r2-arch orphan-recovery late finding:** surfaced a latent bug in `compact_events` (rename-then-panic orphans `events.compact.<pid>`) that Option E would amplify. Mandated 4th implementation gate. Primary-source verified at `mailbox.rs:99/197/260` (line numbers pre-simplifier-deletion).

Each caught before committing. Cognitive hygiene ✓.

## File map

| Path | Role |
|---|---|
| `src/mailbox.rs` | NDJSON mailbox — write/drain/compact + NEW `compact_async` opt-in + orphan recovery |
| `benches/mailbox.rs` | Criterion harness @ ITERATIONS=100 + `XBREED_BENCH_TMPDIR` env discipline |
| `docs/reports/mailbox-bench-baseline.json` | `function_wall_ms` evidence @ ITERATIONS=100 (2.93% variance) |
| `docs/reports/mailbox-r3-r1-2026-04-16.md` | R1 findings report (9 moves, 4 rejections, hallucinations logged) |
| **`docs/reports/handoff-mailbox-r3-2026-04-16.md`** | **this file** |
| `docs/reports/handoff-mailbox-latency-2026-04-16.md` | PRIOR session handoff (R0→R2 mission that led into R3) |

## Provenance

- **Orchestrator:** Claude Opus 4.7 (1M context), `/xbgst /wwkd` skills, godspeed mode, team `mailbox-r3-0416`, 2-round Pareto + implementation round
- **Team:** 12 members dispatched (planner, scout, reviewer, 4× labrats, critic R1 late, R2 critic re-dispatch, connector, simplifier, mutation-tester, distiller, 2× executors, scribe). All shutdown_approved cleanly. TeamDelete next.
- **Connector:** `ccs-connector-r3` CC-native sonnet (user override — gemini rate-limited, no xask for connector this session)
- **Gemini:** rate-limited session-wide; codex fallback on scout/reviewer/labrats/mutation-tester. Connector emitted no xask (user override).
- **Path:** OAuth-CLI throughout (ChatGPT for codex). No API framing.
- **Next session:** fresh `/xbgst` if pursuing R4 compact-cadence + incremental drain, OR stand down — R3 charter satisfied.

Godspeed.
