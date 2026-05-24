Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$envPath = Join-Path $repoRoot ".env"
$examplePath = Join-Path $repoRoot ".env.example"

if (Test-Path -LiteralPath $envPath) {
    Write-Host ".env already exists"
    exit 0
}

Copy-Item -LiteralPath $examplePath -Destination $envPath

$secretBytes = New-Object byte[] 32
$rng = [System.Security.Cryptography.RandomNumberGenerator]::Create()
$rng.GetBytes($secretBytes)
$rng.Dispose()
$secret = ([System.BitConverter]::ToString($secretBytes)).Replace("-", "").ToLowerInvariant()

$content = Get-Content -LiteralPath $envPath -Raw
$content = $content.Replace("change-me-generate-a-strong-local-secret", $secret)
Set-Content -LiteralPath $envPath -Value $content -NoNewline

Write-Host ".env created with local random secret"
