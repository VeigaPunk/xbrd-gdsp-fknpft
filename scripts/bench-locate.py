#!/usr/bin/env python3
"""Locate teammate jsonls by name in project transcript dir.

Usage: bench-locate.py <project-dir> <name1> <name2> ...
Prints one TSV line per name: <name>\t<path-or-MISSING>
"""
import glob
import json
import os
import re
import sys


def main():
    proj = sys.argv[1]
    names = sys.argv[2:]
    found = {}
    for f in sorted(glob.glob(proj + "/*.jsonl"), key=lambda p: -os.path.getmtime(p)):
        try:
            with open(f) as fh:
                for i, line in enumerate(fh):
                    if i > 5:
                        break
                    try:
                        d = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    if d.get("type") != "user":
                        continue
                    msg = d.get("message", {}) or {}
                    c = msg.get("content", "")
                    t = c if isinstance(c, str) else json.dumps(c)
                    m = re.search(
                        r'<teammate-message\s+teammate_id="team-lead"[^>]*>\s*\nYou are\s+`([^`]+)`',
                        t,
                    )
                    if m and m.group(1) in names and m.group(1) not in found:
                        found[m.group(1)] = f
                        break
        except OSError:
            continue
    for n in names:
        print(f"{n}\t{found.get(n, 'MISSING')}")


if __name__ == "__main__":
    main()
