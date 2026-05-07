---
name: Parliament Review — Climate Engineer — route-rust-architecture
slug: R1-climate-engineer-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: climate-engineer
round: 1
status: draft
rubric_version: v1.0
author: climate-engineer
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Climate Resilience Engineer

## Overall

The decision to use FEMA SFHA miles as the primary D1 proxy is defensible for v1.0 and explicitly flagged in §9 — I appreciate the honest scope boundary. However, `fema_sfha_miles` as a single field loses geographic clustering, which is the operationally important characteristic: 10 consecutive miles of flood exposure at a river crossing is a single-point-of-failure; 10 scattered miles of marginal flood zone is background noise. Beyond D1, the architecture has no field for wildfire risk, no field for extreme heat pavement impact, and no field for sea level rise exposure at coastal termini — all of which are material to the 25-year investment horizon. These are acknowledged gaps in §9 but should be more prominently flagged as limitations on what D1 scores actually mean.

## What works

**FEMA SFHA shapefile as a v1.0 proxy (§4.1, §9)**: The Special Flood Hazard Area shapefile is publicly available, machine-readable as `.shp`, and aligns with the no-GDAL constraint. Using it for D1 flood exposure is the right pragmatic choice given the alternative (NOAA climate projection rasters requiring a different parsing stack). The explicit deferral in §9 is honest.

**D2 (Multimodal Integration) as a resilience dimension, not just a service dimension**: Wiring intermodal integration into the resilience scoring framework is correct. When a highway floods, rail provides backup freight capacity — but only if the intermodal connection exists. The architecture correctly treats D2 as both a service quality and a resilience indicator.

**`fema_sfha_miles` in `CorridorAttributes` (§4.3)**: The field exists. That is the prerequisite for D1 scoring. The join from FEMA polygon to corridor centerline (line-in-polygon query using R-tree) is standard geo computation and works with the existing `rstar` dependency.

**§9 honest about raster exclusion**: Explicitly stating that NOAA climate projection rasters are out of scope for v1.0 is better than silently omitting them. Users of the output scores will understand the limitation.

## What doesn't work

**`fema_sfha_miles` loses clustering (§4.3)**: A single float representing total SFHA miles on a 2,000-mile corridor discards the geographic distribution of that exposure. I-10 through Louisiana has long contiguous stretches of coastal flood exposure at highway grade; I-10 through Texas crosses flood-prone rivers at bridges. Both may have similar total SFHA miles; the risk profiles are completely different. A more useful field would be `max_consecutive_sfha_miles` (longest uninterrupted flood-exposed segment) alongside the total. The longest contiguous exposed segment is what determines whether a storm closure is a 10-mile detour or a 200-mile one. This is computable from the same FEMA join with minimal additional logic.

**No wildfire field (§4.3)**: The western US wildfire risk to highway infrastructure has materially increased since 2015. I-5 in California, I-90 in Washington, US-2 through the Cascades — all have experienced significant closures from wildfire smoke and active fire. The USFS Wildfire Hazard Potential raster is in `data/sources.md` but there is no field in `CorridorAttributes` for it. I understand rasters are deferred, but the USFS also publishes a vector (polygon) version of high/very-high wildfire hazard areas that could be joined as `wildfire_hazard_miles` using the same approach as the FEMA join. This is not a raster problem. It is a polygon-overlay problem that the existing stack handles.

**EV charging infrastructure has no data field (§4.3, §5)**: The spec lists EV charging corridor as a key Interstate 2.0 feature (in the design spec §5), and `data/sources.md` includes DOE Alternative Fuels Station Locator. But `CorridorAttributes` has no field for DC fast charger density. D2 (Multimodal Integration) is the natural home for this — it captures the corridor's readiness for the EV transition which is a long-horizon resilience factor (petroleum supply chain is a climate risk; electrification reduces it). Add `dcfc_per_100mi` (DC fast charger count per 100 miles) to `CorridorAttributes`. The DOE data is a CSV with coordinates; the join is a point-in-buffer query over the corridor, same as the population join.

## The question I'd push on

D1 (Climate Resilience) scores high when a corridor has high FEMA SFHA exposure. But the D1 scoring anchor (higher = more at risk) means a high D1 score is a flag for investment in resilience hardening, not a reason to deprioritize the corridor for investment. In the gap analysis, `route gap --type resilience` will surface high-D1 corridors — but the output needs to distinguish "this corridor needs hardening" from "this corridor should not be built in this location." The spec does not say how the resilience gap finding is interpreted. A coastal corridor that serves 15 million people and scores 9.0 on D1 needs hardening investment; a proposed greenfield corridor scoring 9.0 on D1 through a coastal flood zone may not be buildable at reasonable cost. The gap output and design proposal framework need to accommodate this distinction before the climate engineer stake can be fully evaluated.
