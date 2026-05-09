---
name: ROUTE Rust Architecture — route CLI v1.0
slug: route-rust-architecture
type: spec
status: reviewed
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources:
  - "FHWA National Highway System documentation"
  - "FHWA HPMS Data Item and Model Definitions, 2023"
  - "FHWA Freight Performance Measures, Annual Report 2023"
  - "ATRI Top 100 Truck Bottleneck Report 2024"
  - "FAF5 Freight Analysis Framework v5.6, BTS/FHWA 2022"
  - "apportionment/bisect Rust codebase (C:\\src\\apportionment)"
  - "ROUTE design spec (specs/2026-05-06-route-design.md)"
---

# ROUTE Rust Architecture — `route` CLI v1.0

## §1. Purpose

This spec defines the Rust implementation of the ROUTE data and analysis pipeline. The binary is named `route`. It fetches authoritative federal highway data, builds a corridor graph, scores corridors against the 16-dimension pool, identifies network gaps, and renders corridor maps. All computation is in Rust; no GDAL, no Python runtime.

The design follows the `bisect` CLI pattern from the apportionment sibling project: manifest-driven data fetching, pure-Rust shapefile parsing, graph-based analysis, and markdown/CSV/PNG output.

---

## §2. Workspace Layout

```
route/
├── Cargo.toml                  ← workspace root
└── crates/
    ├── route-data/             ← fetch + parse FHWA shapefiles + CSV joins
    ├── route-network/          ← highway graph: nodes, edges, network metrics
    ├── route-score/            ← 16-dimension scoring engine
    ├── route-map/              ← corridor map rendering (SVG → PNG)
    ├── route-report/           ← markdown corpus entry + CSV output
    └── route-cli/              ← main binary; all subcommands
```

Each crate has one job. `route-cli` owns no logic — it parses args and dispatches. All business logic lives in the library crates so it can be tested independently.

---

## §3. Key Dependencies

```toml
[workspace.dependencies]
# Geometry
shapefile   = "0.6"      # pure Rust ESRI .shp reader — LineString + Polygon
geo         = "0.28"     # geometry operations: length, buffer, bbox, contains
geo-types   = "0.7"      # Coord, LineString, MultiLineString, Point
rstar       = "0.12"     # R-tree spatial indexing for proximity joins

# Graph analysis
petgraph    = "0.6"      # Dijkstra shortest path, Brandes betweenness centrality
# metis — multilevel graph partitioning for bottleneck/cluster detection
#   NOT added yet (YAGNI — not needed until gap analysis phase).
#   Will depend on a standalone giodl73-repo/METIS crate, not bisect-metis.

# Data handling
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
csv         = "1"
zip         = "2"        # NHS shapefiles distributed as .zip
toml        = "0.8"      # runtime scoring config (config/scoring.toml)

# CLI
clap        = { version = "4", features = ["derive"] }

# Rendering
resvg       = "0.42"     # SVG → PNG rasterizer (same as bisect-map)
tiny-skia   = "0.11"     # pixel buffer backend

# Utilities
rayon       = "1.10"     # parallel segment processing
thiserror   = "2"
anyhow      = "1"
reqwest     = { version = "0.12", features = ["blocking"] }
```

No GDAL. No Python. No PROJ. CRS handling is explicit: NHS shapefiles ship in EPSG:4269 (NAD83 geographic); we project to EPSG:5070 (Albers Equal Area) for area and length calculations, same as `bisect`.

---

## §4. Data Model

### 4.1 Raw inputs

