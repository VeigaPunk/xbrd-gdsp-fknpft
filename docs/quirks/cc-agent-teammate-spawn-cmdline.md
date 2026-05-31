# Claude Code agent-teammate spawn cmdline

Surfaced 2026-05-30 while orchestrating `bench-3q-models-053026`. When Claude Code spawns a teammate Agent (via the `Agent` tool with `team_name` set), the underlying subprocess invocation is the full `claude` CLI with experimental team flags. Two captures from the same session:

## Capture 1 — `general-purpose` teammate, trailing arg truncated

```bash
cd /home/vhpnk/repos/xbrd-gdsp-fknpft && env CLAUDECODE=1 CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1 /home/vhpnk/.local/share/fnm/node-versions/v24.15.0/installation/lib/node_modules/@anthropic-ai/claude-code/bin/claude.exe --agent-id cdx-gpt54mini-r2@bench-3q-models-053026 --agent-name cdx-gpt54mini-r2 --team-name bench-3q-models-053026 --agent-color purple --parent-session-id d3228b9f-4ccb-48fa-88d5-78e7aee53d51 --agent-type general-purpose --dangerously-skip-permissions --mod
```

## Capture 2 — `reviewer` role teammate, full trailing `--model sonnet`

```bash
cd /home/vhpnk/repos/xbrd-gdsp-fknpft && env CLAUDECODE=1 CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1 /home/vhpnk/.local/share/fnm/node-versions/v24.15.0/installation/lib/node_modules/@anthropic-ai/claude-code/bin/claude.exe --agent-id cdx-reviewer-gpt55@bench-3q-models-053026 --agent-name cdx-reviewer-gpt55 --team-name bench-3q-models-053026 --agent-color orange --parent-session-id d3228b9f-4ccb-48fa-88d5-78e7aee53d51 --agent-type reviewer --dangerously-skip-permissions --model sonnet
```

Capture 2 resolves the `--mod` truncation seen in Capture 1 — it is `--model <opus|sonnet|haiku>`, mapped from `Agent.model`. It also shows `--agent-type reviewer` (a proper xbgst role from `~/.claude/agents/reviewer.md`) rather than the catch-all `general-purpose`, and a different `--agent-color` (orange vs purple) per teammate.

Bonus oddity: Capture 2 was pasted into the orchestrator chat with a leading `he` typo (`hecd ...`), and the user's shell helpfully emitted the Ubuntu `command-not-found` block:

```
Command 'hecd' not found, did you mean:
  command 'hpcd' from deb hfsplus (1.0.4-19build1)
  command 'head' from deb coreutils-from-gnu (0.0.0~ubuntu25)
  ...
  command 'hcd' from deb hfsutils (3.2.6-16ubuntu2)
  command 'herd' from deb shepherd (1.0.9-3)
Try: sudo apt install <deb name>
```

Which incidentally confirms the host: Ubuntu 25-series under WSL2 — explaining the `.exe`-suffixed Linux binary path in the spawn line.

## Notable flags

- `CLAUDECODE=1` + `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` — gates the team subsystem.
- `--agent-id <name>@<team>` — `agentId` namespacing in `~/.claude/teams/<team>/config.json`.
- `--agent-name`, `--team-name`, `--agent-color` — propagated to the parent's notification stream so teammate messages render with their color tag.
- `--parent-session-id` — links the spawned session back to the orchestrator's session UUID so message routing works.
- `--agent-type <role>` — comes straight from the `subagent_type` argument passed to `Agent`. Values seen: `general-purpose`, `reviewer` (any agent file under `~/.claude/agents/` is valid).
- `--dangerously-skip-permissions` — teammate sessions auto-approve tool calls; the orchestrator's permission context does NOT carry over interactively.
- `--model <opus|sonnet|haiku>` — maps from `Agent.model` parameter. Omitted when caller didn't set one (inherits parent).

Path note: `claude.exe` under `fnm` `node-versions` is the Linux-resident binary shipped in the `@anthropic-ai/claude-code` npm package — the `.exe` suffix is cosmetic on WSL2.
