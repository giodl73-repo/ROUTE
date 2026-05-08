---
reviewer: Susan Hanson
persona: Susan Hanson — Distinguished University Professor Emerita, Clark University School of Geography; past president, Association of American Geographers; specialist in transport geography and spatial accessibility
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The spatial analysis methodology is more transparent than most transport-equity papers I review, and the explicit non-overlapping catchment correction is a meaningful methodological improvement over naive buffer-sum approaches. The C3 correlation finding (r=0.68) is genuinely interesting and worth pursuing. My concerns are: (1) the 30-mile radius is not calibrated to any empirically derived travel shed; it is a round number that the paper treats as self-evident; (2) the ACS B08201 proxy for transit dependency captures vehicle ownership but not travel behavior, and the two diverge significantly in rural areas; (3) the overlap correction methodology is described but not tested for sensitivity to hub location uncertainty. These are fixable methodological issues, not fundamental flaws — the spatial framework is sound in principle.

## What Works

The non-overlapping catchment construction is methodologically careful. Allocating population to the nearest hub (rather than allowing overlap) is the correct approach for a facility-location problem, and the paper implements it explicitly. This prevents the double-counting error that plagues many transit access analyses. The 12.4M figure, whatever its operational validity (which other reviewers address), is at least not inflated by catchment overlap.

The C3 alignment finding deserves more attention than the paper gives it. A Pearson correlation of r=0.68 between an economic opportunity rubric dimension and transit-dependent household density at hub locations is a substantive finding — it suggests the freight-optimized hub location methodology produces equity-relevant outcomes as a structural byproduct, not by design. This is worth a dedicated subsection and should be connected to the distributional justice literature in transport geography (Martens, 2016; Lucas, 2012).

The use of T1/T1 intersection locations as hub candidates is spatially justified — these are, by definition, the nodes of highest network connectivity in the I2.0 system, and network centrality correlates with population density in the US highway system.

## What Doesn't Work

The 30-mile radius is not defended empirically. Intercity bus catchment areas vary significantly by urban density, local transit connectivity, and the availability of feeder services. In a dense metropolitan area with good local transit, 15 miles may be the effective catchment. In a rural area with no local transit, 5 miles may be the operational limit for transit-dependent travelers (walking or cycling access only). The paper should either (a) cite empirical intercity bus catchment literature to justify 30 miles, or (b) run a sensitivity analysis at 10, 20, and 30 miles and report how the 12.4M figure changes.

ACS B08201 (vehicle availability by household) is a valid first-order proxy for transit dependency, but it overstates transit dependency in rural areas where zero-vehicle households are often elderly non-drivers served by family members, not transit-dependent commuters. The paper should cross-reference with ACS B08301 (means of transportation to work) to identify the subset of zero-vehicle households that actually use transit as their primary mode — this population is genuinely transit-dependent and will be a fraction of the B08201 estimate in rural hub catchments.

The spatial error model (if any) for hub location uncertainty is not discussed. T1/T1 hub locations are conceptual — the paper acknowledges that exact locations will be determined in the I2.0 design phase. If hub locations shift by 5-10 miles from the conceptual locations used in the catchment analysis, how sensitive is the 12.4M figure? A Monte Carlo perturbation analysis on hub locations (even with small N) would bound this uncertainty appropriately.

## The Question I'd Push On

The C3 correlation of r=0.68 is your most interesting spatial finding. What is the spatial structure of the residual? Are there hub locations where C3 scores are high but transit-dependent density is low (freight-priority hubs that happen not to serve transit-dependent communities)? Conversely, are there high-transit-dependency areas that are not near any T1/T1 hub — the spatial equity gaps in your own network? Mapping these residuals would tell you whether the T1/T1 hub network has systematic equity blind spots, which is a more important finding than the aggregate correlation.
