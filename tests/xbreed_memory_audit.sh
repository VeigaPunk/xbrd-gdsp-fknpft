#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script="$repo_root/scripts/xbreed-memory"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

db="$tmpdir/xbreed.db"
sqlite3 "$db" < "$repo_root/data/xbreed-schema.sql" >/dev/null

XBREED_MEMORY_DB="$db" "$script" put mission-alpha 1 A "baseline recall candidate" +
XBREED_MEMORY_DB="$db" "$script" put mission-alpha 1 B '{"axis_id":"A","round":1,"direction":"+"}' 0
XBREED_MEMORY_DB="$db" "$script" put mission-beta 2 C "fresh mission" -

latest="$(XBREED_MEMORY_DB="$db" "$script" latest-mission)"
[ "$latest" = "mission-beta" ]

grep_out="$(XBREED_MEMORY_DB="$db" "$script" grep "recall")"
printf '%s\n' "$grep_out" | grep -F 'mission-alpha r1 A [+]: baseline recall candidate' >/dev/null

redflags_out="$(XBREED_MEMORY_DB="$db" "$script" redflags)"
printf '%s\n' "$redflags_out" | grep -F 'mission-alpha r1 B [0]: {"axis_id":"A","round":1,"direction":"+"}' >/dev/null

mission_scoped="$(XBREED_MEMORY_DB="$db" "$script" redflags mission-alpha)"
printf '%s\n' "$mission_scoped" | grep -F 'mission-alpha r1 B [0]: {"axis_id":"A","round":1,"direction":"+"}' >/dev/null

printf 'xbreed-memory audit helpers: ok\n'
