# xask-gate-regress-0420 R1 — xask Template Path + Gate Hardening
**Status:** COMPLETE | **Date:** 2026-04-20 | **Session:** 1

> **Framing note (reviewer late correction):** The G-axis fix (M3) is *protocol hardening*, not a regression reversal. Per-role verbatim gate strings were absent from `commands/xbgst.md` since initial commit ec13a52 — commit 128e724 did NOT strip them. Early report drafts cited 128e724 as G-axis cause; that attribution is INCORRECT. Do not propagate it.

---

## Round Overview

**Mission:** Reproduce and fix confirmed regressions in the xask dispatch gate; converge on fix-shape before R2 executor.

**Confirmed regressions:**
- **T-axis** — installed `~/.local/bin/xask` resolves templates to `~/.local/templates/dispatch` (nonexistent); silent fallback to raw `$QUERY` (16 bytes vs 1013 bytes with template)
- **G-axis** — `commands/xbgst.md` Phase 2 never carried verbatim per-role xask gate strings since ec13a52; gap silently enabled skip behavior (reframed as protocol hardening)

**Axes audited:** T (template path) · G (gate mandate) · D (design intent / install contract) · F (fail-loud robustness) · V (validation / test coverage)

**Teammates (R1):**

| Name | Model | Axis | xask target |
|------|-------|------|-------------|
| the-revenger-xask-intent | codex-R | D | xask install contract + commit trail 5eeb772, 324402d, f3882aa, 128e724, 89a4efb |
| cdx-mutation-xask | codex-spark | F/V | 4-shape fix tournament + mutation score |
| g-connector-mesh | gemini-high | cross-axis | T × G × DEBUG trap × 128e724 × OAuth collapse |
| ccs-critic-approach | sonnet-medium | adversarial | heuer Layer 0 on 4 fix shapes |
| ccs-reviewer-xbgstmd | sonnet-medium | T+G scope | adversarial review, propose-diff-only |
| ccs-distiller | sonnet-medium | synthesis | dedup + conflict flag (SYNTHESIS_READY) |
| ccs-scribe-r1 | sonnet-medium | documentation | this report |

**Codex pre-landing:** `/home/vhpnk/quickdir/fixwxbgst.md`
**Wall-time:** R1 ran concurrent with Pareto scoring (scribe + distiller dispatched simultaneously per protocol)

---

## Per-Teammate Findings

### the-revenger-xask-intent
**MOVE:** Patch Makefile install target to sync `templates/dispatch/` → `~/.local/templates/dispatch/`
**AXIS:** D (design intent)
**CLAIM:** Designer of xask knew the path was configurable via `XBREED_DISPATCH_DIR` escape hatch (documented in 324402d). However, the default path (`$SCRIPT_DIR/../templates/dispatch`) was never made install-reachable — `make install` synced the script but not the adjacent template tree.
**EVIDENCE:** `Makefile` install target pre-fix: no `cp templates/dispatch/` line; `~/.local/templates/dispatch/` absent post-install; `XBREED_DISPATCH_DIR` override in commit 324402d confirms designer awareness of configurability.
**REJECTED ALTERNATIVE:** None stated; revenger scoped to root-cause identification.
**CONFIDENCE:** Strong

---

### cdx-mutation-xask
**MOVE (M1/M2):** Makefile install-copy + fail-loud at `scripts/xask:124-132`
**AXIS:** F (fail-loud robustness)
**CLAIM:** Fix shape (a) fail-loud wins tournament on reachability(5) + reversibility(5), tier=runtime, bypass-surface=4/5. Shape (b) include_str! ranked #1 by initial codex-spark pass before critic rebuttal.
**EVIDENCE:**
- Symptom bytes: `1284` (no env, -d flag) vs `1257` (with `XBREED_DISPATCH_DIR`, -d flag) — envelope present/absent divergence
- Mutation score: **1/1 (100%)** after M1+M2
- Surviving mutant pre-fix: `else PROMPT="$QUERY"` at `scripts/xask:124-125`
- Killed by M2: `echo "xask: dispatch template not found at $TEMPLATE" >&2; exit 1`
- Regression guard: `XBREED_DISPATCH_DIR=/tmp/nonexistent bash scripts/xask codex 'probe'` → exit 1 + stderr
- G-axis addendum (M3): `commands/xbgst.md` Phase 2 now carries verbatim 12-row per-role gate table + pre-dispatch grep assertion; closes silent-skip mutation on G-axis. Regression guard: `grep commands/xbgst.md "FIRST tool call MUST be Bash: xask"` → ≥1 hit.
**REJECTED ALTERNATIVE:** Shape (b) include_str! — raised to conflict; see below.
**CONFIDENCE:** Certain (mutation gate closed)

---

### g-connector-mesh
**MOVE:** Pair fix — (c) Makefile install-copy + G-axis gate hardening; not T-axis alone
**AXIS:** cross-axis (T × F × G)
**CLAIM:** Third-order effect: `--effort` flag silently no-ops without template because `{{THINKING_BUDGET}}` is never substituted in raw-$QUERY mode. If only fail-loud lands without Makefile fix, the gate regresses from silent degradation to hard failure with no recovery path for default installs.
**EVIDENCE:** `scripts/xask:104-115` — effort→thinkingBudget mapping runs inside `if [ -f "$TEMPLATE" ]` branch; missing template means xhigh/high/medium/low all route identically, stripping multi-level gemini effort routing.
**REJECTED ALTERNATIVE:** T-only fix (shape a alone) — insufficient without Makefile pairing.
**CONFIDENCE:** Strong

