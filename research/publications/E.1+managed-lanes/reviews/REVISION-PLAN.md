---
paper: E.1+managed-lanes
title: "Managed Freight Lanes: Throughput, Transit Time, and NPV"
round: 1
date: 2026-05-07
---

# Revision Plan — Round 1

## P1 Blocking Items (must resolve before re-review)

- [ ] **P1.1** Replace uniform 2,400 pcphpl with corridor-by-corridor HCM7 managed lane capacity estimates.
  - [ ] For each of the 7 corridors, identify access point density (interchanges per mile) from HPMS.
  - [ ] Apply HCM7 Chapter 13/14 merge/diverge/weave adjustment for access-point frequency.
  - [ ] Apply HCM7 grade correction for mountain corridors: I-80 Donner, I-70 Vail Pass, I-90 Snoqualmie (report grade %, grade length, and correction factor used).
  - [ ] Verify truck PCE application: start from mixed-traffic freeway capacity (not ideal-conditions base) when computing truck throughput in vehicles/hour.
  - [ ] Produce corridor-level capacity table: corridor name, access points/mile, dominant grade %, HCM7 adjusted capacity (pcphpl), adjusted truck throughput (trucks/lane/hr), revised vpd per corridor.
  - [ ] Update 57,600 vpd/corridor figure to corridor-specific estimates with aggregate range (low/central/high).
  - [ ] Propagate revised capacity estimates through NPV calculation; report revised $121B program cost and $115B NPV if capacity assumptions change cost or benefit estimates.

- [ ] **P1.2** Source freight demand growth rate and add NPV sensitivity table.
  - [ ] Cite FAF4 reference scenario freight growth rate for the 30-year appraisal horizon (or equivalent FHWA source); report figure, scenario name, and publication.
  - [ ] Cite FHWA VMT projections for the 2.4%/yr passenger growth rate assumption; report source and scenario.
  - [ ] Produce two-axis sensitivity table: rows = freight demand growth (1.5%/yr, 1.8%/yr, 2.4%/yr); columns = managed lane capacity scenario (low, central, high per P1.1); cells = aggregate NPV and B/C ratio.
  - [ ] Flag any cell where B/C falls below 2.0 and note what program modification would restore positive economics.

- [ ] **P1.3** Specify tolling model and model demand uptake.
  - [ ] State explicitly: is $0.05/mile access fee mandatory (all trucks on managed lanes pay) or voluntary (trucks can choose GP lanes at no fee)?
  - [ ] If mandatory: cite statutory authority under 23 USC 129 or alternative; note regulatory/legislative pathway.
  - [ ] If voluntary: model demand uptake using freight value-of-time segmentation. Minimum: segment by (a) time-sensitive freight (refrigerated, automotive JIT, parcel); (b) time-insensitive freight (bulk, grain, construction materials). Estimate uptake fraction for each segment at $0.05/mile premium. Report toll revenue under low (40% uptake), central (65% uptake), and high (85% uptake) scenarios.
  - [ ] Reconcile toll revenue arithmetic: show VMT calculation (vpd × corridors × avg corridor length × utilization fraction × 365 × $0.05) and confirm $2.3B/yr or revise.

## P2 Important Items (strongly recommended before submission)

- [ ] **P2.1** Restructure platooning benefit as scenario, not point estimate. Report savings at 15% market penetration (central case, consistent with 2026 commercial deployment trajectory) and 30% (high scenario). State the year by which 30% penetration is assumed and what deployment trajectory this implies. Revise abstract/conclusion to report the central-case platooning benefit.
- [ ] **P2.2** Add decarbonization co-benefit section or appendix. Include: (a) order-of-magnitude estimate of stop-and-start fuel savings on managed lanes (reference NREL managed lane fuel consumption studies if available); (b) qualitative discussion of electrification infrastructure right-of-way value along managed lane corridors. Flag that this is additive to the transportation NPV.
- [ ] **P2.3** Compute GP lane expansion counterfactual B/C. For at least 3 of the 7 corridors, estimate the B/C of standard GP lane expansion (same lane-miles, no access control) and compare to managed freight lane B/C. Report the improvement over the best alternative in the policy implications section.
- [ ] **P2.4** Use peak-hour truck volume fractions for GP lane LOS analysis. For I-75 Atlanta and any other corridor where V/C improvement is cited, replace daily average truck fraction with peak-hour truck fraction from HPMS or ATR data. Report revised V/C improvement estimates.
- [ ] **P2.5** Add modal shift induced demand sensitivity. Estimate the impact on managed lane V/C trajectory if modal shift from rail to truck adds 0.3%/yr incremental freight demand above the baseline growth rate. Report the year at which managed lane V/C reaches 0.85 under this scenario, and whether a design modification (additional lane, access restriction tightening) would maintain PTI<1.15.
