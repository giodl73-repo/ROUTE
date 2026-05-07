# Plan: Missing Links — Gap Analysis of the US Interstate Network

**Track**: B — Gap Analysis
**Venue**: Transportation Research Part A
**Target**: 9,000–11,000 words

## The Question

20.4% of Americans — 66.5 million people — live in counties whose centroid is more than 30 miles from any interstate on-ramp. That is the gap. Where are they? What kind of gap do they face? Which proposed corridors would close those gaps most efficiently? And does the gap correlate with the economic opportunity dimension (C3) that ROUTE already measures?

## The Method

1. **Coverage analysis**: 3,139 US county centroids (Census Gazetteer 2023) × nearest interchange node in the HighwayGraph. Measure distance to nearest on-ramp. Report: % within 20/30/50 miles, by county, by state, by rural classification.
2. **Gap taxonomy**: classify gap counties into four types based on geographic clustering and cause:
   - Type 1: T1 Geographic Gaps (entire zones with no T1 within 300 miles)
   - Type 2: T2 Missing Links (urban/suburban populations between T1 corridors)
   - Type 3: Rural Isolation (sparse counties >30 miles with no viable T2 candidate)
   - Type 4: Economic Opportunity Gaps (below-national-median GDP per capita counties in gap zones)
3. **Priority corridors**: for each gap zone, identify the proposed corridor that would close the most gap-county coverage. Score proposed corridors against calibrated ROUTE rubric. Rank by: gap population served ÷ estimated corridor cost.
4. **Validation**: after adding each priority corridor to the graph, re-run coverage analysis. Measure: how many gap counties move inside 30-mile threshold?

## The Primary Finding

- 1,510 continental US gap counties; 66.5M people (20.4% of US)
- Gap clusters in 4 zones:
  - **Northern Tier** (ND/MT/MN, 44+44 counties): Northern Tier interstate gap confirmed
  - **Appalachians** (WV/KY/VA/TN, 26+66+30+42 counties): no E-W interstate through central Appalachians
  - **Rural South/Gulf** (LA/MS/AR/TX rural, 45+40+50+145 counties): between I-10 and I-20
  - **Rural West** (NM/NV/UT/CO/ID, 21+13+20+44+28 counties): sparse but enormous land area
- 12 priority corridors would bring ~85% of gap population within 30 miles
- Gap counties have mean C3 (Economic Opportunity) score 40% higher than non-gap counties
  — the coverage gap IS an economic opportunity gap

## Quantification Contract

- **Primary number**: 66.5M Americans (20.4% of US) in counties >30 miles from any interstate
- **Secondary**: 12 priority corridors × gap population served ÷ estimated cost = ranking
- **Tertiary**: C3 score differential — gap counties vs non-gap counties

## Sections

1. Introduction
2. Background
3. Data and Methods
4. The 30-Mile Standard — national findings
5. Gap Taxonomy — four types, four zones
6. Priority Corridors — top 12 ranked by coverage efficiency
7. Validation — does adding corridors close the gap?
8. Conclusion

## Data Sources

- data/coverage-gaps.csv (1,510 gap counties, from route coverage)
- data/scores-all.csv (227 existing corridor scores)
- ROUTE HighwayGraph (TIGER 2023 + HPMS 2018)
- Census Gazetteer 2023 county centroids
- ACS 2022 county population
- BEA CAINC4 2022 (for C3 economic opportunity scoring)
- FHWA proposed corridor studies (FHWA Future Interstate Study 2000 + state LRTPs)
