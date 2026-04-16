# M7 Framing Audit — Heuer ACH on umwelt/metis/anti-forestry

**Agent:** `cco-critic-frame` (opus 4.7 high, Skill('heuer-planning') Layer 0 loaded)
**Brief under test:** `docs/reports/test-brief-2026-04-16.md` §Theoretical Framing (lines 37-66)
**Date:** 2026-04-16
**Round:** M7 adversarial axis of the xbgst handoff brief

---

## Layer 1 — Verbatim xask stdout

<raw_output>
Scale: `2` = strongly consistent, `1` = mixed/partial, `0` = inconsistent.

| Commitment | H_A | H_B | H_C | Diagnostic note |
|---|---:|---:|---:|---|
| Cross-model divergence mandate | 2 | 0 | 2 | Strong `umwelt` fit: divergence is explicitly preserved as signal, not noise in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:50) and [test-brief](/home/vhpnk/projects/xbrd-gdsp-fknpft/docs/reports/test-brief-2026-04-16.md:264). |
| Model-prefix naming | 0 | 2 | 1 | Best evidence against `H_A`: the repo itself demotes this to naming hygiene, not mechanism in [xbreed-superiority-r1](/home/vhpnk/projects/xbrd-gdsp-fknpft/docs/reports/xbreed-superiority-r1-2026-04-16.md:32), despite the convention in [AGENTS.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/AGENTS.md:29). |
| Axis→profile mapping | 2 | 0 | 2 | Strong load-bearing fit: the SSoT routes axes to model/profile pairs in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:54). |
| Raw-quote gate | 1 | 1 | 2 | Mixed: it fits `umwelt` preservation, but it is also well-explained as generic provenance control in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:38). |
| Per-`axis_family` evidence schema | 2 | 0 | 2 | Strong `metis` fit: local evidence shapes are explicitly non-uniform in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:137). |
| Filter-exempt allowlist keyed on `axis_family` | 2 | 0 | 2 | Strong `metis` fit: closed enum, anti-free-text memory in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:146). |
| Enforcement tier hierarchy | 2 | 0 | 2 | Strong `anti-scientific-forestry` fit: the repo explicitly distinguishes Build/CI, Runtime, Protocol in [xbreed-shared.md](/home/vhpnk/projects/xbrd-gdsp-fknpft/commands/references/xbreed-shared.md:82). |
| `include_str!` SSoT binding | 1 | 1 | 2 | Mixed: it supports the anti-flattening "enforce at the catching layer" story, but the exact mechanism is also just ordinary build-hardening in [src/protocol.rs](/home/vhpnk/projects/xbrd-gdsp-fknpft/src/protocol.rs:1). |

Totals: `H_A = 12`, `H_B = 4`, `H_C = 15`.

Verdict: `H_C` wins. The lenses are load-bearing for the routing/evidence/enforcement core, but not for every attached artifact. Load-bearing: cross-model divergence, axis→profile mapping, per-`axis_family` evidence schema, closed-enum exemption, enforcement tiers. Post-hoc/decorative: model-prefix naming. Partial/overdetermined: raw-quote gate and `include_str!` binding. So the defensible claim is exactly the one you gave: descriptive lenses for design commitments, not evidence of superiority.
</raw_output>

---

## ACH Consistency Matrix (brief spec format)

**Legend:** ✓ = consistent (codex score 2) · ? = partial/mixed (score 1) · × = inconsistent (score 0)

| # | Commitment | Lens owner | H_A load-bearing | H_B post-hoc | H_C partial | Rationale (one-line) |
|---|---|---|:---:|:---:|:---:|---|
| 1 | Cross-model divergence mandate (`xbreed-shared.md:51`, test-brief:264) | umwelt | ✓ | × | ✓ | Raw-quote preservation of dissent (NOT voting/averaging) is umwelt-specific; ensemble ML would collapse it. |
| 2 | Model-prefix naming (`AGENTS.md:29-40`) | umwelt | × | ✓ | ? | **R1 primary-source contradiction** (`xbreed-superiority-r1:32`) explicitly demotes to "naming hygiene, not mechanism." |
| 3 | Axis → Profile Mapping (`xbreed-shared.md:54-74`) | umwelt | ✓ | × | ✓ | Routing table pairs each axis to the model whose priors/tooling fit — umwelt-respecting by construction. |
| 4 | Raw-quote gate (`xbreed-shared.md:38`, Layer 2) | umwelt | ? | ? | ✓ | Overdetermined: umwelt preservation + generic provenance/anti-hallucination discipline both predict the gate. |
| 5 | Per-`axis_family` evidence schema (`xbreed-shared.md:129-146`) | metis | ✓ | × | ✓ | Executor vs labrat vs planning each declare distinct evidence shapes; one schema would erase local knowledge. |
| 6 | Filter-exempt allowlist keyed on `axis_family` (`xbreed-shared.md:146`) | metis | ✓ | × | ✓ | Closed enum keyed on institutional memory of which axes carry executable evidence — not free-text self-classification. |
| 7 | Enforcement Tiers (`xbreed-shared.md:82-98`) | anti-forestry | ✓ | × | ✓ | Build/CI > Runtime > Protocol hierarchy with documented bypass surface per tier; enforces at catching layer, not polite layer. |
| 8 | `include_str!` SSoT binding (`src/protocol.rs:4`) | anti-forestry | ? | ? | ✓ | Overdetermined: anti-forestry "build tier > protocol tier" + Rust-idiomatic compile-time binding both predict the choice. |

