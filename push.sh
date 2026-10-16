#!/usr/bin/env bash
# Push tsukimi-danmaku to GitHub.
#
# Usage:  bash push.sh /path/to/token.txt
#   token.txt must contain ONLY your GitHub Personal Access Token (classic, repo scope)
#   on a single line. Create it yourself on your own machine, e.g.:
#       printf '%s' 'ghp_xxx' > ~/gh_token.txt
#
# The token is read from the file at runtime and NEVER stored in the repo or
# in any committed file. After pushing, the remote URL is reset to a token-free
# form.
set -euo pipefail

TOKEN_FILE="${1:-$HOME/gh_token.txt}"
if [ ! -f "$TOKEN_FILE" ]; then
  echo "Token file not found: $TOKEN_FILE"
  echo "Create it on your own machine with:  printf '%s' 'ghp_xxx' > ~/gh_token.txt"
  exit 1
fi

TOKEN="$(tr -d '[:space:]' < "$TOKEN_FILE")"
USER="rdevils7"
REPO="tsukimi-danmaku"

echo "Creating GitHub repo '$REPO' (ignored if it already exists)..."
curl -s -o /tmp/gh_repo.json -w "  HTTP %{http_code}\n" -X POST "https://api.github.com/user/repos" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"$REPO\",\"description\":\"tsukimi media player with custom danmaku (danmu_api) server support — auto select source & episode\",\"private\":false,\"has_issues\":true,\"has_wiki\":false}" || true

echo "Configuring remote and pushing..."
git remote remove origin 2>/dev/null || true
git remote add origin "https://$USER:$TOKEN@github.com/$USER/$REPO.git"
git push -u origin main

# Strip the token from the stored remote URL
git remote set-url origin "https://github.com/$USER/$REPO.git"
echo "DONE  ->  https://github.com/$USER/$REPO"
