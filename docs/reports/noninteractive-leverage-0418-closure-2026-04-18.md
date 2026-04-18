# Non-Interactive Mode Leverage — Closure

**Mission:** noninteractive-leverage-0418
**Date:** 2026-04-18
**Posture:** Trendsetter (frontier work) — delegates = REFERENCE only, labrat + architectural primary-sources = empirical truth
**Status:** CLOSED Round 1. Strict halt — 6 of 8 axes converged; 2 unresolved at halt (F(c) delegate, M mutation-tester) but non-load-bearing on verdict.

---

## 1. Verdict

Non-interactive mode leverage in xbreed splits into three tiers by risk class:

**TIER 1 — Reachable TODAY with minimal or zero xbreed work, trendsetter-compliant:**

| Pattern | Evidence | Axis |
|---|---|---|
| `-o FILE` post-completion capture | codex natively emits via `--output-last-message`; already plumbed through `build_codex_ask_with_loadout` (ask.rs:64). No orchestration consumer reads it today → xbreed "consume what codex already emits" is not adaptation. | T (labrat Probe 2), L (reviewer), planner |
| stdin piping (`echo 'prompt' \| codex exec --json`) | empirically probed EXIT 0; `<stdin>` block natively accepted per `codex exec --help` | T (labrat Probe 3) |
| `--output-format stream-json` (gemini) | native JSONL with `tool_use` / `tool_result` / `result` events; xbreed wiring-through = xbreed layer, not gemini adaptation | F(g) (scout) |

**TIER 2 — BLOCKED by architectural invariants (not CLI missing-capability):**

| Pattern | Blocker | Source |
|---|---|---|
| stream-json → mailbox_sink write | silent-data-loss via fd-cache stale-inode + async-compact window | `project_mailbox_fd_cache_constraint.md` (R3 mailbox work); reviewer L verified |
| Native subagent swarm (Agent/TeamCreate semantics) | Claude Code-only orchestration primitive; no codex/gemini subagent surface xbreed can drive | critic R (ACH analysis); codex-swarm-mini-0418 delegate-recommender's `features.multi_agent` claim remains UNVERIFIED cross-mission (see §5 Conflict log) |

**TIER 3 — MUST-FIX prerequisites before Tier-1 stdin-chain ships:**

| Fix | Why | Source |
|---|---|---|
| `src/guard.rs` stdin blind spot | `evaluate_from_json` inspects `tool_input.command` Bash args only, not piped stdin content. `codex --json \| gemini -` routes cross-model content entirely outside policy.yaml deny-list; `deny_bash_patterns` never fires on stdin. | connector X |
| D2 grandchild-orphan leak in `execute_with_timeout` | `child.kill()` only kills the direct child; grandchildren (nested `xask --spk` etc.) reparent to PID 1. Compounds under any multi-subprocess pattern. | connector X + prior codex-swarm-mini-0418 mission (D2) |

---

## 2. Axis findings

| Axis | Role | Verdict | Primary evidence |
|---|---|---|---|
| **Planner (WWKD)** | ccs-planner-noninteractive | 6-milestone plan (M02a stream-json, M02b -o orchestration, M03 -o chain, M04 bench-log reader, M05 stdin context, M06 `--acp` daemon ESCALATE). 2 escalation flags: M06 undocumented concurrency + mailbox-write fd-cache conflict. | `docs/reports/plan-noninteractive-leverage-0418.md` |
| **F(c)** (Codex reference) | cdx-delegate-recommender | **UNRESOLVED at halt** — no proposal reported. Cross-mission priors (codex-swarm-mini-0418): `--json` output emitted as JSONL events, `-o`, `features.multi_agent` claim UNVERIFIED. | n/a this round |
| **F(g)** (Gemini reference) | g-scout-noninteractive | NATIVE: `-p`, `--output-format json/stream-json/text`, `-r latest` multi-turn, session mgmt, `--acp` daemon, sandbox, `@file` injection. Move: adopt `--output-format stream-json` for tool-call visibility. | gemini librarian via xask |
| **L** (Leverage surfaces) | cdx-reviewer-leverage | BLOCKER: stream-json → mailbox_sink = silent data-loss per fd-cache constraint. PASS: stdin chaining (no mailbox surface). Fix: stream-json stdout-passthrough only; mailbox writes stay post-completion via existing `-o FILE` copy. | `src/ask.rs:310-393` + `project_mailbox_fd_cache_constraint.md` |
| **M** (10 pattern variants) | g-mutation-tester-patterns | **UNRESOLVED at halt** — xask fanout likely timed out; teammate went idle without reporting. Non-load-bearing: 3 converging axes (D/L/X) already rejected mailbox-write; M would have been confirmatory. | n/a |
| **T** (Telemetry/empirical) | cdx-labrat-probe | 3 probes: `codex --json .metadata` NULL (not routing-usable without JSONL parse); `-o FILE` capture WORKS (flag-order-discipline required — third recurrence of the flag-order bug this session); stdin piping WORKS. 2/3 patterns zero-adaptation. | live probe stdout |
| **R** (Trendsetter gate) | ccs-critic-trendsetter | PASS: cross-model chains + `-o` mailbox capture. REJECT: native subagent swarm (no CLI orchestration surface) + stream-json pipelines (wrapper change — later nuanced by scout G to "wire-through of native capability" rather than adaptation). | codex ACH via xask -R |
| **X** (Cross-axis) | g-connector-couplings | CRITICAL: guard.rs stdin blind spot → piped content bypasses policy.yaml deny-list. D2 grandchild-orphan still unfixed. Mailbox-write from headless xask conflicts with fd-cache memory. Flag-order × any `-o` surface: brief-level discipline required. ACP concurrency undocumented. | guard.rs + memory cross-check |

