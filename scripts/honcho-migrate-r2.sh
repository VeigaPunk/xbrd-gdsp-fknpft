#!/usr/bin/env bash
set -euo pipefail

# Migration script for xbreed-judge/the-judge pollution records (R2 Mission honcho-stress-0418)
# Red state: 9 records (5 broken, 3 valid, 1 stray)
# Green state (B): 3 valid kept, 6 deleted

usage() {
  cat <<'EOF'
Usage:
  ./honcho-migrate-r2.sh \
    --workspace xbreed-judge \
    --option B \
    --base-url https://api.honcho.dev \
    --token "$HONCHO_TOKEN" \
    [--clean-workspace xbreed-judge-clean] \
    [--stray-ids "id1,id2"] \
    [--dry-run]

Options:
  --workspace        Source workspace name/id (required)
  --option           One of A/B/C/D (required)
  --base-url         Honcho API base URL (default: HONCHO_BASE_URL env)
  --token            Bearer token (default: HONCHO_TOKEN env)
  --clean-workspace  Target workspace for option C
  --stray-ids        Comma-separated IDs to treat as stray (for option B)
  --dry-run          Print actions without making changes
EOF
  exit 1
}

WORKSPACE=""
CLEAN_WORKSPACE=""
OPTION=""
API_BASE="${HONCHO_BASE_URL:-}"
TOKEN="${HONCHO_TOKEN:-}"
STRAY_IDS_CSV=""
DRY_RUN="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --workspace)       WORKSPACE="$2"; shift 2 ;;
    --clean-workspace) CLEAN_WORKSPACE="$2"; shift 2 ;;
    --option)          OPTION="$2"; shift 2 ;;
    --base-url)        API_BASE="$2"; shift 2 ;;
    --token)           TOKEN="$2"; shift 2 ;;
    --stray-ids)       STRAY_IDS_CSV="$2"; shift 2 ;;
    --dry-run)         DRY_RUN="true"; shift 1 ;;
    *) usage ;;
  esac
done

[[ -z "$WORKSPACE" || -z "$OPTION" || -z "$API_BASE" ]] && usage
[[ "$OPTION" =~ ^[ABCD]$ ]] || { echo "ERROR: --option must be A, B, C, or D"; exit 1; }
[[ "$OPTION" == "C" && -z "$CLEAN_WORKSPACE" ]] && { echo "ERROR: option C requires --clean-workspace"; exit 1; }
[[ -z "$TOKEN" ]] && { echo "ERROR: --token or HONCHO_TOKEN required"; exit 1; }

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

all_ndjson="$tmpdir/all.ndjson"
broken_ndjson="$tmpdir/broken.ndjson"
valid_ndjson="$tmpdir/valid.ndjson"
stray_ndjson="$tmpdir/stray.ndjson"
post_old_ndjson="$tmpdir/post_old.ndjson"
post_clean_ndjson="$tmpdir/post_clean.ndjson"

AUTH=(-H "Authorization: Bearer ${TOKEN}" -H "Content-Type: application/json")

is_broken() {
  local content="$1"
  local trimmed
  trimmed=$(echo "$content" | sed 's/^[[:space:]]*//')
  [[ "$trimmed" == --mission* || "$trimmed" == '{"mission'* ]]
}

in_csv_set() {
  local needle="$1"
  local csv="$2"
  [[ -z "$csv" ]] && return 1
  IFS=',' read -r -a arr <<< "$csv"
  for x in "${arr[@]}"; do
    [[ "$x" == "$needle" ]] && return 0
  done
  return 1
}

line_count() { [[ -f "$1" ]] && grep -c '' "$1" || echo 0; }

