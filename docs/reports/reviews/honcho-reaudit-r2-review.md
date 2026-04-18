# R2 Synth Review — honcho-reaudit-0418

**Reviewer:** cdx-reviewer-diffs-r2  
**Date:** 2026-04-18  
**Mission:** honcho-reaudit-0418 Round 2 corrected distiller synthesis  
**Audit hash:** sha256:9e5adca4b35a2106601344da2174be61e09171aca71da7d8f471f147cbecfe0d

---

## Verdict: accept-with-corrections

The Round 2 correction is substantively sound. R0 confirmed, R1-NL-prose correctly deferred (not killed), R5 durably killed, K1 correctly reclassified as wrapper-bug misclassification. Two protocol hygiene corrections required before closure: audit_hash reproducibility and 1898ms provenance labeling.

---

## Audit Hash Verification

**Status: attestation gap — not independently reproducible**

No published SOURCE_MAP or exact `move_id:source_prefix` serialization is present in the retained artifacts (team-lead.json or ccs-distiller-r1.json). Standard hash derivations against visible move/source lines do not reproduce `9e5adca4...`. This is a procedural gap — no evidence of content fraud — but the hash claim cannot be independently verified. Remediation: either publish the exact serialization method and input corpus, or downgrade the claim to "content-fingerprint (not independently reproducible from retained artifacts)."

---

## Spot-Checks: 3 Primary-Source Evidence Quotes

### SC-1: schemas/api.py:485 — session_id native on ConclusionCreate

**Claim:** `session_id` is a native field on `ConclusionCreate`, not a wrapper injection.

**Verified:**
```python
# honcho/src/schemas/api.py:485
session_id: str | None = Field(
    default=None,
    description="A session ID to store the conclusion in, if specified",
)
```
**Result: CONFIRMED.** Field is native. K1's "incapable of accepting session_id" was a wrapper-bug misclassification.

---

### SC-2: crud/document.py:770+782 — session_name persistence

**Claim:** `session_id` is persisted as `session_name` in both embedding and non-embedding document paths.

**Verified:**
```python
# crud/document.py:770
session_name=obs.session_id,   # embedding path
# crud/document.py:782
session_name=obs.session_id,   # non-embedding path
```
**Result: CONFIRMED.** Both construction paths persist the field.

---

### SC-3: utils/filter.py:50-56 + document.py:270+303 — filter mapping and apply_filter paths

**Claim:** `session_id -> session_name` mapping exists in filter utility; `apply_filter` is called on both vector and ID-based query paths.

**Verified:**
```python
# utils/filter.py:50-56
ALLOWED_EXTERNAL_TO_INTERNAL_COLUMN_MAPPING_DOCUMENTS = {
    "session_id": "session_name",
    ...
}
# document.py:270
stmt = apply_filter(stmt, models.Document, filters)   # ID-based path
# document.py:303
stmt = apply_filter(stmt, models.Document, filters)   # pgvector path
```
**Result: CONFIRMED.** Mapping and both apply_filter call sites verified.

---

## SPOOF Flags

### SF-1: "1898ms median latency" — UNVERIFIED PROVENANCE [HIGH]

**Claim:** A1 attributes "1898ms median latency" to R1 labrat measurement.

**Finding:** The number appears in `docs/reports/honcho-reaudit-phase0-2026-04-18.md` (lines ~83 and ~109) as Phase-0 data. It does **not** appear in any R1 proposer evidence block. Presenting this as "R1 labrat confirmed" is a misattribution. The corrected synthesis already flags this as unresolved — that handling is correct.

**Required correction:** Keep `1898ms` labeled as "inherited Phase-0 / unverified-for-R1-R2" or omit from closure prose entirely. Do not promote to "R1 confirmed."

### SF-2: No additional SPOOF candidates identified

All other load-bearing evidence quotes in m13-m21 either trace to verified primary sources (see SC-1 through SC-3 above) or are explicitly flagged as unresolved in the corrected synthesis.

---

## K1 Reclassification Verification

**A6 claim (overclaim):** K1 is a permanent capability gap — "Honcho API fundamentally cannot accept session_id."

**A3/A1 reclassification:** K1 was a Python SDK wrapper bug; the REST API natively accepts `session_id` via `ConclusionCreate.session_id` (api.py:485).

**Verdict: A3/A1 framing is CORRECT.** Primary source at api.py:485 is unambiguous — the field exists natively on the schema. A6's "permanent gap" framing fails the primary-source test. K1b (the live R1 constraint, whatever it is) remains valid; K1 itself was misclassified.

---

## A2 Scope-Stretch Assessment

**Actions taken:** modified `scripts/xbreed-memory`, wrote `docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md`, created `tests/xbreed_memory_audit.sh`.

