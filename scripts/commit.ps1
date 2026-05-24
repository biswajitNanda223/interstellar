param(
    [Parameter(Mandatory = $true)]
    [string] $Message
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw "git not found on PATH"
}

$pattern = '^(feat|fix|docs|chore|refactor|test|ci|build|perf|security|revert)(\([a-z0-9._-]+\))?!?: .+'
if ($Message.Length -gt 72) {
    throw "commit subject too long ($($Message.Length)/72)"
}

if ($Message -notmatch $pattern) {
    throw "commit message must use Conventional Commits, e.g. chore: bootstrap rust monorepo"
}

& powershell -ExecutionPolicy Bypass -File ./scripts/check.ps1
if ($LASTEXITCODE -ne 0) {
    throw "checks failed; commit aborted"
}

git commit -m $Message
if ($LASTEXITCODE -ne 0) {
    throw "git commit failed with exit code $LASTEXITCODE"
}
