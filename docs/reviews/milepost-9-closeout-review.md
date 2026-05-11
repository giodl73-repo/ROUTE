# Milepost 9 Closeout Review

Date: 2026-05-10

Review scope: Milepost 9 evidence-window ledger, release-manifest metadata, closeout claims, and gate coverage.

## Findings

### Fixed During Review: Evidence-Window Rows Pointed To Milepost 8 Reviews

Severity: low.

The Milepost 9 evidence-window rows correctly represented the current evidence state, but their `review_artifact` fields still pointed to Milepost 8 review records. Milepost 9 now has its own evidence-operations review, so those rows should point to the Milepost 9 decision.

Resolution: both rows in `data/t1-evidence-windows.csv` now point to `docs/reviews/milepost-9-evidence-operations-review.md`.

### Fixed During Review: Release Manifest Used Live Polling As Verification

Severity: medium.

The release manifest listed `scripts/poll-t1-iowa511.ps1` as the verification command for the Iowa repeat-window record and the script itself. That script fetches live data, writes date-stamped caches, and accumulates event rows. It is an evidence-operation runner, not a stable release verification command.

Resolution: those manifest rows now use `manual review`, with notes explaining that executing the script mutates live evidence caches.

## Residual Risks

- The evidence-window gate validates source-window contracts and snapshot promotion rules, but it does not parse dates or cross-check event counts against the raw event table.
- The Iowa row remains `snapshot_only` and `promotion_eligible=false`; the near-1.0 annual probability in the failure ledger remains low-confidence and caveated.
- The INDOT/OHGO row remains an `enrichment_blocker` with zero observation-grade rows.

## Verdict

Milepost 9 remains internally consistent after the metadata fixes. The milestone added source-window controls and a non-promoting evidence operation; T1/T1 diamond recovery remains held.
