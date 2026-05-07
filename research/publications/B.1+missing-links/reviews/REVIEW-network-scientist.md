---
reviewer: Network Scientist
persona: Lada Adamic (Meta/Michigan) — network analysis, graph algorithms, spatial networks
round: 1
date: 2026-05-07
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

## Overall

The coverage analysis is technically defensible for a policy paper, but the methodology section understates its limitations in ways that network scientists will notice immediately. The nearest-interchange metric ignores network topology: a county centroid 25 miles from an interchange in a straight line may be 60 miles by road if there is no direct connecting route. The paper would benefit from explicitly acknowledging the distinction between Euclidean distance and network distance, and from a spot-check of the discrepancy for a representative sample of gap counties.

## What Works

**The graph construction is transparent and reproducible.** The use of TIGER/Line geometry and the ROUTE HighwayGraph (with documented interchange node identification criteria) is methodologically appropriate. The 1,465 interchange node count is plausible for the national interstate network. The Haversine distance metric is correct for Euclidean approximation.

**The corridor validation** (Section 7.1): Adding proposed corridors to the graph as edges with interchange nodes at 25-mile intervals and re-running the coverage analysis is the right validation approach. It is simple, reproducible, and directly tests the coverage claim. The finding that combined corridors raise coverage from 79.6% to 83.0% (not the full efficiency estimate) is a useful reality check — it shows the model is honest about diminishing returns and county overlap.

**The gap zone clustering is well-motivated.** Using DBSCAN with $\epsilon = 100$ miles and minimum cluster size 5 is a reasonable choice for identifying contiguous geographic zones. The four zones that emerge (Northern Tier, Appalachians, Gulf South, Rural West) match both geographic intuition and policy literature. The method section correctly notes the parameters.

## What Doesn't Work

**Euclidean distance ≠ network distance, and the paper doesn't quantify the discrepancy.** The nearest-interchange metric measures Euclidean (Haversine) distance from a county centroid to the nearest highway interchange. But a county centroid 28 miles from an interchange as the crow flies might be 50 miles by road if the terrain requires a mountain pass or a river crossing. For the 30-mile threshold, the discrepancy matters: a county that appears to be within 30 miles Euclidean may be outside it by road, and vice versa. The paper notes this is a limitation but does not quantify it. For a handful of the most mountainous gap counties (eastern Kentucky, western Virginia, rural Montana), providing the road-network distance versus Euclidean distance would demonstrate the paper's honest calibration of this limitation.

**1,465 interchange nodes seems low.** The TIGER Primary Roads file for the US interstate system should have many more intersection points than 1,465. This low number suggests the graph may be missing some interchanges — particularly at T-intersections where a US route meets an interstate without continuing through (e.g., a US highway dead-ending at an I-80 interchange). If interchange nodes are underrepresented, the coverage analysis understates accessibility and overestimates the gap. The paper should verify this count against FHWA interchange inventory data.

**The combined corridor validation doesn't handle overlapping service areas.** Section 7.1 reports that 12 corridors combined serve 11.1M gap-county residents, noting this is lower than the sum of individual estimates due to "overlap." But the paper doesn't quantify how much overlap there is or which corridor combinations have the most complementary coverage. Understanding corridor interaction effects (do I-3 and I-14 serve entirely different zones, or do they share service area?) would inform phasing decisions.

## The Question I'd Push On

The paper uses county centroids rather than a more granular spatial unit (census tracts, block groups). For analysis purposes, this is understandable — county is the standard unit for policy data. But the county-centroid approach implicitly assumes uniform population distribution within the county. For counties with strongly clustered populations (a city in the northwest corner, empty desert in the southeast), this assumption introduces systematic error. Have you tested whether a subsample of gap counties — specifically the large-area western counties — shows the same coverage patterns at the census tract level as at the county centroid level? If the tract-level analysis puts most of the California and Nevada "gap population" inside the 30-mile threshold, the paper's headline number and zone taxonomy need revision.
