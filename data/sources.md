---
name: Data Source Catalog
slug: data-sources
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
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
| FHWA HPMS (Highway Performance Monitoring System) | Segment-level traffic, pavement condition, lane count | Requires state DOT data access; summary reports public |
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
| FHWA National Bridge Inventory (NBI) | Bridge condition, sufficiency rating, age, posted weight limits | https://www.fhwa.dot.gov/bridge/nbi.cfm |
| FHWA HPMS Pavement Data | IRI (International Roughness Index) by segment | Via HPMS DataFinder |
| FHWA Pavement Performance | % of NHS in good/fair/poor condition by state | https://www.fhwa.dot.gov/policyinformation/statistics.cfm |

## Population and geographic data

| Source | What it provides | Notes |
|---|---|---|
| US Census Bureau TIGER/Line | Road network, census tract boundaries, county boundaries | https://www.census.gov/geo/maps-data/data/tiger.html |
| Census ACS 5-Year Estimates | Population by tract, income, poverty rate | https://data.census.gov |
| USDA Economic Research Service | Rural classification, agricultural data, farm-to-market metrics | https://www.ers.usda.gov |
| HRSA Health Resources | Rural hospital access, trauma center locations | https://data.hrsa.gov |

## Climate and resilience

| Source | What it provides | Notes |
|---|---|---|
| NOAA National Centers for Environmental Information | Climate normals, extreme weather event data | https://www.ncei.noaa.gov |
| FHWA Climate Change and Extreme Weather Vulnerability Assessment | Corridor-level climate risk by hazard type | FHWA Office of Planning |
| FEMA National Flood Hazard Layer | SFHA (Special Flood Hazard Area) boundaries | https://msc.fema.gov |
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
| DOE Alternative Fuels Station Locator | EV fast charger locations and density | https://afdc.energy.gov/stations |

---

## Access notes

- HPMS DataFinder requires a registration (free): https://datahub.transportation.gov/browse?q=hpms
- FAF5 data is available in GIS and tabular formats; large file sizes
- State DOT LRTPs vary significantly in format and detail; access via individual state DOT websites
- ArcGIS project data is local only; results should be described in prose with geographic references, not as raw GIS files
