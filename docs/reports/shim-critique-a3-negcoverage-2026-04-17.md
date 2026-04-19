# A3 Negative-Coverage Census — Adversarial Review of §4

**Author:** cco-critic-negcoverage (opus 4.7 high, heuer-planning loaded)
**Target:** `docs/reports/shim-test-plan-2026-04-17.md` §4 (4 rows)
**Axis:** A3 negative-coverage, direction ↑
**Evidence:** none — adversarial-design axis (non-executable). Claims grounded in primary-source read of `~/.bashrc:228-248` and codex xask verification on bash 5.2.21.

---

## CRITIQUE

**CRITIQUE:** §4's 4-row table treats failure-mode enumeration as done, but the installed trap has three structural vulnerabilities (no `set -T`, no `*)` default, space-only regex) that §4 ignores — actual coverage is ~30%.
**SEVERITY:** RETHINK
**CURRENT:** §4 covers (a) env-empty / non-interactive shell, (b) wrong-tier name mismatch, (c) outer-session sticky export, (d) sonnet doesn't honor effort.
**ALTERNATIVE:** Expand §4 to ≥12 rows covering shell-shape (non-interactive bypass, functrace, subshell scope), parser (equals-form, charset, pre-expansion), semantics (silent fallthrough, false-positive inert commands, DEBUG framework conflict, double-trigger race).
**TRADE-OFF:** Current approach ships faster but the passing gates (M2-1 `UNSET` / M2-2 `high`) can both be green while 8+ production spawn shapes silently inherit the wrong level.
**FAILURE-MODE:** Any teammate spawned via shell function, `$(...)`, background `&`, or with `$agent`-variable flags bypasses the trap — user sees "gates green, round-times unchanged" and concludes sonnet-doesn't-honor (row 4) when the real cause is scope loss.
**CONFIDENCE:** high

---

## ACH matrix — "§4 is adequate"

| Hypothesis | Consistent evidence | Inconsistent evidence | Diagnostic |
|---|---|---|---|
| H1: adequate | 4 rows cover common symptoms | trap source lacks `set -T`, has no `*)` default, regex is space-only, `case` on BASH_COMMAND is substring — all three are §4-invisible | FAILS diagnostic |
| H2: inadequate (shell-shape blind) | codex local-verified functrace & subshell scope loss on bash 5.2.21 against the installed trap | none | SURVIVES |

H2 dominates. §4 is a symptom table, not a failure-mode census.

---

## Missing failure modes (≥4 required; 10 delivered)

