---
name: Data Source Catalog
slug: data-sources
type: spec
status: reviewed
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-07-11
sources: []
---

# Data Source Catalog

All data used in ROUTE corridor scoring must be cited to one of these sources. If a source not listed here is needed, add it here first.

No raw data files are committed to this repo. Data is accessed from authoritative sources and cited by specific publication, year, and access URL.

---

## Traffic and volume data

| Source | What it provides | Notes |
|---|---|---|
| FHWA Highway Statistics (annual) | AADT by route, VMT by functional class, truck percentage | https://www.fhwa.dot.gov/policyinformation/statistics.cfm |
| FHWA HPMS (Highway Performance Monitoring System) | Segment-level traffic, pavement condition, lane count | Current portal: https://data.transportation.gov/Roadways-and-Bridges/Highway-Performance-Monitoring-System-HPMS-/jc5k-rzm8. ROUTE's implemented `route fetch-hpms` path uses public state-hosted 2018 ArcGIS services; keep that vintage explicit. |
| FHWA National Freight Strategic Plan | Corridor-level freight context | fhwa.dot.gov/fastact/publications/freightstrategicplan/ |

## Freight data

| Source | What it provides | Notes |
|---|---|---|
| ATRI Top Truck Bottleneck Report (annual) | Congestion cost ranking for major truck freight points | https://truckingresearch.org — free public download |
| BTS Freight Facts and Figures (annual) | Ton-miles by mode, commodity flows, freight value by corridor | https://www.bts.gov/freight |
| FAF5 (Freight Analysis Framework 5) | Origin-destination commodity flow by mode and region | https://ops.fhwa.dot.gov/freight/freight_analysis/faf/ |
| FHWA Freight Performance Measures | Travel time reliability for freight on NHS corridors | https://ops.fhwa.dot.gov/freight/freight_analysis/perform_meas/ |

## Infrastructure condition

| Source | What it provides | Notes |
|---|---|---|
| FHWA National Bridge Inventory (NBI) | Bridge condition, age, posted weight limits | Current public delimited files: https://www.fhwa.dot.gov/bridge/nbi/ascii2025.cfm. NBI is excluded from reviewed I-80 regeneration until a fixture-backed raw-to-route summary adapter exists. |
| FHWA HPMS Pavement Data | IRI (International Roughness Index) by segment | Via HPMS DataFinder |
| FHWA Pavement Performance | % of NHS in good/fair/poor condition by state | https://www.fhwa.dot.gov/policyinformation/statistics.cfm |

## Population and geographic data

| Source | What it provides | Notes |
|---|---|---|
| US Census Bureau TIGER/Line | Road network, census tract boundaries, county boundaries | ROUTE manifest currently pins 2023. Current candidate directory: https://www2.census.gov/geo/tiger/TIGER2025/PRIMARYROADS/; update only after parser compatibility tests. |
| Census ACS 5-Year Estimates | Population by tract, income, poverty rate | `route fetch-acs` and `route fetch-acs-income` read `CENSUS_API_KEY` from the environment. The reviewed report remains fixed to the 2022 vintage consumed by scoring. |
| BEA CAINC4 | County personal income and population context used by the C3 source contract | Candidate download: https://apps.bea.gov/regional/zip/CAINC4.zip. ROUTE manifest names the 2022 ZIP but has no clean-clone fetch and join path; generated reports must not cite it unconditionally. |
| USDA Economic Research Service | Rural classification, agricultural data, farm-to-market metrics | ROUTE downloads the official 2023 RUCC CSV from https://www.ers.usda.gov/media/5768/2023-rural-urban-continuum-codes.csv?v=66892 and normalizes it to `data/cache/rucc_2023.csv`. |
| HRSA Health Resources | Rural hospital access, trauma center locations | https://data.hrsa.gov |

## Safety

| Source | What it provides | Notes |
|---|---|---|
| NHTSA FARS 2022 national CSV | Fatal crashes and roadway context | Official bulk ZIP: https://static.nhtsa.gov/nhtsa/downloads/FARS/2022/National/FARS2022NationalCSV.zip. A5 is excluded from reviewed I-80 regeneration until route matching and the VMT denominator are fixture-validated. |

## Climate and resilience

| Source | What it provides | Notes |
|---|---|---|
| NOAA National Centers for Environmental Information | Climate normals, extreme weather event data | https://www.ncei.noaa.gov |
| FHWA Climate Change and Extreme Weather Vulnerability Assessment | Corridor-level climate risk by hazard type | FHWA Office of Planning |
| FEMA National Flood Hazard Layer | SFHA (Special Flood Hazard Area) boundaries | Service: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer. ROUTE's legacy tile query is not an I-80 coverage adapter and did not sustain corridor-scale requests; FEMA is excluded from reviewed I-80 regeneration pending replacement. |
| USFS Wildfire Hazard Potential | Wildfire risk classification by area | https://www.firelab.org |

## Proposed corridors

| Source | What it provides | Notes |
|---|---|---|
| FHWA Future Interstate Study (2000) | Proposed new interstate designations with justifications | FHWA archive |
| AASHTO proposed corridor designations | Industry-proposed national freight corridors | AASHTO policy documents |
| State DOT Long-Range Transportation Plans (LRTPs) | State-proposed corridor improvements and new designations | Per-state DOT websites; 20-year planning horizon |
| User ArcGIS Project | User-mapped potential corridors | `C:\Users\giodl\OneDrive\Documents\ArcGIS\Projects\Truck Highways\` — local; not committed |

## Multimodal

| Source | What it provides | Notes |
|---|---|---|
| AAR (Association of American Railroads) | Freight rail network map, intermodal terminal locations | https://www.aar.org |
| FHWA Intermodal Connector Study | Intermodal facility access and connector road condition | FHWA Office of Freight |
| BTS Intermodal Passenger Connectivity Database | Transit connections at major interchanges | https://www.bts.gov |
| DOE Alternative Fuels Station Locator | EV fast charger locations and density | Locator: https://afdc.energy.gov/stations. The developer API is credentialed; DCFC is excluded from reviewed I-80 regeneration until a fixture-tested adapter exists. |

---

## Access notes

- ROUTE's current HPMS fetcher uses public 2018 state ArcGIS services; current-year HPMS acquisition is a separate unresolved path.
- Census ACS API credentials must be supplied from the environment once command support is added.
- FAF5 data is available in GIS and tabular formats; large file sizes
- State DOT LRTPs vary significantly in format and detail; access via individual state DOT websites
- ArcGIS project data is local only; results should be described in prose with geographic references, not as raw GIS files
