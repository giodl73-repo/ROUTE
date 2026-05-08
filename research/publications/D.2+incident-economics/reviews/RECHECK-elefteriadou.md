---
reviewer: Elefteriadou
paper: D.2+incident-economics
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked:
  - PP1.2
verdict: PASS
score: 3/4
---
> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.2 — Lognormal duration assumption unvalidated

**Concern**: The incident duration distribution was assumed lognormal without empirical validation. The distributional assumption is load-bearing for the cost model: if duration is heavier-tailed than lognormal, expected cost is materially higher. I required either (a) a goodness-of-fit test against real incident data, or (b) a clearly labeled fallback distribution with a stated sensitivity.

**Revision**: Two validation datasets added. Caltrans incident log (n=612, Shapiro-Wilk p=0.23 after log-transform) and FHWA Dallas corridor data (n=1,847, p=0.11 after log-transform) both fail to reject lognormality at the 5% level. The Gulf Coast triangular fallback is retained as a sensitivity check with a stated <12% difference in expected cost.

**Verdict**: PASS. The Caltrans n=612 with p=0.23 is reassuring; the FHWA Dallas sample at n=1,847 with p=0.11 further strengthens the case. The fallback comparison (<12% difference) shows the cost model is not sensitive to moderate distributional misspecification. My objection is resolved. Score holds at 3/4 — the original Round 1 score was already appropriate for a paper with a validated core methodology; this recheck confirms it.

**P3 note**: The datasets support lognormality in the central range, but I recommend also testing for heavy-tailed alternatives (e.g., Pareto) for the extreme-duration events, which drive a disproportionate share of expected cost in these models. This is a recommendation for a future revision, not a blocking condition.

## Verdict

Both validation datasets support the lognormal assumption and the triangular fallback bounds the sensitivity. The distributional concern is resolved; score confirmed at 3/4 and paper is ready to advance.
