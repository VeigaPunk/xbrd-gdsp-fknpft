# Teammate Benchmark — Phase A (godspeed model benchmark)

**Mission:** `/xbgst /wwkd | godspeed` — teammate benchmark M4
**Team:** `bench-model-0417` (6 teammates: 3× sonnet + 3× opus, session xhigh)
**Date:** 2026-04-17
**Plan:** `docs/plans/teammate-benchmark-plan-2026-04-17.md`
**Gate:** PASS — 6 TSV rows, ≥3-replica statistical power on model axis.

---

## Design

Same hard task as Phase C (per-teammate effort propagation design). 3× replication per model gives variance estimates on an apples-to-apples comparison (all at session `effortLevel: xhigh`).

---

## Metrics (TSV)

| teammate | model | wall_s | out_tokens | tok/s | tool_count | tool_breakdown | input_tokens | cache_read | msgs |
|---|---|---|---|---|---|---|---|---|---|
| ccs-bench-sonnet-a | sonnet-4-6 | 201.1 | 18,578 | 92.4  | 7  | Read:3,SendMessage:3,ToolSearch:1           | 24      | 340,580 | 12 |
| ccs-bench-sonnet-b | sonnet-4-6 | 196.1 | **24,987** | **127.4** | 7  | Read:3,SendMessage:3,ToolSearch:1           | 28      | 392,058 | 14 |
| ccs-bench-sonnet-c | sonnet-4-6 | 191.9 | 12,867 | 67.1  | 7  | Read:3,SendMessage:3,ToolSearch:1           | 25      | 282,169 | 11 |
| cco-bench-opus-a   | opus-4-7   | 188.3 | 10,182 | 54.1  | 7  | Read:3,SendMessage:3,ToolSearch:1           | 31      | 413,576 | 11 |
| cco-bench-opus-b   | opus-4-7   | 186.2 | 12,341 | 66.3  | 7  | Read:3,SendMessage:3,ToolSearch:1           | 31      | 414,605 | 11 |
| cco-bench-opus-c   | opus-4-7   | 178.3 | 11,339 | 63.6  | 10 | Read:4,SendMessage:3,Bash:2,ToolSearch:1    | **14,645** | 483,894 | 14 |

Raw data: `data/bench-phase-a.tsv`

## Per-model aggregates

| | sonnet (3) | opus (3) | delta (sonnet / opus) |
|---|---|---|---|
| wall_s mean | 196.4 | 184.3 | sonnet +6.6% |
| out_tokens mean | **18,811** | **11,287** | sonnet +67% (produces more) |
| tok/s mean | 95.6 | 61.3 | sonnet +56% (writes faster) |
| tok/s stddev | 30.2 | 6.2 | sonnet **5× wider variance** |
| quality mean (M5) | 14.67 | 16.00 | opus +9% |
| quality stddev | 1.53 | 3.00 | opus wider on quality (floor 13, ceiling 19) |

## Quality scores (advisor-as-judge, 4-axis)

| teammate | sound | compl | impl | depth | total | notes (abbreviated) |
|---|---|---|---|---|---|---|
| ccs-bench-sonnet-a | 4 | 4 | 4 | 4 | **16** | Wrapper + pane-title lookup; F1 flagged LINCHPIN; mandatory pre-impl probe |
| ccs-bench-sonnet-b | 2 | 3 | 4 | 4 | **13** | `xbreed team-spawn` bypasses TeamCreate — architectural-break risk |
| ccs-bench-sonnet-c | 3 | 4 | 5 | 3 | **15** | Wrapper + --teammate-id argv + prefix fallback; limited alternatives |
| cco-bench-opus-a   | 4 | 5 | 5 | 5 | **19** | Rust shim with 3-tier name-resolution cascade; 7 failure-modes; 7 tests ★ top Phase A |
| cco-bench-opus-b   | 4 | 4 | 4 | 4 | **16** | Bash shim with cascade; explicit ship-blocker gate; concise |
| cco-bench-opus-c   | 3 | 3 | 4 | 3 | **13** | PATH shim per-team via settings.env; pane_title asserted without equal rigor |

**Phase A means:** sonnet = 14.67, opus = 16.00.

## Observations

1. **Sonnet produces ~67% more output tokens on the same task.** More verbose, more elaboration. Mean 18,811 vs 11,287.
2. **Sonnet writes ~56% faster (tok/s).** Makes sense — less careful, more streaming.
3. **Sonnet quality floor is lower (13) AND higher architectural-break risk.** `ccs-bench-sonnet-b` proposed bypassing CC's TeamCreate entirely, relying on undocumented mailbox internals — dropped to soundness 2 for the unverified architectural assumption.
4. **Opus is tighter: 5× lower tok/s variance, narrower wall-time range, smaller token count.** Also produces higher-quality output on average (16.00 vs 14.67 mean).
5. **Outlier: cco-bench-opus-c** — 14,645 input tokens (~15× peers). Read-all-context pattern or exploration spiral. Did NOT improve quality — scored 13 (bottom). Classic "more tokens ≠ better" signal.
6. **Tool breakdowns are nearly identical** (7 calls: Read×3 + SendMessage×3 + ToolSearch×1) except two opus teammates added Bash. Low signal from tool-breakdown alone.

## Cost-per-quality-point (pseudo-metric)

| model | mean out_tokens | mean quality | tokens / quality-point |
|---|---|---|---|
| sonnet | 18,811 | 14.67 | 1,283 |
| opus   | 11,287 | 16.00 | 706   |

**Opus delivers ~45% fewer tokens per quality point** on this task. If token-cost dominates, opus is strictly better. If latency dominates, the picture is narrower — opus is ~6% faster wall-time but produces less.

## Gate verdict

- ✅ 6 teammates all sent PROPOSAL + SYNTHESIS_READY inside 8-minute godspeed window.
- ✅ 6 TSV rows + 6 quality scores.
- ✅ Model-axis comparison clear across all 6 metrics.

No tasks remain for Phase A.
