---
paper: B.4+t1-intersection-resilience
round: 1
date: 2026-05-07
stage: revision
---

# Panel Synthesis — Round 1
## T1/T1 Intersection Resilience: Diamond Interchange Zones and k-Connectivity in the National Highway Network

---

## Headline Assessment

The paper makes a valuable and original contribution: the systematic identification of k-connectivity class across all 15 T1/T1 intersections in the national highway graph, combined with a structured investment portfolio anchored by a novel design concept (diamond interchange zone). The panel finds the core argument compelling and the NPV analysis credibly structured. However, the paper has one **blocking methodological gap** — unvalidated k-connectivity results due to TIGER/Line junction snapping artifacts — and three important secondary gaps: incomplete ramp design specification, absence of climate hazard analysis, and implementation feasibility limitations. Revision to address the blocking gap is required before the paper can advance.

---

## Earned Stakes

**E1 — k-connectivity framing is the right analytical unit for junction resilience.**
All five reviewers accepted the premise that graph-theoretic k-connectivity is the appropriate lens for identifying single points of failure in the national freight network. No reviewer proposed an alternative primary metric. The B2_product prioritization was accepted as internally consistent by Adamic, McKinnon, and Puentes (with caveats on freight value validation).

**E2 — The diamond interchange zone concept addresses a real structural problem.**
Elefteriadou confirmed that conventional interchange geometry does not address k-connectivity in the graph-theoretic sense, and that the 50-mile access-controlled zone concept is a meaningful engineering upgrade. Chester noted that the zone distributes the interchange function in a way that could also improve physical resilience if properly sited.

**E3 — The NPV model and B/C ratio are within defensible range.**
McKinnon accepted the portfolio NPV structure as sound, finding the 2.76:1 B/C ratio consistent with comparable interchange improvement studies. Puentes confirmed the ratio clears informal INFRA competitiveness thresholds. Both noted that specific assumptions (VoT, closure probability) need to be made explicit.

**E4 — The portfolio framing (15 intersections, national program) is the right funding unit.**
Puentes explicitly endorsed the national program structure as well-suited to IIJA's NHFP and INFRA mechanisms. The panel consensus is that evaluating each intersection independently would understate the case for investment.

---

## Contested Stakes

| Stake | Proponent | Opponent | Status |
|---|---|---|---|
| k=1 classifications are valid as reported | Authors (implied) | Adamic (hard) | **Unresolved — P1 blocking** |
| 65-mph ramp design speed is achievable at all three priority sites | Authors (implied) | Elefteriadou (firm) | **Unresolved — P2** |
| Climate exposure does not materially alter NPV | Authors (implicit) | Chester (firm) | **Unresolved — P2** |
| B2_product tracks freight economic value adequately | Authors (implied) | McKinnon (firm) | **Unresolved — P2** |
| Implementation timeline is consistent with stated cost estimates | Authors (implicit) | Puentes (firm) | **Unresolved — P2** |

---

## P1 — Blocking Items

**P1.1 — Manual validation of k-connectivity for top-5 intersections.**
Adamic requires either: (a) manual validation of k-classification for the five highest-B2_product intersections against HPMS geometry, state DOT interchange schematics, or aerial imagery, with documented methodology; or (b) a sensitivity analysis demonstrating k=1 classification is stable across TIGER snapping tolerances from ±10m to ±100m. The paper must identify which two intersections are unresolved and confirm whether they appear in the top-5 priority ranking. Without this, the core result — 9 of 15 intersections are k=1 — cannot be published with confidence.

---

## P2 — Important Items

**P2.1 — Ramp geometry worked example.**
Elefteriadou requires at least one worked example of ramp design at a representative priority site (Atlanta I-75/I-85 is the most complex). The example should reference AASHTO Green Book design speed criteria, state ramp length and sight distance assumptions, and justify the ramp configuration (directional, semi-directional, or loop) used in the cost estimate.

**P2.2 — Cost breakdown by component.**
Elefteriadou requires a cost breakdown separating roadway construction, ROW acquisition, and WIM/enforcement infrastructure at each priority site. The current single-line cost ($210M–$380M) is insufficient for peer review.

**P2.3 — Climate hazard cross-reference.**
Chester requires a cross-reference of all 15 intersection sites against FEMA FIRM maps and NOAA sea-level-rise projections (2050 mid-range scenario). For Jacksonville specifically, the paper should quantify expected physical disruption days per decade under the SLR scenario and show the impact on NPV.

**P2.4 — FAF5 validation of B2_product rankings.**
McKinnon requires a comparison of the B2_product rankings against FAF5 flow data at the junction level for the three priority sites. If FAF5 data confirms the freight value implied by the NPV model, state that explicitly. If it diverges, explain the discrepancy.

**P2.5 — Closure probability empirical anchor.**
McKinnon requires a stated annual closure probability per intersection — from incident, maintenance, or weather — derived from FHWA incident data or comparable empirical source. This is the dominant term in the NPV reliability benefit calculation.

**P2.6 — EIS timeline and STIP status for priority sites.**
Puentes requires an assessment of the current EIS status, prior NEPA documentation, and state DOT capital program position for each of the three priority sites. The phasing plan should incorporate realistic EIS timelines (minimum 4–6 years for major interchange reconstruction).

**P2.7 — Directed k-connectivity analysis.**
Adamic requires a brief discussion of directed versus undirected k-connectivity and whether the classification of the 15 intersections would change under a directed formulation. If the paper uses undirected k-connectivity for computational tractability, that limitation should be explicitly stated and bounded.

---

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Lada Adamic (network-scientist) | 2/4 | TIGER snapping → k-connectivity validity |
| Lily Elefteriadou (traffic-engineer) | 3/4 | Ramp geometry underspecified; cost breakdown missing |
| Mikhail Chester (transport-resilience) | 3/4 | Climate hazard absent from NPV and site analysis |
| Alan McKinnon (freight-economist) | 3/4 | VoT and closure probability unstated; B2_product unvalidated |
| Robert Puentes (transport-policy) | 3/4 | EIS timeline and STIP feasibility not addressed |
| **Panel mean** | **2.8/4** | |

---

## Next Steps

1. **Authors** address P1.1 (k-connectivity validation) before any other revision — this is the gate.
2. After P1.1 is resolved, address P2.1–P2.7 in revision.
3. Return to panel for round 2 review (targeted: Adamic + Elefteriadou as primary reviewers).
4. SYNTHESIS-R2 will be produced after round 2 reviews are received.
