---
paper: C.2+national-max-flow
round: 1
date: 2026-05-07
stage: revision
---

# Panel Synthesis — Round 1
## National Max-Flow: Capacity and Bottleneck Analysis of the Interstate Network

---

## Headline Assessment

This paper makes a genuine methodological contribution — applying Edmonds-Karp max-flow to a 227-corridor national freight network with FAF5 demand data — and the bottleneck identification and closure simulation results are the paper's most valuable findings. The Donner+I-35 compound failure scenario, producing I-40 network failure (V/C 1.11), is an important resilience result. However, the single-commodity max-flow formulation is the paper's core limitation, and the panel's harshest reviewer (Adamic) argues persuasively that it may misidentify the binding constraints in the real freight network by treating all freight as fungible. The investment recommendations — particularly I-69 at marginally negative NPV — are not sufficiently grounded in the max-flow results alone. Revision is required to address the single-commodity limitation with a sensitivity analysis and to clarify the max-flow → investment recommendation causal chain.

---

## Earned Stakes

**E1 — National max-flow on the TIGER/HPMS graph is a valid and original contribution.**
All five reviewers accepted the graph construction methodology and the Edmonds-Karp implementation as technically sound. Elefteriadou raised questions about capacity value derivation but accepted the overall approach. Adamic affirmed the computational methodology.

**E2 — The three binding bottleneck arcs are correctly identified and cross-validated.**
The I-95 Baltimore-Washington, Donner Pass, and Dallas interchange bottlenecks are consistent with ATRI, NPMRDS, and HPMS data. All reviewers accepted these as real constraints; none proposed alternative primary bottlenecks.

**E3 — The compound failure simulation (Donner + I-35 → I-40 failure) is the paper's most important finding.**
All reviewers found the compound failure result compelling. Puentes explicitly noted its value for FHWA Emergency Relief and FEMA transportation planning. This result does not depend on the single-commodity limitation and is robust to the model's other caveats.

**E4 — I-70W's role as a Donner closure relief valve is well-specified.**
The result (Donner closure flow loss 23%→9% with I-70W) is internally consistent and provides a resilience argument for I-70W that is independent of baseline congestion relief. McKinnon and Puentes both accepted this finding.

---

## Contested Stakes

| Stake | Proponent | Opponent | Status |
|---|---|---|---|
| Single-commodity max-flow identifies the correct binding constraints | Authors (implied) | Adamic (hard) | **Unresolved — P1 blocking** |
| Investment recommendations follow from max-flow results | Authors | Neumark (firm), Puentes (firm) | **Unresolved — P2** |
| I-69 NPV is marginally negative → investment is not justified | Neumark/Puentes | Authors (implicitly recommend I-69) | **Contested — P2** |
| Edge capacity values accurately represent operational capacity | Authors (implied) | Elefteriadou (firm) | **Unresolved — P2** |
| Max-flow utilization rate supports investment case | Authors (implied) | Neumark (hard) | **Unresolved — P2** |

---

## P1 — Blocking Items

**P1.1 — Single-commodity sensitivity analysis or multi-commodity extension.**
Adamic requires either: (a) a sensitivity analysis that simulates modal substitution — e.g., removing 15–25% of Donner Pass highway demand to represent rail-eligible intermodal diversion — and showing how the bottleneck rankings change; or (b) a two-commodity max-flow formulation (highway freight vs. rail-eligible intermodal) for the Donner Pass corridor at minimum. The goal is to demonstrate that the binding bottleneck identification is robust to the single-commodity assumption, or to quantify how much the ranking changes when intermodal substitution is modeled. Without this, the paper's investment recommendations cannot be derived from the max-flow results with confidence.

---

## P2 — Important Items

**P2.1 — Capacity value derivation documentation.**
Elefteriadou requires a capacity appendix or supplemental table showing, for each of the three binding bottleneck arcs: (a) whether capacity values are from HPMS observed throughput or HCM design tables, (b) the assumed peak-hour factor, and (c) how directional asymmetry is handled. For Dallas interchange specifically, the capacity methodology for a complex multilevel urban interchange must be stated.

**P2.2 — Max-flow utilization ratio.**
Neumark requires the current utilization ratio for each major O-D cluster (actual FAF5 demand / max-flow capacity). This determines whether additional max-flow capacity relieves actual congestion or provides only resilience value — a critical distinction for investment prioritization.

**P2.3 — I-69 NPV sensitivity table.**
Puentes requires NPV computed at 3%, 5%, and 7% discount rates, crossed with low (1.5%/yr) and high (2.5%/yr) freight demand growth scenarios. Present the breakeven discount rate explicitly. The paper must reconcile the max-flow benefit (+18% Gulf→Chicago) with the NPV result and characterize the I-69 investment as a policy judgment conditioned on discount rate and demand growth, not as a max-flow conclusion.

**P2.4 — Commodity composition of investment recommendations.**
McKinnon requires the commodity composition (from FAF5) of the incremental flow gain for each investment recommendation. The economic value of the incremental flow — freight-ton-miles weighted by commodity value — should be presented alongside the max-flow gain in vehicle-equivalents.

**P2.5 — Distinction between congestion-binding and resilience-binding bottlenecks.**
Neumark requires the paper to explicitly distinguish between: (a) arcs where current demand approaches max-flow capacity (congestion bottlenecks, where investment relieves today's constraint), and (b) arcs where demand is well below max-flow capacity but failure would cascade (resilience bottlenecks, where investment provides insurance value). The investment logic differs between these two cases.

**P2.6 — Donner Pass peak-demand caveat.**
Elefteriadou requires a statement that the Donner Pass V/C 0.82 is an annual average daily figure, and that peak-demand days approach or exceed V/C 1.0. The closure simulation results should be contextualized with respect to peak-season conditions.

**P2.7 — I-69 multistate coordination acknowledgment.**
Puentes requires acknowledgment that I-69 completion faces a multistate NEPA coordination challenge that is not primarily a funding problem, and that the federal government cannot accelerate independent state EIS processes.

---

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Lada Adamic (network-scientist) | 2/4 | Single-commodity max-flow may misidentify binding constraints |
| Lily Elefteriadou (traffic-engineer) | 3/4 | Capacity value source and derivation undocumented |
| Alan McKinnon (freight-economist) | 3/4 | Commodity value missing from investment case; I-69 NPV not reconciled |
| David Neumark (rural-economist) | 3/4 | Max-flow → investment causal chain incomplete; utilization rate absent |
| Robert Puentes (transport-policy) | 3/4 | I-69 NPV discount-rate sensitivity not shown; NEPA complexity understated |
| **Panel mean** | **2.8/4** | |

---

## Next Steps

1. **Authors** address P1.1 (single-commodity sensitivity analysis) first — this is the methodological gate.
2. After P1.1, address P2.1–P2.7 in same revision cycle.
3. Return to panel for round 2 (Adamic as primary re-reviewer; Neumark and Puentes as secondary).
4. SYNTHESIS-R2 produced after round 2.
