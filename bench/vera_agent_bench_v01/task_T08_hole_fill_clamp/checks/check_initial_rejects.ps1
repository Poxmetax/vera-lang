$ErrorActionPreference = 'Continue'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..\..')).Path
$src = Join-Path $PSScriptRoot '..\initial\main.vera'
Set-Location $repo
$out = & cargo run -p vera -- $src 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) { Write-Host 'FAIL expected unfilled hole to reject'; Write-Host $out; exit 1 }
Write-Host 'PASS T08 initial-reject'
exit 0