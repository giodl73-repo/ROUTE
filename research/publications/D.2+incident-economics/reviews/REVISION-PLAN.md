---
paper: D.2+incident-economics
title: "The Economics of Corridor Closures: Freight Cost and Redundancy Value"
round: 1
date: 2026-05-07
---

# Revision Plan — Round 1

## P1 Blocking Items (must resolve before re-review)

- [ ] **P1.1** Add sensitivity analysis on key model parameters. Produce a two-way sensitivity table: rows = ATRI unit cost (×0.80, ×1.00, ×1.20); columns = closure frequency (×0.70, ×1.00, ×1.30). For each cell show: top-5 aggregate annual cost and Donner point estimate. Present the $6.2B as a range (e.g., "estimated $4.8B–$8.1B/yr") rather than a point estimate in the abstract and conclusion.
- [ ] **P1.2** Validate or sensitivity-bound the lognormal closure duration assumption.
  - [ ] Pull Caltrans/WSDOT closure duration records for Donner (minimum 10 years).
  - [ ] Pull FHWA or equivalent for one urban corridor (Dallas or Baltimore).
  - [ ] Fit lognormal distribution; report goodness-of-fit statistics (KS test, Anderson-Darling).
  - [ ] If lognormal does not fit, fit empirical distribution or report sensitivity of E[cost] to ±1 SD in mean log-duration.
- [ ] **P1.3** Apply demand suppression adjustment for weather-closure-dominated corridors (Donner, I-90 Snoqualmie, I-35 Oklahoma).
  - [ ] Identify or estimate demand reduction factor for severe weather closures (target: Caltrans traffic counts before/during closure events, or FHWA incident data with volume comparison).
  - [ ] Apply factor V_closure = ADV × (1 − demand_reduction); report revised cost estimates alongside unadjusted baseline.
  - [ ] Note directional effect: weather corridor costs likely decrease; urban corridor costs are unaffected.

## P2 Important Items (strongly recommended before submission)

- [ ] **P2.1** Analyze compound closure correlation for Donner redundancy value. For historical Donner closure events: determine fraction where CA-50 or I-15/I-70 reroute was also degraded (>30-min delay added). Revise $1.9B/yr redundancy value estimate if compound closure fraction is >15% of total closure hours.
- [ ] **P2.2** Add no-build counterfactual trend. Report closure frequency trend for Donner and at least two other corridors over the available historical window (minimum 10 years). If frequency is rising, project forward to 2040 cost estimate under status-quo operations.
- [ ] **P2.3** Differentiate policy recommendations by cost driver type. Add a typology table: isolation-dominated corridors (high B1, high per-event cost, long detour) vs. volume-dominated corridors (high ADV, high frequency). Specify appropriate intervention type for each: redundancy investment (isolation-dominated) vs. managed lanes / interchange redesign (volume-dominated). Revise PROTECT/NHPP framing to reflect this distinction.
- [ ] **P2.4** Quantify FHWA rural incident undercount bias. Cite available literature on rural vs. urban incident reporting completeness. Estimate whether rural corridor closure frequency (Donner, I-35 Oklahoma) is likely understated by 10–40% or more. Note whether this makes rural cost estimates conservative lower bounds.
