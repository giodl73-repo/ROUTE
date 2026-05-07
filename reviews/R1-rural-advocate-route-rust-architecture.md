---
name: Parliament Review — Rural Advocate — route-rust-architecture
slug: R1-rural-advocate-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: rural-advocate
round: 1
status: draft
rubric_version: v1.0
author: rural-advocate
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Rural Advocate

## Overall

The architecture acknowledges rural connectivity as a named dimension (C2) and a named gap type (`route gap --type equity`). That is further than most infrastructure analysis gets. My concern is that `pop_within_50mi` as the primary population field — without a rural/urban split — will systematically make rural corridors look low-priority because they serve fewer total people, even when those people have no alternative. I-90 through South Dakota serves 400,000 people in a 50-mile buffer; I-95 through Connecticut serves 4 million. The raw population count makes South Dakota look like a low-priority corridor, but it is the only interstate within 200 miles for a significant portion of that population. The data model needs to distinguish access value from total population served.

## What works

**C2 (Rural Connectivity) as a named dimension**: Rural connectivity is not a footnote or a tiebreaker. It is one of twelve scored dimensions with its own scoring anchor and its own gap type. The architecture respects this. Thank you.

**USDA ERS rural classification in `data/sources.md`**: The Economic Research Service rural-urban continuum codes are the correct classification system for rural status. They are in the declared sources. The gap is that they are not yet wired into `CorridorAttributes`.

**`route gap --type equity`**: Rural underservice is an equity gap. Naming it as a gap type that produces a finding — not just a score — is the right approach. Findings get acted on; scores get filed.

**Anchor corridor I-80**: I-80 crosses Wyoming, Nevada, and rural Iowa — high-rural-connectivity segments mixed with urban California. It is a good anchor for calibrating C2 precisely because it has such variance within a single route.

## What doesn't work

**`pop_within_50mi` without rural/urban decomposition (§4.3)**: Total population in a 50-mile buffer measures service volume, not access value. The two are different things. I-80 through Elko County, Nevada: 50,000 people in a 50-mile buffer, essentially all rural, nearest alternative highway adds 4+ hours. I-80 through Sacramento: 2.5 million people in a 50-mile buffer, multiple parallel routes, BART, Amtrak. The Elko segment scores low on C1 (Population Reach) and the Sacramento segment scores high — which is correct — but C2 (Rural Connectivity) needs to be the corrective. To make C2 meaningful, `CorridorAttributes` needs:
  - `rural_pop_within_50mi: Option<u64>` (USDA ERS rural codes 4-9 within buffer)
  - `pct_rural_in_buffer: Option<f32>` (rural as share of total buffer population)
  - `nearest_parallel_us_highway_miles: Option<f32>` (distance to nearest non-interstate alternative)

Without these, C2 scores will be approximated from `pct_pop_below_poverty`, which is a proxy for rural condition but an imprecise one.

**No field for on-ramp spacing in rural segments (§4.3)**: For a rural farmer, the distance from the farm to the nearest on-ramp matters more than whether the interstate exists at all. I-80 through Wyoming has 80-100 mile gaps between on-ramps in some segments. I-90 through Montana has similar gaps. This is a distinct rural access characteristic not captured by any current `CorridorAttributes` field. Add `max_rural_interchange_gap_miles: Option<f32>` — the longest gap between interchanges in segments classified as rural. This is computable from the NHS shapefile (interchange locations are nodes in the graph) joined with USDA rural classification. A 100-mile interchange gap is the geographic definition of a corridor that exists on paper but does not serve the communities it passes through.

**Equity gap threshold is unspecified**: I share Foxx's concern. `route gap --type equity` needs a defined algorithm. My specific addition: the equity gap should be triggered by a combination of high C2 (rural connectivity score, meaning high rural need) AND low C1 (low total population, meaning low political visibility). Rural communities that are underserved and politically invisible are the ones the gap analysis should surface. A corridor that scores 8.0 on C2 (critical rural access) and 2.0 on C1 (low total population) is the equity gap that needs a name and a finding.

## The question I'd push on

C2 (Rural Connectivity) has a scoring anchor: "0 = primarily urban; 10 = primary access route for large agricultural region." But the spec doesn't say how `route-score` distinguishes "primarily urban" from "primary access for agricultural region" computationally. If C2 scoring is derived from `pct_pop_below_poverty` (the only current rural-proxy field), then a low-poverty rural corridor — say, a prosperous agricultural region in Iowa — scores incorrectly low on C2 despite being a critical grain-to-elevator corridor. How does the scoring function distinguish rural agricultural importance from poverty? The two are related but not the same, and the scoring function as implied by the current data model cannot separate them.
