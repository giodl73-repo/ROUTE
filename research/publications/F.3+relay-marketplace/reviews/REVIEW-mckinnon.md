---
reviewer: Alan McKinnon
persona: Alan McKinnon — Professor of Logistics, Kühne Logistics University Hamburg; freight transport economics, decarbonization, supply chain efficiency
round: 1
date: 2026-05-08
score: 3/4
---

> **Note:** AI-generated simulated review.

## Overall

The relay marketplace proposal is the most practically-oriented platform design paper I have reviewed in this area. The utilization argument is compelling, the hub economics are directionally correct, and the AV transition linkage is insightful. My substantive concerns are: (1) the paper's decarbonization case is understated and should be a selling point, not an afterthought; (2) the RDU standardization claim needs to acknowledge FTC implications; and (3) the Mode 2 IC gig model has serious labor classification exposure in California that the paper elides. All three are addressable without restructuring.

## What Works

The compound efficiency argument — relay reduces empty miles (C.4 linkage), enables platooning (E.1 linkage), and improves asset utilization — is the paper's strongest economic contribution even if the individual elements are not fully quantified. These are additive effects: a relay system that reduces empty miles by 30% (C.4 finding), enables 3-truck platooning for 25% fuel savings, and improves asset utilization from 48% to 98%, produces compounding efficiency gains that dwarf the direct driver-cost comparison. The paper gestures at this but doesn't put numbers on the combination.

The Mode 1 (hub operator employs W-2 relay drivers) model is correctly identified as the durable architecture. The analogy to airport ground handling is precise: ground handlers are employed by neutral third-party companies (Swissport, Menzies), not by the airlines, and this separation is what makes inter-airline coordination possible without antitrust issues. The same structure works for relay: the hub operator employs relay drivers and provides neutral service to all carriers, avoiding the coordination problem.

The load security protocol (timestamp, GPS, driver ID, seal scan at each handoff) is a genuine improvement over current solo-driver security, where no intermediate custody record exists. The observation that relay is *better* security than solo driving for regulated loads (pharma, alcohol, firearms) is counterintuitive and worth emphasizing — it removes a likely shipper objection.

## What Doesn't Work

**Decarbonization case is understated.** This paper's audience at *Management Science* and in the policy arena increasingly expects decarbonization analysis. The paper's relay model produces measurable emissions reductions through three mechanisms: (a) empty mile reduction (relay returns drivers to home hubs, reducing empty repositioning miles — estimated 30% reduction in C.4); (b) asset utilization improvement (98% vs. 48% — the same freight moved with half the truck fleet means half the manufacturing emissions for fleet turnover); (c) platooning enablement (relay trucks traveling together between hubs at fixed relay intervals are natural platooning candidates — 25–35% aerodynamic fuel savings with 3-truck platoons). These are not quantified anywhere in the paper. A rough back-of-envelope: the NY-LA corridor moves ~8,000 trucks/day; if relay reduces empty miles by 30% and enables platooning on the relay segments (at 25% fuel savings), the combined emissions reduction is likely in the range of 15–25% per ton-mile on that corridor. This is not a minor co-benefit — for a federal program seeking IIJA or IRA funding, it is a primary justification.

**RDU standardization and FTC implications.** The paper states that the Relay Display Unit (RDU) specification "requires no federal rulemaking" because it is a voluntary industry standard. This is technically correct for the standard itself. But if: (a) RDU becomes a condition of hub access; and (b) hub access becomes essential infrastructure for interstate relay operations; then RDU is effectively a mandatory standard with a private standards body setting terms — which creates FTC concerns about standard-essential patents, tying arrangements, and exclusionary conduct. The paper should add one sentence acknowledging this and recommending that the RDU standard be developed through an open standards process (IEEE, SAE, or similar) rather than by a proprietary consortium. This is a minor addition that eliminates a potential regulatory objection.

**Mode 2 IC model: California AB5 exposure.** The paper's Mode 2 (independent contractor gig relay drivers) is described as supplemental but viable. In California — the single largest freight state, handling ~40% of US container import volume — AB5 (California Labor Code § 2775 et seq.) applies a stringent ABC test to contractor classification. Under AB5, a CDL holder regularly performing relay driving through a platform would almost certainly qualify as an employee, not an IC. The California Supreme Court has applied the ABC test to gig economy transportation workers (Dynamex Operations West, Inc. v. Superior Court, 2018). The paper should acknowledge this directly: Mode 2 IC is legally problematic in California and other AB5-equivalent states (Massachusetts, New Jersey). The durable recommendation — Mode 1 W-2 hub operator employment — is reinforced by this analysis, not undermined.

## The Question I'd Push On

The paper presents relay as a path to dramatically lower per-ton-mile cost. But the trucking industry's economic equilibrium involves millions of owner-operators whose business model is the solo long-haul trip. If relay displaces 30% of solo long-haul trips (a plausible market penetration estimate), what happens to those owner-operators? They cannot simply convert to relay drivers — relay driving is regional (home every night), which is structurally incompatible with the owner-operator model (truck ownership + long-haul routes + self-scheduling). Does the paper recommend any transition mechanism for displaced owner-operators, or does it treat this as an acceptable market outcome? At *Management Science*, this is an optional consideration; at a policy journal, it would be required. The paper should at minimum acknowledge the distributional question.
