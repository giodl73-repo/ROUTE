param(
    [switch]$SkipTests
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [string]$Name,
        [scriptblock]$Command
    )

    Write-Host ""
    Write-Host "==> $Name"
    & $Command
    if (-not $?) {
        throw "Gate failed: $Name"
    }
    if ((Test-Path Variable:\LASTEXITCODE) -and $LASTEXITCODE -ne 0) {
        throw "Gate failed: $Name"
    }
}

function Test-ReleaseManifestPaths {
    $missing = @()
    Import-Csv data/release-manifest.csv | ForEach-Object {
        if (-not (Test-Path $_.artifact_path)) {
            $missing += $_.artifact_path
        }
    }

    if ($missing.Count -gt 0) {
        $missing | ForEach-Object { Write-Host "missing release artifact: $_" }
        throw "Release manifest references missing artifacts"
    }
}

function Test-ForumDocketPaths {
    $missing = @()
    Import-Csv data/forum-docket.csv | ForEach-Object {
        $row = $_
        $row.artifact -split ';' | ForEach-Object {
            $path = $_.Trim()
            if ($path -and -not (Test-Path $path)) {
                $missing += "missing docket artifact: $path"
            }
        }

        if ($row.status -ne "held" -and -not (Test-Path $row.output_artifact)) {
            $missing += "missing completed docket output: $($row.output_artifact)"
        }
    }

    if ($missing.Count -gt 0) {
        $missing | ForEach-Object { Write-Host $_ }
        throw "Forum docket references missing non-held artifacts"
    }
}

if (-not $SkipTests) {
    Invoke-Checked "Rust workspace tests" { cargo test --workspace }
}

Invoke-Checked "Release manifest path check" { Test-ReleaseManifestPaths }
Invoke-Checked "Forum docket path check" { Test-ForumDocketPaths }
Invoke-Checked "Map atlas gate" { cargo run -q -p route -- map-atlas --gate }
Invoke-Checked "T1 line selector gate" { cargo run -q -p route -- t1-line-selector --gate }
Invoke-Checked "T1 design review gate" { cargo run -q -p route -- t1-design-review --gate }
Invoke-Checked "Beck T2 service standards gate" { cargo run -q -p route -- beck-t2-service-standards --gate }
Invoke-Checked "Beck T2 qualification actions gate" { cargo run -q -p route -- beck-t2-qualification-actions --gate }
Invoke-Checked "Game T2 service overlay gate" { cargo run -q -p route -- game t2-overlays --gate }
Invoke-Checked "Game T2 scenario hook gate" { cargo run -q -p route -- game t2-hooks --gate }
Invoke-Checked "Standards pressure proof gate" { cargo run -q -p route -- standards-proof --gate-pressure }
Invoke-Checked "Standards inventory source gate" { cargo run -q -p route -- standards-inventory --gate --gate-planned }
Invoke-Checked "Pressure scenario L2 readiness gate" { cargo run -q -p route -- pressure-scenarios --gate-l2 --gate-readiness }
Invoke-Checked "Pressure scenario standards coverage gate" { cargo run -q -p route -- pressure-scenarios --coverage --gate-coverage }
Invoke-Checked "Throughput proof gate" { cargo run -q -p route -- throughput-proof --gate }
Invoke-Checked "T1/T1 failure evidence gate" { cargo run -q -p route -- t1-failures --gate-evidence }
Invoke-Checked "T1/T1 event observation gate" { cargo run -q -p route -- t1-failure-events --gate-observations }
Invoke-Checked "T1/T1 evidence-window gate" { cargo run -q -p route -- t1-evidence-windows --gate-windows }
Invoke-Checked "T1/T1 snapshot plan gate" { cargo run -q -p route -- t1-snapshot-plan --gate-plan --script --priority A }
Invoke-Checked "Game campaign gate" { cargo run -q -p route -- game campaign --gate }
Invoke-Checked "Des Moines browser fixture gate" { powershell -ExecutionPolicy Bypass -File docs/game/browser/check-des-moines-browser.ps1 }
Invoke-Checked "Forum docket gate" { cargo run -q -p route -- forum --gate }
Invoke-Checked "Significant moments gate" { cargo run -q -p route -- significant-moments --gate }
Invoke-Checked "Blueprint package gate" { cargo run -q -p route -- blueprint --gate }
Invoke-Checked "Blueprint evidence gate" { cargo run -q -p route -- blueprint-evidence --gate }
Invoke-Checked "Blueprint cost gate" { cargo run -q -p route -- blueprint-costs --gate }
Invoke-Checked "Git whitespace check" { git diff --check }

Write-Host ""
Write-Host "Milepost release gate bundle: PASS"