**Totals:** H_A = 5✓ + 2? + 1× · H_B = 0✓ + 2? + 6× · H_C = 6✓ + 1? + 1× (naming row). H_C dominates; H_B is actively refuted on 6 of 8 cells; H_A fails on model-prefix naming.

---

## Verdict: **H_C** (partial application)

The lenses earn their keep as descriptive vocabulary for a coherent cluster of design commitments, but the cluster is not uniform. Five commitments are **load-bearing** — removing the lens leaves the pattern unmotivated as a pattern (the individual pieces still stand on engineering merits, but the cluster-as-cluster loses its unifying description): cross-model divergence mandate, axis→profile mapping, per-`axis_family` evidence schema, filter-exempt closed-enum allowlist, enforcement tiers. One commitment is **decorative**: model-prefix naming, primary-source-rejected by R1 as "naming hygiene, not mechanism." Two commitments are **overdetermined**: raw-quote gate and `include_str!` SSoT binding — both stand equally on generic engineering justifications (provenance discipline, compile-time invariant enforcement) AND on lens fit.

**Rejected alternative (one, at hypothesis level):** Strict H_A was tempting on the enforcement-tier row because it scores 2/2 on lens-fit — but `xbreed-shared.md:96-98` primary-source-attributes the tier framework's origin to "R3 cco-critic-r3-overclaim (heuer Layer 0 loaded)," i.e. the tiers came from heuer ACH analysis of an overclaim, not from anti-scientific-forestry. Predictive fit survives (anti-forestry retrospectively predicts the same structure), but historical attribution is post-hoc — the honest verdict is H_C, not H_A.

**Non-obvious claim (one, at hypothesis level):** the brief's M7 instruction treats the lenses as a uniform armor — either all three earn their keep or none do. The ACH matrix refutes that framing: the lenses are not fungible, and the honest move is to keep the five load-bearing bullets, drop the decorative one, and annotate the two overdetermined ones with their engineering co-motivation. This is R1-style errata discipline applied to the framing section itself.

---

## Brief's Action (H_C path)

Per the brief's own M7 gate: H_A or H_C keeps the framing section with decoratives dropped. Execute:

1. **Remove** the Model-prefix naming bullet from the Umwelt block (`test-brief-2026-04-16.md:46`): `- **Model-prefix naming** (AGENTS.md Naming Convention: g- / cco- / cdx- / ccs-): the prefix is the umwelt label.` R1 already rejected this as mechanism; the framing section should honor the R1 errata rather than re-assert the rejected claim.
2. **Annotate** the raw-quote-gate bullet (`test-brief:48`) as "(overdetermined — umwelt preservation + generic provenance discipline)" for R1-style honesty about co-motivation.
3. **Annotate** the `include_str!` bullet (`test-brief:65`) as "(overdetermined — anti-forestry build-tier enforcement + Rust-idiomatic compile-time binding)" same rationale.
4. **Keep** the five load-bearing bullets (divergence mandate, axis→profile mapping, per-`axis_family` evidence schema, filter-exempt allowlist, enforcement tiers) as written.
5. **Add** a one-line note at the top of §Theoretical Framing: "M7 ACH verdict: H_C — 5 load-bearing, 2 overdetermined, 1 dropped as decorative. See `m7-framing-audit-2026-04-16.md`."

**Brief's title does NOT change.** H_B would have triggered wholesale section removal and a retitle to "structural"; H_C preserves the frame with the errata above.

---

## Provenance

- **xask Layer 1:** codex high effort ACH, verbatim pasted above (XASK_TIMEOUT_SECS=540 retry; default 60s too short for this payload).
- **Heuer Layer 0:** `Skill('heuer-planning')` loaded first; ACH framework applied to 8-commitment × 3-hypothesis matrix with key-assumptions-check and devil's-advocacy passes.
- **Primary-source-weighted contradictions:** R1 naming-hygiene demotion (`xbreed-superiority-r1:32`) and tier-framework heuer-origin attribution (`xbreed-shared.md:96-98`) weighted more heavily than codex's confirming scores per brief's epistemic rule.
- **Advisor consulted:** opus 4.7 max full-context review before publication; surfaced the 3-category partition (load-bearing / decorative / overdetermined) and the explicit annotation edits above.
