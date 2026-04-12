# Command Flow Reference

How each xbreed command works, from user input to final output.

## Overview

xbreed has two layers of commands:

| Layer | Commands | Runs as |
|-------|----------|---------|
| **Binary** (`xbreed`) | `guard`, `sync`, `claude`, `ask`, `team` | Rust CLI subprocess |
| **Skills** (inside Claude Code) | `/xbreed` (`/xb`), `/xbreed-team` (`/xbt`) | Prompt injection in active session |

The binary commands launch or configure CLI tools. The skills orchestrate
multi-agent workflows inside a running Claude Code session.

---

## Binary commands

### `xbreed guard <cli>`

Policy enforcement gate. Reads a tool-call JSON from stdin, checks it against
the deny-list policy, writes allow/deny to stdout.

```mermaid
flowchart LR
    A[stdin: tool-call JSON] --> B[load policy.yaml]
    B --> C{matches deny pattern?}
    C -->|yes| D[stdout: DENY + reason]
    C -->|no| E[stdout: ALLOW]
```

Used by Claude Code's `hooks` system — wired as a `PreToolUse` hook so every
tool call passes through the policy before execution.

---

### `xbreed sync`

Regenerates per-CLI config files from the shared policy.

```mermaid
flowchart LR
    A[policy.yaml] --> B[sync::write_claude_settings]
    B --> C[~/.config/xbreed/generated/settings.json]
```

---

### `xbreed claude [args]`

Launches Claude Code in max-power mode with model/effort from config.

```mermaid
flowchart TD
    A[xbreed claude] --> B[load policy.yaml]
    B --> C[sync: write settings.json]
    C --> D[load models.yaml]
    D --> E["claude --model opus \n--effort max \n--dangerously-skip-permissions \n--settings generated/settings.json \n[passthrough args]"]
    E --> F[Claude Code TUI session]
```

**Config sources:**
- `~/.config/xbreed/policy.yaml` — deny-list rules
- `~/.config/xbreed/models.yaml` — model + effort per CLI
- `~/.config/xbreed/generated/` — auto-generated settings

---

### `xbreed ask <cli> <prompt> [--with skills]`

Headless one-shot dispatch to any supported CLI.

```mermaid
flowchart TD
    A["xbreed ask <cli> <prompt> --with godspeed,librarian"]
    A --> B{--with provided?}
    B -->|yes| C[resolve loadout from skill dirs]
    B -->|no| D[empty loadout]
    C --> E{which CLI?}
    D --> E

    E -->|claude| F["claude -p <prompt> \n--append-system-prompt <loadout>"]
    E -->|codex| G["codex exec \n-c developer_instructions=<loadout> \n<prompt>"]
    E -->|gemini| H[Gemini auth cascade]

    H --> I{try auth chain in order}
    I --> J["1. OAuth profile: primary"]
    I --> K["2. OAuth profile: fallback"]
    I --> L["3. OAuth default ~/.gemini/"]
    I --> M["4. API key from .env.local"]
    I --> N["5. Fallback API key"]

    J & K & L & M & N --> O{success?}
    O -->|yes| P[stdout: response]
    O -->|429/401| Q[try next auth level]
    O -->|other error| R[bail immediately]
    Q --> I

    F --> P
    G --> P
```

**Loadout injection per CLI:**

| CLI | Mechanism | Flag |
|-----|-----------|------|
| claude | System prompt append | `--append-system-prompt` |
| codex | Developer instructions (TOML) | `-c developer_instructions=` |
| gemini | Prompt prepend (no native flag) | Loadout + `\n---\n` + prompt |

**Gemini auth cascade** (v0.3.5): tries up to 5 auth methods in order.
Cascades only on 429 (quota) or 401/403 (auth) errors. Non-retriable errors
bail immediately without wasting remaining auth levels.

---

### `xbreed team init [--with-beads]`

Scaffolds team infrastructure for a Claude Code agent team session.

```mermaid
flowchart TD
    A[xbreed team init] --> B[create team dirs]
    B --> C[configure beads-db merge-slot]
    C --> D[ready for TeamCreate in Claude Code]
```

### `xbreed team mailbox`

File-backed side-channel for fast teammate signals (bypasses SendMessage polling).

```mermaid
flowchart LR
    subgraph write
        T[teammate] -->|"mailbox write --from=X --kind=alive"| F[mailbox file]
    end
    subgraph drain
        F -->|"mailbox drain"| L[lead reads JSON array]
    end
    subgraph compact
        F -->|"mailbox compact --keep-types=error"| G[prune old events]
    end
```

---

## Skill commands (inside Claude Code)

These are not binaries — they're prompt-injected skills that run inside an
active Claude Code session. The user types `/xbreed` or `/xbt` and the skill
content is loaded into the conversation.

### `/xbreed <prompt>` (alias: `/xb`)

Solo judge pipeline. Single-turn, no persistent team.

