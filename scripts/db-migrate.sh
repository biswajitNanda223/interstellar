#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ ! -f .env ]]; then
  ./scripts/bootstrap-env.sh
fi

if command -v podman >/dev/null 2>&1; then
  compose=podman
elif command -v docker >/dev/null 2>&1; then
  compose=docker
else
  echo "podman or docker not found on PATH" >&2
  exit 1
fi

for migration in infra/postgres/migrations/*.sql; do
  [[ -e "$migration" ]] || continue
  name="$(basename "$migration")"
  echo "Applying migration $name"
  "$compose" compose --env-file .env exec -T postgres psql -v ON_ERROR_STOP=1 -U interstellar_app -d interstellar -f "/migrations/$name"
done
