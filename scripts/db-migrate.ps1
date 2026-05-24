Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Test-Path -LiteralPath ".env")) {
    & (Join-Path $PSScriptRoot "bootstrap-env.ps1")
}

$compose = $null
if (Get-Command podman -ErrorAction SilentlyContinue) {
    $compose = "podman"
} elseif (Get-Command docker -ErrorAction SilentlyContinue) {
    $compose = "docker"
} else {
    throw "podman or docker not found on PATH"
}

Get-ChildItem -LiteralPath "infra/postgres/migrations" -Filter "*.sql" | Sort-Object Name | ForEach-Object {
    Write-Host "Applying migration $($_.Name)"
    & $compose compose --env-file .env exec -T postgres psql -v ON_ERROR_STOP=1 -U interstellar_app -d interstellar -f "/migrations/$($_.Name)"
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

