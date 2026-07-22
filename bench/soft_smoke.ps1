# Smoke (PowerShell). Non-zero exit on unexpected failure.
# Expect: lib tests pass; prove_clamp proved; prove_runtime_hint RUNTIME-CHECKED;
#         prove_refuted REFUTED with exit code 3.
#
# Usage (from repo root):
#   powershell -File bench/soft_smoke.ps1

$ErrorActionPreference = "Continue"
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
$env:Path = "$cargoBin;" + $env:Path

function Fail([string]$msg) {
    Write-Host "SOFT-SMOKE FAIL: $msg" -ForegroundColor Red
    exit 1
}

# Script lives in bench/; repo root is one level up (must contain Cargo.toml).
$Root = Split-Path -Parent $PSScriptRoot
if (-not (Test-Path (Join-Path $Root "Cargo.toml"))) {
    Fail "Cargo.toml not found above bench/; run from a vera-lang checkout"
}
Set-Location $Root

function Invoke-CargoCapture([string[]]$CargoArgs) {
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = "cargo"
    $psi.Arguments = ($CargoArgs -join " ")
    $psi.WorkingDirectory = $Root
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $p = [System.Diagnostics.Process]::Start($psi)
    $stdout = $p.StandardOutput.ReadToEnd()
    $stderr = $p.StandardError.ReadToEnd()
    $p.WaitForExit()
    return @{ ExitCode = $p.ExitCode; Out = ($stdout + $stderr) }
}

Write-Host "== soft_smoke: cargo test -p vera --lib =="
$r = Invoke-CargoCapture @("test", "-p", "vera", "--lib")
Write-Host $r.Out
if ($r.ExitCode -ne 0) { Fail "cargo test --lib exit $($r.ExitCode)" }

Write-Host "== soft_smoke: --prove examples/prove_clamp.vera =="
$r = Invoke-CargoCapture @("run", "-p", "vera", "--", "--prove", "examples/prove_clamp.vera")
Write-Host $r.Out
if ($r.ExitCode -ne 0) { Fail "prove_clamp exit $($r.ExitCode) (want 0)" }
if ($r.Out -notmatch "6 proved") { Fail "prove_clamp expected '6 proved' in summary" }

Write-Host "== soft_smoke: --prove examples/prove_runtime_hint.vera =="
$r = Invoke-CargoCapture @("run", "-p", "vera", "--", "--prove", "examples/prove_runtime_hint.vera")
Write-Host $r.Out
if ($r.ExitCode -ne 0) { Fail "prove_runtime_hint exit $($r.ExitCode) (want 0)" }
if ($r.Out -notmatch "\[RUNTIME-CHECKED\]") { Fail "prove_runtime_hint expected [RUNTIME-CHECKED]" }

Write-Host "== soft_smoke: --prove examples/prove_refuted.vera (expect exit 3) =="
$r = Invoke-CargoCapture @("run", "-p", "vera", "--", "--prove", "examples/prove_refuted.vera")
Write-Host $r.Out
if ($r.ExitCode -ne 3) { Fail "prove_refuted exit $($r.ExitCode) (want 3 REFUTED)" }
if ($r.Out -notmatch "\[REFUTED\]") { Fail "prove_refuted expected [REFUTED]" }

Write-Host "SOFT-SMOKE PASS" -ForegroundColor Green
exit 0
