# Bun Fit Audit — Closure

**Mission:** bun-fit-audit-0418
**Date:** 2026-04-18
**Status:** CLOSED. User halt after Round 1 on connector finding. No adoption.
**Round:** 1 of 4 (halted — 2 axis findings sufficient)

---

## 1. Verdict

**Bun has no role in xbreed at present.** Not rejected on a single missing feature — rejected on two independent gates that compose:

1. **Trendsetter disqualifier (X axis, primary kill):** `src/guard.rs` deny_bash_patterns is structurally blind to JS execution. A pattern like `bun -e "fs.rmSync('/')"` bypasses every policy.yaml regex. Not fixable by adding patterns — requires architectural migration of guard.rs to process/syscall isolation. Hosting Bun as-is forces xbreed to patch its security architecture around Bun's execution model, which is exactly what the trendsetter principle (`memory/user_trendsetter_principle.md`) disqualifies.

2. **Stack redundancy (A axis, secondary):** bash + python + rust cover every present surface. No concrete workflow exists today where Bun unlocks something the current stack cannot do.

The primary kill (1) is architectural and stands regardless of (2). Even if a compelling Bun upside surfaced tomorrow on some new surface, adoption would still be blocked until the guard.rs migration lands — making Bun downstream of a structural prerequisite, not a first-order integration choice.

---

## 2. Axis findings

| Axis | Role | Verdict | Primary evidence |
|---|---|---|---|
| A (Alternatives) | `cdx-labrat-stack` (codex --spark) | REDUNDANT on scripts/xask, scripts/xbreed-memory, bench-*.py, test harness. MARGINAL on hypothetical future MCP server. | codex-spark surface-by-surface audit, HYPOTHESIS/METHOD/RESULT evidence |
| X (Cross-axis) | `g-connector-crossaxis` (gemini high) | BLOCKED by guard.rs policy bypass. Secondary: bun:sqlite + Rust concurrent writes = SQLITE_BUSY on R3/M3 substrate without WAL (already mitigated upstream per labrat cross-DM, so not a new gate — just no new value). | `src/guard.rs` deny_bash_patterns scope analysis; `bun -e "<arbitrary JS>"` bypass vector |
| F (Fit-by-surface) | `cdx-reviewer-fit` (codex -R) | **Initial claim RETRACTED.** First verdict: "scripts/xbreed-memory is the ONE Bun fit via bun:sqlite prepared statements." Withdrawn after peer conflict verification: WAL+busy_timeout already mitigates concurrent-writer risk (`data/xbreed-schema.sql:3`); sql_escape fragility is real but Python sqlite3 parameterized queries — already in-stack — fix it without Bun; guard.rs:52,76 bypass confirmed as independent blocker; install path adds hard runtime dep. Final verdict: **REJECT Bun adoption** on all surfaces. | `xbreed-memory:30` line cite + `xbreed-schema.sql:3` WAL verification + `guard.rs:52,76` bypass vector + install-path coupling |
| U (Upside) | `g-scout-upside` (gemini medium) | **CONCEDE — no irreplaceable Bun upside.** bun:sqlite → rusqlite wins (direct C API, zero JS-engine overhead); bun vs tsx/tsc → solves Node cold-start that xbreed's Rust CLI does not have; MCP TS servers → stdio JSON-RPC means Bun stays out-of-process, never embedded; Bun Shell → solves JS-team scripting that xbreed doesn't need. Prior art (AkeruAI/bun-mixture-of-agents, Vercel AI SDK CLI) shows Bun as rescue-tech for JS-locked teams — zero Rust+Bun hybrid orchestrator citations. | gemini librarian research; prior-art null result |
| R (Risk / trendsetter) | `ccs-critic-trendsetter` (heuer + codex -R) | UNRESOLVED at halt (hit brief-bug on `-s` flag order; fallback retry in progress when user halted). **Not load-bearing** — X axis already applied trendsetter lens and produced the primary kill; R would have been confirmatory, not novel. | n/a |

