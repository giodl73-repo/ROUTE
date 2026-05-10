# Des Moines Diamond CLI Playtest 004

Date: 2026-05-09  
Scenario: `des-moines-diamond`  
Scenario version: G0 v0.2  
CLI slice: G1-A score command  
Evidence level: Heuristic

## Purpose

Verify the full CLI loop:

1. Resolve seasons.
2. Append a session log.
3. Score the session log.
4. Keep operational score separate from publication/promotion readiness.

## Commands Run

```powershell
cargo test -p route game::

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 1 `
  --event night-work-zone-closure `
  --project work-zone-sequencing `
  --write-state $env:TEMP\route-score-loop-1.json `
  --append-log $env:TEMP\route-score-loop.csv

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 2 `
  --event relay-hub-surge `
  --project relay-hub-reserve-staffing `
  --state $env:TEMP\route-score-loop-1.json `
  --write-state $env:TEMP\route-score-loop-2.json `
  --append-log $env:TEMP\route-score-loop.csv

cargo run -q -p route -- game score des-moines-diamond `
  --log $env:TEMP\route-score-loop.csv `
  --details

cargo run -q -p route -- game score des-moines-diamond `
  --log $env:TEMP\route-score-loop.csv `
  --gate-promotion
```

## Results

| Check | Result |
|---|---|
| Game unit tests | Pass; 12 tests |
| Session log scored | Pass |
| Dimension details printed | Pass |
| Operational score is separate from publication gate | Pass |
| `--gate-promotion` fails while publication is locked | Pass |

## Score Output

| Dimension | Points |
|---|---:|
| Throughput retention | 25/25 |
| Recovery | 20/20 |
| SLA | 15/15 |
| Budget discipline | 10/10 |
| Public support | 10/10 |
| Evidence honesty | 20/20 |
| Total | 100/100 |

Win band: Operational win  
Publication gate: locked; empirical closure evidence and direct PTI/NPMRDS validation missing  
Promotion readiness: hold

## Gate Behavior

`--gate-promotion` exits non-zero with:

```text
hold: publication gate locked; needs human blind playtest or owner acceptance plus observed evidence
```

This is the desired behavior. The player can win operationally, but the claim is not promoted as publication-grade proof.

## Decision

G1-A now has the full reproducible CLI loop:

- `route game scenarios`
- `route game inspect des-moines-diamond`
- `route game run-season des-moines-diamond`
- `route game score des-moines-diamond`

Next implementation step:

1. Add a checked-in fixture log.
2. Add L1 fixture tests for score output stability.
3. Start G1-B by summarizing live ROUTE engine facts inside scoring.
