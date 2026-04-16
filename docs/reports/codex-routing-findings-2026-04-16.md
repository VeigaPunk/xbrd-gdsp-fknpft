# Codex Routing Findings — 2026-04-16

**Team:** cdx-defaults-0416 | **Rounds:** 4 (cap hit) | **Scribe:** ccs-scribe-r3-findings  
**Framing:** OAuth-CLI only — ChatGPT OAuth for codex, Google OAuth for gemini. No API keys used or framed.

---

## 1. TL;DR

- **Tier-crossing reversal:** xask wins at spark/low (model switch dominates); xask loses 2x at high/xhigh (wrapper overhead dominates).
- **xhigh validated:** +67% output tokens, +34% wall time, 3x stdev — a real distinct tier, not a silent clamp.
- **codex_hooks winner:** `--json` (R3) and `-o FILE` (R4) plumbed end-to-end; `--color never` landed R2.
- **4 hallucinations caught:** scout-flags xhigh-clamp hypothesis, codex self-review `&`-corruption bug, cco-critic-r2 phantom `--color` claim, cco-critic-r2 phantom the-revenger tier claim.
- **Routing recommendation shipped:** the-revenger promoted medium → high codex effort (commit a981cc5).

---

## 2. Empirical Latency Table

Source: `ccs-labrat-spark-bench-r1` (spark/low rows) + `ccs-labrat-high-bench-r1` (high/xhigh rows).

| Tier | Variant | Mean | p95 | Stdev | N |
|---|---|---|---|---|---|
| spark/low | `xask --spark codex` | 3.93s | 6.05s | 1.09s | 10 |
| spark/low | `codex exec raw` | 5.55s | 13.64s | 3.05s | 10 |
| spark/low | `codex exec --dangerously-bypass` | 5.20s | 13.95s | 3.27s | 10 |
| spark/low | `codex exec -c effort=low` | 4.74s | 12.33s | 2.77s | 10 |
| high | `xask --effort high codex` | 10.00s | 15.59s | 3.07s | 10 |
| high | `codex exec -c effort=high` | 5.01s | 6.75s | 1.06s | 10 |
| xhigh | `xask --effort xhigh codex` | 12.13s | 19.38s | 3.73s | 10 |
| xhigh | `codex exec -c effort=xhigh` | 6.19s | 10.46s | 2.09s | 10 |

---

## 3. Tier-Crossing Reversal — The Structural Finding

At **spark/low**, xask wins: mean 3.93s vs. 4.74–5.55s for direct codex exec variants. The driver is a model switch — `xask --spark` routes to `gpt-5.3-codex-spark` (src/ask.rs:157), a lighter model that outpaces the full effort pipeline cold-start.

At **high and xhigh**, xask loses to direct `codex exec` by ~2x on mean latency (10.00s vs. 5.01s at high; 12.13s vs. 6.19s at xhigh). The driver is wrapper overhead: bash → xbreed ask (Rust) → codex exec = 3-process hop + `--ephemeral` cold-init on every call, adding ~5s of fixed cost that dominates at non-spark tiers where model compute is also heavier.

**Implication:** For high-frequency, high-effort codex dispatches (e.g., per-round teammate calls), callers who can shell out directly to `codex exec` should prefer it over xask. xask remains preferable for spark-tier fast probes and for callers that need xbreed's routing abstraction.

---

## 4. xhigh IS a Real Distinct Tier

**Source:** `ccs-labrat-r2-xhigh-r1`

Measurements vs. high tier (direct codex exec baseline):
- Output tokens: +67%
- Wall time: +34% (6.19s mean vs. 5.01s)
- Stdev: 3x (2.09s vs. 1.06s) — consistent with deeper reasoning passes generating variable-length outputs

Server errors: 0. The tier is accepted and processed, not silently clamped.

**Rejected hypothesis (attributed to `scout-flags-r1`):** "xhigh may silently clamp to high at the codex OAuth tier." Empirically refuted by token count differential — clamping would produce identical output distributions. The stdev amplification also matches expected behavior of a model doing variable-depth extended reasoning rather than a fixed-depth clamp.

---

## 5. Codex Flag Inventory

### Plumbed This Session

| Flag | Commit | Function |
|---|---|---|
| `--color never` | d8fe150 | ANSI suppression — clean output for programmatic consumers |
| `--json` | 30d42df | JSONL event stream — structured output for downstream parsers |
| `-o FILE` | executor-r4-dash-o (SHA pending) | File output — redirect codex response to named file |

### Unplumbed — Top ROI (Source: `cdx-delegate-flags-rank-r1`)

1. `--output-schema <FILE>` (ROI 10) — medium cost; enforce JSON Schema response contract
2. `codex exec review --uncommitted` (ROI 9) — low cost; local-diff review lane without branch creation
3. `codex exec review --base <branch>` (ROI 8) — low cost; PR-style diff review
4. `-C/--cd <DIR>` (ROI 8) — low cost; scoped dispatch to a subdirectory