# Step 1: list-query — enumerate all record ids
get_records() {
  local ws="$1"
  local out="$2"
  local cursor=""
  : > "$out"

  while :; do
    local url="${API_BASE}/v3/workspaces/${ws}/conclusions"
    [[ -n "$cursor" ]] && url="${url}?cursor=${cursor}"

    local resp
    resp=$(curl -sS -X GET "${AUTH[@]}" "$url")

    # Store each record as a base64-encoded line to safely preserve arbitrary content
    while IFS= read -r rec; do
      [[ -n "$rec" ]] && echo "$rec" >> "$out"
    done < <(echo "$resp" | jq -r '(.data // .items // .conclusions // [])[] | @base64')

    cursor=$(echo "$resp" | jq -r '.pagination.next_cursor // .next_cursor // .cursor // empty // ""')
    [[ -z "$cursor" || "$cursor" == "null" ]] && break
  done
}

# Step 2: classify into broken vs valid
classify_records() {
  : > "$broken_ndjson"
  : > "$valid_ndjson"
  : > "$stray_ndjson"

  while IFS= read -r line; do
    local obj id content
    obj=$(echo "$line" | base64 --decode)
    id=$(echo "$obj" | jq -r '.id // ""')
    content=$(echo "$obj" | jq -r '.content // ""')

    if is_broken "$content"; then
      echo "$line" >> "$broken_ndjson"
    elif in_csv_set "$id" "$STRAY_IDS_CSV"; then
      echo "$line" >> "$stray_ndjson"
    else
      echo "$line" >> "$valid_ndjson"
    fi
  done < "$all_ndjson"
}

delete_record() {
  local ws="$1"
  local id="$2"
  local url="${API_BASE}/v3/workspaces/${ws}/conclusions/${id}"
  if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY-RUN] DELETE ${url}"
  else
    curl -sS -X DELETE "${AUTH[@]}" "$url" >/dev/null
    echo "DELETED ${id}"
  fi
}

post_record() {
  local ws="$1"
  local obj_b64="$2"
  local obj payload
  obj=$(echo "$obj_b64" | base64 --decode)
  payload=$(echo "$obj" | jq -c '{content: .content, title: (.title // null), metadata: (.metadata // null)}')

  if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY-RUN] POST /v3/workspaces/${ws}/conclusions  payload=${payload}"
  else
    curl -sS -X POST "${AUTH[@]}" "${API_BASE}/v3/workspaces/${ws}/conclusions" -d "$payload" >/dev/null
    echo "POSTED into ${ws}"
  fi
}

# ---- Step 1: list-query ----
echo "=== Step 1: List records in ${WORKSPACE} ==="
get_records "$WORKSPACE" "$all_ndjson"
current_count=$(line_count "$all_ndjson")
echo "RED STATE: ${current_count} records (expected 9)"

echo ""
echo "=== Step 2: Classify records ==="
classify_records
broken_count=$(line_count "$broken_ndjson")
valid_count=$(line_count "$valid_ndjson")
stray_count=$(line_count "$stray_ndjson")
echo "Broken (--mission or {\"mission prefix): ${broken_count}"
echo "Stray (explicit stray-ids): ${stray_count}"
echo "Valid: ${valid_count}"

echo ""
echo "Broken record IDs:"
while IFS= read -r line; do
  echo "  $(echo "$line" | base64 --decode | jq -r '.id')"
done < "$broken_ndjson"

echo "Valid record IDs:"
while IFS= read -r line; do
  echo "  $(echo "$line" | base64 --decode | jq -r '.id')"
done < "$valid_ndjson"

[[ -s "$stray_ndjson" ]] && echo "Stray record IDs:" && while IFS= read -r line; do
  echo "  $(echo "$line" | base64 --decode | jq -r '.id')"
done < "$stray_ndjson"

# ---- Step 3: migration per option ----
echo ""
echo "=== Step 3: Migrate (option ${OPTION}) ==="