| Source | Format | Key fields | How fetched | Version / citation |
|---|---|---|---|---|
| FHWA NHS shapefile | `.shp` LineString | ROUTE_ID, STATE_CODE, NHS_TYPE, MILES | manifest URL → `.zip` → extract | FHWA NHS, annual release |
| FHWA HPMS summary | CSV | STATE, ROUTE_ID, AADT, PCT_TRUCK (0–100), LANE_COUNT, IRI | manifest URL → CSV | FHWA HPMS 2023 |
| FHWA Freight Performance Measures | CSV | ROUTE_ID, TTI (Travel Time Index), PTI (Planning Time Index) | manifest URL → CSV | FHWA FPM Annual Report 2023 |
| FHWA NBI | CSV | LAT, LONG, ROUTE_NUM, SUFFICIENCY_RATING, YEAR_BUILT, CONDITION | manifest URL → CSV | NBI annual, 2023 |
| ATRI bottleneck seed | CSV (hand-curated) | RANK, LOCATION, ROUTE, ANNUAL_COST_M | committed to `data/atri-bottlenecks.csv` | ATRI Top 100 Truck Bottleneck Report 2024 |
| FAF5 flows | CSV | ORIG_FAF, DEST_FAF, TONS, VALUE_B, DMODE | manifest URL → CSV | FAF5 v5.6, BTS/FHWA 2022 data year |
| Census ACS 5-yr | CSV + `.shp` | GEOID, TOTAL_POP, POVERTY_PCT, NO_VEHICLE_HH_PCT | manifest URL → `.zip` | ACS 2022 5-year estimates |
| USDA ERS rural codes | CSV | FIPS, RUCC_2023 (1–9; ≥4 = rural) | manifest URL → CSV | USDA ERS Rural-Urban Continuum Codes 2023 |
| BEA GDP by county | CSV | FIPS, GDP_2022_M (GDP in millions) | manifest URL → CSV | BEA CAINC4, 2022 |
| FEMA SFHA | `.shp` Polygon | FLD_ZONE, geometry | manifest URL → `.zip` → extract | FHWA NFHL 2024 |
| BTS port connectivity | CSV | PORT_NAME, LAT, LONG, ANNUAL_TONNAGE | manifest URL → CSV | BTS Port Rankings 2023 |

**Unit conventions (enforced in `route-data` at parse time):**
- `PCT_TRUCK`, `POVERTY_PCT`, `NO_VEHICLE_HH_PCT`: stored as proportion (0.0–1.0); HPMS raw field is 0–100 and is divided by 100 at ingest.
- `GDP_2022_M`: stored as millions USD.
- All distances: stored in miles unless suffixed `_km` or `_m`.

### 4.2 Internal graph model

Highway segments are edges; nodes are intersections and termini. The graph is **directed** — freight flows have dominant directions that matter for centrality and throughput analysis.

```rust
pub struct HighwayNode {
    pub id: u64,
    pub coord: Coord<f64>,          // EPSG:4269
    pub is_interchange: bool,       // true if node connects ≥2 routes
}

pub struct HighwayEdge {
    pub id: u64,
    pub route_id: String,           // e.g. "I80", "I95"
    pub state: String,
    pub geometry: LineString<f64>,
    pub length_miles: f64,
    pub lane_count: Option<u8>,     // from HPMS; None if missing
    // Joined attributes (None if join failed; join_failures tracked in build report)
    pub aadt: Option<u32>,
    pub pct_truck: Option<f32>,     // 0.0–1.0
    pub iri: Option<f32>,           // International Roughness Index (m/km)
    pub tti: Option<f32>,           // Travel Time Index (from FHWA FPM)
    pub pti: Option<f32>,           // Planning Time Index — 95th pct / free-flow
}

pub struct HighwayGraph {
    // Directed graph; use petgraph::Directed for centrality with flow semantics
    pub graph: petgraph::Graph<HighwayNode, HighwayEdge, petgraph::Directed>,
    pub node_index: rstar::RTree<[f64; 2]>,
    pub route_index: HashMap<String, Vec<EdgeIndex>>,
    // Centrality scores — None until route score-all computes the full national graph
    pub edge_betweenness: Option<HashMap<EdgeIndex, f64>>,
}
```

**On directed vs. undirected (G1 amendment):** Directed graph is correct for centrality with freight flow semantics. Betweenness centrality on a directed graph captures the asymmetric load that dominant freight directions impose. Undirected centrality would understate strategic importance of farm-to-port corridors.

### 4.3 Corridor model

A **corridor** is a named slice of the graph — all edges sharing a `route_id`. The `route-score` crate aggregates edge-level attributes to corridor-level scores.

