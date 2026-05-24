#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ -f .env ]]; then
  echo ".env already exists"
  exit 0
fi

cp .env.example .env
secret="$(openssl rand -hex 32)"
python - <<PY
from pathlib import Path
p = Path(".env")
s = p.read_text()
s = s.replace("change-me-generate-a-strong-local-secret", "${secret}")
p.write_text(s)
PY

echo ".env created with local random secret"
