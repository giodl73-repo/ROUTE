---
name: ROUTE Rust Architecture — route CLI v1.0
slug: route-rust-architecture
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources:
  - "FHWA National Highway System documentation"
  - "apportionment/redist Rust codebase (C:\\src\\apportionment)"
  - "ROUTE design spec (specs/2026-05-06-route-design.md)"
---

# ROUTE Rust Architecture — `route` CLI v1.0

## §1. Purpose

This spec defines the Rust implementation of the ROUTE data and analysis pipeline. The binary is named `route`. It fetches authoritative federal highway data, builds a corridor graph, scores corridors against the 12-dimension pool, identifies network gaps, and renders corridor maps. All computation is in Rust; no GDAL, no Python runtime.

The design follows the `redist` CLI pattern from the apportionment sibling project: manifest-driven data fetching, pure-Rust shapefile parsing, graph-based analysis, and markdown/CSV/PNG output.

---

## §2. Workspace Layout

```
route/
├── Cargo.toml                  ← workspace root
└── crates/
    ├── route-data/             ← fetch + parse FHWA shapefiles + CSV joins
    ├── route-network/          ← highway graph: nodes, edges, network metrics
    ├── route-score/            ← 12-dimension scoring engine
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
#   Will depend on a standalone giodl73-repo/METIS crate, not redist-metis.

# Data handling
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
csv         = "1"
zip         = "2"        # NHS shapefiles distributed as .zip

# CLI
clap        = { version = "4", features = ["derive"] }

# Rendering
resvg       = "0.42"     # SVG → PNG rasterizer (same as redist-map)
tiny-skia   = "0.11"     # pixel buffer backend

# Utilities
rayon       = "1.10"     # parallel segment processing
thiserror   = "2"
anyhow      = "1"
reqwest     = { version = "0.12", features = ["blocking"] }
```

No GDAL. No Python. No PROJ. CRS handling is explicit: NHS shapefiles ship in EPSG:4269 (NAD83 geographic); we project to EPSG:5070 (Albers Equal Area) for area and length calculations, same as `redist`.

---

## §4. Data Model

### 4.1 Raw inputs

| Source | Format | Key fields | How fetched |
|---|---|---|---|
| FHWA NHS shapefile | `.shp` LineString | ROUTE_ID, STATE_CODE, NHS_TYPE, MILES | manifest URL → `.zip` → extract |
| FHWA HPMS summary | CSV | STATE, ROUTE_ID, AADT, PCT_TRUCK, IRI | manifest URL → CSV |
| FHWA NBI | CSV | LAT, LONG, ROUTE_NUM, SUFFICIENCY_RATING, YEAR_BUILT, PCT_ADT_TRUCK | manifest URL → CSV |
| ATRI bottleneck seed | CSV (hand-curated) | RANK, LOCATION, ANNUAL_COST_M | committed to `data/atri-bottlenecks.csv` |
| FAF5 flows | CSV | ORIG_ST, DEST_ST, TONS, VALUE, DMODE | manifest URL → CSV |
| Census ACS | CSV + `.shp` | GEOID, TOTAL_POP, POVERTY_PCT | manifest URL → `.zip` |
| FEMA SFHA | `.shp` Polygon | FLD_ZONE, geometry | manifest URL → `.zip` → extract |

### 4.2 Internal graph model

Highway segments are **directed edges** in a graph where nodes are intersection points and termini. The `route-network` crate owns this model.

```rust
pub struct HighwayNode {
    pub id: u64,
    pub coord: Coord<f64>,          // EPSG:4269
}

pub struct HighwayEdge {
    pub id: u64,
    pub route_id: String,           // e.g. "I80", "I95"
    pub state: String,
    pub geometry: LineString<f64>,
    pub length_miles: f64,
    // Joined attributes (None if join failed)
    pub aadt: Option<u32>,
    pub pct_truck: Option<f32>,
    pub iri: Option<f32>,           // International Roughness Index
}

pub struct HighwayGraph {
    pub graph: petgraph::Graph<HighwayNode, HighwayEdge, petgraph::Undirected>,
    pub node_index: rstar::RTree<[f64; 2]>,   // spatial lookup
    pub route_index: HashMap<String, Vec<EdgeIndex>>,  // route_id → edges
}
```

