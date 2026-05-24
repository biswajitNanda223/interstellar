#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

message="${1:-}"
pattern='^(feat|fix|docs|chore|refactor|test|ci|build|perf|security|revert)(\([a-z0-9._-]+\))?!?: .+'

if [[ -z "$message" ]]; then
  echo "usage: ./scripts/commit.sh '<type(scope): summary>'" >&2
  exit 1
fi

if [[ "${#message}" -gt 72 ]]; then
  echo "commit subject too long (${#message}/72)" >&2
  exit 1
fi

if [[ ! "$message" =~ $pattern ]]; then
  echo "commit message must use Conventional Commits, e.g. chore: bootstrap rust monorepo" >&2
  exit 1
fi

./scripts/check.sh
git commit -m "$message"