**Peer DM cross-validation during Round 1:**
- labrat → distiller: `CONFLICT: reviewer says bun wins xbreed-memory; labrat says Python fixes same issue` (triggered reviewer's re-review)
- connector → reviewer: `CONFLICT flag: memory surface REDUNDANT per labrat — plus pull-through coupling read`
- connector → critic: "trendsetter already adjudicates" (X pre-empts R)
- labrat → connector: "SQLite WAL probe — already mitigated upstream; bun adds risk not capability"
- scout → connector: "MCP stdio path confirmed, no embedding needed"
- connector → scout: "ACK — MCP viable, install-path still live blocker"
- reviewer → distiller: "revised verdict: bun REDUNDANT, guard.rs security blocker"

**The cross-critique mechanism demonstrably worked.** Reviewer's initial F-axis claim ("xbreed-memory is the one Bun fit") was retracted after labrat + connector peer critique surfaced the WAL-already-mitigates + guard.rs-blocks-regardless findings. This is the godspeed Pareto filter operating as designed: a move that initially looked like a strict improver got dropped when cross-axis evidence showed it didn't survive harms-none verification.

**Round-1 convergence:** 4 of 5 axes landed findings (A, F, U, X); all 4 converge on REJECT. R unresolved but pre-empted by X. The user halt was correctly timed — additional rounds would produce zero axis improvement.

---

## 3. Why halt at Round 1 was correct

- X axis finding (guard.rs policy bypass) is a **structural blocker**, not a weight-of-evidence verdict. Adding F/U/R findings cannot un-block the security-architecture issue.
- A axis confirms **no value lost** by not adopting Bun today — there is no starved workflow.
- Peer DM alignment (labrat + connector agree; connector pre-empts critic's axis) shows the Pareto filter would accept only the block-adoption move. Running more rounds would produce zero axis improvement — the godspeed strict exit.
- Per user pattern (Honcho closure 2026-04-18), durable kill on primary-source architectural evidence is stronger than breadth-of-consensus.

---

## 4. Re-open triggers

| Trigger | Observable | Threshold | Action |
|---|---|---|---|
| **B1 guard.rs migration** | `src/guard.rs` moves from regex deny_bash_patterns to process/syscall isolation (e.g., landlock, seccomp-bpf, or bwrap-style sandbox wrapper around child processes) | migration lands + deny-list coverage extends to JS execution (bun/node/deno) | Re-evaluate Bun adoption; X axis kill is retired |
| **B2 MCP surface materializes** | xbreed starts shipping an MCP server (TypeScript SDK recommended path) | concrete requirement filed, not speculation | Scope-limited Bun re-audit for MCP runtime ONLY — not for replacing bash/python/rust scripts. Adoption still gated on B1 |
| **B3 user principle revision** | `memory/user_trendsetter_principle.md` edits loosen "no client-side patching" gate | explicit rewrite | Re-audit with new gate constraints |

None of these fire today. Closure stands.

---

## 5. Durability statement

This closure stands on:

1. **One architectural kill** (guard.rs policy bypass) that cannot be worked around at the Bun layer — the JS-execution bypass is a property of the runtime, not a configuration. Remediation requires changing xbreed's security surface, not Bun's.
2. **One redundancy finding** (labrat's per-surface audit) that confirms no starved workflow is pressuring adoption. Without pressure, the architectural gate holds indefinitely.
3. **Trendsetter precedent** — Honcho was killed by the same principle on 2026-04-18 for the same class of reason (client-side patching to host the tool). Bun applies the principle a second time on the same day. The principle is governance-active, not decoration.

Axes F and U landed during the shutdown window and independently converge on REJECT (F retracted its initial "xbreed-memory fit" claim after peer critique; U CONCEDED no irreplaceable upside). R remained unresolved but is the same lens X already applied via trendsetter — it would have been confirmatory, not novel. The closure thus stands on **four independent axes** (A REDUNDANT, F REJECT-after-retraction, U CONCEDE, X BLOCKED), not two.

---

## 6. References

- **User principle:** `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/user_trendsetter_principle.md`
- **Honcho closure (same-principle precedent):** `docs/reports/honcho-reaudit-closure-2026-04-18.md`
- **xbreed security surface:** `src/guard.rs` (deny_bash_patterns, PreToolUse hook), `config/policy.yaml`
- **SQLite substrate:** `data/xbreed.db` (R3/M3 — WAL already active)
- **Team roster:** `cdx-reviewer-fit`, `cdx-labrat-stack`, `g-scout-upside`, `ccs-critic-trendsetter`, `g-connector-crossaxis`, `ccs-distiller` (auto-cleaned on TeamDelete)

---

## 7. Forward pointer

Do not audit Bun adoption again unless B1 / B2 / B3 fires. The question is closed. Pivot available: `gemini-offload-0418` (labrat-swarm gemini routing + connector fallback tuning).
