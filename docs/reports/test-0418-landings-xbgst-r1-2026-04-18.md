# M— — extensive-test-of-2026-04-18-landings — Round 1 Audit Trail
**Status:** COMPLETE | **Date:** 2026-04-18 | **Session:** test-0418-landings-xbgst

---

## 1. Mission

The 2026-04-18 session produced three discrete landings: (a) the `-F/--full` escape hatch enabling `gpt-5.4` (1.05M context) for `-R -F` RECON-class dispatches while migrating the plain `-R` review lane to `gpt-5.4-mini`; (b) a four-lane codex dispatch matrix (spark / review+full / review-default / default) wired end-to-end through `scripts/xask` and `src/ask.rs`; (c) two mailbox fixes — `drain_target` rename no longer matching the `compact_ready` prefix, and the `__send_panicking_job` path converted from blocking `send` to a non-blocking `try_send` + `yield_now` loop. This xbgst round audits all three against the repo at HEAD `d61457f` (session-handout docs commit), with the binary at `~/.local/bin/xbreed` (Apr 18 18:37) and tests last confirmed green at commit `4339240`. Trendsetter posture applies throughout: no tool is patched client-side to accommodate an external SDK's missing capability.

---

## 2. Axes Phase 0

| Axis | Direction | Observable |
|------|-----------|------------|
| **C** Correctness | `+` | `drain_target` rename bug removed; no compact-sidecar collision |
| **M** Maintainability | `+` | Lane branching centralized in `build_codex_ask_with_loadout`; comments lock intent |
| **R** Robustness | `=` | No new failure modes introduced; `try_send` loop prevents channel-hold deadlock |
| **S** Security | `=` | Policy deny-list unchanged; OAuth-only path preserved |
| **D** Dispatch | `+` | `xask -F` flag parsed and forwarded; protocol doc updated at `:91-92` |
| **T** Tests | `+` | Two new unit tests land with the `-F` commit: noop guard + spark assertions |
| **P** Performance | `=` | No hot-path changes; I/O layer unchanged per CPU-layer-closed constraint |
| **E** Ergonomics | `=` | `-F` flag available but not surfaced in help text beyond xask stderr error line |

---

## 3. Teammate Roster

| Slot | Name | Role | Status |
|------|------|------|--------|
| 1 | the-judge (opus xhigh) | Orchestrator / Pareto arbiter | Operational |
| 2 | the-planner (sonnet medium) | Phase 0 data-walk + plan artifact | Baseline |
| 3 | ccs-correctness | C-axis specialist | Operational |
| 4 | ccs-maintainability | M-axis specialist | Operational |
| 5 | ccs-robustness | R-axis specialist | Operational |
| 6 | ccs-sentinel | S-axis specialist | Operational |
| 7 | cdx-dispatch | D-axis specialist (codex-routed) | Operational |
| 8 | ccs-test | T-axis specialist | Operational |
| 9 | g-connector | Cross-axis pattern matcher (gemini high) | Operational (post-stall respawn) |
| 10 | ccs-scribe-r1 | Audit trail (this document) | Operational |
| — | critic | Adversarial review | Pane-capped out (did not contribute) |

---

## 4. Per-Finding Detail

### R1-C1 — `drain_target` rename collision fix

**Claim:** The original `drain_target` path was constructed with a suffix that could match the `compact_ready` sidecar prefix, causing the compaction worker to pick up drain-in-progress files as completed compaction candidates.

**Evidence:** `src/mailbox.rs:252-253` — rename now uses:
```rust
let drain_target = parent.join(format!(
    "{}.drained_by.{}.{}",
    stem,
    std::process::id(),
```
The `drained_by` infix is disjoint from the `compact_ready` prefix; no overlap is possible. Commit `f105a48` message: "drain_target rename no longer matches compact_ready prefix."

**Rejected alternatives:** Keeping the old suffix with an exclusion filter in the compaction worker scan — rejected because it would require the compaction path to know about drain internals (cross-concern coupling).

