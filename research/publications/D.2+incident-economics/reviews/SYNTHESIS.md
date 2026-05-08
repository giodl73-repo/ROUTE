---
paper: D.2+incident-economics
title: "The Economics of Corridor Closures: Freight Cost and Redundancy Value"
venue: Transportation Research Part E
round: 1
date: 2026-05-07
synthesizer: panel-review
---

## Headline Assessment

The paper presents a well-structured closure cost model with a genuinely novel B1 isolation penalty multiplier. The redundancy value finding for Donner ($1.9B/yr) is the paper's most policy-relevant result. However, the $6.2B aggregate estimate is a point estimate without confidence interval, and the lognormal closure duration assumption is not validated against empirical data. Neumark scores 2/4 on structural grounds (no sensitivity analysis, no-build counterfactual absent); the other four reviewers find the paper acceptable for revision at 3/4. The paper is publishable at *Transportation Research Part E* after addressing the uncertainty quantification gap.

---

## Earned Stakes (E1–E4)

**E1 — The B1 penalty multiplier (1 + detour_miles/100) is the paper's most original and defensible contribution.**
All five reviewers accept the B1 framework. McKinnon validates the economic logic; Elefteriadou validates the traffic assignment logic; Puentes finds it operationally intuitive. Chester pushes on the compound-closure case (when the alternate is also degraded), which is a refinement, not a rejection.

**E2 — Donner ($2.4B/yr, B1=8.3) costs 4.2× more per closure event than Dallas ($0.8B/yr, B1=5.9) despite lower peak V/C.**
All reviewers accept this as the paper's headline finding. The isolation effect dominates volume effects for sufficiently isolated corridors. This result is the primary policy contribution.

**E3 — The redundancy value concept ($1.9B/yr for Donner with I-70W alternate) provides a tractable NPV floor for redundancy investment appraisal.**
Chester, Puentes, and McKinnon all accept the framework and find it policy-relevant. Chester's concern (what fraction of Donner closures affect both I-80 and the alternate?) would reduce the estimate if compound closures are dominant — this is a P2 item, not a structural rejection.

**E4 — The top-5 corridor ranking by annual closure cost is credible for ordinal ranking purposes.**
Reviewers accept the ordinal ranking even while disputing the precision of the cardinal estimates. Donner being highest-cost is not in doubt; the $2.4B number is uncertain but directionally correct.

---

## Contested Stakes

| Stake | Proponent | Opposition | Status |
|---|---|---|---|
| $6.2B aggregate point estimate is a reliable central estimate | Paper | Neumark (no confidence interval, no sensitivity analysis), McKinnon (ATRI unit cost uncertainty ±20%), Elefteriadou (ADV during closure overstates stranded volume) | **Not established** — point estimate without uncertainty bounds is insufficient for TR Part E |
| Lognormal closure duration distribution is appropriate | Paper | Neumark (not validated), Elefteriadou (event-type dependent; disaster tail is non-lognormal) | **Contested** — assumption is named but not tested; should be validated or sensitivity-bounded |
| B1 multiplier correctly captures urban alternate route cost | Paper | Elefteriadou (secondary congestion on urban alternates is nonlinear; B1 may understate urban corridor costs) | **Contested** — linear detour-miles formulation is a first-order approximation that may undercount urban corridor closure costs |
| Redundancy value $1.9B/yr is accurate for Donner | Paper | Chester (compound event correlation between I-80 and I-70W closures could significantly reduce this), McKinnon (closure frequency variance is large) | **Contested** — directionally correct; point estimate requires compound closure correlation analysis |
| Average daily volume is appropriate for stranded traffic calculation | Paper | Elefteriadou (weather closures suppress demand; ADV overstates volume at closure point) | **Contested** — demand suppression adjustment needed for weather-dominated corridors |

---

## P1 Blocking Items

**P1.1 — Add sensitivity analysis on the three key model parameters.**
A two-way sensitivity table (at minimum: ATRI unit cost ×0.80/1.00/1.20 × closure frequency ×0.70/1.00/1.30) showing the range of the top-5 annual cost estimates is required. The $6.2B aggregate should be presented as a range (e.g., $4.8B–$8.1B), not a point estimate. Without uncertainty bounds, the paper cannot be accepted at *Transportation Research Part E*.

**P1.2 — Validate or sensitivity-bound the lognormal closure duration assumption.**
Report the empirical closure duration distribution for at least Donner (20+ years of Caltrans/WSDOT data available) and at least one urban corridor. If lognormal is a reasonable fit, show the fit statistics. If not, use the empirical distribution or fit a more appropriate parametric form. Quantify the sensitivity of E[cost] to the duration distribution assumption.

**P1.3 — Apply demand suppression adjustment for weather-related closure corridors.**
For corridors where closures are primarily weather-induced (Donner, Snoqualmie, Oklahoma I-35), apply a demand reduction factor to the stranded volume calculation. Report the factor used and its source. This will reduce estimated costs for these corridors; report the revised estimates alongside the unadjusted baseline.

---

## P2 Important Items

**P2.1 — Analyze compound closure correlation for Donner redundancy value.**
What fraction of Donner's historical closure events coincide with closure or capacity reduction on the primary alternate (CA-50, or I-15/I-70 for cross-country rerouting)? If atmospheric river events — which are the dominant tail-risk events — simultaneously close both routes, the redundancy value estimate needs adjustment.

**P2.2 — Add the no-build counterfactual trend.**
Report closure frequency and estimated cost trends over the available historical window (at minimum 10 years). If closure frequency on Donner is increasing (atmospheric rivers are projected to intensify), the $2.4B estimate is a current-conditions baseline; the forward projection matters for investment appraisal.

**P2.3 — Differentiate policy recommendations by cost driver type.**
Separate the isolation-dominated corridors (Donner, rural) from volume-dominated corridors (Dallas, Baltimore). The policy response differs: redundancy investment for isolation-dominated; managed lanes / interchange redesign for volume-dominated. A single "PROTECT/NHPP gap" framing does not fit both types.

**P2.4 — Quantify FHWA rural incident undercount bias.**
The paper acknowledges the undercount but does not estimate its magnitude. Use available literature (e.g., Blincoe et al. on FAR vs. HPMS incident data) to estimate the directional bias for the rural corridors and report whether this makes the rural corridor estimates conservative or liberal.

---

## Score Summary

| Reviewer | Affiliation | Score |
|---|---|---|
| Alan McKinnon | Kühne Logistics University | 3/4 |
| David Neumark | UC Irvine | 2/4 |
| Mikhail Chester | ASU, transport resilience | 3/4 |
| Robert Puentes | Eno Center, transport policy | 3/4 |
| Lily Elefteriadou | University of Florida, traffic engineering | 3/4 |
| **Average** | | **2.8/4** |
| **Minimum** | | **2/4** |

---

## Next Steps

1. Authors add two-way sensitivity analysis table (P1.1) — required before resubmission.
2. Authors validate or bound the lognormal duration assumption (P1.2).
3. Authors apply demand suppression adjustment for weather corridors (P1.3).
4. Authors run compound closure correlation analysis for Donner (P2.1).
5. Re-review by Neumark after P1 revisions; Elefteriadou to confirm traffic modeling revisions.
