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
    D --> E["claude --model opus \n--effort high \n--dangerously-skip-permissions \n--settings generated/settings.json \n[passthrough args]"]
    E --> F[Claude Code TUI session]
```

**Per-teammate effort override:** `~/.bashrc` installs a `__xbreed_effort_trap`
DEBUG trap that inspects `$BASH_COMMAND` for `--agent-name` + `--team-name` on
every spawn and exports `CLAUDE_CODE_EFFORT_LEVEL` based on role-keyword
matching. This env var **takes precedence over agent frontmatter `effort:`**
per Claude Code's model-config precedence rules (see shared.md §Session Effort
Configuration) — the trap is the authoritative per-teammate effort control.

**Config sources:**
- `~/.config/xbreed/policy.yaml` — deny-list rules
- `~/.config/xbreed/models.yaml` — model + effort per CLI
- `~/.config/xbreed/generated/` — auto-generated settings

---

### `xbreed ask <cli> <prompt> [--with skills] [--review|-R] [--spark]`

Headless one-shot dispatch to any supported CLI. The `--review` / `-R` and
`--spark` flags select the codex dispatch lane (see `src/cli.rs`
`Commands::Ask`).

```mermaid
flowchart TD
    A["xbreed ask <cli> <prompt> --with godspeed,librarian [-R|--spark]"]
    A --> B{--with provided?}
    B -->|yes| C[resolve loadout from skill dirs]
    B -->|no| D[empty loadout]
    C --> E{which CLI?}
    D --> E

    E -->|claude| F["claude -p <prompt> \n--append-system-prompt <loadout>"]
    E -->|codex| Cdx{codex lane}
    E -->|gemini| H["gemini -m gemini-3.1-pro-preview \n-p <prompt> --approval-mode yolo \n(env_remove GEMINI_API_KEY; reads ~/.gemini/oauth_creds.json)"]

    Cdx -->|--spark| Csp["codex exec -m gpt-5.3-codex-spark \n-c model_reasoning_effort=low \n(no fast_mode)"]
    Cdx -->|"-R / --review"| Crv["codex exec -m gpt-5.4-mini \n-c features.fast_mode=true \n(-R -F escape hatch → full gpt-5.4)"]
    Cdx -->|default| Cdf["codex exec -m gpt-5.4-mini \n-c features.fast_mode=true \n-c model_reasoning_effort=high"]

    H --> Ha{success?}
    Ha -->|yes| P[stdout: response]
    Ha -->|auth error| Hae["bail: run `gemini login` \nto refresh oauth_creds.json"]
    Ha -->|other error| R[bail immediately]

    F --> P
    Csp --> P
    Crv --> P
    Cdf --> P
