---
paper: F.3+relay-marketplace
title: "The Relay Marketplace: Platform Design for 48-Hour National Freight and the AV Transition"
round: 1
date: 2026-05-08
stage: revision
---

## Headline Assessment

The paper proposes the most detailed relay marketplace architecture in the literature and correctly diagnoses the coordination failure as the primary barrier. The hub operator model, HOS compliance architecture, and AV transition linkage are all genuine contributions. Two issues block promotion: the central cost comparison ($1,050 vs. $1,456/trip) omits the hub fees that are the marketplace's own revenue source, producing a misleading result; and the Mode 2 IC gig model ignores California AB5 and the broader gig precarity question in the single largest freight state. Both are addressable. The mean score (2.6/4) reflects manageable analytical gaps, not a weak thesis. The relay marketplace concept is sound; the paper's economics need to be put on a per-ton-mile basis and the labor model needs honest treatment.

---

## Earned Stakes (uncontested across reviewers)

| Claim | Status | Evidence |
|---|---|---|
| Coordination failure (not economics) is the primary barrier to relay marketplace | **Earned** | Neumark, McKinnon, Puentes all accept the diagnosis |
| Asset utilization argument: solo 48% vs. relay 98%, $90k/truck in avoided fleet cost | **Earned** | Accepted by all reviewers; strongest durable quantitative argument |
| Mode 1 (W-2 hub operator employment) is the correct durable architecture | **Earned** | Neumark (transaction cost), McKinnon (decarbonization), Schmitt (labor) all converge |
| FMCSA rulemaking (not statutory change) is the correct regulatory pathway | **Earned** | Puentes confirms; three definitions are achievable via NPRM |
| Relay hub infrastructure ($5M/station) is dual-use for AV transition | **Earned** | Chester (adjacent paper work) and Adamic accept the design logic |
| Load security is better under relay than under solo long-haul | **Earned** | Adamic, McKinnon accept; genuine counter-intuitive contribution |

---

## Contested Stakes

| Claim | Contesting Voice | Specific Objection | Verdict |
|---|---|---|---|
| Relay is cheaper: $1,050 vs. $1,456/trip | Neumark | Hub fees ($675 at 6 swaps × $112.50) bring relay total to ~$1,725 — more expensive than solo on per-trip basis | **P1 blocker — must reframe as per-ton-mile** |
| Mode 2 IC is viable supplemental model | McKinnon, Schmitt | AB5 (California), ABC test, Dynamex ruling; Mode 2 is legally problematic in largest freight state | **P1 — must acknowledge and recommend Mode 1 as primary** |
| Relay marketplace is a two-sided market (Rochet-Tirole) | Neumark | Carriers are on both sides; not canonical two-sided market; matching market model more appropriate | **P2 — correct framing or drop citation** |
| Williamson (1979) supports the coordination failure diagnosis | Neumark | Cited but not applied; asset-specificity analysis would formally support Mode 1 over Mode 2 | **P2 — apply formally or remove citation** |
| FMCSA definitions achievable in 18-24 months | Puentes | FMCSA NPRMs average 4.1 years (GAO-24-106178); 18-24 months only if mandated by reauthorization | **P2 — revise timeline to 36-60 months base case** |
| RDU standardization requires no federal rulemaking | McKinnon | True for the standard itself; but if hub access is essential infrastructure, RDU as condition of access = effective mandate requiring FTC review | **P2 — recommend open standards process (IEEE/SAE)** |
| Phase 3 full AV (2033+) is achievable | Puentes | FMCSA AV exemption framework does not exist; certification rulemaking will take 3-5 years | **P2 — add regulatory prerequisite acknowledgment** |
| Relay creates "better jobs" than solo long-haul | Schmitt | Driver preferences for autonomy/income over regularity not consulted; empirical question treated as assertion | **P2 — acknowledge as empirical question; cite OOIDA survey literature** |
| Hub siting is a network optimization problem only | Schmitt | Community burden of hub-adjacent diesel exposure not considered | **P2 — add community impact assessment recommendation** |

---

## P1 Blocking Items (must resolve before re-review)

**P1.1 — Hub fee economics: per-trip to per-ton-mile reframing**
The $1,050 vs. $1,456/trip comparison is the paper's central quantitative claim. Including hub fees at the paper's own stated swap fee ($100 midpoint) × 6 swaps = $600 brings relay total to $1,650 — more expensive than solo per trip. The correct resolution is not to remove hub fees from the relay cost, but to change the comparison basis. The relay model does not just move the same freight more cheaply per trip — it moves it faster (better SLA = higher revenue), with better asset utilization (98% vs. 48% = half the fleet capital cost), and with lower driver shortage premium (no signing bonus for regional vs. long-haul). The paper must present a per-ton-mile total system cost comparison that includes: relay driver wages ($1,050 per trip, divided by tonnage × distance); hub fees (number of swaps × swap fee, per ton-mile); asset utilization savings (fleet capital cost reduction, per ton-mile); and driver shortage premium savings (per ton-mile). This table is the paper's correct economic centerpiece. If relay remains cost-competitive on a per-ton-mile fully-loaded basis — which is plausible given the utilization advantage — that is a stronger and auditable result than the current per-trip comparison that omits hub fees.

