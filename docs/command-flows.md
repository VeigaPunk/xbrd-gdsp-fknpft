# Command Flow Reference

How each xbreed command works, from user input to final output.

## Overview

xbreed has two layers of commands:

| Layer | Commands | Runs as |
|-------|----------|---------|
| **Binary** (`xbreed`) | `guard`, `sync`, `claude`, `ask`, `team` | Rust CLI subprocess |
| **Skills** (inside Claude Code) | `/xbreed`, `/xbt`, `/xgs`, `/xbgst` | Prompt injection in active session |

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

    H --> I["1. OAuth profile: primary"]
    I --> Ia{success?}
    Ia -->|yes| P[stdout: response]
    Ia -->|"429/401/403/PERMISSION_DENIED"| J["2. OAuth profile: fallback"]
    Ia -->|other error| R[bail immediately]
    J --> Ja{success?}
    Ja -->|yes| P
    Ja -->|"429/401/403/PERMISSION_DENIED"| K["3. OAuth default ~/.gemini/"]
    Ja -->|other error| R
    K --> Ka{success?}
    Ka -->|yes| P
    Ka -->|"429/401/403/PERMISSION_DENIED"| L["4. API key from .env.local"]
    Ka -->|other error| R
    L --> La{success?}
    La -->|yes| P
    La -->|"429/401/403/PERMISSION_DENIED"| M["5. Fallback API key"]
    La -->|other error| R
    M --> Ma{success?}
    Ma -->|yes| P
    Ma -->|no| R

    F --> P
    G --> P
```

**Loadout injection per CLI:**

| CLI | Mechanism | Flag |
|-----|-----------|------|
| claude | System prompt append | `--append-system-prompt` |
| codex | Developer instructions (TOML) | `-c developer_instructions=` |
| gemini | Prompt prepend (no native flag) | Loadout + `\n---\n` + prompt |

**Gemini auth cascade** (v0.3.5): tries up to 5 auth methods **sequentially**
(not in parallel). Each attempt blocks on `cmd.output()` before the next starts.
Cascades on: 429 (quota), 401, 403, PERMISSION_DENIED, UNAUTHENTICATED,
API_KEY_INVALID. Non-retriable errors bail immediately per-attempt without
trying remaining auth levels. Empirical timing: OAuth ~14s, API key ~5-7s.

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

Solo judge pipeline with cross-model delegation. Single-turn, no persistent team.

```mermaid
flowchart TD
    A["/xbreed <prompt>"] --> B["Read ~/.claude/agents/the-judge.md"]
    B --> C[Adopt judge persona]
    C --> E{need sub-roles?}
    E -->|yes| F["dispatch up to 3 Agent() calls \n(scout→xask gemini, reviewer→xask codex, labrat→xask --spark codex)"]
    E -->|no| G[DRAFT output]
    F --> H["xask gate: first tool call = Bash xask \nraw-quote gate: <raw_output> tags \nepistemic role: at most 1 non-obvious claim"]
    H --> I[aggregate findings]
    I --> J{"cross-model conflict?"}
    J -->|yes| K["populate CONFLICTS block \n(model labels)"]
    J -->|no| G
    K --> G
