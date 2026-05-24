Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Test-Path -LiteralPath ".env")) {
    & (Join-Path $PSScriptRoot "bootstrap-env.ps1")
}

if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman compose --env-file .env up --build
    exit $LASTEXITCODE
}

if (Get-Command docker -ErrorAction SilentlyContinue) {
    docker compose --env-file .env up --build
    exit $LASTEXITCODE
}

throw "podman or docker not found on PATH"

