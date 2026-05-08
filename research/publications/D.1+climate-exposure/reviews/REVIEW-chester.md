---
reviewer: Mikhail Chester
persona: Mikhail Chester — Professor of Civil, Environmental, and Sustainable Engineering, Arizona State University; Director, Metis Center for Infrastructure and Sustainable Engineering
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper makes a genuine contribution to the infrastructure resilience literature by introducing a composite climate exposure metric (D1) that weights consecutive SFHA miles more heavily than aggregate totals. The core insight — that contiguous flood exposure is operationally more catastrophic than equivalent total exposure spread across a corridor — is correct and under-theorized in current FHWA allocation practice. The finding that PROTECT program funding systematically underweights the Louisiana Gulf Coast corridor is the paper's most policy-relevant result and is well-supported by the methodology.

My main concern is with the 2050 projection layer. The paper applies NOAA intermediate SLR projections (0.5m) to current FEMA NFHL boundaries and presents the resulting D1 upgrades (Gulf Coast LA: 8.4→9.1; TX: 7.8→8.8) with a precision that outstrips the method's actual resolution. This is a first-order approximation that conflates several distinct physical processes, and the paper should be more explicit about that.

Score: 3/4 — strong enough to publish with revision; the 2050 projection section needs methodological humility added before this is acceptable for *Nature Climate Change*.

## What Works

The D1 formula (0.7×score_consecutive_sfha + 0.3×score_total_sfha) is well-motivated. The 70/30 weighting reflects a sensible operational judgment: a 127-mile continuous SFHA segment on I-10 Gulf Coast LA creates a single unbroken closure event, while an equivalent total mileage distributed across 15 non-consecutive segments creates 15 manageable short closures. This distinction matters for freight network continuity and the paper articulates it clearly.

The systems framing — treating the interstate corridor as a functional unit rather than a collection of independent segments — aligns with how freight operators actually experience disruptions. A closure at mile 40 of a continuous SFHA corridor strands everything behind it; that asymmetry is captured here and not in the FHWA's current metric.

The data sourcing is appropriate: FEMA NFHL API for current SFHA boundaries, NOAA SLR intermediate scenario for projections, and Caltrans/WSDOT closure logs for winter corridor scoring. The decision to use the NOAA intermediate scenario (0.5m by 2050) rather than the high or low scenarios is defensible given current emission trajectory uncertainty.

The PROTECT program critique is the paper's strongest applied contribution. If FHWA allocates by total SFHA miles, and the Louisiana corridor's consecutive exposure is disproportionate, then the allocation formula is mismatched to the operational risk it is trying to address. This deserves regulatory attention.

## What Doesn't Work

The 2050 projection method applies SLR directly to current NFHL polygon boundaries and scores the resulting expanded inundation zone using the D1 formula. This is problematic for at least three reasons the paper does not acknowledge:

First, land subsidence in coastal Louisiana runs at 8–12mm/year (USGS), which means effective relative SLR in the Gulf Coast LA corridor is substantially higher than the NOAA 0.5m global mean. The projected D1 increase from 8.4 to 9.1 is likely an underestimate, but the paper cannot know by how much without incorporating a subsidence model.

Second, the NFHL boundaries are mapped for a 1% annual chance flood (100-year event) under current conditions. Applying SLR to those boundaries does not correctly model how the 100-year flood polygon expands — that requires storm surge recurrence modeling, not a simple horizontal extension of current zone boundaries.

Third, the paper's winter corridor scoring (I-80 Donner, I-90 Snoqualmie) uses closure frequency as a proxy for climate exposure, but this metric is not dimensionally comparable to the SFHA flood-zone metric. The composite D1 score combines these incommensurable quantities without acknowledging the category error. A 7.8 on D1 means something different for Donner (winter closure frequency) than for Gulf Coast TX (consecutive SFHA miles). This limits cross-corridor comparison.

The infrastructure design life question is also absent. A corridor scoring D1=9.1 by 2050 needs to be matched against the remaining design life of the infrastructure: if I-10 Gulf Coast LA's bridges were built in the 1960s with a 50-year design life, they are already past service life and the projection horizon is moot unless replacement is already planned.

## The Question I'd Push On

The paper claims the 2050 projections establish a "priority order shift" — Gulf Coast I-10 becomes the most critical climate adaptation investment by 2050. But this priority ordering is sensitive to the subsidence assumption. If subsidence rates are incorporated, the Gulf Coast LA projection may already be at D1≥9.5 under a moderate scenario, which would change not just the ranking but the urgency framing.

My question: have the authors run the 2050 projection with and without the Louisiana subsidence rate (say, 10mm/yr) to bound the uncertainty? If subsidence roughly doubles the effective SLR for this corridor, does it change the policy conclusion — and if so, by how much? A two-scenario sensitivity table (NOAA SLR only vs. NOAA SLR + subsidence) would substantially strengthen the credibility of the 2050 priority order claim.
