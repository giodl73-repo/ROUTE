---
paper: C.4+empty-backhaul-relay
title: "Empty Miles and Load Matching: The Relay Marketplace as National Freight Optimizer"
round: 1
date: 2026-05-08
stage: revision
---

## Headline Assessment

The panel finds a paper that is closer to publication-ready than C.3 — the economic methodology is more disciplined, the mechanism is rigorously formulated, and the $135B headline is transparently derived from operating cost data. Two blocking items prevent immediate promotion: the Hungarian algorithm's O(n³) complexity is in tension with the hub-scale input sizes described in Section 3.3, and the 35% baseline empty-mile figure lacks cross-validation beyond ATRI self-reported surveys. Five important items require attention in the same revision pass. The paper is positioned to reach 3.5+/4 after a single targeted revision.

---

## Earned Stakes

**E1 — The relay hub information advantage over spot brokers is real and correctly specified.** All five reviewers accept the core mechanism: 4–8 hour advance arrival scheduling enables bipartite pre-matching that reactive broker markets cannot achieve. The distinction between reactive matching (truck arrives, driver calls a board) and predictive matching (hub scheduler knows arrival 4–8 hours in advance) is correctly framed as an information timing advantage, not an algorithmic one.

**E2 — The bipartite matching formulation is technically correct.** The four feasibility constraints (trailer compatibility, timing, HOS, deadline) and the three-term objective function (loaded miles, home-base alignment, timing match) are correctly specified. McKinnon and Adamic, the most technically engaged reviewers, both accept the formulation as sound. Adamic's concern is the solver choice, not the formulation itself.

**E3 — The $135B efficiency gain is transparently decomposed.** The operating cost basis (ATRI $1.609/mile × 45B empty miles reduced = $72.4B cost avoidance) plus revenue recovery ($1.40/mile × 45B newly loaded miles = $63.0B) equals $135.4B. The arithmetic checks out and the sources are correctly cited. All reviewers accept the arithmetic; Neumark's concerns are about marginal analysis and baseline validity, not about the decomposition itself.

**E4 — The managed-lane capital cost comparison is the paper's strongest policy finding.** The relay hub load-matching function generates $113–135B/year in efficiency gains — exceeding the $121B managed-lane capital cost in the first year of full deployment. This comparison is accepted by all reviewers and belongs in the abstract. It is the finding most likely to influence policy.

---

## Contested Stakes

| Stake | McKinnon | Adamic | Neumark | Puentes | Hanson | Resolution |
|---|---|---|---|---|---|---|
| Hungarian O(n³) scales to hub input | Accepts | No (blocking) | Not raised | Not raised | Not raised | **Contested: algorithm must scale or be replaced** |
| 35% baseline is valid | Needs cross-validation | Not raised | No (needs cross-validation) | Not raised | Not raised | **Contested: FAF5 cross-check required** |
| 20% target is achievable | Partially (UPS gap needs explanation) | Not raised | Partially (marginal analysis absent) | Not raised | Not raised | **Contested: structural floor derivation needed** |
| Rural corridors benefit proportionally | Not raised | Not raised | Not raised | Not raised | No (rural load density limits benefit) | **Contested: rural limitation must be stated** |
| FMCSA data sharing is achievable | Not raised | Not raised | Not raised | No (authority unclear) | Not raised | **Contested: regulatory authority needed** |

---

## P1 Blocking Items

**P1.1 — Algorithm scalability at hub scale.** Section 3.2 claims Hungarian O(n³) "takes under 1 second for n=2,000 on commodity hardware." Section 3.3 reports Chicago peak arrivals of 340 trucks/hour; at a 4–8 hour matching window, the problem instance is 1,360–2,720 trucks per cycle at peak. With load availability of 350–470 loads/hour over the same window, the effective n = min(2,720, 3,760) ≈ 2,720 at peak. At O(2,720³), the runtime claim requires verification. The paper must either: (a) replace Hungarian with a faster algorithm (auction algorithm, approximately O(n² log n); or greedy matching with priority queue, O(n log n)) and provide a complexity analysis showing the match completes within the 5–10 minute latency target at hub scale; or (b) provide an explicit benchmark on specified hardware confirming the Hungarian runtime claim at n=2,720. Adamic identifies this as the blocking concern.

