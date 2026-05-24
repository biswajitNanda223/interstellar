param(
    [string] $Owner = "biswajitNanda223",
    [string] $Name = "interstellar",
    [ValidateSet("public", "private")]
    [string] $Visibility = "public",
    [switch] $Push
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw "git not found on PATH"
}

$repo = "$Owner/$Name"
$remoteUrl = "https://github.com/$repo.git"

if (Get-Command gh -ErrorAction SilentlyContinue) {
    $visibilityFlag = if ($Visibility -eq "private") { "--private" } else { "--public" }
    gh repo create $repo $visibilityFlag --source . --remote origin
    if ($LASTEXITCODE -ne 0) {
        throw "gh repo create failed"
    }
} else {
    $token = [Environment]::GetEnvironmentVariable("GH_TOKEN")
    if (-not $token) {
        $token = [Environment]::GetEnvironmentVariable("GITHUB_TOKEN")
    }
    if (-not $token) {
        throw "Install GitHub CLI or set GH_TOKEN/GITHUB_TOKEN with repo scope"
    }

    $body = @{
        name = $Name
        private = ($Visibility -eq "private")
        description = "Production-minded Rust monorepo for APIs, infra, docs, and automation."
        has_issues = $true
        has_projects = $false
        has_wiki = $false
        auto_init = $false
    } | ConvertTo-Json

    try {
        Invoke-RestMethod `
            -Method Post `
            -Uri "https://api.github.com/user/repos" `
            -Headers @{
                Authorization = "Bearer $token"
                Accept = "application/vnd.github+json"
                "X-GitHub-Api-Version" = "2022-11-28"
            } `
            -ContentType "application/json" `
            -Body $body | Out-Null
    } catch {
        throw "GitHub repo creation failed: $($_.Exception.Message)"
    }

    git remote remove origin 2>$null
    git remote add origin $remoteUrl
}

git remote set-url origin $remoteUrl
Write-Host "GitHub repository ready: https://github.com/$repo"

if ($Push) {
    git push -u origin main
    if ($LASTEXITCODE -ne 0) {
        throw "git push failed"
    }
}

