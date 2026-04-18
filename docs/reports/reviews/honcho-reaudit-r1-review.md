# honcho-reaudit-0418 R1 — Synth-Review

**Reviewer:** cdx-reviewer-diffs-r1
**Date:** 2026-04-18
**Synthesis reviewed:** distiller R1 synthesis — 12 moves m1-m12, 7 axes (R-axis cut), audit_hash sha256:c8b474d114ed65203ca75ec3dad63ef79b0a1a8e8c35c66880b25bba2ee64dc1

---

## Verdict: accept-with-corrections

---

## Audit_hash verification

**Reproduced:** NO

Neither this reviewer nor the codex second-opinion could reproduce the hash from the retained artifact. The synthesis does not include a published `source_prefix` serialization or `SOURCE_MAP` reveal, so the sorted pipe-delimited move-order string cannot be reconstructed independently.

**Assessment:** Protocol gap, not content fraud. The synthesis is not fabricated; the hash cannot be independently verified because the SOURCE_MAP was not attached. Correction required: publish SOURCE_MAP alongside all future synth artifacts.

---

## Spot-checks

Three load-bearing evidence quotes checked against primary source:

| Claim | File:line | Verified? |
|---|---|---|
| `session_id` maps to `session_name` as a first-class filter key | `filter.py:50-56` | YES — `ALLOWED_EXTERNAL_TO_INTERNAL_COLUMN_MAPPING_DOCUMENTS = {"session_id": "session_name", ...}` confirmed |
| `internal_metadata={}` hardcoded for user-created conclusions | `document.py:781` | YES — `internal_metadata={},  # No message_ids since not derived from messages` confirmed at line 781 |
| `ConclusionCreate` has no `metadata` field | `schemas/api.py:479-509` | YES — fields are `content`, `observer_id`, `observed_id`, `session_id` only; no metadata field |

All three primary-source quotes are accurate.

---

## F-axis kill framing — BLOCKER-CLASS CORRECTION

**Synthesis claim:** session_id NULL on all 13 production records is a write-path gap; fixing it = red-flag (a) client-side capability patch.

**Primary source finding:** `schemas/api.py:485` shows `session_id: str | None = Field(default=None, ...)` — session_id IS a native, optional field on `ConclusionCreate`. `filter.py:50-56` confirms it is a first-class server-side filter key mapped internally to `session_name`.

**Correction:** The 13 NULL records are a bug in the now-removed `scripts/honcho` wrapper (it never populated `session_id` on write). Fixing this means populating an existing server field our own code failed to set — that is NOT red-flag (a) "client-side capability patch." Red-flag (a) applies when the server lacks the capability and we compensate client-side; here the server already has it.

**Impact on verdict:** If the F-axis move classifies this as red-flag (a), that classification is wrong. The "three independent kills on R1/R5" reduce to two if F-axis is reframed as "required fix in any new implementation, not gate disqualifier." This is material — two kills vs. three affects whether R0 is Pareto-dominant or whether R1 remains a gated candidate.

**Recommended correction:** Re-classify F-axis finding as "write-path gap requiring fix (session_id is native, populate it in new wrapper)" and score R1/R5 accordingly through the remaining two kills.

---

## T-axis kill — scope reconciliation required

**Synthesis claim:** ConclusionCreate has no metadata field → only `content.icontains()` path → trips red-flag (b) schema smuggling.

**Phase 0 §5 gate matrix:** R1 (NL sidecar) is listed as passing gate item (b) with "content=prose, session_id=mission native fields." This means R1's intended shape — NL prose only, no structured filtering — already passed Phase 0's own gate analysis.

**Contradiction:** If the T-axis kill applies to R1's NL-prose shape, it contradicts the Phase 0 §5 gate-pass. The T-axis kill is valid only for the more complex filtered-metadata use case (axis_id/round/direction filtering) that R1 as Phase-0-defined does not require.

**Recommended correction:** Scope the T-axis kill explicitly: it kills "Schema-v2-style structured filtering via content string" (which DOES trip red-flag (b)), but does NOT kill R1's NL-prose-only shape. The synthesis should specify which role-shape the T-axis kill applies to, rather than applying it globally to R1/R5.

---

## Spoof flags

**None on load-bearing kills.** All three primary-source quotes are accurately cited.

`m9` is inference-backed (X-axis xask blockage disclosed by synthesis itself) — acknowledged confidence caveat, not fabrication.

`m8` wording overreach: phrases like "confirms mission-scoped recall" are stale given session_id:null on all live records. This is archival leakage, not fabrication.

---

## Category leakage

**Flagged — m8 and m10:**

- `m8` reads as an accepted current-state verdict. It is archival method inventory. Its claim that `/conclusions/list` "confirms" mission-scoped recall overstates reality because live data shows session_id:null on all 13 records — scoping only works once the write-path is fixed. Should be demoted to shape-for-record / future-surface.
- `m10` includes conditional evidence ("if gate were lifted") and integration sketches. That belongs under archival/shape-for-record, not under accepted survivors.

Both should be relabeled. The synthesis's "accepted" section overstates their current standing.

---

## R-axis-cut impact on synthesis quality

