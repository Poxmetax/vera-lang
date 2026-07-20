# VeraAgentBench T01 check ā€” run from any cwd
$ErrorActionPreference = 'Continue'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..\..')).Path
$src = Join-Path $PSScriptRoot '..\initial\main.vera'
Set-Location $repo
$out = & cargo run -p vera -- $src 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { Write-Host 'FAIL exit'; Write-Host $out; exit 1 }
if ($out -notmatch 'Hello, VERA') { Write-Host 'FAIL missing Hello, VERA'; Write-Host $out; exit 1 }
Write-Host 'PASS T01'
exit 0