# Claude Code agent-teammate spawn cmdline

Surfaced 2026-05-30 while orchestrating `bench-3q-models-053026`. When Claude Code spawns a teammate Agent (via the `Agent` tool with `team_name` set), the underlying subprocess invocation is the full `claude` CLI with experimental team flags. Captured verbatim:

```bash
cd /home/vhpnk/repos/xbrd-gdsp-fknpft && env CLAUDECODE=1 CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1 /home/vhpnk/.local/share/fnm/node-versions/v24.15.0/installation/lib/node_modules/@anthropic-ai/claude-code/bin/claude.exe --agent-id cdx-gpt54mini-r2@bench-3q-models-053026 --agent-name cdx-gpt54mini-r2 --team-name bench-3q-models-053026 --agent-color purple --parent-session-id d3228b9f-4ccb-48fa-88d5-78e7aee53d51 --agent-type general-purpose --dangerously-skip-permissions --mod
```

Notable flags:

- `CLAUDECODE=1` + `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` — gates the team subsystem.
- `--agent-id <name>@<team>` — `agentId` namespacing in `~/.claude/teams/<team>/config.json`.
- `--agent-name`, `--team-name`, `--agent-color` — propagated to the parent's notification stream so teammate messages render with their color tag.
- `--parent-session-id` — links the spawned session back to the orchestrator's session UUID so message routing works.
- `--agent-type general-purpose` — comes straight from the `subagent_type` argument passed to `Agent`.
- `--dangerously-skip-permissions` — teammate sessions auto-approve tool calls; the orchestrator's permission context does NOT carry over interactively.
- `--mod...` — trailing arg truncated in capture; likely `--model <opus|sonnet|haiku>` mapped from the `Agent.model` parameter.

Path note: `claude.exe` under `fnm` `node-versions` confirms WSL2 (Linux) running the Windows-named CLI binary that ships in the `@anthropic-ai/claude-code` npm package — the `.exe` suffix is cosmetic on this platform.
