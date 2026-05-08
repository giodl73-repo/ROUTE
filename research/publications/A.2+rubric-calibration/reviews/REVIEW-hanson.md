---
reviewer: hanson
persona: Susan Hanson — transport geographer, Clark University; specialist in spatial mobility and accessibility equity
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper is strong on longitudinal consistency (the forward-only protocol is a genuine contribution to how scoring rubrics should be managed across versions) and honest about its spatial data limitations. My concern is geographic: the three new strategic dimensions (A4, B4, C4) each encode a particular view of which places matter — border crossings, military installations, agricultural counties — and this view is spatially uneven in ways the paper does not examine. Corridors serving neither USMCA borders nor military installations nor Corn Belt agriculture receive A4=0, B4=5.0, C4=0, which means the Southeast, the Appalachians, and the Pacific Northwest are structurally undercounted unless they score high on the legacy dimensions.

## What Works

**The dimension-by-dimension table format is analytically transparent.** The I-110 vs. I-80 comparison table shows where the ranking divergence comes from: it is not a single dimension but a structural pattern (A1 rewards congestion, A3-via-IRI rewards rough rural pavement) that produces the inversion. Presenting the full dimension profile rather than only the aggregate score allows geographic interpretation — the reader can see that I-110's high score is driven by urban dimensions (A1, C1, C3) while I-80's improvement under v1.2 is driven by strategic-geographic dimensions (B4, C4).

**C2/C3 correlation acknowledgment is geographically honest.** The paper notes that C3 (Economic Opportunity) tracked C1 (Population Reach) closely in practice because high-population areas are high-GDP areas. This is a genuine geographic insight: the spatial correlation of population and economic productivity means these two dimensions are measuring nearly the same spatial phenomenon. The acknowledgment that C3 is a calibration candidate for v1.3 is appropriate.

**The forward-only protocol handles the spatial equity problem correctly.** The paper could have retroactively rescored corridors under v1.2 and claimed retrospective improvement. Instead, it preserves the v1.0 and v1.1 records — including their spatial biases — as documented history. This matters for geographic analysis: the spatial pattern of v1.0 biases (favoring urban congested corridors) is itself a research finding that would be erased by retroactive rescoring.

**Data limitation documentation is geographically specific.** The B2 partial-graph instability is not reported as a generic data quality problem; the paper names the affected states (TX, TN, VA, WY, and 18 others). This allows a geographic reader to assess which corridor comparisons are unreliable — specifically, comparisons between corridors in covered and uncovered states.

## What Doesn't Work

**The strategic dimension coverage map is absent.** A4 applies only to corridors serving USMCA border crossings — a thin geographic strip along the southern and northern borders. B4's meaningful variation (above the 5.0 STRAHNET baseline) applies only to corridors near major military installations — a spatially clustered set in the Mountain West and Pacific Northwest. C4 applies primarily to Great Plains and Midwest corridors. The paper does not examine whether the three new dimensions together produce systematic geographic gaps — regions where corridors cannot score high on any strategic dimension regardless of their actual importance. The Southeast, Appalachia, and Pacific Northwest all deserve explicit treatment.

**The I-35 A4 score encodes a terminus effect without spatial correction.** I-35 scores A4=8.5 because Laredo is at its terminus. But A4 is applied to the full I-35 corridor (1,568 miles from Laredo to Duluth, MN). The strategic trade function is geographically concentrated in the southern ~300 miles; the northern 1,200 miles through Kansas, Missouri, Iowa, and Minnesota serve a primarily domestic agricultural and urban function. Scoring the entire corridor at A4=8.5 because of the southern terminus inflates the strategic value of the middle and northern segments. A geographically weighted A4 — applying the border function score proportional to distance from the crossing — would be more spatially accurate.

**Dimension independence tests ignore spatial autocorrelation.** The Pearson r values for A4, B4, C4 independence are computed across the 227-corridor sample without accounting for spatial autocorrelation. Corridors in the same region tend to share geographic context — border state corridors share A4 relevance, Great Plains corridors share C4 relevance, Mountain West corridors share B4 relevance. If the 227 corridors are not spatially independent samples, the standard Pearson r significance thresholds do not apply. A Moran's I test on the residuals, or at minimum a note that spatial autocorrelation was not tested, is needed.

## The Question I'd Push On

The paper's three new strategic dimensions were chosen to fix a specific anomaly (I-110 above I-80). The paper shows that A4, B4, C4 resolve this anomaly without creating new ones for the corridors in the before-after table. But has the spatial coverage of the new dimensions been examined across the full 227-corridor corpus? Specifically: are there corridors that moved to lower tiers under v1.2 as a side effect of the threshold rescaling (v1.1 ≥21 → v1.2 ≥26), even though their characteristics did not change? And if so, do those displaced corridors cluster spatially — suggesting the rescaling has a geographic incidence effect?