```rust
pub struct Corridor {
    pub designation: String,        // "I-80"
    pub termini: [String; 2],
    pub states: Vec<String>,
    pub total_miles: f64,
    pub edges: Vec<EdgeIndex>,
    pub attributes: CorridorAttributes,
    pub scores: Option<DimensionScores>,
}

pub struct CorridorAttributes {
    // Flow (Band A) — segment-level data aggregated per §4.5
    pub p90_aadt: Option<f64>,              // 90th-percentile segment AADT (A1 primary)
    pub mean_aadt: Option<f64>,             // mean AADT (context only)
    pub annual_freight_value_b: Option<f64>,// annual freight value in $B (A2 primary, from FAF5 or HPMS proxy)
    pub freight_value_is_hpms_proxy: bool,  // true when estimated from truck AADT
    pub mean_pct_truck: Option<f32>,        // 0.0–1.0; A2 secondary
    pub p90_tti: Option<f32>,               // 90th-pct Travel Time Index (A3 primary)
    pub mean_pti: Option<f32>,              // mean Planning Time Index (A3 secondary)
    pub mean_iri: Option<f32>,              // mean IRI — pavement roughness (D3)

    // Network (Band B)
    pub nearest_parallel_miles: Option<f64>,  // distance to nearest parallel interstate-quality route (B1)
    pub detour_penalty_miles: Option<f64>,    // added miles via best alternate full route (B1)
    pub betweenness_centrality: Option<f64>,  // Brandes centrality; None until score-all (B2)
    pub port_terminus_flag: bool,             // true if a terminus is within 30mi of top-25 port (B3)
    pub nearest_top25_port_miles: Option<f32>,// distance to nearest top-25 port by tonnage (B3)
    pub border_crossing_flag: bool,           // true if route serves a major US-CA/MX crossing (B3)

    // People (Band C)
    pub pop_within_50mi: Option<u64>,          // total population (C1)
    pub rural_pop_within_50mi: Option<u64>,    // RUCC ≥4 population (C2)
    pub pct_rural_in_buffer: Option<f32>,      // rural share of 50mi buffer pop (C2)
    pub max_rural_interchange_gap_miles: Option<f32>, // longest gap between interchanges in rural segments (C2)
    pub corridor_gdp_b: Option<f64>,           // sum GDP of counties in 50mi buffer, $B (C3)
    pub gdp_per_capita_relative: Option<f32>,  // buffer GDP/capita ÷ national GDP/capita (C3; <1.0 = below avg)
    pub pct_pop_below_poverty: Option<f32>,    // 0.0–1.0; C3 secondary

    // Future (Band D)
    pub fema_sfha_miles: Option<f64>,          // total miles in FEMA SFHA flood zone (D1)
    pub max_consecutive_sfha_miles: Option<f32>,// longest contiguous flood-exposed segment (D1)
    pub intermodal_hub_count: u8,              // freight intermodal hubs on route (D2)
    pub dcfc_per_100mi: Option<f32>,           // DC fast charger density per 100 miles (D2)
    pub bridge_count: usize,                   // total NBI bridges on route (D3)
    pub pct_bridges_poor: Option<f32>,         // 0.0–1.0; bridges rated poor condition (D3)
    pub mean_year_built: Option<f32>,          // weighted mean construction year (D3)
}
```

### 4.4 Join failure policy

Every attribute join will fail on some records. Failures are not silent.

- `route build` reports: `N HPMS joins failed (list of route IDs)`, `N NBI bridges unmatched`, etc.
- Failed joins produce `None` in the attribute field — never a default value.
- Scoring functions receiving `None` on a primary field mark the score `estimated: true` and use a stated fallback (see §5).
- A corridor with >3 primary fields as `None` is flagged in the `route build` report as `data-sparse` and excluded from `route calibrate` (it would distort variance stats).

### 4.5 Segment-level aggregation for A1 (G2 amendment)

AADT varies dramatically within a single interstate. Averaging across a 2,000-mile corridor produces a number that describes neither the rural nor the urban segment accurately.

**A1 aggregation rule**: Score A1 from the **90th-percentile segment AADT** (`p90_aadt`) — the worst-performing 10% of route miles drives the corridor's throughput gap score. This captures where the system is actually stressed. `mean_aadt` is retained as a context field but does not feed A1 scoring.

Segment boundaries follow HPMS reporting segments (typically 0.1–1.0 mile). Compute `p90_aadt` by taking the 90th percentile of all segment AADT values weighted by segment length.

### 4.6 FAF5 attribution to corridors (G3 amendment)

FAF5 data is organized by origin-destination FAF zone pairs and mode, not by NHS route ID. Attributing freight flows to a specific corridor requires a join method.

**v1.0 method — FAF zone traversal:**
1. For each corridor, identify all FAF zones whose centroid falls within the corridor's 50-mile buffer.
2. Sum FAF5 freight flows (TONS, VALUE) for all O-D pairs where BOTH origin and destination FAF zones are in the corridor's buffer set AND DMODE includes truck.
3. Store result as `annual_freight_value_b` in `CorridorAttributes`.

