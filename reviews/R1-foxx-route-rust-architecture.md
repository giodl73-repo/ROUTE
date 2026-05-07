---
name: Parliament Review — Foxx — route-rust-architecture
slug: R1-foxx-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: foxx
round: 1
status: draft
rubric_version: v1.0
author: foxx
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Anthony Foxx

## Overall

The architecture is technically credible. My concern is that the equity dimensions — C3 (Equity Access) and C2 (Rural Connectivity) — are implemented with proxies that systematically undercount the populations who have the least transportation choice. `pct_pop_below_poverty` is the most commonly available measure and also the most commonly wrong one for this purpose. Additionally, the data pipeline has no representation of how proposed corridors affect communities — not just who is served, but who is displaced, bisected, or burdened by noise and pollution. If this system is going to inform infrastructure investment decisions, that absence matters.

## What works

**`pop_within_50mi` and `pct_pop_below_poverty` as fields (§4.3)**: These exist. That is more than most infrastructure analysis tools start with. The fact that equity data is in `CorridorAttributes` at all — not as an afterthought but as first-class fields — is the right instinct.

**C3 scoring is dimension-level, not a footnote (§5)**: Equity Access is one of 12 scored dimensions, not a disclaimer. It feeds the gap analysis. `route gap --type equity` is a named gap type. This is the right architecture — equity is a gap to be found and addressed, not a box to check.

**USDA ERS rural classification in data sources**: `data/sources.md` lists USDA ERS rural classification data. This is the right source for understanding rural access. The fact that it is listed — even if not yet wired into `CorridorAttributes` — means someone thought about it.

**`route gap --type equity` (§6)**: A named equity gap command that produces findings. This matters because equity gaps without a named finding mechanism tend to stay invisible.

## What doesn't work

**`pct_pop_below_poverty` is the wrong primary equity field (§4.3)**: Poverty rate captures one dimension of transportation disadvantage. It misses: elderly populations without driving ability, disabled populations, car-free households by choice or necessity, tribal communities (which have their own classification system separate from poverty), and formerly redlined neighborhoods that are now gentrifying but still lack transit access. The equity score built on this single field will systematically undercount transit-dependent populations in medium-income areas — exactly the populations most affected by the original interstate construction.

I am not saying remove it. I am saying it should be one of three or four fields, not the only one. Add at minimum: `pct_no_vehicle_households` (from ACS table B08201) and `tribal_land_miles` (from Census TIGER tribal boundaries). Both are free, both are in the declared data sources, neither requires new fetch infrastructure.

**Proposed corridor alignment is equity-blind (§9)**: The spec explicitly defers proposed corridor geometry to "ArcGIS project exports or hand-drawn alignments." The original interstate system used hand-drawn alignments routed through Black neighborhoods, poor communities, and politically weak areas. If ROUTE's proposed corridor alignments are evaluated only for network efficiency and not for what communities they bisect or burden, the project replicates the original system's equity failures while claiming to address them. This is not a v2.0 feature. The alignment choice IS the equity decision. At minimum, the spec should acknowledge that alignment evaluation for proposed corridors requires community impact analysis and flag it as out-of-scope with an explicit note that scores for proposed corridors cannot fully address C3 without it.

**`intermodal_hub_count` understates multimodal equity (§4.3)**: Intermodal freight hubs serve freight, not people. D2 (Multimodal Integration) is supposed to capture transit potential. A corridor with five intermodal freight hubs and zero intercity bus stops scores high on D2 by this data model but provides nothing for transit-dependent travelers. Add `intercity_bus_stops` and `park_and_ride_count` as fields. Both are available from BTS and state DOTs.

## The question I'd push on

The equity gap detection algorithm is not specified. `route gap --type equity` is listed as a command but §6 does not say what threshold or combination of dimension scores triggers an equity gap flag. Is it C3 score above a threshold? Is it C3 high + C1 low (high need, low absolute service)? Is it a combination of C2 and C3? This matters enormously because the algorithm determines which communities get named as underserved and which don't. A gap detection algorithm that flags Wyoming as an equity gap (low population, high C2 rural connectivity score) but misses a specific tribal nation in the same state is a broken algorithm. I want the equity gap detection logic specified before the gap command is implemented.
