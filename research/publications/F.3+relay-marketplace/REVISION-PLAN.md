---
paper: F.3+relay-marketplace
title: "The Relay Marketplace: Platform Design for 48-Hour National Freight and the AV Transition"
round: 1
date: 2026-05-08
status: in-revision
---

# Revision Plan — Round 1

Generated from SYNTHESIS.md. All P1 items must be resolved before resubmission to reviewers. P2 items are required before journal submission.

---

## P1 — Blocking Items

- [ ] **P1.1a** Build the per-ton-mile total system cost comparison table: relay trip cost = driver wages (per ton-mile) + hub fees (swaps × fee, per ton-mile); solo trip cost = driver wages (per ton-mile) + solo driver shortage premium (signing bonus amortized per ton-mile)
- [ ] **P1.1b** Add asset utilization savings to the relay-side ledger: fleet capital cost reduction from 98% vs. 48% utilization ($90k/truck avoided cost per route-truck-cycle), expressed per ton-mile
- [ ] **P1.1c** Add SLA revenue premium to the relay-side ledger: relay achieves ~70% of Full I2.0 SLA improvement; faster delivery commands premium pricing (estimate from shipper willingness-to-pay literature or ATRI survey data)
- [ ] **P1.1d** Replace the per-trip headline comparison ($1,050 vs. $1,456) with the per-ton-mile fully-loaded comparison as the paper's central quantitative claim; retain the per-trip figures in a footnote with an explicit note that hub fees are excluded from the per-trip relay figure
- [ ] **P1.2a** Add a paragraph in Section 4 (Insurance Framework / Regulatory) or Section 3 (Marketplace Architecture) acknowledging California AB5 (California Labor Code § 2775 et seq.) and the ABC test for contractor classification
- [ ] **P1.2b** Cite Dynamex Operations West, Inc. v. Superior Court (4 Cal.5th 903, 2018) as the controlling California precedent; note that relay IC drivers performing relay as their principal occupation would fail prong B of the ABC test
- [ ] **P1.2c** Identify the AB5-equivalent states: Massachusetts (Prong B test), New Jersey (ABC test), Connecticut (ABC test); note these are high-freight-volume states
- [ ] **P1.2d** Revise Mode 2 recommendation: restrict Mode 2 IC to states without stringent IC classification laws and to genuinely occasional/supplemental use (e.g., surge capacity during peak seasons); make Mode 1 W-2 hub operator employment the explicit primary and durable model
- [ ] **P1.2e** Add sentence: "Mode 1 W-2 is the legally durable relay driver employment model across all US jurisdictions; Mode 2 IC is viable as a supplemental layer only in states without ABC-test contractor classification requirements"

---

## P2 — Required Before Submission

- [ ] **P2.1a** Correct or remove the Rochet-Tirole (2003) two-sided market citation; the relay marketplace is a one-sided matching market with platform intermediation, not a canonical two-sided market
- [ ] **P2.1b** Add one paragraph applying Williamson (1979) asset-specificity framework to the Mode 1 vs. Mode 2 choice: high asset specificity (CDL, drug test currency, HOS reset status) + high relational uncertainty → vertical integration (Mode 1 W-2) is the efficient governance structure per TCE
- [ ] **P2.1c** Reference matching market design literature: Roth and Sotomayor (1990) "Two-Sided Matching"; Roth (2002) "The Economist as Engineer" as the theoretical framing for the slot exchange mechanism
- [ ] **P2.2a** Specify the slot exchange matching mechanism formally: name the algorithm (deferred acceptance / Gale-Shapley recommended); specify what information carriers and drivers submit; specify the matching objective function
- [ ] **P2.2b** Add newsvendor model or M/D/c queueing analysis to derive minimum hub driver pool size for target maximum wait time (e.g., 30-minute maximum at 99th percentile of stochastic driver availability)
- [ ] **P2.3** Replace the Atlanta 750-dock utilization estimate with M/D/c queueing derivation: specify arrival rate λ (trucks/hour), service time μ (15 minutes), derive minimum dock count c for target wait-time SLA; show that 750 docks (or revised figure) satisfies the SLA constraint
- [ ] **P2.4** Revise FMCSA rulemaking timeline from 18–24 months to 36–60 months as the base case; state that 18–24 months is achievable only if mandated by the IIJA successor reauthorization; cite GAO-24-106178 for average FMCSA NPRM timeline
- [ ] **P2.5** Add recommendation that RDU specification be developed through an open standards body (IEEE or SAE) rather than a proprietary consortium; cite IEEE 802.11p / C-V2X and SAE J3016 as precedents; note FTC essential-infrastructure concerns if RDU is a condition of hub access
- [ ] **P2.6** Add one sentence in Section 7 (AV Transition, Phase 3) acknowledging that full commercial AV trunking on managed lanes requires a FMCSA certification rulemaking that does not currently exist; reference FMCSA-2023-0249 as the applicable docket
- [ ] **P2.7a** Quantify compound decarbonization effects of relay: empty mile reduction (~30%, per C.4 simulation findings), platooning fuel savings (~25–35% at 3-truck platoons), asset utilization improvement (fleet size reduction)
- [ ] **P2.7b** Provide order-of-magnitude per-ton-mile emission reduction estimate for the NY-LA corridor as a worked example; note relevance to IRA and IIJA decarbonization program eligibility
- [ ] **P2.8a** Add paragraph explicitly stating Mode 1 W-2 job quality advantages: predictable schedule, home-every-night, FLSA overtime, workers' compensation, OSHA protection, benefits eligibility
- [ ] **P2.8b** Acknowledge Teamster displacement question; recommend priority hiring of displaced long-haul drivers for Mode 1 hub positions as a condition of NFRZ designation
- [ ] **P2.8c** Recommend that hub siting include a community impact assessment for diesel PM, noise, and traffic burden on adjacent communities

---

## Citation Additions

- [ ] Add Roth and Sotomayor (1990) "Two-Sided Matching" to references (market design framing)
- [ ] Add Roth (2002) "The Economist as Engineer" to references (mechanism design framing)
- [ ] Remove or correct Rochet and Tirole (2003) citation if two-sided market characterization is removed
- [ ] Add Dynamex Operations West, Inc. v. Superior Court (2018) to references
- [ ] Add GAO-24-106178 (FMCSA rulemaking timeline study) to references
- [ ] Add FMCSA-2023-0249 (CMV AV regulatory approach docket) to references
- [ ] Add Gale and Shapley (1962) "College Admissions and the Stability of Marriage" if deferred acceptance algorithm is cited

---

## Target Outcome for Round 2

Resolving P1.1 (per-ton-mile reframing) should address Neumark's primary objection and move his score from 2/4 to 3/4. Resolving P1.2 (Mode 2 gig labor acknowledgment) should address Schmitt's primary objection and move her score from 2/4 to 3/4. Expected mean after revision: approximately 3.0/4. Addressing P2 items (especially P2.1 market framing, P2.7 decarbonization, P2.8 labor) will further strengthen the manuscript for journal submission.
