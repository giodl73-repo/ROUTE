---
reviewer: Neumark
paper: E.1+managed-lanes
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked:
  - PP1.2
verdict: PASS-WITH-NOTE
score: 3/4
---
> AI-generated simulated recheck. Not an actual review.

## Items Rechecked

### PP1.2 — Freight demand growth rate unsourced; no NPV sensitivity table

**Concern**: The 1.8%/yr freight demand growth rate was stated without citation. FAF4/FAF5 figures are publicly available and should have been named. Additionally, the NPV of $115B (now revised to $101B) was presented as a point estimate; demand growth rate is a primary driver of NPV in a 30-year horizon model and the paper must show how the result varies across a plausible range.

**What was done**: FAF4/FAF5 citation added for 1.8%/yr. A 3×3 NPV sensitivity table added in §05 crossing capacity assumption (low corridor-specific / central 2,108 pcphpl / high 2,400 pcphpl original) against freight demand growth (1.5%/yr, 1.8%/yr, 2.4%/yr). The revised central estimate of $101B corresponds to the low-capacity/central-growth cell; the high-capacity/central-growth cell yields $115B. The range across the full table is $81B–$158B.

**Is it satisfactory?** Yes. The FAF4/FAF5 citation resolves the sourcing objection. The sensitivity table is well-structured and honestly shows both the pessimistic floor ($81B, B/C 1.7:1) and the optimistic ceiling ($158B, B/C 3.1:1). The $81B floor is particularly important — it shows the pessimistic case remains NPV-positive at any standard public discount rate.

**On the post-write correction**: The abstract and portfolio table are now internally consistent: 2.0:1 B/C, $101B NPV, $11.2B/yr annual benefit. These are the figures I would expect to see given the capacity correction from 2,400 to 2,108 pcphpl. No new methodology concerns arise from the correction itself.

**P3 note**: The E.2 paper now cites E.1 figures ($11.2B/yr, 2.0:1) in its Component 1 description. The propagation from E.1 to E.2 should be verified to be complete — if any E.2 section still references the old $12.7B/yr figure, that creates an internal inconsistency across the Track E papers. This is a cross-paper hygiene note, not a blocking condition for E.1 itself.

## Verdict

The correction is correct and the updated figures are consistent across abstract and portfolio table. The FAF4/FAF5 citation resolves the sourcing objection; the 3×3 sensitivity table shows the NPV range honestly. Score confirmed at 3/4; paper is ready to advance.