**Limitation**: This method over-counts flows that use multiple corridors within a region and under-counts through-flows that traverse the corridor without stopping in buffer zones. It is an approximation, not a routing result. Mark `annual_freight_value_b` as `estimated: true` for all corridors in v1.0.

**v2.0 path**: routing-based flow attribution (assign FAF flows to corridors via shortest-path routing through `HighwayGraph`). This is the correct method but requires the full national graph and significant compute; deferred.

---

## §5. Scoring Engine (`route-score`)

Scoring functions are pure — no I/O, no state. They take `&CorridorAttributes`, return `ScoredDimension`.

**Scoring anchors are runtime-configurable** (G2 amendment — resolved collision between Moses and Freight Economist). Anchors load from `config/scoring.toml` at startup. Changing anchors during calibration requires only editing the config file, not recompiling.

```toml
# config/scoring.toml — scoring anchor maps per dimension
[a1]
# 90th-pct AADT thresholds for score 0, 5, 10
anchor_0  = 5_000      # well below capacity
anchor_5  = 40_000     # moderate urban congestion
anchor_10 = 120_000    # chronic LOS E/F

[a2]
# Annual freight value ($B) thresholds
anchor_0  = 1.0
anchor_5  = 25.0
anchor_10 = 150.0

# ... one section per dimension
```

```rust
pub struct ScoringConfig {
    pub a1: AnchorMap,
    pub a2: AnchorMap,
    // ... all 12
}

impl ScoringConfig {
    pub fn load(path: &Path) -> Result<Self, anyhow::Error> { ... }
    pub fn default() -> Self { ... }  // built-in defaults; no file required
}

pub struct ScoredDimension {
    pub dim: Dimension,
    pub score: f64,            // 0.0–10.0
    pub justification: String, // generated from scoring function output
    pub sources: Vec<String>,
    pub confidence: f32,       // 0.0–1.0 source/coverage quality, not score magnitude
    pub estimated: bool,       // true → mark with † in corpus entry
}
```

Confidence labels are derived from `confidence`: `High >= 0.85`, `Medium >= 0.60`, `Low > 0.0`, `Missing = 0.0`. Keep `estimated` as a compatibility flag for the `†` marker, but use `confidence` for data-quality comparison and planning.

Corridor-level confidence appears two ways: `confidence` is the simple mean across all 16 dimensions, while `score_confidence` is weighted by dimension score so high-scoring dimensions influence the summary more than zero-valued role dimensions.

### Scoring function notes per dimension

**A1 — Throughput Gap**: primary input `p90_aadt`. If `lane_count` is available, compute V/C ratio (p90_aadt / (lane_count × 1,900 pcph per lane)); score from V/C ratio instead of raw AADT. If `lane_count` is None (common), fall back to raw `p90_aadt` with `estimated: true`.

**A2 — Freight Intensity**: primary input `annual_freight_value_b` (FAF5, marked estimated in v1.0). When FAF5 is unavailable, the HPMS fallback estimates cargo value from representative daily truck crossings (`AADT × truck share × 365 × 16 tons/truck × $1,000/ton`) and marks `freight_value_is_hpms_proxy = true`; do not multiply this fallback by corridor miles, because the A2 anchors are annual commodity value, not truck-mile operating cost. Secondary context: `mean_pct_truck × mean_aadt`.

**A3 — Speed Reliability**: primary input `p90_tti` (FHWA FPM Planning Time Index). If missing, fall back to `mean_iri` as a pavement-quality proxy with `estimated: true`. IRI and speed reliability are correlated on rural segments; the fallback is weakest on urban congested corridors.

**B1 — Redundancy**: primary inputs `nearest_parallel_miles` and `detour_penalty_miles`. Score from detour penalty: 0 = <30 miles added, 10 = >300 miles added or no alternative.

**B2 — Network Centrality**: input `betweenness_centrality`. Marked `estimated: true` when absent from single-corridor scoring; `route score-all` computes full national Brandes centrality and clears the B2 estimate flag. Partial-graph centrality must not be persisted as authoritative — a corridor's centrality score is only stable when all atlas candidates are in the graph.

**B3 — Port/Border Access**: inputs `port_terminus_flag`, `nearest_top25_port_miles`, `border_crossing_flag`. Score: 10 if port terminus flag true; 8 if border crossing flag true; scale by distance otherwise.

**C1 — Population Reach**: input `pop_within_50mi`.

