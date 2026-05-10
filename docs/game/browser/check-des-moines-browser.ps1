$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..\..")
$htmlPath = Join-Path $PSScriptRoot "des-moines-diamond.html"
$statePath = Join-Path $root "data\game\des-moines-diamond-state-fixture.json"
$sessionPath = Join-Path $root "data\game\des-moines-diamond-session-fixture.csv"

function Assert-Contains($Text, $Needle, $Label) {
    if (-not $Text.Contains($Needle)) {
        throw "Missing ${Label}: ${Needle}"
    }
}

function Assert-NotContains($Text, $Needle, $Label) {
    if ($Text.Contains($Needle)) {
        throw "Unexpected ${Label}: ${Needle}"
    }
}

$html = Get-Content -Raw -LiteralPath $htmlPath
$state = Get-Content -Raw -LiteralPath $statePath | ConvertFrom-Json
$session = Import-Csv -LiteralPath $sessionPath

Assert-Contains $html "<title>Des Moines Diamond</title>" "document title"
Assert-Contains $html 'aria-label="I-35 and I-80 Des Moines transfer topology"' "map aria label"
Assert-Contains $html 'id="projects"' "project rail"
Assert-Contains $html 'id="evidence-list"' "evidence drawer"
Assert-Contains $html 'id="event-log"' "event log"
Assert-Contains $html 'id="before"' "before playback control"
Assert-Contains $html 'id="after"' "after playback control"
Assert-Contains $html 'id="connector"' "connector path"
Assert-Contains $html "Publication claim locked" "publication lock copy"
Assert-Contains $html "operational win" "operational win copy"
Assert-Contains $html "not independent transfer paths" "topology warning copy"

Assert-Contains $html "season: $($state.season)" "fixture season"
Assert-Contains $html "budget: $($state.budget)" "fixture budget"
Assert-Contains $html "construction_crews: $($state.construction_crews)" "fixture crews"
Assert-Contains $html "political_capital: $($state.political_capital)" "fixture political capital"
Assert-Contains $html "public_patience: $($state.public_patience)" "fixture public patience"
Assert-Contains $html "operations_capacity: $($state.operations_capacity)" "fixture operations capacity"
Assert-Contains $html "evidence_confidence: $($state.evidence_confidence)" "fixture evidence confidence"
Assert-Contains $html $state.publication_gate "fixture publication gate"

if ($session.Count -lt 2) {
    throw "Session fixture must include at least two seasons"
}

Assert-Contains $html "83,423" "incident throughput"
Assert-Contains $html "86,671" "intervention throughput"
Assert-Contains $html "I35xI80 recognized; k=0; 3 connectors needed." "diamond evidence"
Assert-NotContains $html "publication unlocked" "false publication unlock"

Write-Host "Des Moines browser fixture check: PASS"
