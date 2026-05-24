Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw "git not found on PATH"
}

git config --local core.hooksPath .githooks
git config --local commit.template .gitmessage

Write-Host "Git hooks installed"
Write-Host "core.hooksPath=.githooks"
Write-Host "commit.template=.gitmessage"

