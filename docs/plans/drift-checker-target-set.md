# Goal
Generate a cheap mutation set for `scripts/verify-routing.sh` and `scripts/verify-install.sh` that probes the drift-checker branches with one angle per mutation.

# Artifact: markdown

## `scripts/verify-routing.sh`

1. **boundary**
   - HYPOTHESIS: obs/strong - lowering the SSoT floor by one will let an underfilled routing table slip past the guard at `PATTERN_COUNT < 11 || ROLE_COUNT < 9`.
   - METHOD: asm - mutate the floor check to `PATTERN_COUNT < 10 || ROLE_COUNT < 8`, then run the verifier against a fixture that is one pattern and one role short.
   - RESULT: inf/moderate - a weakened floor turns a real drift into a false OK unless another boundary check exists.

2. **operator-flip**
   - HYPOTHESIS: obs/strong - inverting the missing-pattern branch will convert `grep`-misses into passes.
   - METHOD: asm - flip `if ! grep -Fq "$pattern" "$file"` to a positive test, then run the script on a surface with one canonical removed.
   - RESULT: inf/moderate - the mutant should miss a missing-canonical drift and report OK.

3. **return-swap**
   - HYPOTHESIS: obs/strong - swapping the success and drift exits will make the verifier report the wrong terminal status.
   - METHOD: asm - exchange the final `exit 0` and `exit 1` paths, then run the clean repo baseline.
   - RESULT: inf/moderate - clean state becomes a false failure, and a real drift can become a false pass.

4. **error-path**
   - HYPOTHESIS: obs/strong - swallowing the missing-file path will hide absent surfaces instead of flagging drift.
   - METHOD: asm - remove `DRIFT=1` from the `[[ ! -f "$file" ]]` branch and keep the loop continuing.
   - RESULT: inf/moderate - a missing surface is skipped instead of surfaced as DRIFT.

5. **off-by-one**
   - HYPOTHESIS: obs/strong - dropping the last expected surface from the loop will leave one canonical unchecked.
   - METHOD: asm - mutate the expectation walk so the final entry in `EXPECTED_SURFACE_CHECKS` is never visited.
   - RESULT: inf/moderate - the last routing surface becomes invisible to the verifier and can drift silently.

## `scripts/verify-install.sh`

1. **boundary**
   - HYPOTHESIS: obs/strong - relaxing the empty-set checks will let missing install surfaces pass as valid.
   - METHOD: asm - weaken one of the `-eq 0` guards in `check_agents` or `check_dispatch` to accept the empty edge.
   - RESULT: inf/moderate - an empty local install surface becomes a false OK instead of a DRIFT.

2. **operator-flip**
   - HYPOTHESIS: obs/strong - flipping the resolved-path inequality will make the binary-path comparison accept the wrong target.
   - METHOD: asm - change `if [[ "$resolved_tool" != "$resolved_command" ]]` to `==`, then probe with PATH pointing at the wrong binary.
   - RESULT: inf/moderate - a mismatched `xbreed`/`xask` symlink resolves as if it were correct.

3. **return-swap**
   - HYPOTHESIS: obs/strong - swapping the terminal exits will invert the verifier's success/failure signaling.
   - METHOD: asm - make the `DRIFT != 0` branch exit 0 and the clean branch exit 1.
   - RESULT: inf/moderate - broken installs report success and clean installs fail.

4. **error-path**
   - HYPOTHESIS: obs/strong - suppressing `command -v` failure will hide a missing binary from the gate.
   - METHOD: asm - keep `command_path` empty on lookup failure but remove the `fail` call in the `-z "$command_path"` branch.
   - RESULT: inf/moderate - absent `xbreed`/`xask` binaries no longer surface as install drift.

5. **off-by-one**
   - HYPOTHESIS: obs/strong - dropping the last binary target from `for tool in xbreed xask` will miss one live install dependency.
   - METHOD: asm - iterate only the first binary in `check_binaries`, leaving the other unchecked.
   - RESULT: inf/moderate - one executable can disappear from `~/.local/bin` without the verifier noticing.