```

**Key difference from /xbt:** uses one-shot `Agent(subagent_type="general-purpose")`
with inlined personas. No persistent team, no teammate chat, no SendMessage
cross-critique. Everything happens within the judge's single turn.

**xask gate:** every sub-role brief requires `xask gemini`/`xask codex` as the
first tool call. Raw-quote gate requires verbatim CLI output in `<raw_output>` tags.

**Dispatch rule:** prefers team-spawn path if already on a team. Falls back to
`general-purpose` with inlined persona body in solo mode.

For godspeed Pareto mode, use `/xgs` (all-Claude) or `/xbgst` (cross-model).

---

### `/xbreed-team <prompt>` (alias: `/xbt`)

Judge-orchestrated deliberative team with cross-model delegation. Multi-turn, real teammates.

```mermaid
flowchart TD
    A["/xbt <prompt>"] --> B["Read ~/.claude/agents/the-judge.md"]
    B --> C[Adopt judge persona]
    C --> D["TeamCreate(team_name=...)"]
    D --> F[Parse prompt, pick sub-roles]
    F --> G["Spawn 2-3 teammates \n(scout→xask gemini, reviewer→xask codex, labrat→xask --spark codex)"]
    G --> Gx["xask gate: first tool = Bash xask \nraw-quote gate: <raw_output> tags \nepistemic role: at most 1 non-obvious claim"]
    Gx --> H[Create TaskCreate per teammate]
    H --> I[Wait for SendMessage replies]
    I --> J{"Judge mediates: \nchallenge findings? \ncross-model conflict?"}
    J -->|"challenge"| Jc["SendMessage follow-up \nto specific teammate"]
    Jc --> Jd[Teammate refines + re-reports]
    Jd --> J
    J -->|"conflict"| Jf["Populate CONFLICTS block \n(model: gemini vs model: codex)"]
    Jf --> K
    J -->|"satisfied"| K[Aggregate into DRAFT]
    K --> L["Team stays alive \n(soft ceiling: 5 deliberative rounds)"]

    L --> U["Team persists \nUser can: \n- Shift+Down to cycle teammates \n- Chat with any teammate \n- Send follow-up to judge \n- 'clean up the team' to teardown"]
```

For godspeed Pareto mode, use `/xgs` (all-Claude) or `/xbgst` (cross-model).

**Key differences across commands:**

| | `/xbreed` | `/xbt` | `/xgs` | `/xbgst` |
|---|---|---|---|---|
| **Substrate** | One-shot Agent() | Persistent team | Persistent team | Persistent team |
| **Cross-model (xask)** | Yes | Yes | No (all-Claude) | Yes |
| **Iteration** | Single turn | Deliberative (5 cap) | Pareto walk (4 rounds) | Pareto walk (4 rounds) |
| **Cross-critique** | In-session | Teammate DMs | Teammate DMs | Teammate DMs |
| **Speed** | Fast | Slow, pondered | Fast | Medium |

---

### `/xgs <prompt>` — Godspeed Pareto (all-Claude)

Fast team mode. No cross-model delegation. Teammates use CC native tools.

```mermaid
flowchart TD
    A["/xgs <prompt>"] --> B["Read ~/.claude/agents/the-judge.md"]
    B --> C[Adopt judge persona]
    C --> D["TeamCreate(team_name=...)"]
    D --> E["Phase 0: Name 3-5 axes \n(direction + observable)"]
    E --> F["Phase 1: Assign deterministic \nteammate names per axis"]
    F --> G["Phase 2: Spawn all teammates \nwith full peer roster \n(no xask gate — all-Claude)"]
    G --> H["Phase 3: Round N \neach proposes ONE move (<=200 words)"]
    H --> I[Cross-critique via peer DMs]
    I --> J["Pareto filter (judge): \naccept strict improvers \nreject regressions"]
    J --> K{frontier still moving?}
    K -->|"yes, round < 4"| H
    K -->|"no or round = 4"| L["DRAFT with AXES FINAL STATE \n+ CONFLICTS (teammate labels)"]
    L --> M[Team stays alive]
