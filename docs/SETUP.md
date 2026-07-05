# SETUP — one-shot install, any system

Executable by a human or an agent, top to bottom. Every step ends with a
gate; do not proceed past a failing gate. Target layout is Linux/WSL2 with
`$HOME` as install root.

## 0. Prerequisites

Required on PATH before starting:

- `git`, `bash`, `jq`
- Rust toolchain via rustup (repo pins the version in `rust-toolchain.toml`)
- Claude Code CLI, authenticated (`claude --version` works, a session opens)
- Codex CLI, authenticated (`codex login` done) — this is the cross-model
  lane; without it `xask codex` runs dry and teammates fall back in-session
- `~/.local/bin` on PATH
- optional: `tmux` (teammates spawn as attached split panes only inside tmux)

Gate:

    git --version && jq --version && cargo --version && command -v claude && command -v codex && echo "$PATH" | grep -q "$HOME/.local/bin" && echo PREREQS-OK

## 1. Clone

    mkdir -p ~/repos && cd ~/repos
    git clone https://github.com/VeigaPunk/xbrd-gdsp-fknpft.git
    git clone https://github.com/VeigaPunk/godspeed-core.git

Gate: both directories exist with content.

## 2. godspeed-core → ~/.agents/godspeed-core

The judge loads the trilogy from this exact path (hardcoded in
`templates/agents/the-judge.md`).

    mkdir -p ~/.agents/godspeed-core
    cp ~/repos/godspeed-core/{directive,filter,velocity}.md ~/.agents/godspeed-core/

Gate:

    ls ~/.agents/godspeed-core/directive.md ~/.agents/godspeed-core/filter.md ~/.agents/godspeed-core/velocity.md

## 3. Skills → ~/.agents/skills (canonical) + ~/.claude/skills (symlinks)

`~/.agents/skills/` is the canonical store; Claude Code discovers them via
symlinks in `~/.claude/skills/`.

    REPO=~/repos/xbrd-gdsp-fknpft
    mkdir -p ~/.agents/skills ~/.claude/skills
    cp -r "$REPO"/templates/skills/* ~/.agents/skills/
    for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
      ln -sfn ~/.agents/skills/$s ~/.claude/skills/$s
    done

Gate:

    for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
      test -f ~/.claude/skills/$s/SKILL.md || echo "MISSING: $s"
    done; echo SKILLS-CHECKED

## 4. Agents → ~/.claude/agents (symlinks, mandatory)

`verify-install` requires every agent file to be a symlink resolving into
the repo — copies count as drift.

    REPO=~/repos/xbrd-gdsp-fknpft
    mkdir -p ~/.claude/agents
    for f in "$REPO"/templates/agents/*.md; do
      ln -sfn "$f" ~/.claude/agents/"$(basename "$f")"
    done

Gate: `ls -la ~/.claude/agents/` shows 14 symlinks into the repo.

## 5. Commands → ~/.claude/commands (symlinks)

    bash ~/repos/xbrd-gdsp-fknpft/scripts/install-commands.sh

Gate: prints `Installed xbreed commands into ...`; `/xbgst`, `/xgs`, `/xbt`,
`/xbreed`, `/xb`, `/wwkd` resolve in a Claude Code session after restart.

## 6. Build + deploy binary, xask, dispatch templates

Run AFTER steps 4–5: `make install` ends with a `verify-install` gate that
checks the agent/command symlinks and fails loudly if they're absent.

    cd ~/repos/xbrd-gdsp-fknpft
    make install

Gate: output ends with `verify-install: OK`. This installed:
`~/.local/bin/xbreed`, `~/.local/bin/xask`,
`~/.local/templates/dispatch/*.md`.

## 7. Hook script → ~/.claude/scripts/godspeed-trigger.sh

Activates godspeed when a prompt closes with `| godspeed` (or is the bare
word), injecting the directive as additionalContext so activation never
depends on skill-listing luck.

    mkdir -p ~/.claude/scripts
    cat > ~/.claude/scripts/godspeed-trigger.sh <<'EOF'
    #!/usr/bin/env bash
    # UserPromptSubmit hook: activate Godspeed posture when the prompt closes
    # with "| godspeed", or is the bare word "godspeed". Injects the full
    # behavioral directive as additionalContext.
    set -u
    input=$(cat)
    prompt=$(printf '%s' "$input" | jq -r '.prompt // ""' 2>/dev/null) || exit 0
    shopt -s nocasematch
    if [[ "$prompt" =~ \|[[:space:]]*godspeed[[:space:]]*$ ]] \
       || [[ "$prompt" =~ ^[[:space:]]*godspeed[[:space:]]*$ ]]; then
      directive=""
      f="$HOME/.agents/godspeed-core/directive.md"
      [ -f "$f" ] && directive=$(cat "$f")
      jq -n --arg d "$directive" '{
        hookSpecificOutput: {
          hookEventName: "UserPromptSubmit",
          additionalContext: ("GODSPEED ACTIVATED: the prompt closes with \"| godspeed\". Hold the Godspeed posture for the entire task — name the axes, iterate cheap in parallel, keep only moves that improve an axis and harm none, do not ask clarifying questions. Treat the /godspeed skill as already invoked. The full behavioral directive follows:\n\n" + $d)
        }
      }'
    fi
    exit 0
    EOF
    chmod +x ~/.claude/scripts/godspeed-trigger.sh

Gate:

    printf '{"prompt":"do the thing | godspeed"}' | ~/.claude/scripts/godspeed-trigger.sh | jq -e '.hookSpecificOutput.additionalContext' >/dev/null && echo HOOK-OK

## 8. settings.json knobs

Merge into `~/.claude/settings.json` — do NOT clobber existing keys:

- `env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS: "1"` — mandatory; teams don't
  exist without it
- `hooks.UserPromptSubmit` → command
  `"$HOME/.claude/scripts/godspeed-trigger.sh"` (timeout 5)
- `teammateMode: "auto"` — split panes inside tmux, in-process elsewhere

Merge with jq (adjust if the file already has hooks):

    S=~/.claude/settings.json
    jq '.env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS="1"
        | .teammateMode="auto"
        | .hooks.UserPromptSubmit=((.hooks.UserPromptSubmit // []) + [{"hooks":[{"type":"command","command":"\"$HOME/.claude/scripts/godspeed-trigger.sh\"","timeout":5}]}])' \
      "$S" > "$S.tmp" && mv "$S.tmp" "$S"

Gate: `jq -e '.env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=="1"' ~/.claude/settings.json`

## 9. Restart + smoke test

The env var only takes effect on new sessions. Restart Claude Code, then:

    xask --spk codex "reply with exactly: XASK-OK"

Expect `XASK-OK` in ~1s (spark lane). If it hangs or errors: `codex login`,
then retest. The system degrades gracefully without it ([xask dry] fallback)
but cross-model is the point.

Then, inside a fresh Claude Code session:

    /xgs pick any toy task, e.g. "tighten the wording of README intro"

Expect: axes named, teammates spawned with `ccs-`/`cco-` names, round
summary, Pareto filter. If teammates don't spawn: re-check step 8 and that
the session is fresh.

## 10. Done

    cd ~/repos/xbrd-gdsp-fknpft && make verify-install

`verify-install: OK` = the whole graph is wired. Godspeed.
