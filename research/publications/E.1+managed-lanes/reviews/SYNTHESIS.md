---
paper: E.1+managed-lanes
title: "Managed Freight Lanes: Throughput, Transit Time, and NPV"
venue: Transportation Research Part A
round: 1
date: 2026-05-07
synthesizer: panel-review
---

## Headline Assessment

The paper presents a conceptually sound and policy-ready case for managed freight lanes on high-V/C T1 corridors, with a $115B NPV estimate and a 2.3:1 aggregate B/C ratio. The core argument — freight-only lanes prevent induced passenger demand while adding effective freight capacity — is theoretically correct and well-motivated. However, the paper's quantitative claims depend on a 2,400 pcphpl managed lane capacity figure that does not account for access point geometry, grade effects, or truck PCE correctly, and on freight demand growth (1.8%/yr) and platooning penetration (30%) assumptions that are asserted rather than derived. Elefteriadou scores 2/4 on the capacity grounds; the other four reviewers find the paper acceptable for revision at 3/4. Revision is required before the paper is ready for *Transportation Research Part A*.

---

## Earned Stakes (E1–E4)

**E1 — Managed freight lanes prevent induced passenger demand while adding freight capacity.**
All five reviewers accept this as the paper's central conceptual contribution. The demand-segregation logic is theoretically sound and supported by the managed lane literature. This claim survives.

**E2 — The GP lane LOS improvement on high-V/C corridors (I-75 Atlanta: V/C 1.8→1.4) is directionally correct.**
Elefteriadou challenges the peak-hour freight fraction (22%) that drives this result; if the peak-hour truck fraction is 12–15% rather than 22%, the V/C reduction is 1.8→1.6 rather than 1.8→1.4. The direction of the finding is not in dispute; the magnitude requires verification with peak-hour HPMS truck volume data.

**E3 — The I-90 rural corridor is a marginal case (B/C 1.6:1) that should not be funded in Phase 1.**
All reviewers accept the corridor selection exclusion criterion (rural V/C<0.6, I-40) and the B/C range as an honest representation. This finding is the paper's most important concession and strengthens its analytical credibility.

**E4 — The transponder toll revenue mechanism ($0.05/mile via ELD infrastructure) is conceptually feasible.**
Puentes finds the ELD-based implementation plausible; McKinnon finds the funding level reasonable; neither rejects the mechanism. Puentes's concern (voluntary vs. mandatory pricing, and uptake rate) is a P1 item requiring clarification.

---

## Contested Stakes

| Stake | Proponent | Opposition | Status |
|---|---|---|---|
| 2,400 pcphpl managed lane capacity is achievable on T1 corridors | Paper | Elefteriadou (HCM7 ideal-condition figure; access points reduce 5–20%; grade corrections absent for mountain corridors) | **Not established** — requires corridor-by-corridor capacity analysis or explicit bounding |
| 57,600 vpd per corridor capacity addition is accurate | Paper | Elefteriadou (derived from 2,400 pcphpl; same access-point and grade issues apply) | **Not established** — upper-bound estimate, not corridor-representative |
| Freight demand grows at 1.8%/yr through 2056 (30-yr horizon) | Paper | Neumark (no source cited; needs FAF4 or FHWA VMT scenario reference), Adamic (modal shift induces freight demand above baseline) | **Contested** — directionally plausible but unsourced; induced freight demand channel unmodeled |
| 30% platooning market penetration by program opening year | Paper | McKinnon (optimistic; commercial deployment still in demonstration phase as of 2026; 15% more appropriate as central case) | **Contested** — should be presented as high scenario, not central estimate |
| $115B NPV and 2.3:1 B/C are robust to parameter variation | Paper | Neumark (no sensitivity analysis presented), Elefteriadou (capacity assumption alone could shift NPV substantially), McKinnon (platooning penetration ±15%) | **Not established** — all three independent sensitivity levers could shift NPV significantly; no bounds provided |
| $2.3B/yr toll revenue is achievable | Paper | Puentes (voluntary vs. mandatory pricing; uptake rate not modeled; demand segmentation by freight type absent), Adamic (time-sensitive freight uptake may be <100% of volume) | **Contested** — revenue depends on uptake rate that is unspecified |

---

