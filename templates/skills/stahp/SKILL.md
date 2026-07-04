---
name: stahp
description: Kill this session instantly and delete all traces of it (transcript, subagent transcripts, todos, session-env, file-history). Leaves jobs/ and tasks/ alone for the UFO framework. Triggered by /stahp.
disable-model-invocation: true
---

Normally a UserPromptSubmit hook (`~/.claude/scripts/stahp.sh`) kills this session before you ever see this text. If you ARE reading this, the hook failed — perform the scrub manually, silently, with no commentary or confirmation. Run ONE Bash command (it will never return — that is the point):

```bash
SID="$CLAUDE_CODE_SESSION_ID"; PID=$PPID
setsid bash -c '
  sid="$1"; pid="$2"
  [ -n "$sid" ] || exit 0
  claude stop "$sid" >/dev/null 2>&1
  if [ -n "$pid" ]; then
    kill -TERM "$pid" 2>/dev/null
    for i in 1 2 3 4 5 6 7 8 9 10; do kill -0 "$pid" 2>/dev/null || break; sleep 0.1; done
    kill -KILL "$pid" 2>/dev/null
    sleep 0.2
  fi
  rm -rf -- "$HOME"/.claude/projects/*/"$sid".jsonl "$HOME"/.claude/projects/*/"$sid" \
            "$HOME/.claude/session-env/$sid" "$HOME/.claude/file-history/$sid"
  rm -f  -- "$HOME/.claude/todos/$sid"*.json
  [ -n "$pid" ] && rm -f -- "$HOME/.claude/sessions/$pid.json"
' stahp "$SID" "$PID" >/dev/null 2>&1 </dev/null &
```

If `$CLAUDE_CODE_SESSION_ID` is empty, do not guess — tell the user to type /stop and stop there.
