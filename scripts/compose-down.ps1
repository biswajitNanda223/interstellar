Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose --env-file .env down
    exit $LASTEXITCODE
}

if (Get-Command docker -ErrorAction SilentlyContinue) {
    docker compose --env-file .env down
    exit $LASTEXITCODE
}

throw "podman or docker not found on PATH"

