---
reviewer: Alan McKinnon
persona: Alan McKinnon — Professor of Logistics, Kühne Logistics University, Hamburg; lead author, ITF freight decarbonization reports
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper makes a credible case for managed freight lanes as a throughput and reliability investment, with a reasonable first-order NPV calculation. My focus is on two areas: the platooning benefit claim and the decarbonization integration. The platooning calculation is internally consistent but rests on a 30% market penetration assumption by the time the lanes are operational that should be treated as an optimistic scenario rather than a central estimate. The paper does not engage with the decarbonization dimension of the managed freight lane concept, which is a missed opportunity given the venue.

Score: 3/4 — publishable with revision; the platooning penetration assumption and the decarbonization gap are the key items.

## What Works

The core throughput argument — managed freight lanes add effective capacity while preventing induced passenger demand — is well-supported and correctly specified. The distinction between GP capacity additions (which induce both freight and passenger trips) and freight-only managed lanes (which contain the induced demand response) is the paper's central conceptual contribution and is correct.

The platooning fuel savings calculation ($0.039/vehicle-mile from 25% drag reduction × 30% penetration) is internally consistent with the aerodynamics literature (NREL, DOE) and the CALSTART platooning trial data. The 25% drag reduction figure for following vehicles in a 2-truck platoon at 50-foot spacing is consistent with published CFD and wind tunnel results.

The transponder tolling mechanism ($0.05/mile → $2.3B/yr) is a sensible funding proposal. It avoids the constitutional issues with traditional toll roads on federal-aid highways (which require congressional authorization under 23 USC 129) because managed lane access fees on existing interstates have a different statutory basis.

The I-90 weakest B/C (1.6:1, rural-dominant) is an appropriate finding that the paper should prominently acknowledge rather than minimize. A rural-dominant corridor where the traffic volumes do not support the capital cost of a managed lane system is a case against the program, not a weak case for it — the paper's intellectual honesty here is commendable.

## What Doesn't Work

The 30% platooning market penetration assumption drives $0.039/vehicle-mile savings across the program. This is optimistic for the deployment timeline implied by the paper. As of 2026, commercial platooning deployment is still in demonstration phase in the United States: Peloton Technology's Level 2 platooning system has achieved limited commercial deployment; Starsky Robotics has ceased operations; Aurora's commercial deployment has not yet reached the scale implied by a 30% fleet penetration. A 30% penetration figure assumes both widespread commercial adoption and regulatory clearance (platooning requires SAE Level 2 compliance and coordination between trucks from potentially different carriers).

A more appropriate treatment is: central case = 15% penetration (conservative commercial adoption), high case = 30% penetration (accelerated adoption). This bounds the platooning savings at $0.020–$0.039/vehicle-mile and reduces the aggregate platooning NPV contribution by approximately half in the central case. The aggregate NPV ($115B) should be presented as a range rather than a point estimate.

The decarbonization dimension is absent from the paper. Managed freight lanes have two distinct decarbonization benefits beyond platooning: (1) reduced stop-and-start driving reduces fuel consumption beyond platooning savings; (2) dedicated infrastructure creates the access-control environment needed for electric or hydrogen truck charging/fueling infrastructure deployment. A freight-only managed lane corridor is a natural right-of-way for electrification infrastructure, which would substantially increase the social NPV of the program if federal clean-fuel vehicle standards continue to evolve.

For *Transportation Research Part A*, which has published extensively on sustainable transport infrastructure, the omission of the decarbonization dimension is a gap that reviewers at the journal will likely flag.

## The Question I'd Push On

The paper claims a $2.3B/yr transponder toll revenue at $0.05/mile for the managed lane program. What is the assumed annual truck-miles traveled on the managed lanes, and how was this derived from the 57,600 vpd capacity figure?

If the managed lanes carry 57,600 vpd × 8 corridors × average corridor length, the implied annual VMT and toll revenue should be calculable. The $2.3B figure suggests roughly 46 billion managed-lane truck-miles per year (at $0.05/mile), which seems high relative to the 8-corridor footprint. If the math does not close, the toll revenue figure needs to be reconciled with the stated capacity and average corridor length. This matters because the toll revenue is the primary capital recovery mechanism and affects the NPV calculation directly.
