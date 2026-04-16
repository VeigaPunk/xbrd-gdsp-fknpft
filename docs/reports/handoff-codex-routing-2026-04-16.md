# Handoff — codex routing session → next mission: NDJSON mailbox latency

**From:** `/xbgst | godspeed` team `cdx-defaults-0416`, 2026-04-16 (4 rounds + finale + hotfix)
**Pushed:** `a51957a..<HEAD>` to `github.com:VeigaPunk/xbrd-gdsp-fknpft.git` (main → main)
**Depth reference:** `~/claudevlt/claudevault/Projects/xbrd-gdsp-fknpft/codex-routing-extensive-2026-04-16.md` (12-section findings)
**This doc:** quick-start + next session's mission — NDJSON mailbox latency optimization.

## Urgent-fix shipped alongside this handoff

User hit `Error: failed to execute codex: xask-timeout: command did not complete within 60s`. Root cause: `src/ask.rs` default `XASK_TIMEOUT_SECS` was 60s — too tight for high/xhigh codex calls (m7 ACH needed 540s). Raised to **300s** (5 min) as the new default ceiling. Still overridable via the env var; still prevents runaway hangs. Commit landed with this handoff; see the commit table below for the SHA. Binary installed to `~/.local/bin/xbreed`. No further action needed — new default takes effect on the next codex call.

---

## Quick verify (new session, ≤60s)

```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft

# Full 8-gate + cargo. Everything must be green before optimizing anything.
make verify

# Sanity: the codex route pins gpt-5.4 at argv time (M10 sentinel).
cargo test --test ask_with_loadout ask_codex_route_preserves_full_unlock_contract -- --nocapture

# Live codex probe — spark path (should return ~3-4s):
time scripts/xask --spark codex "return DONE"

# Live codex probe — non-spark pin path (-m gpt-5.4 should appear in argv):
scripts/xask --debug --effort high codex "return DONE" 2>&1 | grep -E "\-m|gpt-5"
```

Expected: `make verify` exits 0, M10 integration test passes with `-m gpt-5.4` present, spark probe under 6s.

---

## Commits shipped this session (9)

| SHA | Round | Scope |
|---|---|---|
| `d8fe150` | R2 | `--color never` in codex base command |
| `ee89fff` | R2 | test sentinels `--ephemeral` + `features.fast_mode=true` |
| `a981cc5` | R2 | the-revenger codex effort medium → high |
| `98b6593` | R2 finale | `templates/agents/the-judge.md:29` SSoT drift fix |
| `30d42df` | R3 | `--json` Rust-layer plumbing (cli + main + ask + M8 test) |
| `2c15164` | R4 | `scripts/xask` shell-layer plumbing (`--json` + `-o`) |
| `2e99a24` | R4 | `-o/--output-last-message` Rust-layer plumbing (M9 test) |
| `2b07e53` | R4 | in-repo findings `docs/reports/codex-routing-findings-2026-04-16.md` |
| `30d4aa2` | finale | **`CODEX_DEFAULT_MODEL="gpt-5.4"` pin** (M10 test) |

80 tests + 8 bash gates green. Binary + `scripts/xask` installed to `~/.local/bin/`.

---

## Next session mission — `NDJSON mailbox latency optimization`

### Charter

Empirical latency optimization of `src/mailbox.rs` (the team-level inter-agent message bus, 310 LoC) via `/xbgst | godspeed`:

- **Objective function:** lower end-to-end wall-time of mailbox write → drain cycle. p50 + p95. Latency is the *only* axis that matters.
- **Token budget:** **unconstrained.** Use `xhigh` effort freely anywhere reasoning depth helps (designing probe matrices, evaluating alternative serialization strategies, ACH on architectural trade-offs). Do not premature-optimize for tokens.
- **Empirical discipline:** mutation-tester + labrat probes, multiple rounds, red-before-green, commit per optimization, `make verify` after every change.

### Mailbox current state (primary source — verify before trusting)

| Path | LoC | What |
|---|---:|---|
| `src/mailbox.rs:1-60` | 60 | NDJSON append-per-write (`write_event`) |
| `src/mailbox.rs:40-90` | 50 | atomic drain via `rename`+read (`drain_events`) |
| `src/mailbox.rs:95-170` | 75 | `compact_events` (periodic digest rollup) |
| `src/mailbox.rs` | 310 | total |

Current protocol:
```rust
pub struct Event {
    pub timestamp_ms: u64,
    pub from: String,
    pub event_type: String,
    pub payload: String,
}
// write: serde_json::to_string(&event)? + '\n' → OpenOptions.append
// drain: rename → read_to_string → lines → filter_map(serde_json::from_str)
```

### Optimization hypothesis space (next session empirically validates each)

**Hot path — `write_event`:**

