#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if command -v podman >/dev/null 2>&1; then
  exec podman compose --env-file .env down
fi

if command -v docker >/dev/null 2>&1; then
  exec docker compose --env-file .env down
fi

echo "podman or docker not found on PATH" >&2
exit 1