The R-axis (Heuer Layer-0 ACH across candidate roles) was the systematic competing-hypotheses scoring surface. Its absence means:

- No formal ACH matrix survives in the synthesis
- Role-ranking confidence is reduced
- The binary R0 verdict may be correct, but lacks adversarial hypothesis testing grounding it

With three kills originally claimed (now two after F-axis reframing), the R0 verdict is defensible but not airtight. R1's NL-prose shape has one clean kill (X-axis feedback loop) and one disputed T-axis kill (scope-limited). A properly scoped ACH matrix might score R1 differently.

**Assessment:** R-axis cut does not invalidate the synthesis, but it does lower confidence on the role-ranking conclusion. The verdict should be marked as "R0 preferred under current evidence" rather than "R1/R5 definitively killed."

---

## Corrections required (priority order)

1. **[BLOCKER]** Reclassify F-axis finding: session_id is native to ConclusionCreate; write-path NULL records = old wrapper bug, not server limitation; fixing it is NOT red-flag (a). Re-score R1/R5 gate status accordingly.
2. **[HIGH]** Publish SOURCE_MAP for audit_hash reproducibility.
3. **[MEDIUM]** Scope T-axis kill explicitly to Schema-v2-style structured metadata filtering; confirm whether it applies to R1's NL-prose-only shape given Phase 0 §5 gate-pass.
4. **[MEDIUM]** Demote m8 and m10 from accepted-verdict section to archival/shape-for-record.
5. **[LOW]** Qualify R0 verdict as "preferred under evidence — role-ranking confidence reduced by R-axis cut."

---

## Raw xask output (verbatim)

```
**Findings**
- High: `audit_hash` is not independently reproducible from the retained artifact. The saved synthesis includes `sources:` but not the required sorted `move_id:source_prefix` serialization or a `SOURCE_MAP` reveal. I tested the obvious derivations and none hash to `sha256:c8b474...`. That is a protocol/attestation gap, not proof the content is false.
- Medium: `m8` is category leakage. It reads like an accepted current-state verdict, but it is really archival method inventory. Its line that `/conclusions/list` "confirms" mission-scoped recall overstates reality once `m1` is accounted for: live data still shows `session_id:null` on stored conclusions, so this is only a potential future surface if the write path were fixed.
- Medium: `m10` is also category leakage. Its own evidence is conditional, "if gate were lifted," and preserves integration sketches. That belongs under shape-for-record / future revisit, not under accepted survivors.

**Spot-checks**
- [filter.py](/home/vhpnk/projects/xbrd-gdsp-fknpft/honcho/src/utils/filter.py:50) verifies the DSL maps `session_id -> session_name` and `metadata -> internal_metadata`.
- [document.py](/home/vhpnk/projects/xbrd-gdsp-fknpft/honcho/src/crud/document.py:781) verifies `internal_metadata={}` is hardcoded for user-created conclusions.
- [api.py](/home/vhpnk/projects/xbrd-gdsp-fknpft/honcho/src/schemas/api.py:479) verifies `ConclusionCreate` exposes only `content`, `observer_id`, `observed_id`, and optional `session_id`; no metadata field exists.

**Live check**
- I re-ran the `m1` probe. `conclusions/query` with `session_id:"honcho-stress-0418"` returned `[]`; the same query without `session_id` returned records; `conclusions/list` on workspace `xbreed-judge` returned 13 total records and every one had `session_id: null`. So the load-bearing F-axis kill is real.

**Spoof risk**
- No clear fabrication on the load-bearing kills.
- `m9` is inference-backed, not primary-source-backed; the synthesis itself discloses the X-axis `xask` blockage, so this is a confidence caveat, not a spoof flag.
- `m8` has wording overreach, but it looks like stale/archival leakage, not fabrication.

**R-axis**
- Correctly absent. I did not reconstruct it. Its removal lowers confidence on fine-grained role ranking, but not on the binary outcome `R0` and the no-integration-this-round call.

**Verdict**
- `accept-with-corrections`

Corrections needed: publish the actual `source_prefix` serialization or `SOURCE_MAP` for the hash, and move `m8`/`m10` out of accepted verdicts into archival or shape-for-record.
```

---

# State

- obs: F-axis kill mis-classified as red-flag (a) — session_id is native to ConclusionCreate (api.py:485); write-path NULL = old wrapper bug, not server gap — severity: **blocker** [certain via primary source]
- obs: audit_hash not reproducible — SOURCE_MAP not published — severity: high [certain]
- obs: m8/m10 category leakage — archival/conditional content in accepted-verdict section — severity: medium [certain]
- risk: T-axis kill scope may be overapplied to R1's NL-prose shape, contradicting Phase 0 §5 gate-pass — verify against verbatim move text [moderate]
- risk: "three independent kills" reduces to two after F-axis reclassification — R0 vs R1 verdict less airtight than synthesis implies [moderate]

# Artifact: review

scope: distiller R1 synthesis, 12 moves m1-m12, 7 axes (R-axis cut by judge), audit_hash sha256:c8b474d114ed65203ca75ec3dad63ef79b0a1a8e8c35c66880b25bba2ee64dc1
verdict: **accept-with-corrections**