**Assessment: acceptable tooling, not a scope violation.**

All three artifacts are small, backward-compatible, and directly support the "operationalize triggers" axis. `scripts/xbreed-memory` and the audit shell test are infrastructure tooling. The triggers doc is an archival shape-for-record artifact. This is scope expansion from a reviewer lane (non-standard), but it does not distort findings or create frontier-measurement artifacts that could inflate subsequent round scores. Acceptable with annotation that reviewer lane should not routinely write operational tooling.

---

## Exit-at-R2 Decision Review

**Decision under review:** team declared exit after Round 2.

**Challenge assessment: exit is justified.**

The three remaining open items are all protocol hygiene (audit_hash reproducibility, 1898ms labeling, K1b scope definition) — none would produce a new axis improvement that changes the Pareto frontier. The primary substantive moves (R0 confirmation, R1-NL-prose deferral, R5 kill, K1 reclassification) are settled by primary-source evidence. A Round 3 would not find new capability gaps; it would iterate on documentation quality. Exit-at-R2 is the correct call.

---

## Category Leakage Assessment

**Status: CLEAN**

R1-style m8/m10 archival leakage (archival shape-for-record artifacts presented as accepted verdicts) is not repeated in the corrected R2 synthesis. A2 scope-stretch artifacts are correctly categorized as tooling/archival. No verdict-category contamination found.

---

## Summary

| Check | Result |
|---|---|
| Audit hash reproducibility | FAIL — attestation gap only, no content fraud |
| SC-1: api.py:485 session_id | CONFIRMED |
| SC-2: document.py:770+782 persistence | CONFIRMED |
| SC-3: filter.py + apply_filter paths | CONFIRMED |
| 1898ms SPOOF flag | UNVERIFIED — Phase-0 inherited, not R1 labrat |
| K1 reclassification (A3/A1 vs A6) | A3/A1 CORRECT per primary source |
| A2 scope stretch | ACCEPTABLE — tooling, not distortion |
| Exit-at-R2 | JUSTIFIED — no frontier-moving work remains |
| Category leakage | CLEAN |
| **Overall verdict** | **accept-with-corrections** |

**Required corrections before closure:**
1. Downgrade audit_hash claim to "content-fingerprint (not independently reproducible)" OR publish SOURCE_MAP
2. Label `1898ms` as "inherited Phase-0 / unverified-for-R1-R2" everywhere in closure prose

---

## Raw xask Output

```
**Findings**
1. **[HIGH] `audit_hash` `9e5adca4...` is still not independently reproducible from the retained artifact.** The corrected synthesis is present in team-lead.json, but there is still no published `SOURCE_MAP` or exact `move_id:source_prefix` serialization in either team-lead.json or ccs-distiller-r1.json. I tried the obvious derivations from the visible move/source lines; none hashed to `9e5adca4...`. This is an attestation gap, not evidence of content fraud.

2. **[MEDIUM] The `1898ms` claim remains misattributed unless explicitly labeled as inherited Phase-0 evidence.** The number does exist in honcho-reaudit-phase0-2026-04-18.md (lines ~83 and ~109), but I did not find it in any R1 proposer evidence block. The corrected synthesis already treats this as unresolved. That handling is correct; the number should not be presented as "R1 labrat confirmed."

**Checks**
- `m21` primary-source correction is correct. api.py:485 shows `session_id` is native on `ConclusionCreate`; document.py:769 and document.py:781 persist it as `session_name`; filter.py:50 maps `session_id -> session_name`; document.py:270 and document.py:303 apply filters on both paths. So A3/A1's reclassification beats A6's "permanent" framing.
- K1 permanence conflict is resolved correctly in the corrected synthesis: K1 was a wrapper bug misclassification, not a capability gap. K1b remains the live R1 constraint.
- Category leakage looks materially cleaned up in the corrected R2 synthesis. I do not see the R1-style m8/m10 archival leakage repeated as a verdict error.
- A2 scope stretch is acceptable tooling, not a substantive scope violation. scripts/xbreed-memory, tests/xbreed_memory_audit.sh, and r1-c-axis-reopen-triggers-2026-04-18.md are small, backward-compatible, and directly support "operationalize triggers." It is scope expansion from a reviewer lane, but not harmful or frontier-distorting.

**Verdict**
`accept-with-corrections`

Substantively, the Round 2 correction holds: `R0` is still right for this round, `R1-NL-prose` is deferred rather than killed, and `R5` remains killed on the durable constraints. I would not run Round 3 for frontier movement; the only remaining work is protocol hygiene:

- publish the actual `SOURCE_MAP` or downgrade the hash claim to "not independently reproducible"
- keep `1898ms` marked as inherited/unverified-for-R1-R2, or omit it entirely from closure prose
```