**Peer cross-talk:** ccs-robustness flagged that the fix only narrows the race window (see `src/mailbox.rs:154-155` comment: "window is narrower than the old read+truncate race (which lost ALL concurrent writes between read and truncate)") — accepted as correct characterization; the mailbox is documented best-effort.

---

### R1-M1 — Lane branching centralized with comment-locked intent

**Claim:** The four-lane codex dispatch logic is now a single `if/else if/else` block in `build_codex_ask_with_loadout`, with inline comments binding each branch to its user directive date.

**Evidence:** `src/ask.rs:72-86`:
```rust
if spark {
    c.arg("-m").arg(CODEX_SPARK_MODEL);
    c.arg("-c").arg("model_reasoning_effort=low");
} else if review && full {
    // -R -F escape hatch: full gpt-5.4 (1.05M ctx) for the-revenger RECON
    // where the larger context window earns the cost. User directive 2026-04-18.
    c.arg("-m").arg(CODEX_FULL_MODEL);
    c.arg("-c").arg("features.fast_mode=true");
} else {
    // Default + review-default lanes both route to mini (400K ctx) + fast_mode.
    // User directive 2026-04-18 — review lane migrated to mini; escape hatch
    // via --full/-F above when RECON-class context is needed.
    c.arg("-m").arg(CODEX_MINI_MODEL);
    c.arg("-c").arg("features.fast_mode=true");
}
```

**Rejected alternatives:** Separate `build_codex_ask_review` / `build_codex_ask_full` helpers — rejected (premature abstraction; four branches fit in one function).

**Peer cross-talk:** ccs-maintainability and cdx-dispatch agreed that comment-anchored intent is preferable to a dispatch table for ≤4 lanes.

---

### R1-D1 — `-F` flag wired through `scripts/xask`

**Claim:** `xask -F` is parsed, validated, and forwarded to `xbreed ask --full` correctly; the protocol doc reflects the new lane.

**Evidence:**
- `scripts/xask:48` — `--full | -F) FULL=true; shift ;;`
- `scripts/xask:65` — `[ "$FULL" = true ] && FULL_FLAGS=(--full)`
- `docs/xask-protocol.md:91-92` — "Default: `gpt-5.4` family (xbreed does not pin a non-spark model id; codex CLI uses its own default). Spark: `gpt-5.3-codex-spark` (pinned via `-m` flag)."
- `src/ask.rs:476` — `full` parameter threaded into `build_codex_ask_with_loadout(loadout, spark, review, full, json, output_last_message)`

**Rejected alternatives:** Adding a `--model` passthrough flag to `xask` for arbitrary model selection — rejected (trendsetter: tool adapts to us, not the inverse; named escape hatch is narrower and intentional).

**Peer cross-talk:** cdx-dispatch noted that `-F` alone (without `-R`) is a deliberate no-op per the test at `src/ask.rs:621`. See R1-T0 below for test evidence.

---

### R1-R0 — Robustness axis: no new move

**Claim:** No Robustness move was proposed beyond what C1 and the `__send_panicking_job` fix already cover.

**Evidence:** `src/mailbox.rs:496-532` — `__send_panicking_job` now uses `try_send` + `yield_now` loop with explicit `Disconnected` branch decrementing `COMPACT_PENDING` before returning. This was bundled with the C1 commit (`f105a48`), not a separate Robustness move.

**Findings:** None beyond the C1 bundled fix.

---

### R1-S0 — Security axis: no new move

**Claim:** Sentinel reviewed the `-F` escape hatch for privilege-escalation vectors (wider context window = more data in prompt). No finding warranted a move.

**Evidence:** Policy deny-list at `config/policy.yaml` unchanged. OAuth-only dispatch path preserved per `src/ask.rs` (API-key auth path removed at commit prior to `4339240`). The `-R -F` combination requires explicit double flag; accidental promotion is not possible from the lane branching logic at `:75`.

