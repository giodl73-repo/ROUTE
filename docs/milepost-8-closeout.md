# Milepost 8 Closeout — Evidence Campaign

Status: complete.

Milepost 8 selected the T1/T1 failure evidence hold and ran an A-band source campaign against Iowa 511 and INDOT TrafficWise. The result is an improved hold, not a claim promotion.

## Closure Decision

Milepost 8 can close because the selected hold now has a target decision, campaign source checklist, live source attempt, normalized observation outcome, updated claim references, review decision, and passing release gate bundle.

The T1/T1 diamond recovery claim remains held.

## What Changed

| Artifact | Change |
|---|---|
| `docs/evidence-campaigns/milepost-8-target.md` | Selected T1/T1 failure evidence as the campaign target |
| `data/evidence-campaign-source-plan.csv` | Created campaign-specific A/B-band source checklist |
| `docs/evidence-campaigns/milepost-8-source-attempt.md` | Recorded Iowa and INDOT source attempts |
| `data/t1-intersection-failures.csv` | Added Milepost 8 source-attempt caveats to Des Moines and I-80/I-90 rows |
| `data/blueprint-evidence-map.csv` | Linked T1-DIAMOND-K hold to Milepost 8 source attempt |
| `docs/reviews/milepost-8-t1-failure-evidence-review.md` | Review decision: continue hold |

## Source Attempt Result

| Site | Source | Result |
|---|---|---|
| `T1X-I35-I80` Des Moines | Iowa DOT 511 ArcGIS | Fetch/import repeated successfully and produced 25 normalized rows; no net-new rows after accumulation |
| `T1X-I80-I90` Indiana/Ohio shared corridor | INDOT TrafficWise GraphQL | Fetch succeeded, but import produced 0 observation-grade timed rows |

## Gate Bundle

The Milepost 8 gate bundle passed on 2026-05-10:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1
```

Result: PASS.

## Remaining Hold

T1/T1 diamond recovery remains held because Milepost 8 did not validate:

- stable annual failure rates,
- multi-site duration distributions,
- truck-capable reroute times,
- throughput retention current versus I2,
- k-connectivity restoration under observed closure conditions.

## Next Evidence Step

The next campaign should either:

1. schedule repeated Iowa 511 polling or obtain Iowa DOT archive history, or
2. enrich INDOT TrafficWise event timing and join Ohio Turnpike/OHGO closure records for the I-80/I-90 shared corridor.

No Blueprint or publication claim should be promoted until one of those paths produces stable historical or repeated-window evidence.
