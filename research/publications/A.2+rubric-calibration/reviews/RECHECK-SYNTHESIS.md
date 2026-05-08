---
paper: A.2+rubric-calibration
round: 1
recheck_date: 2026-05-07
recheck_reviewers: [neumark]
recheck_verdicts: {neumark: PASS}
scores_before: {adamic: 3, elefteriadou: 3, mckinnon: 3, neumark: 2, hanson: 3}
scores_after: {adamic: 3, elefteriadou: 3, mckinnon: 3, neumark: 3, hanson: 3}
avg_before: 2.8
avg_after: 3.0
min_before: 2
min_after: 3
stage: ready
---

# Recheck Synthesis — A.2+rubric-calibration Round 1

## Result

Neumark recheck passes. **Paper advances to `ready`.**

| Reviewer | Round 1 | Recheck | Change |
|---|---|---|---|
| David Neumark (Freight Economist) | 2/4 | PASS — 3/4 | +1 |
| Lada Adamic | 3/4 | not rechecked | held |
| Lily Elefteriadou | 3/4 | not rechecked | held |
| Alan McKinnon | 3/4 | not rechecked | held |
| Susan Hanson | 3/4 | not rechecked | held |
| **Mean** | **2.8/4** | **3.0/4** | **+0.2** |
| **Min** | **2/4** | **3/4** | **+1** |

## What the Revision Resolved

**PP1.1 — External validation (Neumark).** A full "External Validation" subsection has been added to Section 6 with three external validators: STRAHNET alignment (ρ=0.81 across 227 corridors), ATRI bottleneck cost density (ρ=0.72 for T1 classification), and transportation planning document frequency (47/50 state long-range plans, 94%). An anchor stability test is also present: ±25% perturbation of all 10th/90th percentile anchor values produces no T1 tier changes. The paper moves from internally consistent to externally validated.

**PP1.2 — B2-conditioned independence caveat (Adamic, not rechecked).** The caveat was added in the same revision pass. Adamic accepted P1.2 was addressed without requiring a formal recheck; the conditional language in Section 6.3 makes explicit that the r values are computed against the partial 31-state B2 and require revalidation when the full-graph computation is available.

**PP1.3 — BPR-to-PTI path (Elefteriadou, not rechecked).** The paragraph in Section 3.2 explaining why IRI was used rather than BPR-estimated V/C was added; Elefteriadou accepted the response without formal recheck (HPMS V/C data unavailable for WY, NV rural segments was the stated reason).

## Carried Notes (not blocking)

- **Neumark P3:** ρ=0.81 STRAHNET correlation should report a 95% confidence interval (Fisher z-transform; n=227 gives approximately ±0.04, which is tight but should be stated explicitly).

## P2 Items Status

P2.1 (corpus construction), P2.2 (anchor bootstrap), P2.3 (B4 two-component documentation), P2.4 (C4 hand-curation appendix), and P2.5 (geographic coverage analysis) were addressed in the same revision pass and accepted by McKinnon, Hanson, and Adamic without requiring formal recheck rounds.

## Stage Advancement

Paper moves from `revision` to `ready`. No further review required before venue submission.
