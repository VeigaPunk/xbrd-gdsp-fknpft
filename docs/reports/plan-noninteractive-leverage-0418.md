# Plan — Non-Interactive Mode as Leverage for xbreed Orchestration
**Session:** noninteractive-leverage-0418 | **Dispatched by:** the-judge | **Date:** 2026-04-18

## Phase 0 — State map

### What I looked at
- `scripts/xask` (lines 1–233): full flag inventory, model dispatch paths, bench logging
- `src/ask.rs` (lines 1–394): `build_codex_ask_with_loadout`, `execute_with_timeout`, `dispatch`
- `src/mailbox.rs` (lines 1–195): `write_event`, `drain_events`, compact worker
- `commands/references/xbreed-shared.md`: axis → profile mapping, enforcement tier table, labrat swarm spec

### Data Walk artifact

**Exists (load-bearing today):**
- `--json` flag: passed to `codex exec` when `json=true`; output is raw bytes captured by `execute_with_timeout` into `String` — currently passed through unparsed to caller
- `-o/--output-last-message`: writes final assistant turn to disk; consumed by individual callers but never feeds back into a downstream delegate programmatically
- `--ephemeral`: each codex dispatch is stateless by construction
- `XBREED_BENCH_LOG`: structured bench records (wall_s, exit_code, effort, teammate) written per-dispatch; never read back by xbreed at runtime
- `write_event`/`drain_events`: mailbox is the CC-session coordination layer; delegate output reaches judge only via CC SendMessage
- `execute_with_timeout`: pipes stdout+stderr via 2-thread concurrent read; result is a flat `Vec<u8>` → `String` — structure from `--json` is not extracted

**Missing (not yet implemented):**
- No parser for `--json` structured output in `src/ask.rs::dispatch` — structure is swallowed into String
- No file-chaining protocol: `-o <fileA>` output never injected as `-c` context for the next delegate
- No runtime reader for `XBREED_BENCH_LOG`: written but never parsed for degradation/routing signals
- No mailbox-write path from headless `xask` delegates (delegates use CC SendMessage only)
- No gemini piped-stdin multi-turn accumulation (each call reconstructs context from scratch)

**Risk:**
- `--json` parse is model-version-sensitive: codex JSON schema could silently change between gpt-5.4-mini versions — any parser needs schema-version pinning or defensive field access
- File-chain handoffs via `-o` introduce a temp-file lifecycle that must be cleaned up (orphan risk on timeout/kill)
- Trendsetter constraint: all proposed moves MUST use capabilities already present in codex/gemini CLIs; no patches to accommodate missing capability
- Mailbox fd-cache constraint (`project_mailbox_fd_cache_constraint.md`): async compact widens stale-inode window — any mailbox-based output collection must not widen this window further

### Spec/Reality divergences
- `--output-last-message` exists in both `scripts/xask` (`-o` flag) and `src/ask.rs` but has no downstream consumer in the orchestration loop — it exists only as a write primitive, not a read primitive
- `--json` passes through but the JSON structure is never leveraged for routing; the value of structured output is currently zero

---

## WWKD

1. **What:** Identify and sequence the 3-5 highest-leverage non-interactive surfaces in xbreed's codex/gemini dispatch layer, producing a gated milestone plan so downstream executors can implement them in dependency order without ambiguity. Success boundary: plan artifact emitted with runnable gates per milestone; no code written here.

2. **Why:** xbreed dispatches codex/gemini as opaque string-in / string-out calls. Structured non-interactive primitives (`--json`, `-o`, bench log) are already wired at the CLI level but the orchestration layer ignores their structure. This leaves coordination overhead on CC SendMessage (slow, TTY-bound) when file-based async handoffs could achieve the same flow at subprocess latency.