## P1 Blocking Items

**P1.1 — Replace the uniform 2,400 pcphpl assumption with corridor-by-corridor capacity estimates.**
For each of the 7 corridors, apply HCM7 managed lane capacity methodology with: (a) access-point density correction (Chapter 13/14 merge/diverge/weave adjustment); (b) grade correction for mountain corridors (I-80 Donner, I-70 Vail Pass, I-90 Snoqualmie); (c) correct application of 2.0 truck PCE to derive truck throughput in truck vehicles per hour. Present a corridor-level capacity table. The aggregate 57,600 vpd/corridor figure should be replaced with corridor-specific estimates and an aggregate with range.

**P1.2 — Source the freight demand growth rate (1.8%/yr) and provide a sensitivity analysis on NPV.**
Cite FAF4 reference case freight growth projections (or equivalent FHWA source) for the appraisal horizon. Provide a two-axis sensitivity table: aggregate NPV as a function of (freight demand growth rate: 1.5%, 1.8%, 2.4%) × (managed lane capacity: low, central, high per P1.1 revisions). The B/C>2.0 finding is the paper's headline claim; it should survive at least the central-to-high capacity and 1.5%–1.8% freight growth cells.

**P1.3 — Specify the tolling model (voluntary vs. mandatory) and model the demand uptake response.**
State whether the $0.05/mile access fee is mandatory for all freight on managed lanes (requiring separate statutory authority) or voluntary (trucks can choose GP lanes). Under the voluntary model, estimate uptake using value of time segmentation by freight type (time-sensitive vs. time-insensitive). Report the range of toll revenue under low/central/high uptake assumptions.

---

## P2 Important Items

**P2.1 — Revise platooning penetration to a high/central/low scenario structure.**
Present platooning savings at 15% penetration (central), 30% penetration (high). Report the NPV contribution from platooning at each penetration level and note that the 15% central case is more consistent with current commercial deployment trajectory.

**P2.2 — Add the decarbonization co-benefit dimension.**
For *Transportation Research Part A*, add a section or appendix addressing: (a) stop-and-start fuel savings on managed lanes beyond platooning; (b) right-of-way value for electrification infrastructure along managed lane corridors. Provide an order-of-magnitude estimate of the decarbonization NPV, with appropriate uncertainty caveats.

**P2.3 — Compute B/C for the most plausible counterfactual (GP lane expansion).**
Calculate the B/C of standard GP lane expansion on the same 7 corridors and compare against the managed freight lane B/C. The improvement over the best alternative (rather than over do-nothing) is the more compelling policy framing.

**P2.4 — Use peak-hour truck volume fractions for the GP lane LOS analysis.**
For I-75 Atlanta (and other corridors where GP lane LOS improvement is cited), use HPMS peak-hour truck fraction rather than daily average truck fraction. If the peak-hour fraction is substantially lower than the daily average, revise the V/C improvement estimates accordingly.

**P2.5 — Reconcile toll revenue arithmetic.**
Show the calculation: implied annual managed-lane VMT from 57,600 vpd × 7 corridors × average corridor length × (fraction of capacity utilized) × 365 days × $0.05/mile. Confirm this produces the $2.3B/yr figure. If not, revise.

---

## Score Summary

| Reviewer | Affiliation | Score |
|---|---|---|
| Lily Elefteriadou | University of Florida, traffic engineering | 2/4 |
| Alan McKinnon | Kühne Logistics University | 3/4 |
| David Neumark | UC Irvine | 3/4 |
| Robert Puentes | Eno Center, transport policy | 3/4 |
| Lada Adamic | Meta AI Research / U Michigan | 3/4 |
| **Average** | | **2.8/4** |
| **Minimum** | | **2/4** |

---

## Next Steps

1. Authors run corridor-by-corridor HCM7 capacity analysis for all 7 corridors (P1.1) — this is the structural revision that changes all downstream numbers.
2. Authors source freight demand growth rate from FAF4 and add two-axis NPV sensitivity table (P1.2).
3. Authors specify voluntary vs. mandatory tolling model and model uptake (P1.3).
4. Re-review by Elefteriadou after P1.1 revision; Neumark and McKinnon to confirm NPV and platooning revisions (P1.2, P2.1).