**C2 — Rural Connectivity**: primary inputs `pct_rural_in_buffer`, `max_rural_interchange_gap_miles`. High score = high rural share + long interchange gaps = people who depend on this corridor with no alternative nearby.

**C3 — Economic Opportunity Access**: measures whether the corridor connects people to economic activity, not historical harm. Primary inputs `gdp_per_capita_relative` and `corridor_gdp_b`. A corridor serving a region with below-national-average GDP per capita scores higher — it has higher opportunity value because the region has more to gain from connectivity. High score = below-average regional GDP, limited alternatives. Secondary: `pct_pop_below_poverty`.

**D1 — Climate Resilience**: primary `max_consecutive_sfha_miles` (contiguous flood exposure is the operational risk); secondary `fema_sfha_miles` (total exposure). FEMA SFHA tile joins must intersect route-edge geometry bounding boxes rather than whole-corridor bounding boxes, so sparse or long corridors do not inherit flood exposure from empty space between disjoint route segments.

**D2 — Multimodal Integration**: inputs `intermodal_hub_count` and `dcfc_per_100mi`.

**D3 — Infrastructure Vintage**: inputs `pct_bridges_poor`, absolute poor bridge count (`bridge_count × pct_bridges_poor`), `mean_year_built`, and `mean_iri` fallback. Score is the stronger of condition/vintage risk and absolute maintenance-backlog risk, so long corridors with many poor bridges are visible even when their percentage poor is modest.

---

## §6. CLI Subcommands

```
route fetch [--year 2023]
    Download all manifest sources to ~/.route/data/.

route build
    Parse NHS shapefile → build HighwayGraph → serialize to ~/.route/cache/graph.bin.
    Join HPMS, HPMS FPM, NBI, FEMA SFHA, BEA GDP, USDA rural codes onto edges/nodes.
    Report: N edges, N nodes, N routes, N join failures per source, N data-sparse corridors.

route score <designation> [--estimated]
    Score one corridor against the 16-dimension pool using config/scoring.toml anchors.
    Outputs: terminal table + corpus/existing/{slug}.md (or proposed/ with --estimated).
    B2 omitted from the fixture until score-all computes national centrality.

route score-all
    Score all corridors. Computes full national betweenness centrality (unlocks B2).
    Writes data/scores-all.csv with route, score, tier, rubric_version, estimated, dimension scores, and per-dimension confidence.
    Runs parallel via Rayon where graph operations allow it.

route gap [--type missing-link|bottleneck|resilience|intermodal]
    Planned: analyze scored corpus and identify corridors above threshold on gap-type dimensions.
    Current CLI labels this command planned and does not write a gap file.
    (Equity gap type deferred — economic opportunity analysis added in future pass.)

route map <designation> [--output path/to/map.png] [--color-by a2|d1|b1|...]
    Render corridor map. Corridor polyline over US outline base. Color by dimension score.

route report <designation>
    Regenerate corpus entry markdown from current graph attributes and scores. Idempotent.

route calibrate
    Compute variance stats per dimension across all scored corridors.
    Report: mean, IQR, correlation matrix, retirement candidates.
    Does NOT retire automatically — prints findings for human review.
```

---

## §7. Output Formats

### Corpus entry (markdown)

`route report I-80` produces `corpus/existing/i-80.md` following `corpus/SCHEMA.md`. Scores are machine-written; justifications generated from scoring function output. Estimated fields marked `†`. Human review annotates before `validated` promotion.

### Scoring ledger

`route score-all` writes `data/scores-all.csv` with:

| Column | Meaning |
|---|---|
| `route` | Normalized route id, e.g. `I80` |
| `score` | Total score on the current 160-point rubric |
| `tier` | T1/T2/T3/T4 label from v1.4 promotion thresholds: T1 >= 70.0, T2 >= 50.0, T3 >= 30.0 |
| `rubric_version` | Rubric version from `config/scoring.toml` |
| `estimated` | `true` when any dimension score is estimated/proxy |
| `confidence` | Mean confidence across all 16 dimensions |
| `score_confidence` | Score-weighted confidence for the points driving the corridor total |
| `A1`..`D3` | Dimension score on the 0.0-10.0 rubric |
| `A1_conf`..`D3_conf` | Per-dimension confidence on a 0.0-1.0 source/coverage scale |

The map renderer reads route scores and applies the same v1.4 thresholds for tier coloring. Calibration ledgers are a separate planned output.

### Map output (PNG)

