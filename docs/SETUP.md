# SETUP — one-shot install, any system

Executable by a human or an agent, top to bottom. Every step ends with a
**gate**; do not proceed past a failing gate.

Target layout is Linux/WSL2 with `$HOME` as install root. This guide is
**hook-free**: it never writes Claude/Grok/Codex hooks, never installs
`UserPromptSubmit` scripts, and never patches lifecycle automation into
`settings.json` beyond the two team-required knobs below.

**Design rule:** skills, agents, and commands load by discovery (dirs +
symlinks). Godspeed activates because the skill is installed and you invoke
it (`godspeed`, `/godspeed`, or “apply godspeed”) — not because a hook
rewrites your prompt.

---

## 0. Prerequisites

Required on PATH before starting:

| Tool | Why |
|---|---|
| `git`, `bash`, `jq` | clone + gates |
| Rust via rustup | repo pins version in `rust-toolchain.toml` |
| Claude Code CLI, authenticated | primary harness (`claude --version` works) |
| Codex CLI, authenticated (`codex login`) | cross-model lane for `xask codex` |
| `~/.local/bin` on `PATH` | where `xbreed` / `xask` land |
| optional: `tmux` | teammates as split panes only inside tmux |
| optional: Grok Build CLI | if you also want skills under `~/.grok/` |

Gate:

```bash
git --version && jq --version && cargo --version \
  && command -v claude && command -v codex \
  && echo "$PATH" | grep -q "$HOME/.local/bin" \
  && echo PREREQS-OK
```

---

## 1. Clone

```bash
mkdir -p ~/repos && cd ~/repos
git clone https://github.com/VeigaPunk/xbrd-gdsp-fknpft.git
git clone https://github.com/VeigaPunk/godspeed-core.git
```

If you already have a checkout elsewhere, set `REPO` to that path for the
rest of this doc (examples below assume `~/repos/xbrd-gdsp-fknpft`).

```bash
export REPO="${REPO:-$HOME/repos/xbrd-gdsp-fknpft}"
export GSCORE="${GSCORE:-$HOME/repos/godspeed-core}"
```

Gate: both trees exist with content:

```bash
test -d "$REPO/.git" && test -d "$GSCORE/.git" && echo CLONE-OK
```

---

## 2. godspeed-core → `~/.agents/godspeed-core`

The judge loads the trilogy from this exact path (hardcoded in
`templates/agents/the-judge.md`).

```bash
mkdir -p ~/.agents/godspeed-core
cp "$GSCORE"/{directive,filter,velocity}.md ~/.agents/godspeed-core/
```

Gate:

```bash
ls ~/.agents/godspeed-core/directive.md \
   ~/.agents/godspeed-core/filter.md \
   ~/.agents/godspeed-core/velocity.md \
  && echo GSCORE-OK
```

---

## 3. Skills → `~/.agents/skills` (canonical) + harness symlinks

`~/.agents/skills/` is the canonical store. Harnesses discover via their own
skill roots (symlinks, not copies).

```bash
mkdir -p ~/.agents/skills ~/.claude/skills
cp -r "$REPO"/templates/skills/* ~/.agents/skills/
for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
  ln -sfn ~/.agents/skills/$s ~/.claude/skills/$s
done
```

**Optional — Grok Build** (same skills, Grok discovery path):

```bash
if command -v grok >/dev/null 2>&1; then
  mkdir -p ~/.grok/skills
  for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
    ln -sfn ~/.agents/skills/$s ~/.grok/skills/$s
  done
  echo GROK-SKILLS-LINKED
fi
```

Gate:

```bash
for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
  test -f ~/.claude/skills/$s/SKILL.md || echo "MISSING: $s"
done
echo SKILLS-CHECKED
```

---

## 4. Agents → `~/.claude/agents` (symlinks, mandatory)

`verify-install` requires every agent file to be a **symlink** resolving into
the repo — copies count as drift.

```bash
mkdir -p ~/.claude/agents
for f in "$REPO"/templates/agents/*.md; do
  ln -sfn "$f" ~/.claude/agents/"$(basename "$f")"
done
```

**Optional — Grok** (if your Grok build scans `~/.grok/agents/`):