3. **Assumptions/Risks:**
   - Codex `--json` schema is stable enough across gpt-5.4-mini versions for defensive parsing (delegate to cdx-delegate-recommender to verify)
   - Gemini non-interactive pipe mode is already supported (delegate to g-scout-noninteractive to verify; plan gates on this)
   - `-o` temp file lifecycle can be handled by `execute_with_timeout`'s existing kill + cleanup path
   - Trendsetter: zero patching of codex/gemini CLI to accommodate xbreed — if a capability doesn't exist upstream, the surface is REJECT

4. **How (milestone order):**
   - M01: Verify delegate capabilities (gate: empirical probe of `--json` schema + gemini pipe)
   - M02: Parse `--json` output in `dispatch()` → expose structured fields to caller
   - M03: `-o` file-chain protocol for A→B delegate handoffs
   - M04: Bench log reader → runtime degradation routing signal
   - M05: Gemini piped-context accumulation (gated on M01 verification)

5. **Escalation points:**
   - If cdx-delegate-recommender reports `--json` schema is unstable → M02 becomes REJECT; escalate to judge before executor dispatch
   - If g-scout-noninteractive reports gemini CLI lacks pipe-stdin support → M05 is REJECT; remove milestone
   - If critic (ccs-critic-trendsetter) finds any M01-M04 milestone requires patching codex/gemini → that milestone is REJECT per trendsetter constraint

---

## Milestones

**M01 STATUS: COMPLETE — Round 2 peer verdicts integrated (2026-04-18)**

Round 2 findings (cdx-labrat M01 gate, ccs-critic-trendsetter Axis R, cdx-reviewer-leverage Axis L, g-connector-couplings Axis X):

- **Codex `--json` fields confirmed:** `type`, `thread_id`, `id`, `text`, `usage`, `input_tokens`, `cached_input_tokens`, `output_tokens`. `text` extractable via `jq .text`; `type` usable for event-type discrimination. No xbreed JSONL parser required — `jq .text` or `jq select(.type=="...")` suffices.
- **Trendsetter Axis R REJECTS stream-json (M02a):** gemini `--output-format` not exposed via xask (docs/xask-protocol.md:157 — adding it requires xbreed adaptation = trendsetter violation). M02a PRUNED.
- **Trendsetter Axis R PASSES:** cross-model text stdout handoffs, `-o` file capture (already wired: `src/ask.rs:64`, `scripts/xask:50`).
- **Reviewer Axis L:** stream-json → mailbox_sink is silent data-loss path (fd-cache constraint — async concurrent writes widen stale-inode window). Stdout-passthrough only if stream-json ever enabled. Mailbox writes must remain post-completion serialized.
- **Connector Axis X — hard constraint:** `guard.rs` only scans `Bash tool_input.command` args — NOT piped stdin content. Any pipeline composing `codex | gemini` bypasses `deny_bash_patterns` for the pipe target. Every pipeline milestone must include a policy evaluation step covering the full composed command string.
- **Flag-order constraint confirmed (labrat empirical):** `-o` MUST precede model arg in xask or flag leaks into positional slots silently. Executor briefs must hard-specify `-o <file> codex "<prompt>"` order.

