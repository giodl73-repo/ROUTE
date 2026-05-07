---
reviewer: Freight Economist
persona: Alan McKinnon (Kuhne Logistics University) — freight transport, decarbonization, logistics
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

## Overall

The paper correctly identifies that aggregate-score tier classification misplaces strategic freight arteries behind locally congested connectors. From a freight logistics perspective, this matters: a shipper routing transcontinental freight needs to know which corridors are structural (unavoidable) vs. congested (situational), and the current aggregate scoring treats both equivalently. The centrality-adjusted classification is the right tool for this purpose. My concerns are about the A2 (Freight Intensity) dimension's reliability and the absence of commodity value from the arterial ranking.

## What Works

**The congestion-stress paradox matters for freight logistics.** A logistics manager allocating a dedicated fleet route needs to know whether I-80 through Wyoming is structurally necessary (there is no other way across the northern Rockies) or merely conventionally used (habit + infrastructure investment). Betweenness centrality answers the structural question; aggregate score answers neither clearly. The paper's insight is directly applicable to freight network analysis.

**The ATRI validation** (Section 5.2): ATRI data is the logistics industry's standard for measuring operational freight network performance. The correlation between centrality-adjusted tier and ATRI bottleneck density validates the classification against the most operationally relevant external dataset available. T1 corridors showing 1.8 ATRI locations per 100 miles vs. 0.2 for T4 is a meaningful gradient.

**Table 5's freight-specific features** (managed freight lanes, intermodal spurs, truck EV charging, enhanced rest areas): The paper correctly assigns these features to T1 only or T1+T2. The EV charging standard (≥350kW at freight terminals) reflects the real requirements of commercial electric trucking, which needs charging power 2–3× higher than passenger EVs. This operational awareness is welcome.

**The "metro map" analogy**: In freight logistics, route planning tools produce exactly this kind of hierarchical network visualization — primary lanes, secondary lanes, local pickup/delivery. The analogy to transit cartography is pedagogically effective.

## What Doesn't Work

**A2 (Freight Intensity) scores are marked estimated throughout.** The paper derives the centrality-adjusted tier classification from B2 centrality + aggregate score, where aggregate score includes A2 (Freight Intensity). But A2 scores are all estimated via FAF5 zone traversal (acknowledged as a proxy). The tier classification therefore uses a composite signal where one of the two primary components (B2) is partial-graph estimated and the other (A2 within aggregate score) is FAF5-approximated. Both are directionally correct but neither is precise. The paper should quantify how sensitive the tier assignments are to A2 accuracy — if true A2 scores differ from estimated by 20%, does any corridor change tier?

**Commodity value is absent from the freight intensity signal.** A2 measures truck-miles or estimated freight value, but not commodity criticality. A corridor moving pharmaceuticals from a major distribution hub has different strategic value than one moving gravel, even at the same AADT. The pharmaceutical corridor cannot be easily rerouted; the gravel corridor can. For a freight arterial classification, commodity criticality should be part of the signal. The Freight Analysis Framework provides value by commodity type — the paper could compute a "commodity criticality" weight alongside volume.

**Decarbonization is absent from the tier framework.** Electric freight trucking (Tesla Semi, Freightliner eCascadia) has a range of 300–500 miles between charges. For a T1 corridor, this means charging infrastructure every 250–300 miles is a practical requirement, not aspirational. The paper mentions EV charging in Table 5 but doesn't analyze which T1 corridors currently have adequate charging and which have gaps. This is the most time-sensitive I2.0 investment — the trucks are being deployed now, the infrastructure is 5–10 years behind.

## The Question I'd Push On

The paper's key distinction is between "structurally necessary" (high betweenness — you must go through this corridor) and "locally stressed" (high A1 — this corridor is congested). But for freight shippers, a third dimension matters: "commercially non-substitutable." I-35 is the only direct corridor from Laredo to the US interior for Mexican import traffic — there is no ocean freight alternative, no rail alternative for most commodities, no air alternative. Is this non-substitutability captured by betweenness centrality? Or does centrality only capture geographic irreplaceability (you can't get from A to B any other way) without capturing commercial irreplaceability (even if you could go another way, you can't because of port, border, or modal constraints)?
