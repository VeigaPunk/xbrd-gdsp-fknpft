# Handoff — NDJSON mailbox latency mission → next session

**From:** `/xbgst | godspeed` team `mailbox-latency-0416`, 2026-04-16 (2 rounds + finale)
**Pushed:** `da01f0c..f0b2142` (4 commits, main → main, local only — push when ready)
**Depth reference:** `docs/reports/mailbox-latency-r1-2026-04-16.md` (8 findings, 6 hallucinations logged, R2+ candidates)
**Bench harness:** `benches/mailbox.rs` + `docs/reports/mailbox-bench-baseline.json` — criterion + dual-regime drain probe

## Mission outcome

**Charter-compliant null result on empirical-latency + correctness-axis win + infrastructure for R3+.**

Per handoff §Exit conditions ("Null result is acceptable IF the empirical data shows no optimization wins on p50 + p95 → commit a benchmark harness so the claim is verifiable and document the negative result"), the mailbox-latency-0416 mission shipped:

1. **R0 baseline** (`da01f0c`) — criterion bench harness + `mailbox-bench-baseline.json` `function_wall_ms` snapshot across 4 sizes × 3 functions
2. **R1 sidecar correctness fix** (`1463936`) — closes pre-existing I7 compact concurrent-write clobber race via `events.compact_ready.<pid>` atomic-rename sidecar pattern; `drain_events` unions live mailbox + sidecars on read; 3 new tests; PIPE_BUF comment correction per revenger's kernel-source citations
3. **R1 findings report** (`09743a0`) — 8 findings, 6 session hallucinations logged per `feedback_critic_hallucination.md`
4. **R2 dual-regime bench + mmap REJECT definitive** (`f0b2142`) — `collect_drain_dual_regime_ms` writes `drain_dual_regime_ms` section of baseline JSON; `drain_events_mmap` as opt-in pub fn with empty-file guard

## Empirical latency verdicts (locked)

Measured at n=100000, ITERATIONS=25, WSL2 ext4 `/tmp`:

| Hypothesis | p50 delta | p95 delta | Verdict |
|---|---:|---:|---|
| H7 `std::fs::read` + `serde_json::from_slice` | **+9.7%** | **+29.2%** | REJECT — `serde_json` validates UTF-8 per-value internally; `read_to_string`'s upfront scan is near-free |
| H10 mmap drain (cold-cache regime) | **+28.1%** | **+29.9%** | REJECT — mmap setup + TLB dominates |
| H10 mmap drain (warm-cache regime) | **+29.6%** | **+27.5%** | REJECT — warm page cache already eliminates kernel-copy |
| H15 `serde_json::to_writer` + `Vec<u8>` compact | **+12.8%** | **−24.9%** | REJECT — non-monotonic + I7 blocker + bench-path confound |

All four R1/R2-probed optimizations fail the "both p50 AND p95 improve" gate.

## Correctness verdicts (landed)

| Invariant | Status | Enforced by |
|---|---|---|
| I1 one event == one NDJSON line | preserved | `round_trip_single_event`, `drain_skips_malformed_lines_preserving_valid_events` |
| I2 O_APPEND non-interleaving for concurrent writers | preserved + accurately documented | `m02_concurrent_writer_torn_lines` (5120B > PIPE_BUF), corrected comment at `src/mailbox.rs:35-41` |
| I3 drain rename-before-read | preserved | `drain_empties_the_file` |
| I4 PID-scoped temp files | preserved + extended to sidecars | code + doc comment |
| I5 malformed-line filter_map skip (both drain variants) | preserved + newly tested | `drain_skips_malformed_lines_preserving_valid_events`, `drain_events_mmap_skips_malformed_lines` |
| I6 compact rename-before-read | preserved | `compact_preserves_kept_types_verbatim` |
| **I7 compact concurrent-write clobber race (NEW)** | **CLOSED via sidecar** | `compact_sidecar_preserves_concurrent_writes`, `drain_merges_compact_sidecar_even_if_mailbox_gone` |
| I8 partition rule (keep-type OR recent) | preserved | `compact_no_keep_types_collapses_to_digest` |

`cargo test --lib mailbox`: **13/13 green**. `make verify`: **8/8 gates green**.

## Architecture — sidecar pattern (R1 R2 stable ground truth)

`compact_events` no longer writes back to live `events.ndjson`. Instead:

```
compact_events:
  path = .xbreed/mailbox/events.ndjson
  rename(path, compact.<pid>)              # claim snapshot; path now absent
  read + partition compacted + kept
  write kept to compact_ready.<pid>.tmp
  rename(compact_ready.<pid>.tmp, compact_ready.<pid>)  # atomic publish
  remove(compact.<pid>)

drain_events:
  rename(path, drain.<pid>) → read + delete  # snapshot any live events
  for each compact_ready.* sidecar:
    rename(sidecar, drained_by.<pid>.<src>)   # claim exclusive ownership
    read + delete
  merge contents + per-line filter_map skip
```

Precondition: at most one concurrent `compact_events` caller per mailbox dir (race-free but wasteful otherwise). Writers are **never** blocked or affected by compaction.

## Quick verify (next session, ≤60s)

```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft

make verify                                # 8 bash gates
cargo test --lib mailbox                   # 13/13 expected
git log da01f0c..HEAD --oneline            # 4 commits this mission
cat docs/reports/mailbox-bench-baseline.json | jq '.drain_dual_regime_ms'
# Expected: cold+warm cache deltas both +27-30% for mmap (regression, both regimes)
```