**P1.2 — Mode 2 IC gig labor classification**
The paper presents Mode 2 (independent contractor relay drivers) without acknowledging that California AB5 (California Labor Code § 2775 et seq., codifying Dynamex Operations West, Inc. v. Superior Court, 2018) would classify relay IC drivers as employees in California — the state handling approximately 40% of US container import volume through POLA/POLB and 12%+ of total US freight. Under the ABC test, a CDL holder performing relay driving as their principal occupation through a platform fails prong B (the work is the same type as the platform's main business). Required revision: (a) acknowledge AB5 and ABC-test states (Massachusetts, New Jersey, Connecticut) by name; (b) state that Mode 2 is legally untenable in these states for regular relay drivers; (c) revise the recommendation to make Mode 1 W-2 the primary and durable model; (d) restrict Mode 2 to states without stringent IC classification laws and to truly occasional/supplemental use.

---

## P2 Items (should address before submission)

**P2.1 — Market structure framing**
Correct or remove the Rochet-Tirole two-sided market citation. The relay marketplace is a one-sided matching market with platform intermediation (carriers are on both sides). The appropriate theoretical framing is matching market design (Roth and Sotomayor 1990; Roth 2002 "The Economist as Engineer"). Apply the Williamson (1979) asset-specificity framework formally to the Mode 1 vs. Mode 2 choice: high asset specificity (CDL, drug test currency, HOS reset) and high relational uncertainty → vertical integration (Mode 1 W-2) is the efficient governance structure. This is a one-paragraph addition that converts a cited-but-unused reference into applied mechanism.

**P2.2 — Matching mechanism specification**
Specify the slot exchange matching mechanism formally: what information do carriers and drivers submit; what is the matching objective function; how are conflicts resolved. Minimum requirement: name the algorithm (deferred acceptance / Gale-Shapley, or priority auction, or first-come-first-served) and specify what properties it optimizes. Add a newsvendor model or queueing analysis (M/D/c) to derive the minimum hub driver pool size needed to achieve a target maximum wait time (e.g., 30 minutes at the 99th percentile of stochastic driver availability).

**P2.3 — Atlanta hub capacity queueing analysis**
Replace the 750-dock utilization estimate with an M/D/c queueing derivation: arrival rate λ (trucks/hour by time of day), service time μ (15 minutes per relay handoff), server count c (docks). Derive the minimum dock count for a target wait-time SLA. This is reproducible from stated parameters and will significantly strengthen the hub business case.

**P2.4 — FMCSA rulemaking timeline**
Revise 18–24 months to 36–60 months as the base case, with 18–24 months achievable only if the rulemaking is mandated by the IIJA successor reauthorization. Cite GAO-24-106178 (average FMCSA NPRM timeline). Note that FMCSA-2023-0249 (CMV AV regulatory approach) is the applicable docket for Phase 3 AV integration.

**P2.5 — RDU open standards process**
Add one sentence recommending that the RDU specification be developed through an open standards body (IEEE 802.11p / C-V2X standards as precedent for V2I; SAE J3016 for AV level definition) rather than a proprietary consortium. This avoids FTC essential-infrastructure concerns and makes adoption more likely across the fragmented 70,000-carrier market.

**P2.6 — Phase 3 AV regulatory prerequisite**
Add one sentence in Section 7 (AV Transition) acknowledging that Phase 3 (full AV commercial trunking, 2033+) requires a FMCSA rulemaking for commercial AV certification that does not currently exist; reference FMCSA-2023-0249 as the current regulatory docket.

**P2.7 — Decarbonization compound effects**
Quantify the compounding emission reductions from relay: (a) empty mile reduction (~30%, from C.4 findings); (b) platooning fuel savings (~25–35% aerodynamic benefit at 3-truck platoons); (c) asset utilization improvement (98% vs. 48% → half the fleet → half the manufacturing/end-of-life emissions). Provide a rough order-of-magnitude estimate of combined per-ton-mile emission reduction for the NY-LA corridor as a worked example. This strengthens the paper's policy relevance for IRA/IIJA decarbonization programs.

**P2.8 — Labor: Mode 1 job quality argument**
Add one paragraph explicitly stating that Mode 1 W-2 hub employment provides materially better job quality than solo long-haul OTR on the dimensions that matter for retention: predictable schedule, home-every-night, FLSA overtime, workers' compensation, OSHA protection, and benefits eligibility. Acknowledge the Teamster displacement question and recommend: priority hiring of displaced long-haul drivers for Mode 1 hub positions; representation of driver labor in NFRZ governance.

---

## Score Summary

| Reviewer | Expertise | Score |
|---|---|---|
| David Neumark | Labor economics, market structure | 2/4 |
| Lada Adamic | Network science, platform design | 3/4 |
| Robert Puentes | Federal transport policy | 3/4 |
| Alan McKinnon | Freight economics, decarbonization | 3/4 |
| Angie Schmitt | Transportation equity, labor | 2/4 |
| **Mean** | | **2.6/4** |

Promotion threshold: 3.0/4 average, minimum 2/4 from all reviewers. Current mean (2.6) is below threshold. Two reviewers at 2/4 (Neumark, Schmitt); both have specific, addressable objections. Resolving P1.1 (per-ton-mile reframing) should move Neumark to 3/4. Resolving P1.2 (Mode 2 gig labor acknowledgment) should move Schmitt to 3/4. Mean would reach approximately 3.0.

---

## Next Steps

1. **Resolve P1.1** — Build the per-ton-mile total system cost comparison table; include hub fees on the relay side; demonstrate relay cost-competitiveness on the correct comparison basis.
2. **Resolve P1.2** — Add AB5/ABC-test paragraph; revise Mode 2 recommendation; make Mode 1 the explicit primary and durable model.
3. **Address P2.1–P2.8** — Market structure framing, matching mechanism, hub queueing, FMCSA timeline, RDU standards process, AV regulatory prerequisite, decarbonization quantification, labor job quality.
4. Submit revised manuscript for Round 2 review — Neumark and Schmitt are the priority reviewers to re-engage.
