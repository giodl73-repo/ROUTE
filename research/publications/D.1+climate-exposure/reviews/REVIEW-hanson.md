---
reviewer: Susan Hanson
persona: Susan Hanson — Distinguished University Professor Emerita of Geography, Clark University; former editor, Annals of the Association of American Geographers
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper addresses a genuine gap: existing federal climate-adaptation funding metrics for highway infrastructure do not adequately weight the spatial concentration of flood exposure along continuous corridor segments. The D1 composite metric is a meaningful methodological advance over simple total-mileage accounting, and the policy implication — that PROTECT program allocation is systematically skewed against the Louisiana Gulf Coast — is worth publishing.

My reservations are primarily spatial-methodological. The paper's reliance on FEMA NFHL as its primary flood exposure layer introduces systematic coverage gaps that are not acknowledged. In rural portions of the corridor network, NFHL coverage is incomplete, which means the analysis may underestimate exposure in exactly the corridors where alternative data are most sparse. This is a known limitation of the NFHL and should be foregrounded, not buried.

Score: 3/4 — publishable with revision addressing the NFHL coverage gap issue and the rural corridor representation problem.

## What Works

The conceptual distinction between consecutive and total SFHA exposure is spatially astute. Flood zone geography is not uniform along a corridor — the typical pattern is alternating high-ground and low-ground segments, with the low-ground segments (river crossings, coastal floodplains, urban fill areas) creating the operational risk. The D1 metric correctly identifies that a corridor whose low-ground segments are concatenated — as on I-10 across coastal Louisiana — is categorically more vulnerable than one where the same total mileage is distributed across isolated crossings.

The spatial ranking produced by D1 is credible on its face: I-10 Gulf Coast LA at the top, followed by I-10 TX, with winter-closure corridors (Donner, Snoqualmie) in the middle tier. This roughly matches what a geographer would expect from the physical geography of these corridors, which is mild evidence of construct validity.

The NOAA SLR projection application, while methodologically imprecise (see below), at least uses a standard federal scenario rather than cherry-picking a high-alarm or low-alarm projection. This choice is defensible for a policy-facing paper in *Nature Climate Change*.

## What Doesn't Work

The FEMA NFHL has well-documented spatial coverage gaps, particularly in rural areas. Approximately 35% of stream miles in the United States lack an effective FIRM panel, and coverage is substantially lower in sparsely populated counties. The analysis covers several rural corridors — I-80 Donner, I-90 Montana, I-10 TX through rural counties — where NFHL coverage may be incomplete. An undercount of SFHA miles in these segments would understate their D1 scores and could alter the ranking.

The paper does not report whether it verified NFHL panel completeness for each corridor or applied any gap-filling method. This omission is a significant weakness for a paper making cross-corridor comparisons. At minimum, the authors should report the percentage of corridor miles with effective FIRM coverage vs. preliminary or unmapped status, and discuss how gaps could affect the D1 rankings.

A related issue: FEMA updates NFHL continuously, and the publication lag means that newly flooded areas (post-Harvey, post-Ida) may not be captured in the current NFHL panels the authors used. Post-disaster remapping typically takes 3–5 years. The Gulf Coast LA corridor, which experienced major flooding in 2021 (Hurricane Ida), may have pre-Ida FIRM panels underlying the analysis, which would understate its current SFHA exposure.

The winter corridor scoring (Caltrans/WSDOT closure logs) is not georegistered to the SFHA framework. The paper presents Donner and Snoqualmie D1 scores alongside the Gulf Coast scores without making clear that these are computed through entirely different proxy mechanisms. A reader comparing D1=7.8 for Donner against D1=7.8 for I-10 Gulf Coast TX will assume these are commensurable. They are not.

## The Question I'd Push On

FEMA SFHA mapping is a legal and regulatory product — it determines flood insurance rates and building permit requirements — not a scientific best-estimate of actual inundation probability. There is growing evidence that SFHA boundaries systematically understate flood risk in areas with nuisance flooding (frequent but shallow flooding that does not meet the 1% annual chance threshold) and may overstate it in areas where coastal engineering has altered hydrology since the original FIRM was produced.

My question: have the authors compared the NFHL-based D1 scores for any corridors against First Street Foundation flood risk data or USGS flood frequency estimates? If First Street's 100-year flood zone is systematically larger than NFHL for the Gulf Coast LA corridor (which it likely is), then D1=8.4 may be a conservative lower bound. Conversely, if NFHL overstates risk in corridors with upgraded levee systems (portions of I-10 TX), D1 for those segments may be inflated. Understanding this directional bias is important for interpreting the PROTECT allocation critique.
