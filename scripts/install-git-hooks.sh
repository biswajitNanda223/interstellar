#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

git config --local core.hooksPath .githooks
git config --local commit.template .gitmessage

echo "Git hooks installed"
echo "core.hooksPath=.githooks"
echo "commit.template=.gitmessage"