1. **`serde_json::to_writer` instead of `to_string` + `write_all`** — skips the intermediate `String` allocation. Per-event win should be small but adds up at high throughput. **Probe:** `/usr/bin/time -f '%e'` 10k writes via stub loop, before + after.
2. **`&str`/`Cow<'_, str>` on `Event` fields** — current code does 3× `.to_string()` per write. If call sites hold owned strings already, a borrowed variant avoids copies. **Risk:** lifetime plumbing; the convenience of owned fields may win.
3. **Pre-allocated `Vec<u8>` buffer** passed through `to_writer` — amortize allocator pressure across burst writes.
4. **`O_APPEND` atomicity boundary (4096 byte PIPE_BUF)** — messages longer than 4096 bytes may tear under concurrent writers. Current code doesn't guard. **Probe:** write a 5KB payload concurrently from 4 stubs, check for interleaving. If real, add length-prefix framing or `flock`.

**Hot path — `drain_events`:**

5. **`BufReader` + line-by-line `serde_json::from_str`** instead of `read_to_string` — large mailboxes (100+ events) don't need full buffer. Lower p95 under mailbox bloat.
6. **`serde_json::Deserializer::from_reader(...).into_iter::<Event>()`** — streaming deserializer, skips per-line intermediate `&str` + avoids the silent `.ok()` drop pattern. Misformed events become explicit errors (kill-switch) or get routed to a corruption log.
7. **`std::fs::read` + `from_slice`** — skips UTF-8 validation twice (once in `read_to_string`, once in `from_str`).
8. **`memchr::memchr` for newline splitting** vs `str::lines()` — SIMD-accelerated line-break scan. Worth it only if p95 mailbox is >1 MB; probe first.

**Architecture-level:**

9. **Binary framing instead of NDJSON** (e.g., `bincode` / `postcard` / length-prefixed `serde_cbor`) — zero ambiguity on message boundaries, no escape overhead, but breaks `cat events.ndjson` human inspection. **Trade-off:** machine-speed vs debuggability. Split decision — keep NDJSON on-disk; use binary only on hot in-process paths if they exist.
10. **`mmap` for drain on large mailboxes** — zero-copy read. Only shines past a threshold. Benchmark with 1k / 10k / 100k event mailbox sizes.

### Protocol

Follow `/xbgst | godspeed` with these constraints baked in:

- **Phase 0:** `the-planner` (opus 4.7 high + `Skill('wwkd')` Layer 0) data-walks `src/mailbox.rs` primary-source-first. Don't trust the hypothesis list above blindly — it's a starting menu, not a spec.
- **Phase 1 axes (aim for 7-8):** `empirical-latency` (≥2 labrats — one write path, one drain path), `correctness` (reviewer: no regression of the atomicity guarantee or the drain/write race note at `mailbox.rs:40-50`), `reverse-engineering` (`the-revenger` if concurrent-writer semantics look suspect), `test-validation` (mutation-tester on the new hot-path code: comment out a buffer, remove a `?`, flip a `filter_map` → `map`, assert the test catches it), `adversarial-design` (`cco-critic` with Heuer: is binary framing worth the debuggability loss?), `research` (scout — prior art on NDJSON vs framed alternatives), `execution` (land winners, red-before-green, commit per optimization).
- **Mutation-tester rounds:** integrate tightly. After each optimization lands, run `ccs-mutation-tester-mailbox` against the NEW code. Every winner must survive mutation probes at the integration layer (not just unit tests). R2-finale of this session caught that integration-layer coverage often lags unit; do not repeat that gap here.
- **Empirical gate:** no optimization commits without `/usr/bin/time -f '%e'` N≥10 measurements showing p50 + p95 improvement. State the delta explicitly in the commit message.
- **Codex effort guidance:** this session proved `xhigh` is a real distinct tier (+67% tokens, +34% wall, 3× stdev vs high). Use `xask --effort xhigh codex` liberally for the adversarial-design axis (critic, Heuer ACH on binary-vs-NDJSON). Use `xask --spark codex` for bulk-probe labrats where throughput beats depth. Codex delegate's R1 top-3 unplumbed — `--output-schema`, `codex exec review --uncommitted`, `codex exec review --base` — are all potentially useful for the mutation-tester round; consider plumbing one if the round calls for it.
- **Telemetry:** enable `XASK_DEBUG=1` on every probe. Instrument `mailbox.rs` with `#[cfg(feature = "bench")]` lap counters if the naïve time measurement is too noisy. Codex self-recommended `codex debug prompt-input` as an observability hook — consider for the debug axis.

### Budget

