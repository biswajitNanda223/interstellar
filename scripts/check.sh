#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  exit 0
fi

if ! command -v docker >/dev/null 2>&1; then
  echo "cargo not found on PATH and docker fallback unavailable" >&2
  exit 1
fi

echo "check: cargo not found; using dockerized Rust toolchain"
repo_path="$(pwd -W 2>/dev/null || pwd)"
docker run --rm \
  -v "${repo_path}:/workspace" \
  -w /workspace \
  docker.io/library/rust:1.95-trixie \
  sh -lc "export PATH=/usr/local/cargo/bin:\$PATH && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace"
