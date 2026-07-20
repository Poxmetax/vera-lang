$ErrorActionPreference = 'Continue'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..\..')).Path
$src = Join-Path $PSScriptRoot '..\initial\main.vera'
Set-Location $repo
$out = & cargo run -p vera -- --diag-json $src 2>&1 | Out-String
if ($out -notmatch 'add-match-arms') { Write-Host 'FAIL missing add-match-arms'; Write-Host $out; exit 1 }
if ($out -notmatch '"ephemeral"\s*:\s*true') { Write-Host 'FAIL ephemeral not true'; Write-Host $out; exit 1 }
Write-Host 'PASS T05 diag'
exit 0