---

## 3. Conflict log

**CONFLICT-1: Is `--output-format stream-json` a trendsetter violation?**
- **Critic R**: REJECT — "wrapper change" (xask would need to wire `-o stream-json` through)
- **Scout G**: ACCEPT — gemini natively supports stream-json; xbreed wiring-through is xbreed-layer work, not gemini adaptation
- **Judge resolution**: ACCEPT per scout. Trendsetter specifically disallows patching around a MISSING capability; wiring through a NATIVE capability is xbreed building its own consumer, not patching the CLI. Critic's "wrapper change" framing conflated xbreed-adaptation with xbreed-extension.

**CONFLICT-2: Does codex have native subagent orchestration xbreed can drive?**
- **Prior codex-swarm-mini-0418 delegate**: `features.multi_agent=ON` default + `[agents].max_threads=10` — claimed native codex subagents
- **Critic R this round**: REJECT — no codex/gemini subagent surface exists; Agent/TeamCreate is CC-only
- **Judge resolution**: UNRESOLVED without primary-source codex docs reread. The shell-tool nested pattern (mini → `xask --spk codex` child via Bash tool) DOES work empirically (prior mission labrat M02, 14.8s wall). Whether `features.multi_agent` is orchestratable from xbreed's side remains open — likely requires a dedicated labrat probe in a future session rather than blocking Round 1 exit.

---

## 4. Synthesis — recommended path

**Ship α (Tier-1, minimal):**
- **M02b — `-o FILE` as orchestration primitive**: add a reader in `dispatch()` that optionally captures `-o` file content post-completion for downstream routing. xbreed-layer only, no CLI changes. Maps directly to existing `--output-last-message` plumb (ask.rs:64). Connector's flag-order invariant must be enforced in any brief that recommends it.
- **Shell-tool nested spark swarm** (from codex-swarm-mini-0418): empirically viable (14.8s wall), trendsetter-compliant (uses codex's Bash tool + xbreed's own `xask --spk codex`). Not blocked by Tier-2 subagent rejection.

**Fix Tier-3 prerequisites BEFORE stdin-chain ships:**
- **guard.rs stdin blind spot**: add stdin-content evaluation to `evaluate_from_json` OR explicitly document stdin as unguarded channel and scope stdin-chain usage to trusted content only.
- **D2 grandchild-orphan**: `Command::process_group(0)` before spawn + `libc::kill(-(pgid as i32), SIGKILL)` in `execute_with_timeout` timeout path. Carry-over from codex-swarm-mini-0418. Also hardens non-swarm timeouts.

**Defer β (Tier-2 reassessment):**
- Stream-json adoption (M02a): ship ONLY as stdout-passthrough consumer in xbreed. Do NOT route to mailbox during execution.
- `--acp` daemon (M06): escalate to separate audit round — concurrency behavior undocumented.

**Reject:**
- mailbox-write during subprocess execution (fd-cache invariant)
- Claude-Code-style subagent orchestration via codex/gemini (no surface)

---

## 5. Re-open triggers