```bash
if command -v grok >/dev/null 2>&1; then
  mkdir -p ~/.grok/agents
  for f in "$REPO"/templates/agents/*.md; do
    ln -sfn "$f" ~/.grok/agents/"$(basename "$f")"
  done
fi
```

Gate: 14 symlinks into the repo:

```bash
n=$(find ~/.claude/agents -type l | wc -l)
test "$n" -ge 14 && ls -la ~/.claude/agents/ | head \
  && echo "AGENTS-OK ($n symlinks)"
```

---

## 5. Commands → `~/.claude/commands` (symlinks)

```bash
bash "$REPO"/scripts/install-commands.sh
```

Gate: script prints install confirmation; after a Claude restart,
`/xbgst`, `/xgs`, `/xbt`, `/xbreed`, `/xb`, `/wwkd` resolve.

---

## 6. Build + deploy binary, xask, dispatch templates

Run **after** steps 4–5: `make install` ends with a `verify-install` gate that
checks agent/command symlinks and fails loudly if they're absent.

```bash
cd "$REPO"
make install
```

Gate: output ends with `verify-install: OK`. This installed:

- `~/.local/bin/xbreed`
- `~/.local/bin/xask`
- `~/.local/templates/dispatch/*.md`

---

## 7. Claude settings (teams only — no hooks)

Merge into `~/.claude/settings.json`. **Do not clobber** existing keys.
**Do not add** `hooks`, `UserPromptSubmit`, or any `~/.claude/scripts/*`
trigger.

Required knobs:

| Key | Value | Why |
|---|---|---|
| `env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` | `"1"` | teams do not exist without it |
| `teammateMode` | `"auto"` | split panes in tmux; in-process elsewhere |

Safe merge (creates the file if missing; never injects hooks):

```bash
S=~/.claude/settings.json
test -f "$S" || echo '{}' > "$S"
jq '.env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS = "1"
    | .teammateMode = "auto"
    | del(.hooks)   # strip any prior UserPromptSubmit / auto-hooks if present
   ' "$S" > "$S.tmp" && mv "$S.tmp" "$S"
```

If you prefer to keep unrelated hooks you installed yourself, drop the
`del(.hooks)` line and only set the two keys.

Gate:

```bash
jq -e '
  .env.CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS == "1"
  and .teammateMode == "auto"
  and ((.hooks // {}) | length) == 0
' ~/.claude/settings.json \
  && echo SETTINGS-OK-NO-HOOKS
```

If the last clause fails because you intentionally keep other hooks, re-run
the check without the hooks clause — still require the two team keys.

---

## 8. Restart + smoke test

Team env only takes effect on **new** sessions. Restart Claude Code, then:

```bash
xask --spk codex "reply with exactly: XASK-OK"
```

Expect `XASK-OK` in ~1s (spark lane). If it hangs or errors: `codex login`,
then retest. Without Codex the system degrades to in-session fallback; with
it, cross-model is the point.

Inside a fresh Claude session:

1. Invoke godspeed by skill (not by hook): say `godspeed: tighten README intro`
   or run `/godspeed` if your harness maps the skill.
2. Run orchestration: `/xgs pick any toy task, e.g. "tighten the wording of README intro"`

Expect: axes named, teammates with `ccs-` / `cco-` names, round summary,
Pareto filter. If teammates don't spawn: re-check step 7 and that the session
is fresh.

---

## 9. Final verify

```bash
cd "$REPO" && make verify-install
```

`verify-install: OK` = the whole graph is wired.

Optional inventory:

```bash
command -v xbreed && command -v xask
ls ~/.agents/godspeed-core/
ls -la ~/.claude/skills/godspeed ~/.claude/agents/the-judge.md
test ! -e ~/.claude/scripts/godspeed-trigger.sh && echo NO-GODSPEED-HOOK
jq -e '(.hooks // {}) | length == 0 or true' ~/.claude/settings.json
```

---

## 10. Done

You have:

- binaries on `PATH`
- godspeed trilogy under `~/.agents/godspeed-core`
- skills + agents + slash commands linked into Claude (and optionally Grok)
- team settings **without** prompt-submit hooks

Godspeed is opt-in via skill / slash / explicit prompt — never forced on every
submit. Improve one axis, harm none. Walk stops when nothing improves without
a tradeoff.
