param(
    [string]$SiteId = "T1X-I35-I80",
    [double]$Latitude = 41.658,
    [double]$Longitude = -93.800,
    [double]$RadiusMiles = 30,
    [string]$OutputRoot = "data/cache/t1-evidence-windows/iowa511",
    [string]$AccumulatedOutput = "data/t1-failure-events.csv"
)

$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [string]$Label,
        [scriptblock]$Command
    )

    Write-Host ""
    Write-Host "==> $Label"
    & $Command
}

$stamp = (Get-Date).ToUniversalTime().ToString("yyyyMMdd-HHmmss")
$runDir = Join-Path $OutputRoot $stamp
New-Item -ItemType Directory -Force -Path $runDir | Out-Null

$rawOutput = Join-Path $runDir "iowa511-events.json"
$normalizedOutput = Join-Path $runDir "iowa511-t1-failure-events.csv"

Invoke-Checked "Fetch Iowa 511 snapshot" {
    cargo run -q -p route -- t1-fetch-iowa511 --output $rawOutput
}

Invoke-Checked "Normalize Iowa 511 snapshot" {
    cargo run -q -p route -- t1-import-iowa511 `
        --input $rawOutput `
        --output $normalizedOutput `
        --site-id $SiteId `
        --lat $Latitude `
        --lon=$Longitude `
        --radius-miles $RadiusMiles
}

Invoke-Checked "Accumulate T1/T1 event observations" {
    cargo run -q -p route -- t1-accumulate-events `
        --input $normalizedOutput `
        --output $AccumulatedOutput
}

Invoke-Checked "Event observation gate" {
    cargo run -q -p route -- t1-failure-events --events $AccumulatedOutput --gate-observations
}

Invoke-Checked "Evidence-window guard" {
    cargo run -q -p route -- t1-evidence-windows --gate-windows
}

Write-Host ""
Write-Host "Iowa 511 polling run complete:"
Write-Host "  raw:        $rawOutput"
Write-Host "  normalized: $normalizedOutput"
Write-Host "  accumulated: $AccumulatedOutput"
