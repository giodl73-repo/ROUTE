# Milepost 9 Closeout - Evidence Operations

Status: complete.

Milepost 9 turned the Milepost 8 T1/T1 improved hold into an evidence operation. The result is a stronger hold with executable source-window controls, not a claim promotion.

## Closure Decision

Milepost 9 can close because the selected T1/T1 evidence operation now has:

- source-window metadata,
- an executable Iowa repeat-polling path,
- a documented INDOT/OHGO enrichment blocker,
- a snapshot-history promotion guard,
- updated T1/T1 failure and Blueprint evidence references,
- a review decision,
- a passing release gate bundle.

## What Changed

| Artifact | Change |
|---|---|
| `data/t1-evidence-windows.csv` | Added source-window ledger for snapshot, blocker, and future repeated-window evidence |
| `route t1-evidence-windows --gate-windows` | Added CLI guard that prevents snapshot-only evidence from becoming promotion eligible |
| `scripts/poll-t1-iowa511.ps1` | Added date-stamped Iowa 511 polling/import/accumulation runner |
| `docs/evidence-campaigns/milepost-9-iowa-repeat-window.md` | Documented the Des Moines repeated-polling path |
| `docs/evidence-campaigns/milepost-9-indot-ohgo-enrichment.md` | Documented the I-80/I-90 enrichment blocker and OHGO/Turnpike path |
| `docs/evidence-campaigns/milepost-9-snapshot-history-guard.md` | Documented the promotion guard |
| `data/t1-intersection-failures.csv` | Linked Des Moines and I-80/I-90 rows to the evidence-window guard |
| `data/blueprint-evidence-map.csv` | Linked T1-DIAMOND-K hold to the evidence-window guard |
| `docs/reviews/milepost-9-evidence-operations-review.md` | Review decision: continue hold |

## Gate Bundle

The Milepost 9 release gate bundle passed on 2026-05-10:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1
```

Result: PASS.

## Remaining Hold

T1/T1 diamond recovery remains held because Milepost 9 did not validate:

- stable annual failure rates,
- historical or repeated-window closure probability,
- multi-site duration distributions,
- truck-capable reroute times,
- throughput retention current versus I2,
- k-connectivity restoration under observed closure conditions.

## Next Evidence Step

Run the Iowa polling script across multiple capture dates or obtain Iowa DOT archive history. For the I-80/I-90 shared corridor, enrich TrafficWise timing or join OHGO/Turnpike closure records before attempting annualization.
