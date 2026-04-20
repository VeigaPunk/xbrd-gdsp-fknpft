# xask-gate-regress-0420 R1 — xask Template Path + Gate Restore
**Status:** COMPLETE | **Date:** 2026-04-20 | **Session:** 1

---

## Round Overview

**Mission:** Converge on fix-shape for two confirmed regressions in the xask dispatch gate:
- **T-axis** — installed `~/.local/bin/xask` resolves templates to `~/.local/templates/dispatch` (nonexistent), silently falling back to raw `$QUERY`
- **G-axis** — commit 128e724 stripped verbatim per-role xask gate mandate strings from `commands/xbgst.md` Phase 2

**Axes audited:** T (template path) · G (gate mandate) · D (design intent / install contract) · F (fail-loud robustness) · V (validation / test coverage)

**Teammates (R1):**

| Name | Model | Axis | xask target |
|------|-------|------|-------------|
| the-revenger-xask-intent | codex-R | D | xask install contract + commit trail 5eeb772, 324402d, f3882aa, 128e724, 89a4efb |
| cdx-mutation-xask | codex-spark | F/V | 4-shape fix tournament + mutation score |
| g-connector-mesh | gemini-high | cross-axis | T × G × DEBUG trap × 128e724 × OAuth collapse |
| ccs-critic-approach | sonnet-medium | adversarial | heuer Layer 0 on 4 fix shapes |
| ccs-reviewer-xbgstmd | sonnet-medium | T+G scope | adversarial review, propose-diff-only |
| ccs-distiller | sonnet-medium | synthesis | dedup + conflict flag |
| ccs-scribe-r1 | sonnet-medium | documentation | this report |

**Codex pre-landing:** `/home/vhpnk/quickdir/fixwxbgst.md`
**Wall-time:** R1 ran concurrent with Pareto scoring (scribe + distiller dispatched simultaneously per protocol)

---

## Per-Teammate Findings

### the-revenger-xask-intent
**MOVE:** Patch Makefile install target to sync `templates/dispatch/` → `~/.local/templates/dispatch/`
**AXIS:** D (design intent)
**CLAIM:** Designer of xask knew the path was configurable via `XBREED_DISPATCH_DIR` escape hatch (documented in 324402d). However, the default path (`$SCRIPT_DIR/../templates/dispatch`) was never made install-reachable — `make install` synced the script but not the adjacent template tree.
**EVIDENCE:** `Makefile` install target pre-fix: no `cp templates/dispatch/` line; `~/.local/templates/dispatch/` absent post-install; `XBREED_DISPATCH_DIR` override in commit 324402d confirms designer awareness.
**REJECTED ALTERNATIVE:** None stated; revenger scoped to root-cause identification.
**CONFIDENCE:** Strong

---

### cdx-mutation-xask
**MOVE (M2):** Replace `scripts/xask:124-125` silent fallback (`else PROMPT="$QUERY"`) with fail-loud: stderr + exit 1
**AXIS:** F (fail-loud robustness)
**CLAIM:** Fix shape (a) fail-loud wins tournament on reachability(5) + reversibility(5), tier=runtime, bypass-surface=4/5. Shape (b) include_str! ranked #1 by initial codex-spark pass.
**EVIDENCE:**
- Symptom bytes confirmed: `1284` (no env, -d flag) vs `1257` (with `XBREED_DISPATCH_DIR`, -d flag) — envelope present/absent divergence
- Mutation score: **1/1 (100%)** after M1+M2
- Surviving mutant pre-fix: `else PROMPT="$QUERY"` at `scripts/xask:124-125`
- Killed by M2: `echo "xask: dispatch template not found..." >&2; exit 1`
- Regression guard: `XBREED_DISPATCH_DIR=/tmp/nonexistent bash scripts/xask codex 'probe'` → exit 1 + stderr
**REJECTED ALTERNATIVE:** Shape (b) include_str! — raised to conflict; see below.
**CONFIDENCE:** Certain (mutation gate closed)

---

### g-connector-mesh
**MOVE:** Pair fix — (c) Makefile install-copy + G-axis gate restore (not just T-axis alone)
**AXIS:** cross-axis (T × F × G)
**CLAIM:** Third-order effect surfaced — `--effort` flag silently no-ops without template because `{{THINKING_BUDGET}}` is never substituted. If only fail-loud lands without Makefile fix, the gate regresses from silent degradation to hard failure with no recovery path for default installs.
**EVIDENCE:** `scripts/xask:104-115` — effort→thinkingBudget mapping runs awk substitution inside `if [ -f "$TEMPLATE" ]` branch; missing template means xhigh/high/medium/low all route identically.
**REJECTED ALTERNATIVE:** T-only fix (shape a alone) — insufficient; leaves default install broken.
**CONFIDENCE:** Strong

---