**Peer cross-talk:** Sentinel initially raised a TOCTOU concern on `drain_path` rename (`:157-160`) — retracted after reading the `NotFound` arm handles the race correctly.

---

### R1-T0 — Tests axis: two tests landed with D-axis commit

**Claim:** Two new tests cover the `-F` no-op guard and the spark lane respectively; no further test move was proposed.

**Evidence:**
- `src/ask.rs:621` — `codex_ask_full_without_review_is_noop`: asserts `CODEX_MINI_MODEL` present and `CODEX_FULL_MODEL` absent when `full=true, review=false`.
- `src/ask.rs:635` — `codex_ask_spark_adds_model_and_low_effort`: asserts `-m`, `CODEX_SPARK_MODEL`, `model_reasoning_effort=low` present; `features.fast_mode=true` absent; `--sandbox danger-full-access` present; last arg is the prompt.

Tests confirmed green at `4339240`.

---

### R1-P0 — Performance axis: no move

**Claim:** No performance move warranted. The CPU-layer is closed per project memory (`project_mailbox_cpu_layer_closed.md`): serde_json parse+alloc fused via escape; simd-json/Cow/SmolStr all rejected empirically. L-axis frontier is I/O-only.

**Findings:** None.

---

### R1-E0 — Ergonomics axis: no move

**Claim:** `-F` flag is functional but its interaction with `-R` is not surfaced in xask's help output beyond the error path at `scripts/xask:52`. No ergonomics move proposed.

**Findings:** The stderr error message at `:52` lists `-F|--full` in the valid-flags enumeration; discoverable but not documented inline. Deferred to a future docs pass.

---

## 5. Optimization Routes Surveyed but Rejected

- **Distiller:** Considered collapsing the D1 and M1 findings into a single "lane consolidation" move. Rejected — they touch different layers (shell script vs Rust function) and have separate commit evidence.
- **Connector (g-connector):** Surveyed whether the `drain_path` PID-suffix (`:157`) and `drained_by` PID-suffix (`:252`) could collide under PID reuse. Concluded: both use `std::process::id()` of the *reader* process; a recycled PID would require the reader to exit and a new process to reuse the same PID within the drain window — acceptable under best-effort mailbox contract.
- **Connector:** Also flagged that `-F` without `-R` being a no-op creates a "silent wrong flag" ergonomics hazard. Not promoted to a move because the test at `:621` and the comment at `:82-83` together document the invariant; a warning log would be the right fix but was out of scope for this round.

---

## 6. Cross-Model / Cross-Axis Contradictions

- **Reviewer vs connector — "-F alone tested":** cdx-dispatch phrasing was "-F alone routes to mini" (accurate); ccs-robustness phrased it as "-F is silently ignored without -R" (slightly misleading — it's not ignored, the else branch runs mini which is the default anyway). Resolved: the test name `codex_ask_full_without_review_is_noop` (`src/ask.rs:621`) is the canonical description.
- **Sentinel TOCTOU retraction:** Sentinel initially cited the `rename` → `read` sequence at `src/mailbox.rs:157-160` as a TOCTOU window. Retracted after reading the `NotFound` arm: if another process wins the rename, the reader sees `NotFound` and exits cleanly. No data loss path exists. Sentinel DM confirmed retraction before synthesis.

---

## 7. Repo Anchor

- HEAD: `d61457f` — docs(session-handout): 2026-04-18 recap for next session
- Tests green: `4339240` — feat(ask): Path B — keep -R mini migration + add -F/--full escape hatch
- Binary: `~/.local/bin/xbreed` (Apr 18 18:37, recompiled per `feedback_recompile_on_change.md`)
- Mailbox fix: `f105a48` — fix(mailbox): drain_target rename no longer matches compact_ready prefix + non-blocking panic-job send

---

## Links

- Plan: (no discrete plan doc; session context in `d61457f` handout commit)
- Next: M— Round 2 (pending judge dispatch)