```mermaid
flowchart TD
    A["/xbreed <prompt>"] --> B["Read ~/.claude/agents/the-judge.md"]
    B --> C[Adopt judge persona]
    C --> D{prompt contains 'godspeed'?}

    D -->|no| E[Judge directly]
    E --> F{need sub-roles?}
    F -->|yes| G["dispatch up to 3 Agent() calls \n(scout, reviewer, labrat)"]
    F -->|no| H[DRAFT output]
    G --> I[aggregate findings]
    I --> H

    D -->|yes| J[Godspeed Pareto walk]
    J --> K[Name 3-5 axes]
    K --> L["Round N: spawn <=4 Agent() \nwith inlined personas"]
    L --> M[Pareto filter: keep strict improvers]
    M --> N{frontier still moving?}
    N -->|yes, round < 4| L
    N -->|no or round = 4| O["DRAFT with AXES FINAL STATE"]
```

**Key difference from /xbt:** uses one-shot `Agent(subagent_type="general-purpose")`
with inlined personas. No persistent team, no teammate chat, no SendMessage
cross-critique. Everything happens within the judge's single turn.

**Dispatch rule:** prefers team-spawn path if already on a team. Falls back to
`general-purpose` with inlined persona body in solo mode (architectural quirk:
user-scope agent names only resolve inside team context).

---

### `/xbreed-team <prompt>` (alias: `/xbt`)

Judge-orchestrated persistent team. Multi-turn, real teammates.

```mermaid
flowchart TD
    A["/xbt <prompt>"] --> B["Read ~/.claude/agents/the-judge.md"]
    B --> C[Adopt judge persona]
    C --> D["TeamCreate(team_name=...)"]
    D --> E{prompt contains 'godspeed'?}

    E -->|no| F[Parse prompt, pick sub-roles]
    F --> G["Spawn 2-3 teammates \n(scout, reviewer, labrat) \nvia Agent() with team_name"]
    G --> H[Create TaskCreate per teammate]
    H --> I[Wait for SendMessage replies]
    I --> J[Aggregate into DRAFT]
    J --> K[Team stays alive]

    E -->|yes| L[Godspeed team walk]
    L --> M[Name 3-5 axes]
    M --> N["Assign axes to specialist profiles"]
    N --> O["Spawn <=4 teammates per round"]
    O --> P["Round N: each proposes ONE move \n(<=200 words)"]
    P --> Q[Cross-critique via DMs]
    Q --> R["Pareto filter (judge)"]
    R --> S{frontier still moving?}
    S -->|yes, round < 4| O
    S -->|no or round = 4| T["DRAFT with AXES FINAL STATE"]
    T --> K

    K --> U["Team persists \nUser can: \n- Shift+Down to cycle teammates \n- Chat with any teammate \n- Send follow-up to judge \n- 'clean up the team' to teardown"]
```

**Key differences from /xbreed:**

| | `/xbreed` (solo) | `/xbt` (team) |
|---|---|---|
| **Substrate** | One-shot `Agent()` calls | Persistent `TeamCreate` + teammates |
| **Communication** | Results return to judge only | Teammates DM each other directly |
| **User interaction** | Judge session only | Shift+Down into any teammate |
| **Persistence** | Single turn, then done | Lives until user says "clean up" |
| **Cross-critique** | Judge does it in-session | Teammates DM critiques to peers |
| **Godspeed rounds** | Agent() batches per round | Real teammate spawns per round |

---

## How it all connects

```mermaid
flowchart TD
    subgraph "Rust binary (xbreed)"
        guard["xbreed guard"]
        sync["xbreed sync"]
        claude_cmd["xbreed claude"]
        ask["xbreed ask"]
        team_cmd["xbreed team"]
    end

    subgraph "Claude Code session"
        xb["/xbreed (solo judge)"]
        xbt["/xbt (team judge)"]
        agents["~/.claude/agents/*.md \n(8 agent definitions)"]
        skills["templates/skills/ \n(4 skills)"]
    end

    subgraph "External CLIs"
        cc["claude CLI"]
        codex["codex CLI"]
        gemini["gemini CLI"]
    end

    claude_cmd -->|launches| cc
    ask -->|dispatches to| cc & codex & gemini
    guard -.->|PreToolUse hook| cc

    xb -->|"Agent() one-shot"| cc
    xbt -->|"TeamCreate + Agent()"| cc
    xb & xbt -->|reads personas| agents
    xb & xbt -->|loads| skills

    xb -->|"xbreed ask (delegation)"| ask
    xbt -->|"xbreed ask (delegation)"| ask
    team_cmd -->|"mailbox side-channel"| xbt
```

---

## Quick reference

| Command | What it does | Needs team? |
|---------|-------------|-------------|
| `xbreed guard` | Policy check on stdin JSON | No |
| `xbreed sync` | Regenerate CLI configs | No |
| `xbreed claude` | Launch Claude Code (max power) | No |
| `xbreed ask <cli>` | Headless one-shot to any CLI | No |
| `xbreed team init` | Scaffold team infra | Creates one |
| `xbreed team mailbox` | Fast teammate signal channel | Uses existing |
| `/xbreed` (`/xb`) | Solo judge pipeline | No (uses Agent()) |
| `/xbreed-team` (`/xbt`) | Persistent judge + team | Yes (TeamCreate) |