## R3+ candidates (if revisited)

Deferred explicitly. Scope requires its own planning + charter — NOT automatic continuations.

### Architectural (higher-leverage)

1. **Option E: defer compact to idle thread.** Redis AOF `no-appendfsync-on-rewrite` + SQLite WAL `wal_autocheckpoint(0)` prior art (scout R1). Compact runs on `std::thread::spawn`-backed idle worker; caller never blocks on compact wall-time (currently ~24ms at n=100k on top of drain). Composes with sidecar — compact already decoupled from live path; idle defer adds caller-invisibility. **Expected win:** 100% removal of compact latency from caller path.

2. **serde_json_borrow / Cow<str> Event fields.** Scout R1 cited 451 MB/s → 1.118 GB/s (gh-archive shape data); ~17ms savings at n=100k drain. **Cost:** Event struct gets a lifetime parameter — ripples through `format_hook_injection`, CLI callers in `src/main.rs:77-98`, and every consumer that stores Events. Substantial refactor; worth only if R3 measures ≥10% on p50+p95 AND callers accept the lifetime burden.

### CPU-bound (format-axis)

3. **simd-json per-line.** NDJSON-compatible if called per-line (not bulk). ~2× parse speed on `nativejson-benchmark`. **Cost:** mutable padded-buffer constraint (needs `AlignedVec`-style allocation), unsafe entry points. **Expected win:** substantial on drain's serde_json parse bottleneck (connector R1: "serde_json parse CPU is the bottleneck"). Connector's format-axis framing is the strongest R3 candidate.

### Write-path

4. **H12 persistent fd write-burst.** Planner R1 addendum-5: the 1000-event burst path (not single-drain) is where `write_event`'s per-call `open(2) + close(2)` accumulates. **Gate:** ≥10% on p50+p95 burst wall-time; fd-invalidation (drain/compact rename → stat-per-write) costs too much to justify if invalidation logic bloats. Realistic outcome: 10-20% or reject.

### Methodology / infrastructure

5. **H10 regime reopen on a different substrate.** Current verdict (REJECT in both regimes) holds on WSL2 ext4. On disk-backed cold-cache deploys (bare-metal Linux + rotating disk, or VMs with small page cache), mmap's zero-copy could plausibly win. Pattern: run R3 dual-regime bench on a non-WSL2 target; if mmap wins ≥10% there, enable via feature flag. **Not urgent** — no known deploy target matches.

6. **Bench sample-size correction.** Criterion bench at `CRITERION_SAMPLE_SIZE=10` was used to save time this session; defaults (100) would reduce variance but take ~15min/run. If R3 needs tighter numbers, bump `CRITERION_SAMPLE_SIZE` + `CRITERION_MEASUREMENT_TIME`.

## Session epistemic log

Six hallucinations / methodology misreadings caught by primary-source verification:

1. **codex xhigh via cco-critic-arch initial ACH:** mmap "-26% p50" — never reproduced, self-corrected after labrat empirical falsification
2. **cdx-delegate-mailbox initial bench cite:** H10 "+19.2% win" at n=100k — retracted by cdx itself as 25-iter measurement variance
3. **cdx-delegate-mailbox ext4-cold-cache hypothesis:** pre-empted by labrat's existing /home probe showing +25.3% regression
4. **ccs-scout-priorart rename-into-place fix for I7:** incomplete — rename ALSO unlinks the fresh inode concurrent writer created (planner primary-source analysis); scout retracted
5. **g-connector-mailbox line number citations** (L217/L214/L347): drifted from BLOCKED content, retracted in distiller audit
6. **MEMORY.md index label "WSL2 ext4 beats tmpfs for sequential reads":** corrected — `/tmp` is ext4 on this WSL2 host per `df -T`; asymmetry is ext4-vs-ext4 path variance, not FS-type difference

Each caught before committing. Cognitive hygiene ✓.

## File map (for cold reader)

| Path | Role |
|---|---|
| `src/mailbox.rs` | NDJSON mailbox — write/drain/compact/drain_events_mmap (opt-in) |
| `benches/mailbox.rs` | Criterion harness + dual-regime drain probe |
| `docs/reports/mailbox-bench-baseline.json` | `function_wall_ms` + `drain_dual_regime_ms` evidence |
| `docs/reports/mailbox-latency-r1-2026-04-16.md` | R1 findings (8 sections, per-finding detail) |
| `docs/reports/handoff-codex-routing-2026-04-16.md` | PRIOR session handoff (led into this mission) |
| **`docs/reports/handoff-mailbox-latency-2026-04-16.md`** | **this file** |

## Provenance

- **Orchestrator:** Claude Opus 4.7 (1M context), `/xbgst` skill, godspeed mode, team `mailbox-latency-0416`, 2-round Pareto + direct finale
- **Team:** 13 members (the-planner, 3 labrats, reviewer, critic, revenger, scout, cdx-delegate, mutation-tester, connector, distiller, scribe). All shutdown_approved cleanly. TeamDelete successful.
- **Gemini:** rate-limited session-wide (2026-04-16 project memory entry); codex fallback on scout/revenger/mutation-tester; connector emitted BLOCKED and composed from in-session Grep
- **Path:** OAuth-CLI throughout (ChatGPT for codex). No API framing.
- **Next session:** fresh `/xbgst` if user wants to pursue R3 hypothesis E/simd-json/H12, OR stand down — mission charter satisfied.

Godspeed.