```

**4-phase spawn protocol:** axes must be named before teammate names are assigned,
and all names must be committed before any spawn. This prevents the peer-roster
ordering bug where early teammates lack peer names for cross-critique DMs.

---

### `/xbgst <prompt>` — Godspeed Pareto + Cross-Model Delegation

The full crossbreed. Godspeed Pareto walk with xask cross-model delegation.

```mermaid
sequenceDiagram
    participant U as User
    participant J as Judge (Opus 4.7 max)
    participant T as Teammates (N)
    participant X as xask CLI
    participant G as Gemini CLI
    participant C as Codex CLI

    U->>J: /xbgst <prompt>
    Note right of J: Phase 0: Name axes

    J->>J: Phase 1: Assign teammate names
    Note right of J: All names committed before spawn

    par Phase 2: Spawn all teammates
        J->>T: scout brief (axis + xask gemini gate)
        J->>T: reviewer brief (axis + xask codex gate)
        J->>T: labrat brief (axis + xask --spark codex gate)
        J->>T: connector brief (axis + xask gemini gate)
    end

    par Phase 3a: Cross-model delegation (~14s gemini, ~6s codex)
        T->>X: xask gemini '<question>'
        X->>G: gemini -m gemini-3.1-pro-preview -p '<prompt>'
        Note right of G: Auth cascade: OAuth first → API key fallback
        G-->>X: response
        X-->>T: stdout (for <raw_output> tags)
        T->>X: xask codex '<question>'
        X->>C: codex exec '<prompt>'
        C-->>X: response
        X-->>T: stdout (for <raw_output> tags)
    end

    Note right of T: Epistemic role: at most 1 non-obvious claim

    par Phase 3b: Cross-critique DMs
        T->>T: scout DMs reviewer (one-line critique)
        T->>T: reviewer DMs scout (one-line critique)
        T->>T: labrat DMs all peers
    end

    T->>J: SendMessage: proposals + <raw_output> + CONFLICT flags

    J->>J: Pareto filter: accept strict improvers

    alt Highest-divergence unchallenged claim
        J->>X: Falsification probe (one xask, opposing model, ~7s)
        X-->>J: result
    end

    alt Frontier still moving (round < 4)
        J->>T: Round N+1 dispatch (current frontier as baseline)
    else Frontier reached or round = 4
        J->>U: DRAFT + AXES FINAL STATE + CONFLICTS (model labels)
    end
```

**Timing annotations** (from empirical labrat probes, 2026-04-12):

| Phase | Wall time | Bottleneck |
|-------|-----------|------------|
| Teammate spawn (4x parallel) | ~3s | CC agent initialization |
| xask gemini (per call) | ~14s | Gemini CLI + OAuth cascade |
| xask codex (per call) | ~6s | Codex exec |
| xbreed ask gemini --with godspeed | ~13s | Loadout resolution + dispatch |
| Cross-critique DMs | ~2-5s | Turn-boundary polling |
| Pareto filter (judge) | ~1-3s | In-session, no I/O |
| Falsification probe (optional) | ~7s | Single targeted xask |

**CONFLICTS block** uses model labels (not teammate labels):
```
CONFLICTS:
  - claim: <contested fact>
    model: gemini (via <teammate>) — <position>
    model: codex (via <teammate>) — <position>
    judge_resolution: <chosen + rationale>
```

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
        xb["/xbreed (solo + xask)"]
        xbt["/xbt (deliberative + xask)"]
        xgs["/xgs (godspeed, all-Claude)"]
        xbgst["/xbgst (godspeed + xask)"]
        agents["~/.claude/agents/*.md \n(8 agent definitions)"]
        skills["templates/skills/ \n(6 skills)"]
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
    xgs -->|"TeamCreate + Agent()"| cc
    xbgst -->|"TeamCreate + Agent()"| cc
    xb & xbt & xgs & xbgst -->|reads personas| agents
    xb & xbt & xgs & xbgst -->|loads| skills

    xb -->|"xask (cross-model)"| ask
    xbt -->|"xask (cross-model)"| ask
    xbgst -->|"xask (cross-model)"| ask
    xgs -.->|"no xask (all-Claude)"| cc
    team_cmd -->|"mailbox side-channel"| xbt & xbgst
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
| `/xbreed` (`/xb`) | Solo judge + xask delegation | No (uses Agent()) |
| `/xbreed-team` (`/xbt`) | Deliberative team + xask | Yes (TeamCreate) |
| `/xgs` | Godspeed Pareto, all-Claude | Yes (TeamCreate) |
| `/xbgst` | Godspeed Pareto + xask | Yes (TeamCreate) |
