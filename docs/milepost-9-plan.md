# Milepost 9 Plan - Evidence Operations

Status: complete.

Milepost 9 turns the Milepost 8 improved hold into a repeatable evidence operation. The immediate target remains T1/T1 failure evidence, but the goal shifts from one-time source acquisition to repeatable windows, archive requests, freshness checks, and promotion gates that can distinguish live snapshots from historical evidence.

## Goal

Build the operating loop needed to decide whether T1/T1 diamond recovery can eventually move from `held` to a bounded evidence claim.

Milepost 9 should produce either:

- repeated-window evidence that strengthens one or more T1/T1 rows,
- an archive-access record that unlocks historical validation,
- or a reviewed blocker that proves the current source path cannot support promotion.

## Scope

In scope:

- Iowa 511 repeated polling or archive-history request for `T1X-I35-I80`.
- INDOT TrafficWise enrichment or Ohio Turnpike/OHGO join plan for `T1X-I80-I90`.
- Source freshness fields that show when each observation window was captured.
- A gate or review rule that prevents snapshot-only rows from being treated as annual history.
- Updated release and Blueprint references if the evidence state changes.

Out of scope:

- promoting T1/T1 diamond recovery on the basis of a single live snapshot,
- expanding to every T1/T1 candidate before the operating loop works,
- using unpublished manual estimates as recovery-grade evidence,
- rewriting Blueprint scope unless a reviewed evidence update requires it.

## Checklist

| Slice | Task | Status | Exit Gate / Artifact |
|---|---|---|---|
| B9-A | Write Milepost 9 plan and operating rules | ✅ done | `docs/milepost-9-plan.md` |
| B9-B | Add source-window fields or companion ledger for repeated T1/T1 evidence | ✅ done | `data/t1-evidence-windows.csv` and `route t1-evidence-windows --gate-windows` expose source, capture window, freshness, and snapshot/history status |
| B9-C | Implement or document Iowa 511 repeat-window path | ✅ done | `scripts/poll-t1-iowa511.ps1` and `docs/evidence-campaigns/milepost-9-iowa-repeat-window.md` define the repeated polling path for `T1X-I35-I80` |
| B9-D | Implement or document INDOT/OHGO enrichment path | ✅ done | `docs/evidence-campaigns/milepost-9-indot-ohgo-enrichment.md` keeps `T1X-I80-I90` as `enrichment_blocker` until timed rows or archive history exist |
| B9-E | Add a snapshot-history guard | ✅ done | `route t1-evidence-windows --gate-windows` and `docs/evidence-campaigns/milepost-9-snapshot-history-guard.md` fail promotion when only snapshot-only evidence exists |
| B9-F | Update T1/T1 failure and Blueprint evidence references | ✅ done | `data/t1-intersection-failures.csv` and `data/blueprint-evidence-map.csv` now point to the evidence-window guard and preserve the hold |
| B9-G | Attach Milepost 9 evidence-operations review | ✅ done | `docs/reviews/milepost-9-evidence-operations-review.md` continues the hold |
| B9-H | Run release gate bundle and write closeout | ✅ done | `scripts/check-mileposts.ps1` passes; `docs/milepost-9-closeout.md` records the result |

## Promotion Rule

Milepost 9 may only promote a T1/T1 failure claim when the evidence has:

- a named source and capture window,
- repeated observations or historical archive depth,
- duration or closure-state fields suitable for the claim being made,
- a review record that bounds annualization and recovery assumptions,
- propagated status updates in pressure, Blueprint, release, and spec artifacts.

If those conditions are not met, Milepost 9 should improve the hold rather than force a pass.

## Done Criteria

Milepost 9 is done when the selected T1/T1 evidence operation has a repeatable acquisition path or explicit blocker, evidence-window metadata, a snapshot-history guard, updated claim references, review decision, passing release gate bundle, and closeout.
