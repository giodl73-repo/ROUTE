---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou — Professor and Director, University of Florida Transportation Institute (UFTI); Herbert Wertheim College of Engineering; specialist in traffic flow theory, highway capacity analysis, and freeway operations
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The compound resilience concept is well-formulated and the policy implications are important. I focus my review on the technical claims in the capacity and closure cost calculations, which underpin the paper's entire NPV argument. The paper uses traffic flow assumptions that are not grounded in HCM methodology, and the capacity-based disruption cost formula has errors in its treatment of peak versus off-peak traffic conditions that inflate the daily cost estimates. These are not minor methodological quibbles — they affect the primary quantitative claim of the paper. The Donner $1.6B/year figure, and therefore the $15.8B NPV, requires recalculation with correct traffic engineering inputs. Score: 2/4.

## What Works

The compound exposure identification methodology (B1 > 7 AND D1 > 6) is sound and the resulting corridor list in Table 2 is credible. The conceptual distinction between structural isolation and environmental vulnerability is well-drawn. The policy sections (Section 6) are among the strongest in the paper — the PROTECT program reform recommendation is specific and actionable, and the National Critical Highway Infrastructure designation argument is well-supported.

The phased investment program in Section 5 is realistically structured. The observation that Phase 1 (Donner tunnel) requires dedicated authorization outside the formula program structure is accurate from a federal funding perspective. The treatment of Snoqualmie and Siskiyou as addressable within standard NHPP formula programs is reasonable given their lower cost.

The formal NPV structure (Equation 1) is correctly specified and the three-way comparison in Table 3 (tunnel vs. US-50 alternate vs. snowshed hardening) is the right analytical structure for validating the compound investment argument.

## What Doesn't Work

**The 8,000 trucks/day figure is used without adjustment for peak hour/off-peak distribution.** The paper applies 8,000 trucks/day as if closures occur at average daily traffic levels. But highway capacity analysis (HCM 7th Edition, Chapter 11) requires distinguishing between peak-hour and off-peak flow conditions. Donner Pass closures occur primarily during winter storm events, which do not coincide uniformly with peak freight hours. If the modal split of the 50 annual closures is: 20 overnight closures (10pm-6am, when AADTT is perhaps 30% of the daily average) and 30 daytime closures (8am-6pm), the affected truck population per event differs significantly from 8,000 × 18/24 hours. The paper should compute the expected truck exposure per closure event using the hourly traffic distribution (available from Caltrans PeMS loop detector data at Truckee) rather than assuming uniform daily flow.

**The rerouting cost per truck calculation conflates route distance with travel time.** The paper states that rerouting via I-40 adds "310 miles and 5.5 hours of driving." 310 miles at 55 mph (the correct commercial vehicle speed for mountainous terrain on I-40 through Barstow) takes 5.6 hours of pure driving time. But the paper then applies this time increment at the full $225/hr ATRI operating cost rate. This is correct for the additional operating time. However, the 310 additional miles also incur fuel and tire costs beyond the time-based operating cost: at $0.72/mile (2024 ATRI fuel cost per mile), 310 additional miles cost $223/truck in additional fuel. The paper may be double-counting: if the $225/hr figure already includes fuel cost per hour, and the truck covers 55 miles/hour, then the $225/hr includes $0.72 × 55 = $39.60/hour in fuel, and the additional 310 miles add 5.6 × $39.60 = $222 in fuel — which is approximately what a distance-based calculation would give. But the paper should state this explicitly to confirm there is no double-counting in the $2,625/truck rerouting cost figure.

**The waiting cost calculation uses an incorrect cost rate.** The paper applies $225/hr (full loaded operating cost) to the waiting fraction (0.6) of stranded trucks, calculating waiting cost as 8,000 × 0.103 × 365 × 0.6 × 18 hours × $225/hr. But $225/hr is the cost of a truck in motion — fuel consumption at highway speed, tire wear, engine wear, driver time at full operating rate. A truck waiting at a closure checkpoint has near-zero fuel consumption (idle at <0.8 gal/hr vs. 6-7 gal/hr at highway speed), zero tire wear, and reduced maintenance accrual. The correct waiting cost is: driver time (legally required pay during delay, ~$70/hr) + idle fuel cost ($0.80/hr) + fixed overhead allocation (~$20/hr). Total waiting cost is approximately $91/hr, not $225/hr. Applying the correct waiting cost rate to the 0.6 waiting fraction reduces the D1 (waiting) benefit component of the annual cost estimate from approximately $700M/year to approximately $280M/year. The B1 (rerouting) component remains at approximately $900M/year. Total annual benefit falls to approximately $1.18B/year rather than $1.6B/year.

This is a material error. At $1.18B/year annual benefit, the Donner tunnel 30-year NPV at 7% discount rate falls from $15.8B to approximately $11.6B, and the CBR falls from 5.75:1 to approximately 4.0:1. The investment case remains strongly positive, but the headline NPV figure requires correction.

## The Question I'd Push On

The paper's closure cost model treats each closure event as independent: 50 closures/year, each with a defined pool of affected trucks, each resolved independently. But I-80 freight traffic is not independent across events. Shippers who experience repeated Donner closures adapt: they resequence shipments, reroute loads through Los Angeles ports instead of Oakland/Stockton, or shift to rail for Sierra crossings. If some fraction of the 8,000 trucks/day has already been diverted from I-80 Sierra crossing because shippers have pre-adapted to the closure pattern, the current traffic count already reflects an adaptation equilibrium — the truck counts represent trucks that remain despite the closure risk, not the full universe of trucks that would use this route in a world with no closure risk. If this is true, the benefit of eliminating closures (the tunnel) is the *additional* traffic attracted by eliminating the risk, not the avoided cost to current traffic. Has the paper considered the pre-adaptation baseline problem, and how does it affect the closure cost estimate?
