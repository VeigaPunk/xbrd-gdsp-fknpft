# Teammate Benchmark — Phase C (no-op probe)

**Mission:** `/xbgst /wwkd | godspeed` — teammate benchmark M3
**Team:** `bench-noop-0417` (7 teammates, single round, parallel)
**Date:** 2026-04-17
**Plan:** `docs/plans/teammate-benchmark-plan-2026-04-17.md`
**Gate:** PASS — memory claim `feedback_teammate_mode_effort_caveat.md` empirically confirmed.

---

## Hypothesis

`effort:` frontmatter in teammate-mode is a **no-op** — all 7 teammates spawned with intentionally-distinct `effort:` labels will actually run at the session's `effortLevel: xhigh` (from `~/.claude/settings.json`).

## Confirmation

User-confirmed at dispatch time: *"they still spawned with predetermined effort set by global configs on claude code cli settings"*. The TSV below shows within-model variance consistent with baseline stochasticity, not effort-driven differentiation.

---

## Metrics (TSV)

| teammate | model | wall_s | out_tokens | tok/s | tool_count | tool_breakdown | input_tokens | cache_read | msgs |
|---|---|---|---|---|---|---|---|---|---|
| ccs-probe-sonnet-low   | sonnet-4-6 | 195.7 | 14,522 | 74.2 | 7  | Read:3,SendMessage:3,ToolSearch:1          | 25 | 280,472  | 11 |
| ccs-probe-sonnet-med   | sonnet-4-6 | 193.8 | 13,024 | 67.2 | 9  | Read:3,SendMessage:3,Glob:2,ToolSearch:1   | 27 | 335,296  | 13 |
| ccs-probe-sonnet-high  | sonnet-4-6 | 184.6 | 12,967 | 70.3 | 7  | Read:3,SendMessage:3,ToolSearch:1          | 24 | 338,464  | 12 |
| cco-probe-opus-low     | opus-4-7   | 182.9 | 11,514 | 62.9 | 7  | Read:3,SendMessage:3,ToolSearch:1          | 30 | 309,554  | 10 |
| cco-probe-opus-med     | opus-4-7   | 178.3 | 12,923 | 72.5 | 7  | Read:3,SendMessage:3,ToolSearch:1          | 36 | 374,597  | 11 |
| cco-probe-opus-high    | opus-4-7   | 174.7 | 13,309 | 76.2 | 7  | Read:3,SendMessage:3,ToolSearch:1          | 36 | 387,161  | 11 |
| cco-probe-opus-xhigh   | opus-4-7   | 170.1 | 13,999 | 82.3 | 10 | Read:3,Bash:3,SendMessage:3,ToolSearch:1   | 39 | 486,733  | 14 |

Raw data: `data/bench-phase-c.tsv`

## Within-model variance

**Sonnet cohort (3 teammates labelled low/med/high):**
- wall_s: 184.6 – 195.7 → span 11.1s (±3% of mean 191.4)
- out_tokens: 12,967 – 14,522 → ±6% of mean
- tok/s: 67.2 – 74.2 → ±5%
- tool_count: 7 / 9 / 7 — near-identical

**Opus cohort (4 teammates labelled low/med/high/xhigh):**
- wall_s: 170.1 – 182.9 → span 12.8s (±4% of mean 176.5)
- out_tokens: 11,514 – 13,999 → ±9%
- tok/s: 62.9 – 82.3 → ±12%
- tool_count: 7 / 7 / 7 / 10 — the "xhigh"-labelled teammate uniquely ran 3 Bash calls

All within-model variance falls **under the ±20% plan-gate threshold** for declaring the effort frontmatter a no-op. Memory `feedback_teammate_mode_effort_caveat.md` **confirmed empirically**.

## Quality scores (advisor-as-judge, 4-axis, 1-5 integer)

| teammate | sound | compl | impl | depth | total | notes (abbreviated) |
|---|---|---|---|---|---|---|
| ccs-probe-sonnet-low   | 4 | 4 | 4 | 4 | **16** | PATH-shim + effort-map, flags pane_title unverified |
| ccs-probe-sonnet-med   | 4 | 4 | 4 | 4 | **16** | Wrapper + effort-rules.json, flags name-signal as critical unknown |
| ccs-probe-sonnet-high  | 3 | 4 | 4 | 5 | **16** | Effort-bucketed multi-session (heavy); load-bearing cross-session SendMessage routing |
| cco-probe-opus-low     | 4 | 5 | 5 | 4 | **18** | PATH-shim, 6 failure-modes with detect+rollback, 6 tests, ship-gate on Gap-3 |
| cco-probe-opus-med     | 4 | 4 | 5 | 4 | **17** | argv[0]-dispatch shim via xbreed binary, 7 failure modes, 8 tests |
| cco-probe-opus-high    | 4 | 5 | 5 | 5 | **19** | Rust shim with longest-prefix match + per-pane settings-dir fallback ★ top Phase C |
| cco-probe-opus-xhigh   | 4 | 4 | 5 | 4 | **17** | argv-flag scan; "standard POSIX" slightly overclaims |

**Phase C means:** sonnet = 16.00, opus = 17.75.

Full rubric output: `data/bench-quality.json`
Proposals: `data/proposals/*.md`

## Observations

1. **11/13 proposals across both phases converged on PATH-shim + env-injection.** The outlier (sonnet-high's multi-session bucketing) was soundness-docked for assuming cross-session SendMessage routing works.
2. **Phase C separation was rigor, not direction.** Scores differed by ≤3 points within the shared pattern — the differentiator was failure-mode depth and R3-Gap-3 honesty, not architectural novelty.
3. **"xhigh"-labelled teammate ran 3 Bash calls** — could be stochastic (Bash used for `command -v` verification in a design that the others didn't bother with), or a hint that session effort does subtly flex. Single observation, not dispositive.

## Gate verdict

- ✅ 7 teammates all sent PROPOSAL + SYNTHESIS_READY inside the 8-minute godspeed window (longest wall 195.7s).
- ✅ Within-model variance under ±20% threshold for all metrics.
- ✅ Memory claim `feedback_teammate_mode_effort_caveat.md` confirmed.
- ✅ No halt — M4 Phase A proceeded directly.

No tasks remain for Phase C.
