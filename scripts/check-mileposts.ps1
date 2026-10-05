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

# Build once: every following gate runs the same native CLI and reads its data
# at runtime. Repeated cargo run calls can invalidate a large build between gates.
$script:RouteBinary = $null
Invoke-Checked "Build Rust CLI" {
    cargo build --locked -p route --message-format=json-render-diagnostics | ForEach-Object {
        $artifact = $_ | ConvertFrom-Json
        if ($artifact.reason -eq "compiler-message" -and $artifact.message.rendered) {
            Write-Host $artifact.message.rendered
        }
        if ($artifact.reason -eq "compiler-artifact" -and $artifact.target.name -eq "route" -and $artifact.executable) {
            $script:RouteBinary = $artifact.executable
        }
    }
    if ($LASTEXITCODE -ne 0) { throw "Rust CLI build failed" }
    if (-not $script:RouteBinary -or -not (Test-Path -LiteralPath $script:RouteBinary)) {
        throw "Cargo did not report a built ROUTE executable"
    }
}

Invoke-Checked "Release manifest path check" { Test-ReleaseManifestPaths }
Invoke-Checked "Release manifest metadata gate" { & $script:RouteBinary release-manifest --gate }
Invoke-Checked "Source fetch cache policy gate" { & $script:RouteBinary source-fetch-policy --gate }
Invoke-Checked "Optimizer manifest gate" { & $script:RouteBinary optimizer-manifest --gate }
Invoke-Checked "Forum docket path check" { Test-ForumDocketPaths }
Invoke-Checked "Map atlas gate" { & $script:RouteBinary map-atlas --gate }
Invoke-Checked "T1 line selector gate" { & $script:RouteBinary t1-line-selector --gate }
Invoke-Checked "T1 design review gate" { & $script:RouteBinary t1-design-review --gate }
Invoke-Checked "T1 design policy gate" { & $script:RouteBinary t1-design-policy --gate }
Invoke-Checked "T1 score exception gate" { & $script:RouteBinary t1-score-exceptions --gate }
Invoke-Checked "Beck T2 service standards gate" { & $script:RouteBinary beck-t2-service-standards --gate }
Invoke-Checked "Beck T2 qualification actions gate" { & $script:RouteBinary beck-t2-qualification-actions --gate }
Invoke-Checked "Game T2 service overlay gate" { & $script:RouteBinary game t2-overlays --gate }
Invoke-Checked "Game T2 scenario hook gate" { & $script:RouteBinary game t2-hooks --gate }
Invoke-Checked "Standards pressure proof gate" { & $script:RouteBinary standards-proof --gate-pressure }
Invoke-Checked "Standards inventory source gate" { & $script:RouteBinary standards-inventory --gate --gate-planned }
Invoke-Checked "Pressure scenario L2 readiness gate" { & $script:RouteBinary pressure-scenarios --gate-l2 --gate-readiness }
Invoke-Checked "Pressure scenario standards coverage gate" { & $script:RouteBinary pressure-scenarios --coverage --gate-coverage }
Invoke-Checked "Throughput proof gate" { & $script:RouteBinary throughput-proof --gate }
Invoke-Checked "T1/T1 failure evidence gate" { & $script:RouteBinary t1-failures --gate-evidence }
Invoke-Checked "T1/T1 event observation gate" { & $script:RouteBinary t1-failure-events --gate-observations }
Invoke-Checked "T1/T1 evidence-window gate" { & $script:RouteBinary t1-evidence-windows --gate-windows }
Invoke-Checked "T1/T1 snapshot plan gate" { & $script:RouteBinary t1-snapshot-plan --gate-plan --script --priority A }
Invoke-Checked "Game campaign gate" { & $script:RouteBinary game campaign --gate }
Invoke-Checked "Des Moines browser fixture gate" { powershell -ExecutionPolicy Bypass -File docs/game/browser/check-des-moines-browser.ps1 }
Invoke-Checked "Forum docket gate" { & $script:RouteBinary forum --gate }
Invoke-Checked "Significant moments gate" { & $script:RouteBinary significant-moments --gate }
Invoke-Checked "Blueprint package gate" { & $script:RouteBinary blueprint --gate }
Invoke-Checked "Blueprint evidence gate" { & $script:RouteBinary blueprint-evidence --gate }
Invoke-Checked "Blueprint cost gate" { & $script:RouteBinary blueprint-costs --gate }
Invoke-Checked "Git whitespace check" { git diff --check }

Write-Host ""
Write-Host "Milepost release gate bundle: PASS"
