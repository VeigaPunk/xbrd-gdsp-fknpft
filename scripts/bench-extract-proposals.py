#!/usr/bin/env python3
"""Extract PROPOSAL SendMessage payloads from teammate jsonls.

Usage: bench-extract-proposals.py <name> <jsonl-path> [<name> <jsonl-path>...]
Writes one file per teammate at data/proposals/<name>.md.
"""
import json
import os
import sys


def extract(path):
    last_proposal = None
    with open(path) as fh:
        for line in fh:
            try:
                d = json.loads(line)
            except json.JSONDecodeError:
                continue
            if d.get("type") != "assistant":
                continue
            for b in d.get("message", {}).get("content", []) or []:
                if not isinstance(b, dict):
                    continue
                if b.get("type") == "tool_use" and b.get("name") == "SendMessage":
                    inp = b.get("input", {}) or {}
                    msg = inp.get("message", "")
                    if isinstance(msg, str) and "PROPOSAL" in msg:
                        last_proposal = msg
    return last_proposal


def main():
    args = sys.argv[1:]
    os.makedirs("data/proposals", exist_ok=True)
    for i in range(0, len(args), 2):
        name = args[i]
        path = args[i + 1]
        p = extract(path)
        out = f"data/proposals/{name}.md"
        if p:
            with open(out, "w") as fh:
                fh.write(p)
            print(f"{name}\t{len(p)} chars\t{out}")
        else:
            print(f"{name}\tNO_PROPOSAL\t-")


if __name__ == "__main__":
    main()
