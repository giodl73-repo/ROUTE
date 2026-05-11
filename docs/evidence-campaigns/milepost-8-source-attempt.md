# Milepost 8 Source Attempt — T1/T1 Failure Evidence

Date: 2026-05-10

Target: T1/T1 failure evidence for A-band sites in `data/t1-snapshot-plan.csv`.

## Commands Run

```powershell
cargo run -q -p route -- t1-fetch-iowa511 --output data/cache/iowa511-events.json
cargo run -q -p route -- t1-import-iowa511 --input data/cache/iowa511-events.json --output data/cache/iowa511-t1-failure-events.csv --site-id T1X-I35-I80 --lat 41.658 --lon=-93.800 --radius-miles 30
cargo run -q -p route -- t1-fetch-indot-trafficwise --output data/cache/indot-trafficwise-events.json
cargo run -q -p route -- t1-import-indot-trafficwise --input data/cache/indot-trafficwise-events.json --output data/cache/indot-trafficwise-t1-failure-events.csv --site-id T1X-I80-I90
cargo run -q -p route -- t1-accumulate-events --input data/cache/iowa511-t1-failure-events.csv --output data/t1-failure-events.csv
cargo run -q -p route -- t1-failure-events --gate-observations
```

## Results

| Site | Source | Fetch | Import | Observation Result |
|---|---|---:|---:|---|
| `T1X-I35-I80` Des Moines | Iowa DOT 511 ArcGIS | pass | 25 rows | Observation gate passes; no net-new rows because checked-in ledger already contained these events |
| `T1X-I80-I90` Indiana/Ohio shared corridor | INDOT TrafficWise GraphQL | pass | 0 rows | Feed reachable, but current importer finds no timed observation-grade rows |

## Interpretation

The Iowa path is repeatable for snapshot evidence. It still does not produce a publication-grade annual closure probability because a single snapshot/polling window is not a stable historical archive.

The INDOT path is source-accessible but not observation-grade. The next evidence step is to enrich TrafficWise detail timing or join Ohio Turnpike/OHGO closure history before it can populate `data/t1-failure-events.csv`.

## Claim Status

No claim is promoted by this source attempt. The Milepost 8 result so far is an improved hold:

- Des Moines remains empirical but low-confidence and snapshot-only.
- I-80/I-90 remains source-needed for normalized observations.
- T1/T1 diamond recovery remains held until rates, durations, reroutes, and throughput retention are validated.