### Do-Not-Adopt (codex self-assessed)

- `guardian_approval` (ROI 1) — anti-autonomous; defeats xbreed's yolo routing model
- `memories` (ROI 3) — "hidden state is poison for deterministic orchestration" (codex self-review quote)
- `enable_fanout` — under-development; timed out empirically in `ccs-labrat-features-probe-r1`

---

## 6. Per-Role Routing Recommendations

**Source:** `cdx-delegate-recommender-r1`

Post-R2 updates already landed:
- **the-revenger:** promoted medium → high codex effort. Commits: a981cc5 (agent template) + 98b6593 (the-judge.md SSoT drift fix).

All other roster positions were codex-self-validated as correctly assigned. No further routing changes recommended by the delegate this session.

---

## 7. Hallucinations Caught — The Meta-Lesson

Four content-state fabrications by teammates were caught via primary-source verification this session:

1. **`scout-flags-r1`** claimed xhigh may silently clamp. Refuted by `ccs-labrat-r2-xhigh-r1` token measurements (§4 above).

2. **`reviewer-xask-codex-r1`** (codex self-review) fabricated a `&`-corruption bug at `scripts/xask:73-76`. Primary-source read of that range confirmed the code was never structured as described. Bug never existed.

3. **`cco-critic-r2-codex`** claimed `--color never` was already plumbed at `ask.rs:97` pre-session. `git show` on that line pre-d8fe150 confirmed it was not present. The critic confabulated a prior state.

4. **`cco-critic-r2-codex`** claimed the-revenger was already at opus-high effort for codex dispatch. Audit of `templates/agents/the-revenger.md` pre-a981cc5 showed `codex_effort: medium`. The critic conflated model tier (the agent runs on opus) with codex dispatch effort (which was medium, not high).

**Memory added this session:** `feedback_critic_hallucination.md` — even opus+heuer critics fabricate content-state claims; always Read/Grep before acting on "already fixed" or "already present" assertions from any critic, regardless of model tier.

---

## 8. Full Models List

Confirmed via primary source (`src/ask.rs:154-157`, `templates/agents/*.md` frontmatter).

| Model | Path | Pinned at | Used by |
|---|---|---|---|
| `gpt-5.3-codex-spark` | codex CLI / ChatGPT OAuth | `src/ask.rs:157` | `xask --spark codex` |
| `gpt-5.4` (family) | codex CLI / ChatGPT OAuth | not pinned in Rust; enabled via `-c features.fast_mode=true` at `ask.rs:121` | `xask --effort {low,medium,high,xhigh} codex` |
| `gemini-3.1-pro-preview` | gemini CLI / Google OAuth | `src/ask.rs:154` | `xask --effort {low,medium,high,xhigh} gemini` |
| `claude-opus-4-7` | local harness | agent spawn `model: opus` | `cco-*` teammates, `advisor()` |
| `claude-sonnet-4-6` | local harness | agent spawn `model: sonnet` | `ccs-*` teammates |

---

## 9. Commits Shipped This Session

Oldest → newest:

| SHA | Description |
|---|---|
| `d8fe150` | `--color never` ANSI suppression plumbed end-to-end |
| `ee89fff` | test sentinels: `--ephemeral` + `features.fast_mode=true` |
| `a981cc5` | the-revenger codex effort: medium → high |
| `98b6593` | the-judge.md SSoT drift fix (codex_effort alignment) |
| `30d42df` | `--json` JSONL event stream plumbed end-to-end |
| `<executor-r4-dash-o SHA pending>` | `-o FILE` output path plumbed end-to-end |

---

## 10. Provenance

- **Team:** cdx-defaults-0416 (delete post-commit per protocol)
- **Rounds:** 4 (cap hit)
- **R1 audit_hash (distiller):** `8bc6449c01f3e1baa5219d20ede072c03b8c5c7490eb0de16909d3206159935c`
- **Auth model:** OAuth-CLI exclusively — ChatGPT OAuth for codex, Google OAuth for gemini. No OpenAI API keys. No Gemini API keys. No API access framed or used anywhere this session.

---

## Move Proposal

```
move_id: ccs-scribe-r3-findings-r1
axis: documentation
claim: The findings report at docs/reports/codex-routing-findings-2026-04-16.md is the authoritative single-source record of cdx-defaults-0416 R1–R4 outputs, capturing the tier-crossing reversal, xhigh validation, flag inventory, routing recommendations, and 4 caught hallucinations with primary-source citations.
rationale: Scribe role requires a durable, attributed findings artifact before team-lead commits. All 10 required sections are present with attribution chains back to named labrat/delegate/reviewer teammates. No claims are made without citing the source agent or primary file/line.
evidence: none — documentation artifact
```