---

### ccs-critic-approach
**MOVE:** Ranked fix shapes (a)>(b)>(d)>(c); structural rebuttal of shape (b)
**AXIS:** adversarial-design
**CLAIM:** `include_str!` in `src/ask.rs` has zero effect on `scripts/xask:124` because shell performs `$PROMPT` substitution at `xask:116-123` BEFORE `xbreed ask` runs. Shape (b) addresses a different surface than the regression.
**EVIDENCE:** `scripts/xask:199-202` — shell-side baking confirmed; `xbreed ask` receives an already-rendered prompt string. Verified against `src/ask.rs`: no template path injection on the Rust side.
**REJECTED ALTERNATIVE:** Shape (b) include_str! — structurally misses regression surface.
**CONFIDENCE:** Certain (structural rebuttal, verified against source)

---

### ccs-reviewer-xbgstmd
**MOVE (M3 proposal):** `commands/xbgst.md` Phase 2 must carry verbatim per-role gate strings + grep-self-check mandate
**AXIS:** G (gate mandate)
**CLAIM:** T and G are separate regressions. G fix is hardening, not reversal — gate strings were never present since ec13a52 (late correction, supersedes earlier 128e724 attribution).
**EVIDENCE:** Pre-fix `grep "FIRST tool call MUST be Bash: xask" commands/xbgst.md` → 0 hits. Post-fix → 8 hits.
**REJECTED ALTERNATIVE:** Propose-diff-only per brief constraint; no alternative considered.
**CONFIDENCE:** Strong

---

## Cross-Model Conflicts

### CONFLICT: Fix shape tier ranking — (b) include_str! vs (a) fail-loud

| Source | Claim |
|--------|-------|
| codex-spark (cdx-mutation-xask, initial pass) | Shape (b) include_str! ranks #1 |
| codex-R (ccs-critic-approach) | Shape (b) structurally misses regression surface; (a) wins |
| gemini-high (g-connector-mesh) | Pair (c)+(G restore) required; (a) alone insufficient without Makefile |

**Judge resolution:** Critic's structural rebuttal verified — `scripts/xask:199-202` confirms shell-side rendering; `src/ask.rs` never sees a template path. Shape (b) DROPPED. Pareto verdict: ship **(a) + (c) combined** — (c) makes the default install work, (a) catches future regressions. Shape (b) accepted as FINDING: correct long-term if `scripts/xask` is ever retired in favor of a Rust-native dispatch path.

---

## Pareto Verdict

| ID | Move | Axis | Verdict |
|----|------|------|---------|
| M1 | Makefile install sync + xask fail-loud (T+D+F) | T/D/F | **ACCEPT** — judge-reproduced byte delta 16→1013; post-install `ls ~/.local/templates/dispatch/` → claude.md codex.md gemini.md; rm-templates probe → 3 stderr lines + exit 1 |
| M2 | `commands/xbgst.md` verbatim gate strings (G-axis) | G | **ACCEPT AS HARDENING** — gate strings absent since ec13a52 (not a regression reversal); pre-dispatch grep-self-check added; 8 occurrences post-fix |
| M3 | Fix-shape (a) wins Pareto | F | **ACCEPT** — cross-model confirmed; mutation score 1/1 |
| M4 | Shape (b) include_str! = correct long-term if xask retired | — | **ACCEPT AS FINDING** — no R1 action |
| M5 | T and G are separate regressions | — | **ACCEPT AS FINDING** — structurally self-evident |
| M6 | Historical degraded-period runs = noise-as-signal | — | **DEFER** — LOW confidence, single source, unenumerable window |

---

## Optimization Routes Surveyed

Four fix shapes evaluated by tournament:

| Shape | Description | Verdict |
|-------|-------------|---------|
| **(a) fail-loud** | Replace `else PROMPT="$QUERY"` with stderr + exit 1 at `scripts/xask:124` | **SHIP** — catches regression, mutation-verified (1/1) |
| **(b) include_str!** | Embed template bytes in `src/ask.rs` at compile time | **FINDING ONLY** — correct if xask retired; wrong surface for current shell regression |
| **(c) install-copy** | Patch Makefile to `cp -R templates/dispatch/` during `make install` | **SHIP** — makes default install work; paired with (a) |
| **(d) env-required** | Require `XBREED_DISPATCH_DIR`; no default fallback | **DEFERRED** — breaks all installs without env-var migration plan |

G-axis hardening (verbatim gate strings in xbgst.md) is a separate dimension, not one of the 4 shapes.

---

## Spoof Flags

None. All claims traced to primary-source file reads or measured byte outputs. Shape (b) drop is critic structural rebuttal verified at `scripts/xask:199-202`. G-axis 128e724 attribution in early draft was corrected by reviewer before report finalization — not a spoof, propagated-error-corrected.

---

## Audit Hash

```
audit_hash (distiller): 29c3583fa958ac03cffbee2c03c694508a33eab84aad4b695697698dcfea7d59
pre-commit HEAD:        b32b8e27036af3c380108a71d7684aca1b099871
make verify:            70/70
```

---

## Links

- Pre-landing Codex assessment: `/home/vhpnk/quickdir/fixwxbgst.md`
- Dispatch templates (verified present post-M1): `templates/dispatch/gemini.md` (1038B) · `codex.md` (658B)
- Next: R2 candidate — `tests/xask_template_missing.sh` test file (cdx-mutation-xask pending write); requires orchestrator scope approval
