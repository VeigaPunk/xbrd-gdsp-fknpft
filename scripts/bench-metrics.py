#!/usr/bin/env python3
"""Extract benchmark metrics from a teammate CC jsonl transcript.

Usage: bench-metrics.py <path-to-jsonl> [--header]

Emits one TSV row per file. With --header, prints the column names first.
"""
import json
import re
import sys
from collections import Counter
from datetime import datetime

COLS = [
    "team", "teammate", "model",
    "wall_s", "out_tokens", "tok_per_s",
    "tool_count", "tool_breakdown",
    "input_tokens", "cache_read_tokens", "assistant_msgs",
    "file",
]

TEAMMATE_RE = re.compile(
    r'<teammate-message\s+teammate_id="[^"]+">\s*'
    r'You are\s+`(?P<name>[^`]+)`\s+on team\s+`(?P<team>[^`]+)`',
    re.DOTALL,
)


def parse_ts(s):
    return datetime.fromisoformat(s.replace("Z", "+00:00"))


def extract(path):
    team = teammate = model = None
    first_ts = last_ts = None
    out_tok = in_tok = cache_read = 0
    msgs = 0
    tools = []
    models = set()

    with open(path) as fh:
        for line in fh:
            try:
                d = json.loads(line)
            except json.JSONDecodeError:
                continue
            t = d.get("type")
            if t == "user" and team is None:
                msg = d.get("message", {})
                content = msg.get("content", "")
                text = content if isinstance(content, str) else json.dumps(content)
                m = TEAMMATE_RE.search(text)
                if m:
                    teammate = m.group("name")
                    team = m.group("team")
            elif t == "assistant":
                msgs += 1
                m = d.get("message", {})
                models.add(m.get("model"))
                u = m.get("usage", {}) or {}
                out_tok += u.get("output_tokens", 0) or 0
                in_tok += u.get("input_tokens", 0) or 0
                cache_read += u.get("cache_read_input_tokens", 0) or 0
                ts = d.get("timestamp")
                if ts:
                    if first_ts is None:
                        first_ts = ts
                    last_ts = ts
                for b in m.get("content", []) or []:
                    if isinstance(b, dict) and b.get("type") == "tool_use":
                        tools.append(b.get("name"))

    if len(models) > 1:
        sys.stderr.write(f"WARN: mixed models in {path}: {models}\n")
    model = (models.pop() if len(models) == 1 else (models.pop() if models else "unknown"))
    wall = (parse_ts(last_ts) - parse_ts(first_ts)).total_seconds() if first_ts and last_ts else 0.0
    tps = (out_tok / wall) if wall > 0 else 0.0
    breakdown = ",".join(f"{k}:{v}" for k, v in Counter(tools).most_common())

    return {
        "team": team or "unknown",
        "teammate": teammate or "unknown",
        "model": model or "unknown",
        "wall_s": f"{wall:.1f}",
        "out_tokens": out_tok,
        "tok_per_s": f"{tps:.1f}",
        "tool_count": len(tools),
        "tool_breakdown": breakdown or "-",
        "input_tokens": in_tok,
        "cache_read_tokens": cache_read,
        "assistant_msgs": msgs,
        "file": path.rsplit("/", 1)[-1],
    }


def main():
    args = [a for a in sys.argv[1:] if a != "--header"]
    if "--header" in sys.argv:
        print("\t".join(COLS))
    if not args:
        return
    for path in args:
        r = extract(path)
        print("\t".join(str(r[c]) for c in COLS))


if __name__ == "__main__":
    main()
