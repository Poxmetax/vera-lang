$ErrorActionPreference = 'Continue'
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..\..')).Path
$src = Join-Path $PSScriptRoot '..\initial\main.vera'
Set-Location $repo
$out = & cargo run -p vera -- --prove $src 2>&1 | Out-String
$code = $LASTEXITCODE
if ($code -ne 1) { Write-Host "FAIL expected exit 1 got $code"; Write-Host $out; exit 1 }
if ($out -notmatch 'error:') { Write-Host 'FAIL missing error line'; Write-Host $out; exit 1 }
if ($out -match '\[PROVED\]') { Write-Host 'FAIL forged PROVED present'; Write-Host $out; exit 1 }
Write-Host 'PASS T09'
exit 0