- Tokens: unconstrained (user directive: "Tokens are no object").
- Rounds: cap 4.
- Teammates: cap 12/round. Spawn the cdx-delegate + the cco-critic + the-revenger on adversarial-design; don't be stingy.
- Wall-time: no explicit cap, but each round should commit something before starting the next.

### Exit conditions (strict)

- **Success:** at least ONE mailbox-path optimization with empirically-validated p50 and p95 improvement lands as a commit, carrying both a mutation-tester pass and a regression-free `make verify`.
- **Null result is acceptable** IF the empirical data shows no optimization wins on p50 + p95 → commit a benchmark harness so the claim is verifiable and document the negative result. Negative results *with harness* beat positive claims without measurement.

---

## Parked from this session (low priority — codex routing, not mailbox)

All detailed in the vault report §11. Short list:

- `--output-schema <FILE>` (ROI 10, medium cost) — JSON-schema response contract for distiller/scribe
- `codex exec review --uncommitted` + `--base <branch>` (ROI 9 / 8, low cost) — native PR-review lanes
- `-C/--cd <DIR>` (ROI 8, low cost) — scoped dispatch root
- `codex_hooks` feature — revisit when codex promotes it past under-development
- ~~`XASK_TIMEOUT_SECS` default — 60s too tight for some ACH runs~~ **RESOLVED this session: raised to 300s (5min) default. Override via env var still works. See hotfix commit.**
- 3-way benchmark harness — isolate `direct codex exec` vs `xbreed ask codex` vs `scripts/xask → xbreed ask codex`

Only take these up if the mailbox mission stalls or closes fast — mailbox latency is the priority.

---

## Handoff checklist

- [ ] Run the "Quick verify" block above. Expect 100% green (8 bash gates + 80 cargo tests + M10 argv sentinel).
- [ ] Read this doc + the vault report (`~/claudevlt/claudevault/Projects/xbrd-gdsp-fknpft/codex-routing-extensive-2026-04-16.md`) for codex-routing context if needed.
- [ ] Read `src/mailbox.rs` end-to-end before trusting the hypothesis menu in §Optimization hypothesis space.
- [ ] Spawn the-planner FIRST with `Skill('wwkd')` at Phase 0 per protocol. Make the planner data-walk real — don't let it paraphrase this doc.
- [ ] Dispatch ≥2 labrats on empirical-latency (write path + drain path) with telemetry enabled.
- [ ] Dispatch mutation-tester for post-commit validation on each winning optimization.
- [ ] Use `xhigh` effort liberally per user directive. Tokens are unconstrained.
- [ ] Emit final DRAFT + auto-cleanup + TeamDelete per `/xbgst` Step 6.

---

## File map (for a cold reader)

| Path | Role |
|---|---|
| `src/mailbox.rs` | **MISSION TARGET** — NDJSON mailbox write/drain/compact |
| `src/ask.rs:100-151` | codex dispatch builder (post-session, pins `gpt-5.4` or `gpt-5.3-codex-spark`) |
| `src/ask.rs:178-188` | CODEX_SPARK_MODEL + CODEX_DEFAULT_MODEL constants |
| `tests/ask_with_loadout.rs:189-260` | M7+M8+M9+M10 integration sentinels for codex argv contract |
| `scripts/xask` | shell-layer dispatcher (post-session: parses `--json`, `-o`, `--effort`, `--spark`) |
| `commands/references/xbreed-shared.md` | SSoT for axis→profile mapping + Layer-1 gate strings |
| `docs/reports/codex-routing-findings-2026-04-16.md` | in-repo lightweight findings |
| `~/claudevlt/claudevault/Projects/xbrd-gdsp-fknpft/codex-routing-extensive-2026-04-16.md` | vault extensive findings (12 sections) |
| `docs/reports/handoff-codex-routing-2026-04-16.md` | **this file** |
| `docs/reports/handoff-xbgst-r2-2026-04-16.md` | prior session's handoff (pre-codex-routing) |

---

## Provenance

- **Prior orchestrator:** Claude Opus 4.7 (1M context), `/xbgst` skill, godspeed mode, team `cdx-defaults-0416`, 4-round Pareto + direct finale.
- **Session commits:** `a51957a..30d4aa2` (27 commits ahead of pre-session head including the prior handoff session).
- **Path exclusively OAuth-CLI.** No API keys, no API framing. User memory `feedback_oauth_not_api.md` enforced.
- **Hallucination budget:** 4 fabrications caught this session (scout-flags xhigh-clamp, codex self-review `&`-bug, cco-critic-r2 phantom `--color` + phantom the-revenger). Primary-source verify via Read/Grep before acting on any "already fixed" / "already present" assertion. Memory `feedback_critic_hallucination.md`.
- **Next session start command:** `/xbgst` (with this doc's mission section as input). `/wwkd` skill loads inside the-planner's Layer 0 automatically.

Godspeed.