case "$OPTION" in
  A)
    echo "Option A: delete ALL records in ${WORKSPACE} (fresh start, no history)"
    while IFS= read -r line; do
      id=$(echo "$line" | base64 --decode | jq -r '.id')
      delete_record "$WORKSPACE" "$id"
    done < "$all_ndjson"
    ;;
  B)
    echo "Option B: delete broken + stray, keep 3 valid (simplest + preserves history)"
    while IFS= read -r line; do
      id=$(echo "$line" | base64 --decode | jq -r '.id')
      delete_record "$WORKSPACE" "$id"
    done < "$broken_ndjson"
    while IFS= read -r line; do
      id=$(echo "$line" | base64 --decode | jq -r '.id')
      delete_record "$WORKSPACE" "$id"
    done < "$stray_ndjson"
    ;;
  C)
    echo "Option C: copy 3 valid into ${CLEAN_WORKSPACE}, then delete all in source"
    while IFS= read -r line; do
      post_record "$CLEAN_WORKSPACE" "$line"
    done < "$valid_ndjson"
    while IFS= read -r line; do
      id=$(echo "$line" | base64 --decode | jq -r '.id')
      delete_record "$WORKSPACE" "$id"
    done < "$all_ndjson"
    ;;
  D)
    echo "Option D: read-time annotation only (no server-side changes)"
    {
      echo "id,classification,content_preview"
      while IFS= read -r line; do
        obj=$(echo "$line" | base64 --decode)
        id=$(echo "$obj" | jq -r '.id')
        content=$(echo "$obj" | jq -r '.content // ""')
        preview=$(echo "$content" | head -c 60 | tr '\n' ' ')
        if is_broken "$content"; then
          status="broken"
        elif in_csv_set "$id" "$STRAY_IDS_CSV"; then
          status="stray"
        else
          status="valid"
        fi
        echo "${id},${status},${preview}"
      done < "$all_ndjson"
    } | tee "$tmpdir/annotation.csv"
    echo "Annotation written (dry summary only, no server mutations)"
    ;;
esac

# ---- Step 4: verification gate ----
echo ""
echo "=== Step 4: Verification gate ==="
get_records "$WORKSPACE" "$post_old_ndjson"
post_old_count=$(line_count "$post_old_ndjson")
echo "Post-migration count in ${WORKSPACE}: ${post_old_count}"

if [[ "$OPTION" == "C" ]]; then
  get_records "$CLEAN_WORKSPACE" "$post_clean_ndjson"
  post_clean_count=$(line_count "$post_clean_ndjson")
  echo "Post-migration count in ${CLEAN_WORKSPACE}: ${post_clean_count}"
fi

case "$OPTION" in
  A)
    [[ "$DRY_RUN" == "true" ]] || { [[ "$post_old_count" -eq 0 ]] || { echo "VERIFY FAIL: expected 0 in ${WORKSPACE}, got ${post_old_count}"; exit 1; }; }
    echo "GREEN STATE (A): ${WORKSPACE}=0"
    ;;
  B)
    [[ "$DRY_RUN" == "true" ]] || { [[ "$post_old_count" -eq "$valid_count" ]] || { echo "VERIFY FAIL: expected ${valid_count} in ${WORKSPACE}, got ${post_old_count}"; exit 1; }; }
    echo "GREEN STATE (B): ${WORKSPACE}=${valid_count} (valid records preserved)"
    ;;
  C)
    [[ "$DRY_RUN" == "true" ]] || {
      [[ "$post_old_count" -eq 0 ]] || { echo "VERIFY FAIL: expected 0 in ${WORKSPACE}, got ${post_old_count}"; exit 1; }
      [[ "${post_clean_count:-0}" -eq "$valid_count" ]] || { echo "VERIFY FAIL: expected ${valid_count} in ${CLEAN_WORKSPACE}, got ${post_clean_count:-0}"; exit 1; }
    }
    echo "GREEN STATE (C): ${WORKSPACE}=0, ${CLEAN_WORKSPACE}=${valid_count}"
    ;;
  D)
    [[ "$post_old_count" -eq "$current_count" ]] || { echo "VERIFY FAIL: expected ${current_count} in ${WORKSPACE}, got ${post_old_count}"; exit 1; }
    echo "GREEN STATE (D): ${WORKSPACE}=${current_count} (unchanged)"
    ;;
esac

echo ""
echo "VERIFY OK (option ${OPTION})"
