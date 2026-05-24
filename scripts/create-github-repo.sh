#!/usr/bin/env bash
set -euo pipefail

owner="${OWNER:-biswajitNanda223}"
name="${NAME:-interstellar}"
visibility="${VISIBILITY:-public}"
push_after="${PUSH:-false}"

cd "$(dirname "$0")/.."

repo="$owner/$name"
remote_url="https://github.com/$repo.git"

if command -v gh >/dev/null 2>&1; then
  if [[ "$visibility" == "private" ]]; then
    visibility_flag="--private"
  else
    visibility_flag="--public"
  fi
  gh repo create "$repo" "$visibility_flag" --source . --remote origin
else
  token="${GH_TOKEN:-${GITHUB_TOKEN:-}}"
  if [[ -z "$token" ]]; then
    echo "Install GitHub CLI or set GH_TOKEN/GITHUB_TOKEN with repo scope" >&2
    exit 1
  fi

  private=false
  if [[ "$visibility" == "private" ]]; then
    private=true
  fi

  curl -fsSL \
    -X POST \
    -H "Authorization: Bearer $token" \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/repos \
    -d "{\"name\":\"$name\",\"private\":$private,\"description\":\"Production-minded Rust monorepo for APIs, infra, docs, and automation.\",\"has_issues\":true,\"has_projects\":false,\"has_wiki\":false,\"auto_init\":false}"

  git remote remove origin 2>/dev/null || true
  git remote add origin "$remote_url"
fi

git remote set-url origin "$remote_url"
echo "GitHub repository ready: https://github.com/$repo"

if [[ "$push_after" == "true" ]]; then
  git push -u origin main
fi