### ccs-critic-approach
**MOVE:** Ranked fix shapes (a)>(b)>(d)>(c); rebuts shape (b)
**AXIS:** adversarial-design
**CLAIM:** include_str! in `src/ask.rs` has zero effect on `scripts/xask:124` because shell performs `$PROMPT` substitution at `xask:116-123` BEFORE `xbreed ask` runs. Shape (b) addresses a different surface than the regression.
**EVIDENCE:** `scripts/xask:199-202` — shell-side baking confirmed; `xbreed ask` receives an already-rendered prompt string, not a template path. Critic verified against `src/ask.rs` to confirm no template path injection exists on the Rust side.
**REJECTED ALTERNATIVE:** Shape (b) include_str! — structurally misses the regression surface.
**CONFIDENCE:** Certain (structural rebuttal, verified against source)

---

### ccs-reviewer-xbgstmd
**MOVE (M3 proposal):** `commands/xbgst.md` Phase 2 item 4 must carry verbatim per-role gate strings + grep-self-check mandate
**AXIS:** G (gate mandate)
**CLAIM:** T and G are separate regressions. G is not a consequence of T — it's an independent strip in commit 128e724 where the long "FIRST tool call MUST be Bash: xask" per-role table was replaced with a brief `| godspeed` suffix.
**EVIDENCE:** `commands/xbgst.md` Phase 2 item 4 pre-fix: 0 hits for `"FIRST tool call MUST be Bash: xask"`. Post-fix: 8 hits (grep confirmed by judge).
**REJECTED ALTERNATIVE:** Proposed diff-only per brief constraint; no alternative considered.
**CONFIDENCE:** Strong

---

## Cross-Model Conflicts

### CONFLICT: Fix shape tier ranking — (b) include_str! vs (a) fail-loud

| Source | Claim |
|--------|-------|
| codex-spark (cdx-mutation-xask, initial pass) | Shape (b) include_str! ranks #1 |
| codex-R (ccs-critic-approach) | Shape (b) structurally misses the regression surface; (a) wins |
| gemini-high (g-connector-mesh) | Pair (c)+(G restore) required; (a) alone insufficient without Makefile |

**Judge resolution:** Critic's rebuttal verified — `scripts/xask:199-202` confirms shell-side rendering; `src/ask.rs` never sees a template path. Shape (b) DROPPED. Pareto verdict: ship **(a) + (c) combined** — (c) makes the default install work, (a) catches any future regression.

---

## Pareto Verdict

| Move | Axis | Verdict | Evidence |
|------|------|---------|----------|
| M1 — Makefile sync templates/dispatch | D/T | ACCEPT + LANDED | Post-install `ls ~/.local/templates/dispatch/` → claude.md codex.md gemini.md; `xask -d gemini "ping"` (no override) produces full envelope |
| M2 — fail-loud at scripts/xask:124-132 | F | ACCEPT + LANDED | `rm-templates` probe → 3 stderr lines + exit 1; mutation score 1/1 |
| M3 — xbgst.md Phase 2 gate strings restored | G | ACCEPT + LANDED | `grep "FIRST tool call MUST be Bash: xask" commands/xbgst.md` → 8 hits (was 0) |
| M4 — xbreed-shared.md Naming Convention extended | V | ACCEPT + LANDED (bonus) | `make verify` green — unblocked `protocol_required_sections_have_body` test |
| (b) include_str! | — | DROPPED | Critic rebuttal: misses regression surface; codex-spark initial ranking overridden |

---

## Optimization Routes Surveyed

Four fix shapes were evaluated by the tournament:

| Shape | Description | Verdict |
|-------|-------------|---------|
| **(a) fail-loud** | Replace `else PROMPT="$QUERY"` with stderr + exit 1 at `scripts/xask:124` | **SHIP** — catches regression, mutation-verified |
| **(b) include_str!** | Embed template bytes in `src/ask.rs` at compile time | **DROPPED** — misses surface (shell renders before Rust runs) |
| **(c) install-copy** | Patch Makefile to `cp -R templates/dispatch/` during `make install` | **SHIP** — makes default install work; paired with (a) |
| **(d) env-required** | Require `XBREED_DISPATCH_DIR` env var; no default fallback | **DEFERRED** — breaks all installs without env-var migration plan |

G-axis fix (verbatim gate strings in xbgst.md) was treated as a separate dimension, not one of the 4 shapes.

---

## Spoof Flags

None. All claims traced to primary-source file reads or measured byte outputs. Shape (b) drop is critic structural rebuttal + verified at `scripts/xask:199-202` — not a content-state assertion.

---

## Audit Hash

```
pre-commit HEAD: b32b8e27036af3c380108a71d7684aca1b099871
diff-sha (4 files, 39 insertions): 580fe1cc1e2f
make verify: 9 PASS / 0 FAIL
```

---

## Links

- Pre-landing Codex assessment: `/home/vhpnk/quickdir/fixwxbgst.md`
- Dispatch templates (verified present): `templates/dispatch/gemini.md` (1038B) · `codex.md` (658B)
- Next: R2 (if triggered) — executor lands `tests/xask_template_missing.sh` test file (cdx-mutation-xask pending write); scope expansion requires orchestrator approval
