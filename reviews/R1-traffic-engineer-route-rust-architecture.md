---
name: Parliament Review — Traffic Engineer — route-rust-architecture
slug: R1-traffic-engineer-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: traffic-engineer
round: 1
status: draft
rubric_version: v1.0
author: traffic-engineer
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Traffic Engineer

## Overall

Technically competent architecture with good data source selection. IRI from HPMS for D3 (Infrastructure Vintage) is the right pavement metric. The NBI join for bridge condition is correct. My primary concerns are operational: the bridge proximity join tolerance is too loose for dense interchange areas, A3 (Speed Reliability) is pointed at the wrong data source, and the `parallel_interstate_count` field for B1 (Redundancy) is too coarse to be operationally useful. These are fixable before the anchor run.

## What works

**IRI from HPMS for pavement condition (§4.1)**: International Roughness Index is the correct field for pavement quality assessment. It is measured, reported, and available at the segment level in HPMS. Using it for D3 (Infrastructure Vintage) scoring is correct — IRI captures actual pavement condition, not just age, and a recently-repaved pre-1970 corridor scores better than an unrenovated post-1990 one. Good.

**NBI for bridge condition (§4.1)**: The National Bridge Inventory sufficiency rating and condition fields are the authoritative source for bridge quality. Using coordinate proximity join to snap NBI records to NHS segments is the right approach given that NBI and NHS have no shared identifier.

**NHS_TYPE field for route filtering (§4.1)**: The NHS shapefile includes `NHS_TYPE` which distinguishes principal arterials, intermodal connectors, and other NHS routes. Using this to filter to full-access-controlled interstates for the corpus is necessary — otherwise we'd be scoring NHS principal arterials that don't meet interstate design standards.

**`route build` join report (§6)**: "Report: N edges, N nodes, N routes, N join failures." Visibility into join failures is essential QA. Most pipeline specs omit this; this one doesn't.

## What doesn't work

**Bridge join tolerance is too loose (§8)**: "R-tree, ≤0.01° tolerance" — at 40°N latitude, 0.01° longitude is approximately 850 meters. In a dense urban interchange area (I-80/I-280 junction in San Francisco, or the I-90/I-94/I-290 mess in Chicago), there may be a dozen bridge structures within 850 meters, and the nearest one in the R-tree may not be the one on the NHS route. A tighter tolerance (0.002° ≈ 170m) with a route-name similarity check (`NBI.ROUTE_NUM` contains the interstate number) would eliminate most false joins. The current tolerance will create noise in D3 scores for urban corridors.

**A3 (Speed Reliability) needs a different data source (§4.1)**: The spec lists IRI as the source for A3 alongside D3. IRI measures pavement roughness, not travel speed or reliability. A smooth pavement on a chronically congested corridor (I-405 in LA has excellent pavement and terrible speed reliability) scores incorrectly using IRI for A3. The correct source for A3 is FHWA's Freight Performance Measures program, which publishes truck Travel Time Reliability (TTR) and Truck Travel Time (TTT) by corridor using GPS probe data. Specifically: the Planning Time Index (95th percentile travel time / free-flow travel time) is the right metric. This data is free, published annually by FHWA, and route-level. It needs to be added to §4.1 and wired to A3 separately from IRI.

**`parallel_interstate_count: u8` for B1 is too coarse (§4.3)**: A count of parallel interstates within 50 miles does not capture redundancy quality. I-15 and US-95 running parallel to I-80 across Nevada is very different from I-78 and I-287 running parallel in New Jersey. The count is the same (1 or 2), but the Nevada "alternative" adds 150 miles and crosses a mountain range; the New Jersey alternative adds 20 minutes. Replace `parallel_interstate_count: u8` with `nearest_parallel_miles: f64` (distance to nearest parallel interstate-quality alternative) and `detour_penalty_miles: f64` (additional distance of best alternative route for the full corridor). Both are computable from the graph.

## The question I'd push on

A1 (Throughput Gap) is defined as "current volume vs. designed capacity; congestion severity across route miles." The designed capacity of a standard 2-lane-each-direction interstate at 60mph is approximately 3,200 pcph (passenger car equivalents per hour per lane) under HCM LOS D conditions. But the NHS shapefile does not include lane count per segment. HPMS does, but lane count is one of the fields with the most inconsistent state reporting. Without lane count, V/C ratio is not computable — we can report AADT but not whether that AADT represents LOS B or LOS E. How does `route-data` handle missing or inconsistent lane count data? If we default to 4 lanes (standard interstate), that underestimates capacity on 6- and 8-lane urban sections and overestimates on 2-lane rural sections. This needs a specified fallback before A1 scoring is defensible.
