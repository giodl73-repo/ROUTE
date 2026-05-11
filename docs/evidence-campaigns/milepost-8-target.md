# Milepost 8 Target Decision

Status: selected.

Target hold: **T1/T1 failure evidence**.

## Decision

Milepost 8 will work the T1/T1 failure evidence hold before the other release-visible holds.

The target is narrow enough to execute because ROUTE already has:

- `data/t1-failure-source-plan.csv` naming source families for all 15 curated T1/T1 sites,
- `data/t1-source-health.csv` separating live, key-gated, account-gated, blocked, and source-needed paths,
- `data/t1-snapshot-plan.csv` with two A-band polling candidates,
- `data/t1-failure-events.csv` with a normalized observation schema,
- `route t1-fetch-*`, `route t1-import-*`, and `route t1-accumulate-events` commands for Iowa 511 and INDOT TrafficWise,
- downstream links into `data/t1-intersection-failures.csv`, `data/blueprint-evidence-map.csv`, and the T1/T1 diamond recovery hold.

## Scope

This campaign does not try to validate every T1/T1 site. It starts with the two A-band feeds already present in the snapshot plan:

| Site | Source | Campaign role |
|---|---|---|
| `T1X-I35-I80` Des Moines | Iowa DOT 511 ArcGIS | Existing empirical seed; verify repeatable polling/import/accumulation path |
| `T1X-I80-I90` Indiana/Ohio shared corridor | INDOT TrafficWise GraphQL | Second live feed; document why untimed rows currently fail observation-grade import |

## Expected Outcome

The expected closeout is not automatic promotion. The likely result is an improved hold:

- Des Moines remains low-confidence until a stable archive or polling window exists.
- INDOT remains source-accessible but not observation-grade until event timing/details are captured.
- The T1/T1 diamond recovery package remains held unless failure rates, durations, reroutes, and throughput retention become validated.

## Non-Goals

- Do not claim annual closure probabilities from one live snapshot.
- Do not use untimed TrafficWise rows as duration evidence.
- Do not promote T1/T1 diamond recovery benefits without reroute and throughput-retention evidence.
- Do not broaden into SLA/PTI, managed-lane, or Donner loaded-stressor work during this milestone.
