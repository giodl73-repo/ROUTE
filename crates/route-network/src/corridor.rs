use petgraph::graph::EdgeIndex;
use serde::{Deserialize, Serialize};

/// A corridor — all edges in the graph sharing one route_id.
/// `route-score` aggregates edge attributes into `CorridorAttributes`
/// and produces `DimensionScores`.
#[derive(Debug, Clone)]
pub struct Corridor {
    pub designation: String,  // "I-80"
    pub termini: [String; 2], // ["Teaneck NJ", "San Francisco CA"]
    pub states: Vec<String>,
    pub total_miles: f64,
    pub edge_count: usize,
    pub edges: Vec<EdgeIndex>,
    pub attributes: CorridorAttributes,
}

/// Aggregated corridor-level attributes derived from edge data and spatial joins.
/// All fields are Option — None means the join failed or data is unavailable.
/// Fields marked with their primary dimension use in comments.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorridorAttributes {
    // ── Classification ────────────────────────────────────────────────────────
    /// Whether this is an upgrade candidate (US/state highway) vs existing interstate
    pub is_upgrade_candidate: bool,

    // ── Band A: Flow ──────────────────────────────────────────────────────────
    /// 90th-percentile segment AADT (length-weighted) — primary A1 input
    pub p90_aadt: Option<f64>,
    /// Mean AADT across all segments (context only; not used in A1 scoring)
    pub mean_aadt: Option<f64>,
    /// Theoretical capacity: mean_lane_count × 1,900 pcph × 24h (veh/day)
    /// Used for V/C ratio when lane count is known
    pub daily_capacity: Option<f64>,
    /// Volume/capacity ratio at 90th-pct AADT — direct congestion measure
    pub vc_ratio_p90: Option<f32>,
    /// Mean posted speed limit (mph) — from HPMS when available
    pub mean_speed_limit: Option<f32>,
    /// Mean lane count across segments — determines throughput capacity
    pub mean_lane_count: Option<f32>,
    /// Annual freight value in $B, FAF5 zone-traversal estimate — primary A2 input
    /// Always estimated: true in v1.0 (zone-traversal approximation)
    pub annual_freight_value_b: Option<f64>,
    /// True when annual_freight_value_b is estimated from HPMS truck AADT rather than FAF5.
    pub freight_value_is_hpms_proxy: bool,
    /// Mean truck proportion 0.0–1.0 — A2 secondary
    pub mean_pct_truck: Option<f32>,
    /// 90th-percentile Planning Time Index (95th-pct / free-flow travel time) — primary A3
    pub p90_pti: Option<f32>,
    /// Mean Travel Time Index — A3 secondary
    pub mean_tti: Option<f32>,
    /// Mean IRI (International Roughness Index, m/km) — D3 primary; A3 fallback
    pub mean_iri: Option<f32>,

    // ── Band B: Network ───────────────────────────────────────────────────────
    /// Miles to nearest parallel interstate-quality route — primary B1 input
    pub nearest_parallel_miles: Option<f64>,
    /// Added miles via best alternate route for the full corridor — B1 primary
    pub detour_penalty_miles: Option<f64>,
    /// Brandes betweenness centrality (normalized 0.0–1.0 across corpus) — B2
    /// None until route score-all completes full national graph
    pub betweenness_centrality: Option<f64>,
    /// True if a terminus is within 30 miles of a top-25 US port by tonnage — B3
    pub port_terminus_flag: bool,
    /// Distance in miles to nearest top-25 port — B3
    pub nearest_top25_port_miles: Option<f32>,
    /// True if route serves a major US-Canada or US-Mexico border crossing — B3
    pub border_crossing_flag: bool,

    // ── Band C: People ────────────────────────────────────────────────────────
    /// Total population within 50-mile buffer — C1
    pub pop_within_50mi: Option<u64>,
    /// Rural population (USDA RUCC ≥4) within 50-mile buffer — C2
    pub rural_pop_within_50mi: Option<u64>,
    /// Rural share of 50-mile buffer population 0.0–1.0 — C2 primary
    pub pct_rural_in_buffer: Option<f32>,
    /// Longest gap (miles) between interchanges in segments classified rural — C2
    pub max_rural_interchange_gap_miles: Option<f32>,
    /// Sum GDP of counties in 50-mile buffer, $B — C3
    pub corridor_gdp_b: Option<f64>,
    /// Buffer GDP per capita ÷ national GDP per capita — C3 primary
    /// <1.0 = below national average = higher economic opportunity value of corridor
    pub gdp_per_capita_relative: Option<f32>,
    /// Poverty rate as proportion 0.0–1.0 — C3 secondary
    pub pct_pop_below_poverty: Option<f32>,

    // ── Band D: Future ────────────────────────────────────────────────────────
    /// Total corridor miles in FEMA SFHA flood zone — D1 secondary
    pub fema_sfha_miles: Option<f64>,
    /// Longest contiguous flood-exposed segment (miles) — D1 primary
    pub max_consecutive_sfha_miles: Option<f32>,
    /// Count of freight intermodal hubs on or adjacent to corridor — D2
    pub intermodal_hub_count: u8,
    /// DC fast chargers per 100 miles — D2
    pub dcfc_per_100mi: Option<f32>,
    /// Total NBI bridge count on corridor — D3
    pub bridge_count: usize,
    /// Proportion of bridges in poor/critical condition 0.0–1.0 — D3 primary
    pub pct_bridges_poor: Option<f32>,
    /// Length-weighted mean bridge construction year — D3
    pub mean_year_built: Option<f32>,

    // ── v1.2 New Dimensions ───────────────────────────────────────────────────
    /// A4 International Trade Corridor — USMCA designation score 0–10
    /// 10 = primary Laredo/El Paso/San Diego corridor; 5 = secondary crossing
    pub intl_trade_score: f64,

    // ── v1.4 New Fields ───────────────────────────────────────────────────────
    /// Fatal crash rate per 100M VMT — A5 (from FARS 2022)
    pub fatal_crash_rate: Option<f32>,
    /// Rail parallel flag — B1 modifier (Class 1 railroad within 50mi reduces effective B1)
    pub rail_parallel_flag: bool,
    /// Rail parallel name (e.g. "UP Overland Route")
    pub rail_parallel_name: Option<String>,
    /// Wildfire hazard risk 0-10 (USFS WHP-derived) — D1 component
    pub wildfire_risk: Option<f32>,
    /// Tornado risk 0-10 (SPC tornado probability) — D1 component
    pub tornado_risk: Option<f32>,
    /// Seismic risk 0-10 (USGS sds-derived) — D1 component
    pub seismic_risk: Option<f32>,

    /// BPR-estimated PTI from V/C ratio — better A3 fallback than IRI
    /// PTI_bpr = 1 + 0.15 × (V/C_peak × 1.15)^4  where V/C_peak = p90_aadt × 0.09 / (lanes/2 × 2300)
    /// None if insufficient data (AADT or lane count missing)
    pub pti_bpr_estimate: Option<f32>,

    /// B4 Military/Strategic Designation — STRAHNET + military base proximity 0–10
    pub military_strategic_score: f64,

    /// C4 Agricultural Export Access — grain belt + export terminal proximity 0–10
    pub agricultural_export_score: f64,
}

impl CorridorAttributes {
    /// Count of primary scoring fields that are None.
    /// Corridors with > threshold are flagged as data-sparse.
    pub fn none_count(&self) -> usize {
        [
            self.p90_aadt.is_none(),
            self.annual_freight_value_b.is_none(),
            self.p90_pti.is_none(),
            self.nearest_parallel_miles.is_none(),
            self.betweenness_centrality.is_none(),
            self.pop_within_50mi.is_none(),
            self.gdp_per_capita_relative.is_none(),
            self.fema_sfha_miles.is_none(),
        ]
        .iter()
        .filter(|&&b| b)
        .count()
    }
}
