# Milepost 8 Review — T1/T1 Failure Evidence

Date: 2026-05-10

Review type: source/evidence review.

Artifacts reviewed:

- `docs/evidence-campaigns/milepost-8-target.md`
- `data/evidence-campaign-source-plan.csv`
- `docs/evidence-campaigns/milepost-8-source-attempt.md`
- `data/t1-failure-events.csv`
- `data/t1-intersection-failures.csv`
- `data/blueprint-evidence-map.csv`

## Decision

Continue the hold. Do not promote T1/T1 diamond recovery or annual failure-rate claims.

## Findings

### Iowa 511 Path Is Repeatable But Still Snapshot-Only

The Iowa 511 fetch/import path produced 25 normalized rows and passed `route t1-failure-events --gate-observations`. That is useful operational evidence. It proves the ingestion path can reproduce the current Des Moines observation ledger.

It does not prove a stable annual closure probability. The current rate is still based on a narrow snapshot/polling window and must remain low-confidence until ROUTE has an archive extract or repeated polling window.

### INDOT TrafficWise Is Reachable But Not Observation-Grade Yet

The INDOT TrafficWise fetch command succeeded, but the import produced zero rows for `T1X-I80-I90`. The blocker is not source discovery; it is event timing/detail enrichment. The feed cannot support duration or annual-rate evidence until the importer captures observation-grade timed rows or a separate Ohio/turnpike history join fills the gap.

### Blueprint Holds Are Correct

`B6-P1-DIAMOND-RECOVERY` should remain held. Milepost 8 improves traceability but does not yet validate:

- top-site annual failure rates,
- duration distributions across sites,
- truck-capable reroute time,
- throughput retention current versus I2,
- k-connectivity restoration under observed closures.

## Required Follow-Up

1. Schedule repeated Iowa 511 polling or obtain Iowa DOT historical archive.
2. Enrich INDOT TrafficWise detail timing or pair with Ohio Turnpike/OHGO closure history.
3. Keep `route standards-proof --gate-blueprint` strict for T1/T1 diamond recovery.
4. Re-review only after at least one source produces stable historical or repeated-window evidence.

## Verdict

Milepost 8 can close as an improved-hold evidence campaign if the release gate bundle passes and the closeout records that no promotion occurred.
