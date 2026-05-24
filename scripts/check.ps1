Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

function Invoke-Checked {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Name,
        [Parameter(Mandatory = $true)]
        [scriptblock] $Command
    )

    Write-Host "check: $Name"
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Invoke-Checked "cargo fmt" { cargo fmt --all --check }
    Invoke-Checked "cargo clippy" { cargo clippy --workspace --all-targets -- -D warnings }
    Invoke-Checked "cargo test" { cargo test --workspace }
    exit 0
}

if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    throw "cargo not found on PATH and docker fallback unavailable"
}

Write-Host "check: cargo not found; using dockerized Rust toolchain"
$repoPath = (Resolve-Path $repoRoot).Path
$dockerRepoPath = $repoPath -replace '\\', '/'
if ($dockerRepoPath -match '^([A-Za-z]):/(.*)$') {
    $drive = $Matches[1].ToLowerInvariant()
    $rest = $Matches[2]
    $dockerRepoPath = "/$drive/$rest"
}

Invoke-Checked "docker cargo fmt/clippy/test" {
    docker run --rm `
        -v "${dockerRepoPath}:/workspace" `
        -w /workspace `
        docker.io/library/rust:1.95-trixie `
        sh -lc "export PATH=/usr/local/cargo/bin:`$PATH && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace"
}
