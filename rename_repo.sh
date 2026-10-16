#!/usr/bin/env bash
# Rename the GitHub repository tsukimi-danmaku -> RDplayer.
#
# Usage:  bash rename_repo.sh /path/to/token.txt
#   token.txt must contain ONLY your GitHub Personal Access Token (classic, repo scope)
#   on a single line. Create it yourself on your own machine, e.g.:
#       printf '%s' 'ghp_xxx' > ~/gh_token.txt
#
# The token is read from the file at runtime and NEVER stored in the repo or in
# any committed file.
#
# NOTE: If your network can only reach GitHub through a proxy, prefix the command
# with the proxy env var, e.g.:
#   HTTPS_PROXY=http://127.0.0.1:7897 bash rename_repo.sh ~/gh_token.txt
set -euo pipefail

TOKEN_FILE="${1:-$HOME/gh_token.txt}"
if [ ! -f "$TOKEN_FILE" ]; then
  echo "Token file not found: $TOKEN_FILE"
  echo "Create it on your own machine with:  printf '%s' 'ghp_xxx' > ~/gh_token.txt"
  exit 1
fi

TOKEN="$(tr -d '[:space:]' < "$TOKEN_FILE")"
echo "  token length: ${#TOKEN} (expected: classic=40, fine-grained=93)"
if [ "${#TOKEN}" -lt 30 ]; then
  echo "  ⚠ token looks too short/invalid — make sure the file contains ONLY the raw token, no quotes or extra text."
fi

USER="rdevils7"
OLD="tsukimi-danmaku"
NEW="RDplayer"

echo "Renaming GitHub repo $USER/$OLD -> $USER/$NEW ..."
HTTP_CODE=$(curl -s -o /tmp/gh_rename.json -w "%{http_code}" -X PATCH "https://api.github.com/repos/$USER/$OLD" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"$NEW\"}")
echo "  rename HTTP $HTTP_CODE"
if [ "$HTTP_CODE" != "200" ]; then
  echo "  ⚠ rename failed. GitHub says:"
  sed -n 's/.*"message"[ ]*:[ ]*"\([^"]*\)".*/  \1/p' /tmp/gh_rename.json | head -1
  echo "  (this can also happen if the repo is already named '$NEW', or the token lacks the 'repo' scope)"
  exit 1
fi

echo "DONE  ->  https://github.com/$USER/$NEW"
echo "Next: push the local commit with the new name:"
echo "  HTTPS_PROXY=http://127.0.0.1:7897 bash push.sh \"$TOKEN_FILE\""
