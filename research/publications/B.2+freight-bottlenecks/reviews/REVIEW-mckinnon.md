---
reviewer: mckinnon
persona: Alan McKinnon — freight economist, Kühne Logistics University, Hamburg; TRB Freight Systems Committee
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper is well-grounded in ATRI data and the core finding — T1 corridors account for a disproportionate share of bottleneck cost per location — is empirically robust and policy-relevant. The cascade multiplier is an intuitive framing that TRR readers will appreciate. My concern is methodological: ATRI measures delay cost experienced at a location, but the downstream supply chain cost (inventory carrying, schedule disruption, shipper modal diversion) is not captured, and the paper mentions this limitation without examining its direction or magnitude. For a freight economics paper at TRR, a bound on the omitted cost is needed.

## What Works

**The $22.7B figure is appropriately scoped.** The paper is explicit that this is the direct trucking cost component only — driver time, fuel premium from idling, schedule penalties — and does not include inventory carrying costs or supply chain multipliers. This is the right way to use ATRI data: report what it measures, not what it implies. The acknowledgment that "the true societal cost is substantially higher" is honest without inventing a multiplier.

**The cascade multiplier derivation is transparent.** The formula (cost per T1 location / cost per T2 location) is simple and unambiguous. The 1.73× result falls in a plausible range: T1 corridors carry more trucks per day, rerouting is more expensive, and freight value-at-risk is higher. The three mechanism attribution (volume, rerouting cost, value-at-risk) is correctly framed as an attribution hypothesis, not a measured decomposition.

**I-40 zero-bottleneck finding is the paper's strongest validation.** The convergence between ROUTE's V/C=0.84 score and ATRI's revealed-preference zero-bottleneck result for I-40 is the best evidence in the paper that the tier classification is correct. A scoring framework that puts I-40 in T1 AND predicts no ATRI bottlenecks AND has that prediction confirmed by independent data is demonstrating real validity.

**Atlanta investment logic is honest about the T2 upgrade argument.** The paper does not pretend the I-285 T1 upgrade follows automatically from the rubric — it explicitly frames the bottleneck data as providing the economic justification that "pure network topology does not." This is the correct relationship between the two evidence streams.

## What Doesn't Work

**ATRI cost methodology omits queue spillback and secondary incidents.** The ATRI model counts delay experienced at the bottleneck location. It does not capture queue spillback — trucks delayed 5 miles upstream of a bottleneck that never enters the measured location — or secondary incidents caused by congestion (rear-end collisions triggered by sudden deceleration). Both effects are documented in the traffic engineering literature as materially increasing the true economic cost of bottlenecks (Giuliano & Golob 1992; Schrank et al. 2019 URBAN MOBILITY REPORT). The paper acknowledges inventory costs are excluded but does not mention spillback or secondary incidents. A sentence bounding these omissions — even a rough order-of-magnitude — would significantly strengthen the economic claims.

**The cascade multiplier needs a formal definition in the methods section.** The concept is introduced informally in the abstract and defined mathematically in Section 5, but the methods section (Section 3.4) describes the formula without the verbal rationale for why cost-per-location is the right denominator rather than cost-per-lane-mile or cost-per-truck. For a novel metric being introduced at TRR, the denominator choice needs justification in the methods section. Cost-per-location conflates bottlenecks of different physical extent (a half-mile weave segment vs. a 5-mile mountain pass segment); cost-per-lane-mile might be more comparable across location types.

**The value-at-risk effect is asserted without data.** The third mechanism attributed to the cascade multiplier — "T1 corridors carry disproportionately high-value freight" — is stated without citation. FAF5 commodity flow data contains value-per-ton by commodity and corridor; this assertion is directly testable. Without the data, the cascade multiplier's third mechanism is speculation. Either verify against FAF5 or remove the value-at-risk claim and leave the multiplier explained by volume and rerouting cost alone.

## The Question I'd Push On

The paper concludes that T1 managed freight lane investment provides higher economic return than T2 investment on a per-dollar basis, using the cascade multiplier as evidence. But the cascade multiplier is a ratio of current costs — it says nothing about the cost reduction achievable per dollar of investment on T1 vs. T2. If T1 managed lanes cost 3× more per lane-mile to build than T2 expansion (urban construction premium), the per-dollar return comparison could reverse. What is the expected cost-per-bottleneck-dollar-avoided for T1 managed lanes vs. T2 capacity expansion, using construction cost estimates from FHWA or comparable managed lane projects?