```

**Loadout injection per CLI:**

| CLI | Mechanism | Flag |
|-----|-----------|------|
| claude | System prompt append | `--append-system-prompt` |
| codex | Developer instructions (TOML) | `-c developer_instructions=` |
| gemini | Prompt prepend (no native flag) | Loadout + `\n---\n` + prompt |

**Codex dispatch lanes** (see `src/ask.rs` `build_codex_ask_with_loadout`):

| Flag | Model | Reasoning | fast_mode | Used by |
|------|-------|-----------|-----------|---------|
| `--spark` | `gpt-5.3-codex-spark` | low | off | labrat, xask-gate probes |
| `-R` / `--review` | `gpt-5.4` (full) | xhigh (inherited) | on | reviewer, critic, sentinel, the-revenger |
| default | `gpt-5.4-mini` | high | on | executor, scout-fallback, labrat-non-spark |

**Gemini auth cascade** (v0.4+, OAuth-exclusive): tries up to **3 OAuth levels**
**sequentially** (not in parallel). Each attempt blocks on `cmd.output()` before
the next starts. Cascades on: 429 (quota), 401, 403, PERMISSION_DENIED,
UNAUTHENTICATED, API_KEY_INVALID. Non-retriable errors bail immediately
per-attempt without trying remaining auth levels. The `GeminiAuth::ApiKey`
variant and all `.env.local` / `GEMINI_API_KEY` / `GEMINI_API_KEY_FALLBACK`
parsing were removed in v0.4 — `env_remove("GEMINI_API_KEY")` is applied on
every OAuth attempt to strip any inherited shell env. Empirical timing: OAuth
~14s per attempt.

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
    E -->|yes| F["dispatch up to 3 Agent() calls \n(scout→xask gemini, reviewer→xask -R codex, labrat→xask --spark codex)"]
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
    F --> G["Spawn 2-3 teammates \n(scout→xask gemini, reviewer→xask -R codex, labrat→xask --spark codex)"]
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

**Mandatory connector on every Pareto round** (landed 2026-04-17): the-judge
MUST spawn a `connector` teammate in Round 1 AND every subsequent round — not
optional. Cross-axis pattern matching is structural; focused specialists miss
whole-table regressions. Reference: `shared.md` §Mandatory connector on every
round, `~/.claude/agents/the-judge.md`.

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
        J->>T: reviewer brief (axis + xask -R codex gate)
        J->>T: labrat brief (axis + xask --spark codex gate)
        J->>T: connector brief (axis + xask gemini gate) [MANDATORY every round]
    end

    par Phase 3a: Cross-model delegation (~14s gemini, ~6s codex)
        T->>X: xask gemini '<question>'
        X->>G: gemini -m gemini-3.1-pro-preview -p '<prompt>'
        Note right of G: Auth cascade: OAuth-only (3 levels)
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

**Mandatory connector on every round** (same rule as `/xgs`): the-judge spawns
a connector teammate every Pareto round, Round 1 through terminal. Reference:
`shared.md`, `the-judge.md`.

**Timing annotations** (from empirical labrat probes, 2026-04-12; default-lane
codex calls post-2026-04-17 use `gpt-5.4-mini`, so the `~6s codex` figure
reflects the mini path — `-R` review-lane calls against full `gpt-5.4` may be
slower):

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
        agents["~/.claude/agents/*.md \n(14 xbreed-managed definitions; \nexcludes the-musketeer + the-puppeteer, \nwhich are user-invoked, not xbreed-orchestrated)"]
        skills["commands/*.md (in-repo slash commands) \n+ ~/.claude/skills/ + ~/.agents/skills/ \n(user-managed; templates/ removed 2026-04-17)"]
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

## Model selection

Three independent layers decide which model + effort a spawned teammate
actually runs. Later layers override earlier ones.

| Layer | Source | Controls | Precedence |
|---|---|---|---|
| Frontmatter | `~/.claude/agents/<name>.md` | `model:` (opus/sonnet/full ID) + `effort:` default | lowest |
| DEBUG trap | `~/.bashrc` `__xbreed_effort_trap` | `CLAUDE_CODE_EFFORT_LEVEL` env var via role-keyword match on `--agent-name` | overrides frontmatter `effort:` |
| `CLAUDE_CODE_SUBAGENT_MODEL` env | user shell | full model override for every subagent | overrides everything (rarely set) |

**Current tier map (2026-04-17, sonnet-medium pivot):**

- `*the-judge*` → **high** (orchestrator; opus 4.7 + high — downgraded from xhigh 2026-04-19)
- `cco-*` / `ccs-*` / `cdx-*` / `g-*` → **medium** (every teammate)
- unmapped → NOMATCH (trap leaves env unset; CC falls back to frontmatter `effort:`)

**Every teammate runs `model: sonnet` + `effort: medium` uniformly** — the
earlier opus-medium unified scheme was replaced 2026-04-17 per user
directive ("opus is terrible for being the intermediator"). Only
`the-judge` itself stays on opus 4.7 (orchestrator depth required). The
former critic/connector/planner high-effort exceptions were collapsed
when the tier pivoted; every teammate prefix now maps to medium in the
DEBUG trap.

**Godspeed marker — purest form:** every teammate dispatch appends
` | godspeed` (literal, with leading space) to the Agent() prompt — or
` | godspeed-impl` for the executor lane. No preamble. The single-token
marker IS the whole directive; sonnet-medium teammates read it as
"iterate cheap in parallel, no clarifying questions, act via tool calls".

**Codex dispatch lanes** (`src/ask.rs` `build_codex_ask_with_loadout`):

- `--spark` → `gpt-5.3-codex-spark` + `model_reasoning_effort=low` (no fast_mode)
- `-R` / `--review` → `gpt-5.4` (full) + `features.fast_mode=true` (reasoning inherited from `~/.codex/config.toml` = xhigh)
- default → `gpt-5.4-mini` + `features.fast_mode=true` + `model_reasoning_effort=high`

**Profile vs dispatch-default:** `~/.codex/config.toml` `[profiles.xbreed]` still
pins `model = "gpt-5.4"` (full) — this is the profile codex uses when invoked
outside xbreed's dispatch layer. The `gpt-5.4-mini` default applies only when
`src/ask.rs` overrides the model via `-c` on the CLI invocation. The profile
wasn't moved to mini.

**Gemini auth** is OAuth-exclusive and single-path in code (v0.4+, 2026-04-19 collapse).
`GeminiAuth` enum, named OAuth profiles, API-key fallback, and the cascade retry loop
were all removed — `src/ask.rs` now builds one `build_gemini()` command that reads
`~/.gemini/oauth_creds.json` directly. No canary, no retry. On auth failure,
dispatch bails with a `gemini login` hint.

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