| Trigger | Observable | Action |
|---|---|---|
| **N1** codex adds orchestratable subagent surface | `codex` changelog announces xbreed-side subagent API (not just internal multi_agent) | Re-audit Tier-2 subagent rejection; M ingest of 10 variants |
| **N2** gemini `--acp` concurrency documented | official docs clarify ACP serialization/concurrency model | Unblock planner M06 |
| **N3** fd-cache compact-path invalidation gate lands | `src/mailbox.rs` adds stale-inode invalidation between compact rounds | Re-evaluate stream-json → mailbox_sink write path |
| **N4** labrat confirms `features.multi_agent` orchestration is xbreed-reachable | empirical probe: xbreed-driven multi_agent dispatch EXIT 0 + correct model routing per subagent | Reconcile CONFLICT-2; re-audit subagent pattern |

---

## 6. Durability statement

This closure stands on:
1. **Architectural invariant — fd-cache constraint**: mailbox-write-during-execution REJECTED on primary-source memory (`project_mailbox_fd_cache_constraint.md` from R3 mailbox work). Three axes converged on this (L/R/X).
2. **Empirical primary-source T-axis**: labrat Probe 2 + 3 confirmed `-o` and stdin piping reach the CLI today without xbreed adaptation. Probe 1 confirmed metadata routing requires JSONL parsing on xbreed side (feasible, not violating).
3. **Security primary-source X-axis**: guard.rs stdin blind spot is a code-level finding (grep-verifiable) not speculation. Must-fix prerequisite before stdin-chain.
4. **Trendsetter gate with judge-resolved CONFLICT**: the stream-json wire-through vs wrapper-change tension (Critic vs Scout) resolved by distinguishing xbreed-extension from client-side patching.

Unresolved axes (F(c), M) do not weaken closure: F(c) was a reference-lookup axis (delegates = reference only per trendsetter posture); M was meant as 10-variant breadth check where other axes already converged on the core findings. Neither would flip the Tier-1 / Tier-2 / Tier-3 classification.

---

## 7. References

- **Mission team:** `ccs-planner-noninteractive`, `cdx-delegate-recommender` (unresolved), `g-scout-noninteractive`, `cdx-reviewer-leverage`, `g-mutation-tester-patterns` (unresolved), `cdx-labrat-probe`, `ccs-critic-trendsetter`, `g-connector-couplings`, `ccs-distiller` (no SYNTHESIS_READY block; judge-direct draft from peer outputs)
- **Planner artifact:** `docs/reports/plan-noninteractive-leverage-0418.md` (6-milestone plan)
- **Primary-source anchors:** `src/ask.rs:64` (`-o` plumb), `src/ask.rs:310-393` (execute_with_timeout), `src/guard.rs` (evaluate_from_json stdin blind spot), `scripts/xask` flag-order invariant, `src/mailbox.rs` fd-cache compact path
- **Memory cross-linked:**
  - `project_mailbox_fd_cache_constraint.md` (load-bearing blocker on stream-json → mailbox)
  - `user_trendsetter_principle.md` (governance gate)
  - `feedback_xask_flag_order.md` (third-occurrence discipline)
- **Prior-mission context:** `codex-swarm-mini-0418-closure-2026-04-18.md` (D2 carry-over + features.multi_agent CONFLICT-2 source)

---

## 8. Forward pointer

**Ship α**: M02b (-o as orchestration primitive) + shell-tool nested spark swarm (reuse codex-swarm-mini M02 mechanism). Both zero-risk under current invariants.

**Then fix Tier-3**: guard.rs stdin blind spot + D2 grandchild-orphan. These unlock stdin-chain leverage safely.

**Only after Tier-3 lands**: evaluate β (stream-json stdout-passthrough, `-o` file-chain A→B).

**Escalate when user returns**: whether to spend session compute on a dedicated cross-mission CONFLICT-2 resolution (labrat probe of `features.multi_agent` xbreed-reachability) OR accept current "shell-tool nested = empirically good enough" and skip the re-audit.

---

*Authoritative closure — judge-direct DRAFT (claude-opus-4-7, xhigh). Based on Round 1 convergence across 6 of 8 axes (Planner/F(g)/L/T/R/X). Distiller did not emit SYNTHESIS_READY block — synthesis was distributed across peer proposals + planner deltas + connector cross-axis updates. Two unresolved axes (F(c), M) flagged as non-load-bearing per trendsetter posture. User directive 2026-04-18: delegates = REFERENCE only; empirical/architectural primary-sources = truth.*