- US outline: embedded GeoJSON, Natural Earth 1:50m
- Corridor polyline in highlight color over grey base; state borders
- Optional choropleth by dimension score
- Resolution: 1600×900 default

---

## §8. Data Pipeline for Anchor (I-80)

Phase 1 is manual-first. Run I-80 by hand before scripting anything.

**Step 1 — Fetch**: `route fetch` downloads NHS shapefile, HPMS CSV, HPMS FPM CSV, NBI CSV, FAF5 CSV, Census ACS, USDA ERS, BEA GDP, FEMA SFHA.

**Step 2 — Build**: `route build` builds `HighwayGraph`. Extracts I-80 segments by ROUTE_ID. Joins all attribute sources. Reports join failures. NBI bridge join uses R-tree ≤0.002° tolerance (~170m) with route-name similarity check (`NBI.ROUTE_NUM` contains "80") to reduce false joins in dense interchange areas.

**Step 3 — Score**: `route score I-80`. B2 is estimated (partial graph). FAF5 flows are estimated (zone traversal). Both marked `†` in output.

**Step 4 — Report**: `route report I-80` → `corpus/existing/i-80.md`. Human reviews scores, annotates justifications, checks geometry spot-check against known alignment.

**Step 5 — Map**: `route map I-80` renders corridor. Spot-check geometry.

Once I-80 passes human review: `route score-all` for all NHS trunk routes. Then `route calibrate` for the first rubric amendment pass.

---

## §9. What This Spec Does Not Cover

- **Web interface**: out of scope for v1.0.
- **Real-time traffic data**: NHS and HPMS are annual snapshots. Live feeds (TomTom, HERE, INRIX) require licensing; excluded.
- **Proposed corridor geometry**: no NHS entry. Geometry from ArcGIS project GeoJSON exports. `route score --estimated` handles these; graph build ignores them. Note: proposed corridor alignment choice has equity implications not captured by dimension scores alone.
- **Climate raster data**: NOAA projection rasters out of scope for v1.0. D1 uses FEMA SFHA polygons. Wildfire hazard (USFS polygon data) is a Round 2 addition.
- **Economic opportunity gap detection algorithm**: C3 measures economic opportunity access; gap detection thresholds will be defined after corpus calibration reveals the distribution. The `route gap --type equity` command is deferred until thresholds are empirically grounded.
- **METIS**: deferred to gap analysis phase; will depend on standalone giodl73-repo/METIS.
- **Automatic parliament invocation**: parliament reviews are human-run.

---

## §10. Spec Amendment Log

| Date | Gap | Amendment |
|---|---|---|
| 2026-05-06 | G1 — B2 partial graph | Graph changed to directed; B2 remains estimated when absent; `score-all` computes national B2 and clears the estimate flag |
| 2026-05-06 | G2 — A1 high variance | A1 scores from p90 segment AADT, not mean; added §4.5 |
| 2026-05-06 | G3 — FAF5 attribution | Added §4.6: v1.0 uses FAF zone traversal, marked estimated; v2.0 path is routing-based |
| 2026-05-06 | G4 — B3 missing impl | Added `port_terminus_flag`, `nearest_top25_port_miles`, `border_crossing_flag` to `CorridorAttributes`; B3 scoring function specified in §5 |
| 2026-05-06 | G5 — Rural/urban split | Added `rural_pop_within_50mi`, `pct_rural_in_buffer`, `max_rural_interchange_gap_miles`; USDA ERS rural codes added to §4.1 |
| 2026-05-06 | G6 — B1 field granularity | Replaced `parallel_interstate_count: u8` with `nearest_parallel_miles` + `detour_penalty_miles` |
| 2026-05-06 | G7 — Equity gap algorithm | C3 reframed as Economic Opportunity Access (GDP per capita relative to national, not historical-harm framing); gap detection thresholds deferred to post-calibration |
| 2026-05-06 | G8 — Citations | All sources in frontmatter pinned to specific reports and data years; FAF5 pinned to v5.6 (2022); ATRI pinned to 2024 report |
| 2026-05-06 | Scoring anchors | Moved from compiled constants to `config/scoring.toml` (runtime configurable); `toml` crate added |
| 2026-05-06 | NBI join tolerance | Tightened to ≤0.002° with route-name similarity check |
| 2026-05-06 | Unit conventions | Explicit unit policy added to §4.1; PCT_TRUCK stored as 0.0–1.0 |