### 4.3 Corridor model

A **corridor** is a named slice of the graph — all edges sharing a `route_id`. The `route-score` crate aggregates edge-level attributes to corridor-level scores.

```rust
pub struct Corridor {
    pub designation: String,        // "I-80"
    pub termini: [String; 2],       // ["Teaneck NJ", "San Francisco CA"]
    pub states: Vec<String>,
    pub total_miles: f64,
    pub edges: Vec<EdgeIndex>,
    pub attributes: CorridorAttributes,
    pub scores: Option<DimensionScores>,
}

pub struct CorridorAttributes {
    pub mean_aadt: Option<f64>,
    pub mean_pct_truck: Option<f64>,
    pub mean_iri: Option<f64>,
    pub bridge_count: usize,
    pub pct_bridges_poor: Option<f64>,
    pub pop_within_50mi: Option<u64>,
    pub pct_pop_below_poverty: Option<f64>,
    pub fema_sfha_miles: Option<f64>,   // miles in FEMA flood zone
    pub parallel_interstate_count: u8,  // B1 Redundancy source
    pub intermodal_hub_count: u8,       // D2 Multimodal Integration source
}
```

---

## §5. Scoring Engine (`route-score`)

Each of the 12 dimensions maps to a function over `CorridorAttributes`. The scoring functions are pure — no I/O, no state. They take attributes, return `f64` in [0, 10].

```rust
pub trait DimensionScorer {
    fn score(&self, attrs: &CorridorAttributes) -> ScoredDimension;
}

pub struct ScoredDimension {
    pub dim: Dimension,
    pub score: f64,           // 0.0–10.0
    pub justification: String,
    pub sources: Vec<String>,
    pub estimated: bool,      // true → mark with † in output
}

pub struct DimensionScores {
    pub a1_throughput_gap: ScoredDimension,
    pub a2_freight_intensity: ScoredDimension,
    pub a3_speed_reliability: ScoredDimension,
    pub b1_redundancy: ScoredDimension,
    pub b2_network_centrality: ScoredDimension,
    pub b3_port_border_access: ScoredDimension,
    pub c1_population_reach: ScoredDimension,
    pub c2_rural_connectivity: ScoredDimension,
    pub c3_equity_access: ScoredDimension,
    pub d1_climate_resilience: ScoredDimension,
    pub d2_multimodal_integration: ScoredDimension,
    pub d3_infrastructure_vintage: ScoredDimension,
}
```

**Scoring anchor maps** (from `corpus/SCHEMA.md`) are compiled into the binary as constants. Example for A2 (Freight Intensity):

```rust
fn score_a2(attrs: &CorridorAttributes) -> ScoredDimension {
    let score = match attrs.mean_pct_truck.zip(attrs.mean_aadt) {
        Some((pct, aadt)) => {
            let trucks_per_day = aadt * (pct as f64 / 100.0);
            // Anchor: 0=<500/day, 5=2000-5000/day, 10=>10000/day
            (trucks_per_day / 1000.0).min(10.0)
        }
        None => {
            // No data — score 0, mark estimated
            0.0
        }
    };
    // ...
}
```

**B2 Network Centrality** uses `petgraph`'s betweenness centrality approximation (Brandes algorithm) over the full `HighwayGraph`. Computed once after graph construction; stored per-edge and aggregated per-corridor.

---

## §6. CLI Subcommands

