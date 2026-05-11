# Milepost 9 Review - Evidence Operations

Date: 2026-05-10

Review scope: T1/T1 evidence-window ledger, Iowa repeat-window path, INDOT/OHGO enrichment path, snapshot-history guard, T1/T1 failure ledger propagation, and Blueprint evidence propagation.

## Decision

Continue the hold.

Milepost 9 improves the operating controls around T1/T1 failure evidence, but it does not promote T1/T1 diamond recovery or annual failure-rate claims.

## Findings

### Source-Window Ledger Is Now Explicit

`data/t1-evidence-windows.csv` separates raw event observations from source-window interpretation.

The Des Moines Iowa row is correctly labeled:

- `evidence_mode=snapshot_only`
- `promotion_eligible=false`

The Indiana/Ohio shared-corridor row is correctly labeled:

- `evidence_mode=enrichment_blocker`
- `event_count=0`
- `promotion_eligible=false`

### Snapshot-History Guard Is Executable

`route t1-evidence-windows --gate-windows` prevents snapshot-only rows from being marked promotion eligible. It also requires source-window metadata, event counts, blocker notes, next steps, and review artifacts.

This is the right control for the current evidence state.

### Iowa Path Is Operational But Not Historical Yet

`scripts/poll-t1-iowa511.ps1` gives the Des Moines campaign a date-stamped polling path. That can create a repeated-window record over time, but the current evidence remains a single accumulated snapshot sample until repeated runs or archive history are reviewed.

### INDOT/OHGO Remains Blocked

`docs/evidence-campaigns/milepost-9-indot-ohgo-enrichment.md` correctly keeps `T1X-I80-I90` out of annualization until TrafficWise timing is enriched or OHGO/Turnpike history supplies observation-grade rows.

## Verdict

Milepost 9 can close as an evidence-operations milestone. It added repeatability and a promotion guard, not proof of the held claim.

The T1/T1 diamond recovery package remains held.
