---
reviewer: Alan McKinnon
persona: Alan McKinnon — Professor of Logistics, Kühne Logistics University, Hamburg; lead author, ITF freight decarbonization reports
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper's coastal flood analysis is technically sound, but the composite D1 metric is built on a category error that undermines cross-corridor comparability. The SFHA-based scoring for Gulf Coast corridors and the closure-frequency-based scoring for winter corridors (Donner, Snoqualmie) measure different things. Combining them into a single D1 number and then ranking all corridors together produces a comparison that is not scientifically defensible. I cannot recommend acceptance in its current form.

The policy critique of the PROTECT allocation formula is valuable and should be preserved — but it rests on the coastal flood data alone, and the paper would be stronger if it said so explicitly rather than extending to a heterogeneous composite.

Score: 2/4 — requires major revision to the composite metric design, or a restriction of scope to the coastal flood corridors where the methodology is internally consistent.

## What Works

The SFHA consecutive-mile analysis for coastal corridors (I-10 Gulf Coast LA and TX, I-95 Miami area) is methodologically sound. Using FEMA NFHL polygon data to compute contiguous SFHA segments along corridor centerlines is a reasonable geospatial analysis, and the 70/30 weighting between consecutive and total miles is clearly motivated by operational logic. For these corridors, D1 is a coherent and useful metric.

The NOAA SLR projection application, while imprecise (other reviewers have noted the subsidence omission), at least uses consistent data across corridors, which means the directional findings for coastal corridors are credible even if the point estimates are uncertain.

The finding that Gulf Coast I-10 LA becomes the top-ranked climate adaptation priority by 2050 (under the coastal flood metric) is the paper's most important result, and it is supported by the data presented.

## What Doesn't Work

The fundamental problem is the composite D1 metric. The paper applies D1 to five corridors using two incompatible measurement approaches:

- Coastal corridors (I-10 LA, I-10 TX, I-95 Miami area): D1 computed from FEMA NFHL SFHA polygon overlay — a physical geography measurement of floodplain extent along the corridor centerline.
- Winter corridors (I-80 Donner, I-90 Snoqualmie): D1 computed from Caltrans/WSDOT closure logs — an operational/behavioral measure of historical incident frequency.

These are not the same quantity. SFHA mileage is a slow-moving structural characteristic of the landscape. Closure frequency is a volatile operational metric that reflects DOT maintenance practices, equipment investment, and weather variability year to year. A corridor that received $50M in plowing equipment last year will score differently on the closure proxy than an identical corridor without that investment.

More fundamentally, the closure-frequency proxy does not capture the physical climate exposure of winter corridors — it captures how often the operational threshold for closure was exceeded, given whatever maintenance regime was in place. If two corridors have identical physical snowpack exposure but different maintenance budgets, they score differently on the proxy while having identical underlying climate risk.

The result is that Donner D1=7.8 and Gulf Coast TX D1=7.8 are presented as equivalent levels of climate exposure. They are not. A table that ranks these corridors together implies a precision that the heterogeneous methodology cannot deliver.

The paper should either:
(a) Develop a physically grounded winter closure risk metric (accumulated precipitation above freezing threshold, atmospheric river event frequency, or equivalent) that is dimensionally analogous to the SFHA flood metric; or
(b) Present coastal and winter corridors in separate tables with separate metrics, and note that cross-category comparison requires further methodological work.

A third option — restricting the paper to coastal flood corridors, which is where the novel methodology lives — would make for a more focused and credible paper. The winter corridor findings could be left to a companion paper with appropriate methodology.

## The Question I'd Push On

If the winter closure metric is closure frequency (days closed per year), what is the time window used, and is it the same for both Caltrans and WSDOT? Closure frequency on Donner Pass is known to vary substantially year to year (the Atmospheric River winters of 2016–17 and 2022–23 were outliers). A 10-year window centered on 2016–17 or 2022–23 would score Donner substantially higher than a 20-year window; a window ending in 2020 (La Niña, below-average precipitation) would score it lower.

The paper should report: (1) the time window used; (2) the inter-annual coefficient of variation for the closure frequency proxy on each corridor; and (3) a sensitivity analysis showing how D1 for winter corridors changes across high/average/low precipitation years. Without this, the winter corridor D1 scores are point estimates with unknown — and potentially large — uncertainty bands that could reverse the cross-corridor rankings.
