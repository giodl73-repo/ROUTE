# Milepost 8 Plan — Evidence Campaign

Status: planned.

Milepost 8 is the first post-release evidence campaign. The release candidate from Milepost 7 made the holds visible; this milestone chooses one hold and works it far enough to promote it, keep it held with better evidence, or downgrade it cleanly.

## Candidate Holds

| Candidate | Why It Matters | Current Hold |
|---|---|---|
| C.1 SLA/PTI validation | Unlocks or downgrades the highest-profile reliability claims | Needs NPMRDS/FPM or validated queueing evidence |
| T1/T1 failure evidence | Strengthens diamond recovery and resilience claims | 14 of 15 T1/T1 sites are still source-needed |
| Donner loaded-stressor sensitivity | Converts a no-delta fixture into a real intervention test | Needs loaded queue and alternate-capacity sensitivity |
| Managed-lane pilot evidence | Tests a high-blast-radius conditional expansion package | Needs PTI baseline, demand/toll/merge, mitigation, ROW, lifecycle, and exposure evidence |
| Owner/human game acceptance | Unblocks G0-C for Des Moines or Donner demo promotion | Needs non-author blind playtest or owner acceptance |

Recommended first target: **T1/T1 failure evidence**. It has an existing source plan, normalized event schema, one empirical seed site, and direct links to Blueprint diamond holds without requiring immediate paid NPMRDS/FPM access.

## Done Criteria

Milepost 8 is done when the selected hold has:

- a target decision record,
- source access or an explicit source blocker,
- normalized evidence rows or a documented failed acquisition,
- updated pressure/Blueprint/release references,
- a review record deciding promotion, continued hold, or downgrade,
- a passing gate bundle.

## Checklist

| Slice | Task | Status | Exit Gate / Artifact |
|---|---|---|---|
| B8-A | Select one target hold and write the decision rationale | ✅ done | `docs/evidence-campaigns/milepost-8-target.md` |
| B8-B | Create source acquisition checklist for the target | ✅ done | `data/evidence-campaign-source-plan.csv` |
| B8-C | Run or document source access attempt | ✅ done | `docs/evidence-campaigns/milepost-8-source-attempt.md` |
| B8-D | Normalize observations into an evidence ledger | ✅ done | `data/t1-failure-events.csv`; Iowa snapshot rows remain normalized and INDOT zero-row blocker is documented |
| B8-E | Update pressure and Blueprint claim references | ✅ done | `data/t1-intersection-failures.csv`; `data/blueprint-evidence-map.csv`; `data/release-manifest.csv` |
| B8-F | Attach review record | ✅ done | `docs/reviews/milepost-8-t1-failure-evidence-review.md` continues the hold |
| B8-G | Run release gate bundle | ✅ done | `powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1` |
| B8-H | Write Milepost 8 closeout | ✅ done | `docs/milepost-8-closeout.md` |

## Promotion Rule

Evidence acquisition alone does not promote a claim. Promotion requires source rows, normalized observations, bounded interpretation, and a review record. If the evidence is too weak, Milepost 8 should improve the hold rather than force a pass.
