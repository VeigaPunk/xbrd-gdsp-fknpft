# M01 — xask Routing Consolidation Verification
**Status:** COMPLETE | **Date:** 2026-04-16 | **Session:** 1

## Does
Verifies that the three routing changes in commit `82008cf` — `| godspeed` forwarding through xask, `--spark` routing through `xbreed ask`, and always-on suppression flags — are all live and correct.

## Gate
```bash
cargo test codex_ask 2>&1 | grep -E 'test result|ok'
```
Expected: all 3 codex_ask tests pass, exit 0

Actual:
```
test ask::tests::codex_ask_empty_loadout_has_suppression_flags ... ok
test ask::tests::codex_ask_spark_adds_model_and_low_effort ... ok
test ask::tests::codex_ask_with_loadout_uses_developer_instructions_override ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 54 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

## Touches
- `scripts/xask` — eliminated raw codex exec paths; added `| godspeed` forwarding (inline append to template last line, lines 72-74)
- `src/ask.rs` — always-on suppression flags unconditional at lines 108-113; spark model const + param at lines 115-118
- `src/cli.rs` — added `--spark` flag to Ask subcommand
- `src/main.rs` — wired spark through dispatch
- `tests/ask_with_loadout.rs` — updated codex integration tests for suppression flags

## Out-of-scope
- Gemini suppression equivalent (no native `-c` flags available on gemini path; informational, not a gap)
- `xask --debug` flag echo improvement (DX, deferred)
- Spark effort override (by design: spark = always low effort, not configurable)

## Findings
- `scripts/xask:72-74` — `| godspeed` is appended **inline** to the template last line (`...# Unknowns. | godspeed`), not as a separate line; labrats missed it because they expected a newline-delimited append. Both delivery vectors are active: SKILL.md injection + prompt text trigger.
- `src/ask.rs:106` — labrats conflated xask-layer routing (shell script) with Rust-layer invocation (`build_codex_ask_with_loadout` internally calls `codex exec`); these are distinct architectural layers. The script eliminates raw `codex exec` calls; the Rust layer still emits `codex exec` as the terminal invocation by design.
- `ccs-labrat-spark` probe session ID: `019d96ab-870e-70e1-92b7-b9eb8f02a5da` — immutable trace, recorded for audit trail.
- **FALSE POSITIVE flag** (raised by ccs-labrat-godspeed): `GODSPEED_RECEIVED` from codex probe is self-contaminating — probe text contained the godspeed trigger, so the response cannot distinguish genuine forwarding confirmation from echo of the probe payload. Distinction: `| godspeed` pipe append in debug output is the structural confirmation; `GODSPEED_RECEIVED` text in the response is not independently probative.
- **ADDITIVE posture gap** (raised by ccs-labrat-spark): `--spark` path suppression is structurally verifiable (CLI flags `-c include_permissions_instructions=false` etc. hardcoded unconditionally at ask.rs:108-113). Default `xbreed ask` path suppression relies on the template's `# Adapter posture: ADDITIVE` label being semantically interpreted by the receiving model — this is a semantic claim, not payload-observable. **Open gap: posture enforcement on non-spark path is unverified at the structural level.**
- **Mechanism distinction** (raised by ccs-labrat-gemini): report distinguishes "godspeed text in prompt" (skill injection via SKILL.md) from "`| godspeed` pipe append" (inline template suffix in scripts/xask); debug output confirms the pipe append path; skill injection is inferred from SKILL: godspeed routing confirmation in `--debug` output.

## Links
- Plan: commit `82008cf` on branch `main`
- Next: M02 (if Round 2 warranted by judge)