**P1.2 — Baseline validation with FAF5 cross-check.** The 35% national empty-mile figure from ATRI is self-reported carrier survey data. The paper must add a cross-validation using FAF5 O-D flow data: if corridor X shows Y% more freight tonnage moving eastbound than westbound, the structural empty rate on the westbound leg is bounded from below by that flow imbalance. This cross-check does not need to produce a revised estimate — it needs to show the 35% ATRI figure is consistent with or bounded by the structural flow data. McKinnon and Neumark both flag this as a blocking concern.

---

## P2 Important Items

**P2.1 — The 20% target needs structural floor derivation.** The paper claims relay hub pre-matching reduces the national empty rate from 35% to ~20%. The 15pp reduction is presented as the gap between the current rate and the UPS/FedEx 8% benchmark plus the structural imbalance floor. The paper should explicitly derive the structural imbalance floor: for a given corridor flow imbalance ratio (from Table 1), what is the minimum achievable empty rate even with perfect information and perfect matching? Show that this structural floor, averaged across all T1/T2 corridors, is approximately 18–22%. This derivation makes the 20% target analytically grounded rather than interpolated between the ATRI 35% and the UPS 8%.

**P2.2 — Marginal analysis of empty-mile reduction.** The last 5 percentage points of reduction (from ~25% to 20%) require structural rebalancing across carrier network silos, not just information-friction elimination. The paper should note that the marginal cost of the last 5pp is higher than the marginal cost of the first 10pp, and estimate whether the efficiency gain from the last 5pp justifies the structural intervention required (cross-carrier data sharing, regulatory mandates). Neumark flags this.

**P2.3 — Rural corridor load density limitation.** The paper should explicitly note that the relay hub load-matching function performs better on high-density urban corridor hubs (Chicago, Atlanta, Dallas) than on rural agricultural corridor hubs (Fargo, Bismarck) because catchment load density drives match rate. For rural corridors where empty rates are structurally highest, the relay hub's primary value may be driver welfare (relay drivers sleep at home) rather than load-matching efficiency. This limitation does not undermine the national $135B figure (which is dominated by high-volume urban corridors) but must be stated clearly. Hanson flags this.

**P2.4 — Complete the top-20 corridor analysis.** The introduction claims to rank the top 20 T1/T2 corridors; Table 3 shows only 10. Either add the remaining 10 corridors to Table 3 or revise the introduction to match the presented analysis. Hanson flags the discrepancy.

**P2.5 — FMCSA data sharing authority.** Section 7 raises carrier data sharing as a prerequisite for relay hub pre-matching but does not analyze FMCSA's existing authority to mandate it. Add a paragraph on whether 49 CFR Part 390 or ELD mandate authority (49 CFR Part 395) provides a legal hook for requiring hub data sharing, or whether new legislation is required. Puentes flags this.

---

## Score Summary

| Reviewer | Affiliation | Score | Primary Concern |
|---|---|---|---|
| Alan McKinnon | Kuehne Logistics University (freight-economist) | 3/4 | 20% target gap; $135B headline needs abstract clarity |
| Lada Adamic | Meta / U Michigan (network-scientist) | 2/4 | Algorithm scalability at hub scale (blocking) |
| David Neumark | UC Irvine (rural-economist) | 3/4 | Baseline validation; marginal analysis absent |
| Robert Puentes | Eno Center (transport-policy) | 3/4 | FMCSA data sharing authority unclear |
| Susan Hanson | Clark University (transport-geographer) | 3/4 | Rural corridor limitation; top-20 claim unfulfilled |
| **Average** | | **2.8/4** | |
| **Minimum** | | **2/4** | |

---

## Next Steps

1. Address P1.1 (algorithm scalability) first — requires the most technical work; either switch to auction algorithm with complexity proof, or benchmark Hungarian at n=2,720 on specified hardware.
2. Address P1.2 (FAF5 cross-validation of 35% baseline) alongside P1.1 — the FAF5 O-D data needed for baseline validation are also needed for P2.1 (structural floor derivation).
3. Address P2.1 (structural floor derivation) as a natural extension of the P1.2 FAF5 analysis.
4. Address P2.4 (complete top-20 table) — mechanical extension of the existing corridor analysis.
5. Address P2.2, P2.3, P2.5 in a second pass (marginal analysis, rural limitation paragraph, FMCSA authority citation).
6. Return to panel for Round 2 review targeting 3.5/4 average.