```
route fetch [--year 2023]
    Download all manifest sources to ~/.route/data/.
    Same pattern as `redist fetch`.

route build
    Parse NHS shapefile → build HighwayGraph → serialize to ~/.route/cache/graph.bin.
    Join HPMS and NBI attributes onto edges.
    Report: N edges, N nodes, N routes, N join failures.

route score <designation> [--estimated]
    Score one corridor (e.g. "I-80") against the 12-dimension pool.
    Outputs: terminal table + corpus/existing/{slug}.md (or proposed/ with --estimated).

route score-all
    Score all corridors in the graph. Updates scoring ledger in personas/axis-pool.md.
    Runs in parallel via Rayon.

route gap [--type missing-link|bottleneck|resilience|equity|intermodal]
    Analyze scored corpus. Identify corridors scoring above threshold on gap-type
    dimensions. Output gap findings to gaps/{slug}.md.

route map <designation> [--output path/to/map.png]
    Render a corridor map. Highlights the route on a US outline base map.
    Color by dimension (--color-by a2|d1|etc.).

route report <designation>
    Regenerate corpus entry markdown from current scores.
    Idempotent — reads scores from graph cache, writes corpus/existing/{slug}.md.

route calibrate
    Run the rubric calibration pass: compute variance stats for each dimension
    across all scored corridors. Report retirement candidates. Does NOT retire
    automatically — prints findings for human review.
```

---

## §7. Output Formats

### Corpus entry (markdown)

`route report I-80` produces `corpus/existing/i-80.md` following `corpus/SCHEMA.md` exactly. Scores are machine-written; justifications are generated from scoring function output. Human review may annotate before `validated` promotion.

### Scoring ledger (personas/axis-pool.md)

`route score-all` updates the ledger table in `personas/axis-pool.md`:
- Corridors scored count
- Mean, IQR, min, max per dimension
- Correlation flags

The ledger is the single source of truth for the calibration pass.

### Map output (PNG)

- US outline from embedded GeoJSON (simplified Natural Earth data, 1:50m)
- Corridor polyline in highlight color over grey base
- State borders for reference
- Optional: choropleth overlay by dimension score
- Resolution: 1600×900 default

---

## §8. Data Pipeline for Anchor (I-80)

Phase 1 is manual-first. Run the anchor (I-80) by hand before scripting anything.

**Step 1 — Fetch**: `route fetch` downloads NHS shapefile + HPMS CSV + NBI CSV.

**Step 2 — Build**: `route build` parses NHS, extracts I-80 segments by ROUTE_ID filter, joins HPMS traffic data by state+route, joins NBI bridges by coordinate proximity (R-tree, ≤0.01° tolerance).

**Step 3 — Score**: `route score I-80` produces scores for all 12 dimensions. Data gaps (B2 centrality is estimated in Phase 1 without full national graph) are marked `†`.

**Step 4 — Report**: `route report I-80` writes `corpus/existing/i-80.md`. Human reviews and annotates.

**Step 5 — Map**: `route map I-80` renders the corridor. Spot-check geometry against known I-80 alignment.

Once I-80 passes human review, run `route score-all` for all trunk routes in the NHS. That's the corpus build.

---

## §9. What This Spec Does Not Cover

- **Web interface**: out of scope for v1.0. `route-web` crate is stubbed but empty.
- **Real-time traffic data**: NHS and HPMS are annual snapshots, not live feeds. TomTom, HERE, or INRIX data would require licensing; excluded.
- **Proposed corridor geometry**: proposed corridors have no NHS shapefile entry. Geometry comes from ArcGIS project exports (GeoJSON) or hand-drawn alignments. `route score --estimated` handles these; the graph build ignores them.
- **Climate raster data**: NOAA/FEMA flood zone polygons are in scope (`.shp`); NOAA climate projection rasters are out of scope for v1.0. D1 scores in v1.0 use FEMA SFHA miles as the primary indicator.
- **Automatic parliament invocation**: parliament reviews are human-run, not automated by the CLI.

---

## §10. Spec Amendment Protocol

Changes to the data model, scoring functions, or CLI interface that break the `CorridorAttributes` or `DimensionScores` structs require amending this spec before code changes. The structs are the contract between crates.

| Date | Amendment | Reason |
|---|---|---|
| — | — | — |