| FM# | Category | Observable fingerprint | Gate command (single-line bash) |
|---|---|---|---|
| FM-A | non-interactive `bash -c` / `sh` bypass — `~/.bashrc` never sourced, trap never installs | child `$-` lacks `i`; `trap -p DEBUG` empty | `env -i HOME="$HOME" bash -c 'printf "flags=%s trap=%s\n" "$-" "$(trap -p DEBUG \|\| echo NONE)"'` |
| FM-B | no `set -T` — DEBUG trap does NOT enter functions | spawn wrapped in `f(){ … }` leaves `CLAUDE_CODE_EFFORT_LEVEL=UNSET` despite match-worthy command | `unset CLAUDE_CODE_EFFORT_LEVEL; f(){ : --agent-name ccs-scribe-x --team-name t; }; f; printf '%s\n' "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-C | no `set -T` — trap lost inside `$(…)` command substitution | substitution returns right value internally but parent stays UNSET | `unset CLAUDE_CODE_EFFORT_LEVEL; out=$(: --agent-name ccs-scribe-x --team-name t; echo "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"); printf 'sub=%s parent=%s\n' "$out" "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-D | `BASH_SUBSHELL>0` — export inside `( … )` or `… &` does not cross back to parent env that spawns CC child | inner=high, parent=UNSET | `unset CLAUDE_CODE_EFFORT_LEVEL; ( : --agent-name ccs-scribe-x --team-name t; ); printf '%s\n' "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-E | silent unknown-name fallthrough — no `*)` default | known spawn sets `high`, subsequent unknown spawn inherits stale `high` | `unset CLAUDE_CODE_EFFORT_LEVEL; : --agent-name ccs-scribe-x --team-name t; a=$CLAUDE_CODE_EFFORT_LEVEL; : --agent-name foo-bar --team-name t; printf 'known=%s unknown=%s\n' "$a" "$CLAUDE_CODE_EFFORT_LEVEL"` |
| FM-F | equals-form flag `--agent-name=NAME` — regex requires `[[:space:]]+` separator, never matches | `level=UNSET` even with valid name | `unset CLAUDE_CODE_EFFORT_LEVEL; : --agent-name=ccs-scribe-x --team-name=t; printf '%s\n' "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-G | `BASH_COMMAND` is pre-expansion — `$agent $team` variables never match literal `--agent-name` substring | variable-built invocation produces UNSET | `agent='--agent-name ccs-scribe-x'; team='--team-name t'; unset CLAUDE_CODE_EFFORT_LEVEL; : $agent $team; printf '%s\n' "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-H | false-positive on inert command — `echo`/`printf` mentioning flags mutates env (substring-match on BASH_COMMAND) | grep/log/echo command containing `--agent-name X --team-name Y` as data unexpectedly exports | `unset CLAUDE_CODE_EFFORT_LEVEL; echo --agent-name ccs-scribe-x --team-name t >/dev/null; printf '%s\n' "${CLAUDE_CODE_EFFORT_LEVEL:-UNSET}"` |
| FM-I | agent-name charset truncation — regex `[a-zA-Z0-9_-]+` drops `.`, `/`, `:` and other separators future names may use | `BASH_REMATCH[1]` is a prefix, not the full name, silently routes wrong tier | `s=': --agent-name ccs.scribe-x --team-name t'; [[ $s =~ --agent-name[[:space:]]+([a-zA-Z0-9_-]+) ]] && printf '%s\n' "${BASH_REMATCH[1]}"` |
| FM-J | DEBUG-framework conflict — starship/powerline/bash-preexec also install DEBUG traps; last `trap` wins and clobbers our handler (or vice versa) | `trap -p DEBUG` names something other than `__xbreed_effort_trap`; `PROMPT_COMMAND` references `starship`/`bash-preexec` | `printf 'DEBUG=%s\n' "$(trap -p DEBUG)"; declare -p PROMPT_COMMAND 2>/dev/null \|\| true` |

---

## Devil's advocacy — what the author didn't model

1. **Mirror-imaging the production spawn shape.** M2-2 tests a literal top-level spawn. Real xbgst spawns fire through the tmux-send-keys path which may already be inside a bash function or subshell — exactly where FM-B/C/D live. §4's happy-path success is not evidence the gate works in prod.
2. **Silent fallthrough is weaponized by M2-1.** The "failing baseline" step uses `foo-bar-verify` expecting `UNSET`. But because there's no `*)` default, `UNSET` is achieved ONLY if no prior trap fire wrote a value — any later reshuffle of test order turns this gate into a coin-flip.
3. **Substring-match on `BASH_COMMAND` is promiscuous.** Every `grep --agent-name` in a debugging session, every `echo "spawn agent"` log line, every `set -x` dump becomes a false-positive trap fire. Over a multi-hour session, the last-write-wins sticky state is whatever flag-pattern string happened last, not the last spawn.
4. **Uninstall leaves env contamination.** §5's `sed -i` removes the trap but does not `unset CLAUDE_CODE_EFFORT_LEVEL` — the sticky exported value survives across the supposed-clean state.

---

## Proposed §4 replacement structure

Four-row symptom table → ≥12-row (symptom × cause × observable × gate) matrix. Plus three structural fixes to the trap itself:

1. Add `set -T` at top of `.bashrc` block (or document as prereq + add `trap -p DEBUG` pre-check that asserts it).
2. Add `*)` default: `unset CLAUDE_CODE_EFFORT_LEVEL` — silent fallthrough becomes loud fallthrough.
3. Widen regex to accept `=` separator and broader charset, OR narrow `case` guard to eliminate false-positives (gate on `BASH_COMMAND` starting with a known spawn tool like `tmux send-keys` or `claude `).

---

## Scope notes

- Code bugs (regex correctness, case-branch ordering) → reviewer, not critic. Flagged here only where they create failure-mode gaps §4 ignores.
- Security/injection (e.g., `BASH_REMATCH[1]` echoed unquoted) → sentinel. Not in this census.
- Mutation coverage (branch survival under test) → ccs-mutest-trapbranches (task #4).
