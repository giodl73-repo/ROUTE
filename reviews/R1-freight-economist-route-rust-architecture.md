---
name: Parliament Review — Freight Economist — route-rust-architecture
slug: R1-freight-economist-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: freight-economist
round: 1
status: draft
rubric_version: v1.0
author: freight-economist
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Freight Economist

## Overall

The data sources are credible and the FAF5 commodity flow integration is a genuine strength — this is one of very few highway analysis frameworks that I have seen attempt to wire commodity value into corridor scoring. My concern is that the A2 (Freight Intensity) scoring function as specified collapses freight value to vehicle counts, which loses the economic information that makes commodity flow data worth collecting. A corridor moving pharmaceutical products and one moving gravel can have identical truck counts and completely different economic significance. The architecture supports getting this right; the scoring spec as written does not.

## What works

**FAF5 flows in §4.1**: The Freight Analysis Framework 5 origin-destination data is the right source for commodity flows. It gives us tons, value, and mode by region pair. Most highway analyses stop at AADT and truck percentage; including FAF5 means we can compute commodity value per corridor mile, which is the economically meaningful metric. This is a real advantage over ATRI-only analysis.

**`mean_pct_truck * mean_aadt` for A2 as a starting point (§5)**: The trucks-per-day calculation is directionally correct as a first approximation. For the anchor (I-80), where HPMS data is complete and reliable, this will produce a defensible score.

**B3 (Port/Border Access) as a named dimension**: Port connectivity is an economic multiplier. A corridor connecting the agricultural interior to a Gulf export terminal has higher freight value than its raw truck count suggests because it enables export economics. Naming this as a separate dimension — rather than folding it into A2 — is the right choice.

**`route score-all` with parallel scoring (§6)**: Scoring 70 corridors is a batch job. Rayon parallelism here is correct; corridor scores are independent.

**Scoring functions are pure (§5)**: "No I/O, no state." Good. Pure functions are testable. A scoring function you can unit-test against known inputs is a scoring function you can trust.

## What doesn't work

**A2 scoring loses commodity value (§5)**: The example scoring function for A2 computes `trucks_per_day` from `mean_pct_truck * mean_aadt`. This measures vehicle count, not freight value. I-80 through Nevada carries a different commodity mix than I-10 through Texas near Laredo — the Laredo crossing handles $300B+ in annual trade with Mexico. Both may have similar truck percentages; the economic significance is radically different. `CorridorAttributes` has `FAF5 flows` as a data source in §4.1 but no `commodity_value_per_mile` field in the struct (§4.3). This is the gap. Add `annual_freight_value_b` (annual freight value in billions) to `CorridorAttributes` and make it the primary A2 signal, with trucks-per-day as a secondary check.

**B3 scoring algorithm is entirely unspecified (§5)**: B3 (Port/Border Access) is listed in `DimensionScores` but the spec contains no scoring function, no anchor, and no data field in `CorridorAttributes` for port connectivity. §4.1 lists FAF5 flows as a source, and §4.3 has no `port_access` field. Either a `port_terminus_flag` (bool) or `nearest_top10_port_miles` (f64) needs to be in the struct, and the B3 scoring function needs to be specified. This is a blocker for the freight economist stake — B3 is my secondary dimension and it has no implementation path.

**Scoring anchors compiled as constants is wrong for calibration (§5)**: I agree with Moses on this point and add an economic reason: the calibration pass will almost certainly reveal that the scoring anchors need rescaling after the first 20 corridors are scored. If anchors are compiled constants, each rescaling requires a new binary. For a scoring system designed to iterate, runtime-configurable anchors are correct. A TOML config file loaded at startup costs one afternoon of implementation and saves weeks of friction during calibration.

## The question I'd push on

FAF5 data is organized by origin-destination region pair and mode, not by NHS route ID. To get commodity flows for I-80 specifically, you need to aggregate FAF5 flows for all O-D pairs where I-80 is the likely route — which requires either a routing step (expensive) or an approximation based on FAF5 zones that the corridor traverses (imprecise). The spec is silent on how FAF5 flows are attributed to specific corridors. "Manifest URL → CSV" describes how FAF5 is fetched, not how it is joined to corridors. This is not a minor implementation detail — it is the most analytically complex join in the entire pipeline. How does `route-data` attribute FAF5 flows to a specific corridor? If the answer is "we use the FAF5 zones the corridor passes through," that needs to be stated and its limitations acknowledged. If the answer is "we route flows through the graph," that is a significant piece of work that needs to be in scope before A2 and B3 can be properly scored.