**Plan revision (post-Round 2):**
- M02a (stream-json) → **PRUNED** (trendsetter REJECT; xask would need to expose `--output-format` flag)
- M02b (codex `-o` capture) → **COMMITTED** (trendsetter PASS, free win confirmed)
- M05 (gemini stdin pipe) → **RE-INDEPENDENT** (no longer gated on M02a; stdin pipe is a separate native surface confirmed by g-scout)
- M06 (`--acp` daemon) → still ESCALATED to judge
- New hard constraint added to all pipeline milestones: guard.rs policy coverage for piped commands

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| ~~M01~~ | ~~Empirical probe~~ | DONE | All verdicts received | cdx-labrat ✓, g-scout ✓, critic ✓, reviewer ✓, connector ✓ |
| ~~M02a~~ | ~~Gemini stream-json~~ | **PRUNED** — trendsetter REJECT (flag not exposed via xask) | — | — |
| M02b | Codex `--json` `text`/`type` extraction + `-o` file capture | `xask -o /tmp/out-$$ codex "say: done"` → file non-empty; `jq .text` extracts response; flag order: `-o` before model arg | `cargo test test_codex_json_text_extract` + `test_codex_output_file_capture` | executor |
| M03 | `-o` file-chain A→B handoff — policy-gated | Dispatch A writes `-o /tmp/xask-chain-$$`; guard.rs policy check covers full composed command; B reads file as loadout context; file cleaned on timeout kill | `cargo test test_file_chain_handoff` green; guard policy scan covers pipe target | executor (after M02b; add guard.rs policy gate) |
| M04 | Bench log reader → degradation routing signal | `cargo test test_bench_degradation_signal` with fixture log; reader in `src/ask.rs` parses `XBREED_BENCH_LOG` | Test green | executor (independent) |
| M05 | Gemini stdin-piped context accumulation (independent) | `echo "ctx" \| xask gemini "summarize"` exits 0, non-empty; guard.rs policy covers piped stdin content | `echo ctx \| xask gemini "test"` exits 0 | executor (independent; add guard.rs constraint) |
| M06 | Gemini `--acp` JSON-RPC daemon — persistent session | ESCALATED — ACP concurrency undocumented; judge arbiter required before dispatch | Pending judge | — |

---

## Dependencies

```
M02b — independent (codex -o + --json text extract; free win confirmed by labrat + critic)
M02b → M03 (file-chain builds on -o primitive; also needs guard.rs policy gate)
M04  — independent (bench log reader; no CLI dependency)
M05  — independent (gemini stdin pipe; re-gated after M02a pruned; still needs guard.rs constraint)
M06  — ESCALATE to judge (ACP concurrency undocumented; block executor until judge arbitrates)
```

## Hard Constraints (cross-milestone; from connector Axis X)

1. **guard.rs stdin blindspot:** `guard.rs` policy scan covers only `Bash tool_input.command` args — NOT piped stdin content. Any milestone that pipes output between models (M03, M05) MUST add a policy evaluation step that covers the full composed command string including pipe targets. Without this, `deny_bash_patterns` is silently bypassed for inter-model content relay.

2. **Flag order for `-o`:** `-o <file>` MUST precede model arg in xask invocations (`xask -o <file> codex "..."`) — confirmed empirically by labrat. Executor briefs must hard-specify this order; xask arg parser exits on first positional, so flags after model arg are silently dropped.

3. **Mailbox writes: post-completion only.** No async concurrent writes during subprocess execution — widens the stale-inode compact race window (fd-cache constraint). All mailbox writes remain serialized via existing compact path.

## Leverage Surface Ranking (final post-Round 2)

| Surface | Axis improved | Risk | Verdict |
|---|---|---|---|
| Codex `--json` `text`/`type` + `-o` capture (M02b) | Zero-latency output persistence; `jq .text` extraction free today | None — labrat + critic confirmed | **PASS** |
| `-o` file-chain A→B (M03) | Dispatch latency — eliminates CC round-trip for linear pipelines | Orphan on SIGKILL; guard.rs policy gap on pipe target | **PASS** (with guard gate) |
| Bench log reader (M04) | Resilience — auto-route degraded delegates | Low | **PASS** |
| Gemini stdin pipe (M05) | Context fidelity — accumulated context without reconstruction | guard.rs stdin blindspot for piped content | **PASS** (with guard gate) |
| Gemini `--acp` daemon (M06) | Persistent session — eliminates per-call auth overhead | ACP concurrency undocumented | **CONDITIONAL** — judge arbiter |
| ~~Gemini stream-json (M02a)~~ | ~~Observability~~ | — | **PRUNED** — trendsetter REJECT |
| ~~Codex `--json` metadata routing~~ | ~~Routing fidelity~~ | `.metadata` null | **DOWNGRADED** — empirically disproved |
| ~~Mailbox headless-write~~ | ~~Coordination overhead~~ | fd-cache constraint | **ESCALATED** |

---

evidence: none — planning artifact
