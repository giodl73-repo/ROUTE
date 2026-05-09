use crate::config::ScoringConfig;
use route_network::CorridorAttributes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    A1ThroughputGap,
    A2FreightIntensity,
    A3SpeedReliability,
    A4InternationalTrade,
    A5SafetyRecord,
    B1Redundancy,
    B2NetworkCentrality,
    B3PortBorderAccess,
    B4MilitaryStrategic,
    C1PopulationReach,
    C2RuralConnectivity,
    C3EconomicOpportunity,
    C4AgriculturalExport,
    D1ClimateResilience,
    D2MultimodalIntegration,
    D3InfrastructureVintage,
}

impl Dimension {
    pub fn code(&self) -> &'static str {
        match self {
            Dimension::A1ThroughputGap => "A1",
            Dimension::A2FreightIntensity => "A2",
            Dimension::A3SpeedReliability => "A3",
            Dimension::A4InternationalTrade => "A4",
            Dimension::A5SafetyRecord => "A5",
            Dimension::B1Redundancy => "B1",
            Dimension::B2NetworkCentrality => "B2",
            Dimension::B3PortBorderAccess => "B3",
            Dimension::B4MilitaryStrategic => "B4",
            Dimension::C1PopulationReach => "C1",
            Dimension::C2RuralConnectivity => "C2",
            Dimension::C3EconomicOpportunity => "C3",
            Dimension::C4AgriculturalExport => "C4",
            Dimension::D1ClimateResilience => "D1",
            Dimension::D2MultimodalIntegration => "D2",
            Dimension::D3InfrastructureVintage => "D3",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Dimension::A1ThroughputGap => "Throughput Gap",
            Dimension::A2FreightIntensity => "Freight Intensity",
            Dimension::A3SpeedReliability => "Speed Reliability",
            Dimension::A4InternationalTrade => "International Trade Corridor",
            Dimension::A5SafetyRecord => "Safety Record",
            Dimension::B1Redundancy => "Redundancy",
            Dimension::B2NetworkCentrality => "Network Centrality",
            Dimension::B3PortBorderAccess => "Port/Border Access",
            Dimension::B4MilitaryStrategic => "Military/Strategic",
            Dimension::C1PopulationReach => "Population Reach",
            Dimension::C2RuralConnectivity => "Rural Connectivity",
            Dimension::C3EconomicOpportunity => "Economic Opportunity Access",
            Dimension::C4AgriculturalExport => "Agricultural Export Access",
            Dimension::D1ClimateResilience => "Climate Resilience",
            Dimension::D2MultimodalIntegration => "Multimodal Integration",
            Dimension::D3InfrastructureVintage => "Infrastructure Vintage",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScoredDimension {
    pub dim: Dimension,
    /// 0.0–10.0
    pub score: f64,
    /// One-sentence justification with data cited
    pub justification: String,
    pub sources: Vec<String>,
    /// true → mark with † in corpus entry (estimated value used)
    pub estimated: bool,
}

#[derive(Debug, Clone)]
pub struct DimensionScores {
    pub a1: ScoredDimension,
    pub a2: ScoredDimension,
    pub a3: ScoredDimension,
    pub a4: ScoredDimension, // v1.2
    pub a5: ScoredDimension, // v1.4
    pub b1: ScoredDimension,
    pub b2: ScoredDimension,
    pub b3: ScoredDimension,
    pub b4: ScoredDimension, // v1.2
    pub c1: ScoredDimension,
    pub c2: ScoredDimension,
    pub c3: ScoredDimension,
    pub c4: ScoredDimension, // v1.2
    pub d1: ScoredDimension,
    pub d2: ScoredDimension,
    pub d3: ScoredDimension,
    /// Rubric version these scores were computed under
    pub rubric_version: String,
}

impl DimensionScores {
    pub fn band_a(&self) -> f64 {
        self.a1.score + self.a2.score + self.a3.score + self.a4.score + self.a5.score
    }
    pub fn band_b(&self) -> f64 {
        self.b1.score + self.b2.score + self.b3.score + self.b4.score
    }
    pub fn band_c(&self) -> f64 {
        self.c1.score + self.c2.score + self.c3.score + self.c4.score
    }
    pub fn band_d(&self) -> f64 {
        self.d1.score + self.d2.score + self.d3.score
    }
    pub fn total(&self) -> f64 {
        self.band_a() + self.band_b() + self.band_c() + self.band_d()
    }
    pub fn any_estimated(&self) -> bool {
        [
            &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.b1, &self.b2, &self.b3,
            &self.b4, &self.c1, &self.c2, &self.c3, &self.c4, &self.d1, &self.d2, &self.d3,
        ]
        .iter()
        .any(|d| d.estimated)
    }
}

/// Score a corridor against all 16 dimensions using the provided config.
pub fn score_corridor(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> DimensionScores {
    DimensionScores {
        a1: score_a1(attrs, cfg),
        a2: score_a2(attrs, cfg),
        a3: score_a3(attrs, cfg),
        a4: score_a4(attrs, cfg),
        a5: score_a5(attrs, cfg),
        b1: score_b1(attrs, cfg),
        b2: score_b2(attrs, cfg),
        b3: score_b3(attrs, cfg),
        b4: score_b4(attrs, cfg),
        c1: score_c1(attrs, cfg),
        c2: score_c2(attrs, cfg),
        c3: score_c3(attrs, cfg),
        c4: score_c4(attrs, cfg),
        d1: score_d1(attrs, cfg),
        d2: score_d2(attrs, cfg),
        d3: score_d3(attrs, cfg),
        rubric_version: cfg.meta.rubric_version.clone(),
    }
}

// ── Band A ────────────────────────────────────────────────────────────────────

fn score_a1(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.p90_aadt {
        Some(aadt) => ScoredDimension {
            dim: Dimension::A1ThroughputGap,
            score: cfg.a1.score(aadt),
            justification: format!(
                "90th-percentile segment AADT {aadt:.0} vehicles/day (mean: {:.0}).",
                attrs.mean_aadt.unwrap_or(0.0)
            ),
            sources: vec!["FHWA HPMS 2023".into()],
            estimated: false,
        },
        None => estimated(
            Dimension::A1ThroughputGap,
            "HPMS AADT join failed; no score available.",
        ),
    }
}

fn score_a2(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.annual_freight_value_b {
        Some(val) => ScoredDimension {
            dim: Dimension::A2FreightIntensity,
            score: cfg.a2.score(val),
            justification: format!(
                "Annual freight value ${val:.1}B (FAF5 zone-traversal estimate†); \
                 mean truck share {:.0}%.",
                attrs.mean_pct_truck.unwrap_or(0.0) * 100.0
            ),
            sources: vec!["FAF5 v5.6 BTS/FHWA 2022".into()],
            estimated: true, // zone-traversal is always an estimate in v1.0
        },
        None => estimated(
            Dimension::A2FreightIntensity,
            "FAF5 flow attribution unavailable.",
        ),
    }
}

fn score_a3(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    if let Some(pti) = attrs.p90_pti {
        // Best path: real PTI from FHWA Freight Performance Measures
        ScoredDimension {
            dim: Dimension::A3SpeedReliability,
            score: cfg.a3.score_pti(pti as f64),
            justification: format!(
                "90th-pct PTI {pti:.2} (mean TTI {:.2}). Source: FHWA FPM.",
                attrs.mean_tti.unwrap_or(1.0)
            ),
            sources: vec!["FHWA Freight Performance Measures 2023".into()],
            estimated: false,
        }
    } else if let Some(pti_bpr) = attrs.pti_bpr_estimate {
        // v1.2: BPR-estimated PTI from V/C ratio — better than IRI proxy
        ScoredDimension {
            dim: Dimension::A3SpeedReliability,
            score: cfg.a3.score_bpr_pti(pti_bpr),
            justification: format!(
                "BPR-estimated PTI {pti_bpr:.2} from V/C={:.2} (HPMS AADT + lane count). \
                 PTI = 1 + 0.15×(V/C×1.15)^4. Better than IRI proxy but still estimated.",
                attrs.vc_ratio_p90.unwrap_or(0.0)
            ),
            sources: vec!["FHWA HPMS 2023 (AADT + lanes)".into()],
            estimated: true,
        }
    } else if let Some(iri) = attrs.mean_iri {
        // IRI fallback (last resort) — capped at 5.0 per v1.1 amendment
        ScoredDimension {
            dim: Dimension::A3SpeedReliability,
            score: cfg.a3.score_iri_fallback(iri),
            justification: format!(
                "IRI proxy (last resort), capped at {:.1} (mean IRI {iri:.1} m/km). \
                 Fetch HPMS for BPR-estimated PTI.",
                cfg.a3.iri_fallback_max
            ),
            sources: vec!["FHWA HPMS 2023".into()],
            estimated: true,
        }
    } else {
        estimated(
            Dimension::A3SpeedReliability,
            "PTI, V/C, and IRI all unavailable.",
        )
    }
}

// v1.2 new dimension scoring functions

fn score_a4(attrs: &CorridorAttributes, _cfg: &ScoringConfig) -> ScoredDimension {
    let score = attrs.intl_trade_score;
    if score > 0.0 {
        ScoredDimension {
            dim: Dimension::A4InternationalTrade,
            score,
            justification: format!(
                "USMCA trade corridor score {score:.1}/10 from hard-coded corridor designation. \
                 Higher = more central to US-Mexico or US-Canada freight flows."
            ),
            sources: vec!["FHWA NHS High Priority Corridors; USMCA Annex 31-A".into()],
            estimated: false, // hard-coded designation data is reliable
        }
    } else {
        ScoredDimension {
            dim: Dimension::A4InternationalTrade,
            score: 0.0,
            justification: "No USMCA corridor designation.".into(),
            sources: vec![],
            estimated: false,
        }
    }
}

fn score_a5(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.fatal_crash_rate {
        Some(rate) => ScoredDimension {
            dim: Dimension::A5SafetyRecord,
            score: cfg.a5.score(rate as f64),
            justification: format!(
                "Fatal crash rate {rate:.3} per 100M VMT (FARS 2022). \
                 National interstate avg: 0.54 per 100M VMT.",
            ),
            sources: vec!["NHTSA FARS 2022".into()],
            estimated: false,
        },
        None => estimated(
            Dimension::A5SafetyRecord,
            "FARS crash data not yet joined — run route fetch-fars.",
        ),
    }
}

fn score_b4(attrs: &CorridorAttributes, _cfg: &ScoringConfig) -> ScoredDimension {
    let score = attrs.military_strategic_score;
    if score > 0.0 {
        ScoredDimension {
            dim: Dimension::B4MilitaryStrategic,
            score,
            justification: format!(
                "Military/strategic score {score:.1}/10. STRAHNET designation baseline 5.0; \
                 additional points for proximity to nuclear command, major installations."
            ),
            sources: vec!["FHWA STRAHNET; DoD installation list".into()],
            estimated: false,
        }
    } else {
        ScoredDimension {
            dim: Dimension::B4MilitaryStrategic,
            score: 0.0,
            justification:
                "Not STRAHNET-designated; no major military installation within 30 miles.".into(),
            sources: vec![],
            estimated: false,
        }
    }
}

fn score_c4(attrs: &CorridorAttributes, _cfg: &ScoringConfig) -> ScoredDimension {
    let score = attrs.agricultural_export_score;
    if score > 0.0 {
        ScoredDimension {
            dim: Dimension::C4AgriculturalExport,
            score,
            justification: format!(
                "Agricultural export access score {score:.1}/10. Higher = greater role as export \
                 corridor for grain, beef, cotton, or other commodity production zones."
            ),
            sources: vec!["USDA ERS county production data; export terminal locations".into()],
            estimated: false,
        }
    } else {
        ScoredDimension {
            dim: Dimension::C4AgriculturalExport,
            score: 0.0,
            justification: "No significant agricultural production or export corridor role.".into(),
            sources: vec![],
            estimated: false,
        }
    }
}

// ── Band B ────────────────────────────────────────────────────────────────────

fn score_b1(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.detour_penalty_miles {
        Some(penalty) => {
            let raw_score = cfg.b1.score(penalty);
            // Rail parallel discount: Class 1 freight railroad within 50mi reduces effective isolation
            let rail_discount = if attrs.rail_parallel_flag { 0.80 } else { 1.0 };
            let score = raw_score * rail_discount;
            let rail_note = if attrs.rail_parallel_flag {
                format!(" Rail parallel ({}) within 50mi — B1 discounted 20% (rail provides partial redundancy).",
                    attrs.rail_parallel_name.as_deref().unwrap_or("Class 1 RR"))
            } else {
                String::new()
            };
            ScoredDimension {
                dim: Dimension::B1Redundancy,
                score,
                justification: format!(
                    "Best alternate route adds {penalty:.0} miles (nearest parallel route \
                     {:.0} miles away).{rail_note}",
                    attrs.nearest_parallel_miles.unwrap_or(0.0)
                ),
                sources: vec!["HighwayGraph shortest-path analysis".into()],
                estimated: false,
            }
        }
        None => estimated(
            Dimension::B1Redundancy,
            "Parallel route analysis incomplete.",
        ),
    }
}

fn score_b2(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.betweenness_centrality {
        Some(bc) => ScoredDimension {
            dim: Dimension::B2NetworkCentrality,
            score: cfg.b2.score(bc),
            justification: format!(
                "Normalised betweenness centrality {bc:.3} (0=peripheral, 1=spine)."
            ),
            sources: vec!["HighwayGraph Brandes centrality".into()],
            estimated: false,
        },
        None => ScoredDimension {
            dim: Dimension::B2NetworkCentrality,
            score: 0.0,
            justification: "Betweenness centrality not yet computed — run route score-all \
                            to build full national graph."
                .into(),
            sources: vec![],
            estimated: true,
        },
    }
}

fn score_b3(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    // Stepped scoring: port terminus → 10, border crossing → 8, else distance-based
    let mut missing_data = false;
    let (score, justification) = if attrs.port_terminus_flag {
        (
            10.0,
            format!(
                "Terminus within 30 miles of top-25 US port \
             (nearest {:.0} mi).",
                attrs.nearest_top25_port_miles.unwrap_or(0.0)
            ),
        )
    } else if attrs.border_crossing_flag {
        (
            8.0,
            "Serves major US-Canada or US-Mexico border crossing.".into(),
        )
    } else {
        match attrs.nearest_top25_port_miles {
            Some(d) => (
                cfg.b3.score(d as f64),
                format!("Nearest top-25 port {d:.0} miles from terminus."),
            ),
            None => (0.0, "No port or border crossing data available.".into()),
        }
    };
    if !attrs.port_terminus_flag
        && !attrs.border_crossing_flag
        && attrs.nearest_top25_port_miles.is_none()
    {
        missing_data = true;
    }
    ScoredDimension {
        dim: Dimension::B3PortBorderAccess,
        score,
        justification,
        sources: vec!["BTS Port Rankings 2023".into()],
        estimated: missing_data,
    }
}

// ── Band C ────────────────────────────────────────────────────────────────────

fn score_c1(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.pop_within_50mi {
        Some(pop) => ScoredDimension {
            dim: Dimension::C1PopulationReach,
            score: cfg.c1.score(pop as f64),
            justification: format!("{} people within 50-mile corridor buffer.", format_pop(pop)),
            sources: vec!["Census ACS 2022 5-year estimates".into()],
            estimated: false,
        },
        None => estimated(
            Dimension::C1PopulationReach,
            "Population buffer join incomplete.",
        ),
    }
}

fn score_c2(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    let rural_score = attrs
        .pct_rural_in_buffer
        .map(|r| cfg.c2.rural_share.score(r as f64))
        .unwrap_or(0.0);
    let gap_score = attrs
        .max_rural_interchange_gap_miles
        .map(|g| cfg.c2.interchange_gap.score(g as f64))
        .unwrap_or(0.0);
    let composite = 0.6 * rural_score + 0.4 * gap_score;
    let estimated =
        attrs.pct_rural_in_buffer.is_none() || attrs.max_rural_interchange_gap_miles.is_none();
    ScoredDimension {
        dim: Dimension::C2RuralConnectivity,
        score: composite,
        justification: format!(
            "{:.0}% of buffer population rural; longest rural interchange gap {:.0} mi.",
            attrs.pct_rural_in_buffer.unwrap_or(0.0) * 100.0,
            attrs.max_rural_interchange_gap_miles.unwrap_or(0.0)
        ),
        sources: vec![
            "USDA ERS Rural-Urban Continuum Codes 2023".into(),
            "HighwayGraph interchange analysis".into(),
        ],
        estimated,
    }
}

fn score_c3(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    match attrs.gdp_per_capita_relative {
        Some(rel) => ScoredDimension {
            dim: Dimension::C3EconomicOpportunity,
            score: cfg.c3.score(rel as f64),
            justification: format!(
                "Buffer GDP per capita {:.0}% of national average (${:.1}B total buffer GDP). \
                 Lower relative GDP = higher economic opportunity value of connectivity.",
                rel * 100.0,
                attrs.corridor_gdp_b.unwrap_or(0.0)
            ),
            sources: vec![
                "BEA CAINC4 County GDP 2022".into(),
                "Census ACS 2022 population".into(),
            ],
            estimated: false,
        },
        None => estimated(
            Dimension::C3EconomicOpportunity,
            "BEA GDP data join incomplete.",
        ),
    }
}

// ── Band D ────────────────────────────────────────────────────────────────────

fn score_d1(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    let consec_score = attrs
        .max_consecutive_sfha_miles
        .map(|m| cfg.d1.consecutive_sfha.score(m as f64))
        .unwrap_or(0.0);
    let total_score = attrs
        .fema_sfha_miles
        .map(|m| cfg.d1.total_sfha.score(m))
        .unwrap_or(0.0);
    let flood_score = 0.7 * consec_score + 0.3 * total_score;

    // Multi-hazard extension (v1.4): add wildfire, tornado, seismic components
    let wildfire_score = attrs.wildfire_risk.map(|w| w as f64).unwrap_or(0.0);
    let tornado_score = attrs.tornado_risk.map(|t| t as f64).unwrap_or(0.0);
    let seismic_score = attrs.seismic_risk.map(|s| s as f64).unwrap_or(0.0);
    let has_extended = attrs.wildfire_risk.is_some()
        || attrs.tornado_risk.is_some()
        || attrs.seismic_risk.is_some();
    let has_flood_data =
        attrs.max_consecutive_sfha_miles.is_some() || attrs.fema_sfha_miles.is_some();

    let composite = if has_extended {
        // Four-component weighted composite
        flood_score * 0.40 + wildfire_score * 0.25 + tornado_score * 0.20 + seismic_score * 0.15
    } else {
        // Flood-only (legacy path): preserve pre-v1.4 scoring for corridors without hazard data
        flood_score
    };

    let hazard_note = if has_extended {
        format!(
            " Wildfire {:.1}/10, tornado {:.1}/10, seismic {:.1}/10 (v1.4 multi-hazard).",
            attrs.wildfire_risk.unwrap_or(0.0),
            attrs.tornado_risk.unwrap_or(0.0),
            attrs.seismic_risk.unwrap_or(0.0),
        )
    } else {
        " Wildfire/tornado/seismic not yet joined — run route fetch-hazards.".to_string()
    };

    let mut sources = vec!["FEMA NFHL 2024".into()];
    if has_extended {
        sources.push("USFS Wildfire Hazard Potential 2023".into());
        sources.push("NOAA SPC tornado probability".into());
        sources.push("USGS seismic hazard (Sds)".into());
    }

    ScoredDimension {
        dim: Dimension::D1ClimateResilience,
        score: composite,
        justification: format!(
            "Longest contiguous SFHA segment {:.1} mi; {:.1} total SFHA miles.{hazard_note}",
            attrs.max_consecutive_sfha_miles.unwrap_or(0.0),
            attrs.fema_sfha_miles.unwrap_or(0.0)
        ),
        sources,
        estimated: !has_flood_data && !has_extended,
    }
}

fn score_d2(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    let hub_score = cfg
        .d2
        .intermodal_hubs
        .score(attrs.intermodal_hub_count as f64);
    let ev_score = attrs
        .dcfc_per_100mi
        .map(|d| cfg.d2.dcfc_per_100mi.score(d as f64))
        .unwrap_or(0.0);
    let composite = 0.6 * hub_score + 0.4 * ev_score;
    ScoredDimension {
        dim: Dimension::D2MultimodalIntegration,
        score: composite,
        justification: format!(
            "{} intermodal freight hubs; {:.1} DCFC chargers per 100 miles.",
            attrs.intermodal_hub_count,
            attrs.dcfc_per_100mi.unwrap_or(0.0)
        ),
        sources: vec![
            "AAR intermodal terminal data".into(),
            "DOE AFDC EV charger locator 2024".into(),
        ],
        estimated: attrs.dcfc_per_100mi.is_none(),
    }
}

fn score_d3(attrs: &CorridorAttributes, cfg: &ScoringConfig) -> ScoredDimension {
    let bridge_score = attrs
        .pct_bridges_poor
        .map(|p| cfg.d3.bridges_poor.score(p as f64))
        .unwrap_or(0.0);
    let vintage_score = attrs
        .mean_year_built
        .map(|y| cfg.d3.mean_year_built.score(y as f64))
        .unwrap_or(0.0);
    let composite = 0.6 * bridge_score + 0.4 * vintage_score;
    ScoredDimension {
        dim: Dimension::D3InfrastructureVintage,
        score: composite,
        justification: format!(
            "{} bridges; {:.0}% poor condition; mean construction year {:.0}.",
            attrs.bridge_count,
            attrs.pct_bridges_poor.unwrap_or(0.0) * 100.0,
            attrs.mean_year_built.unwrap_or(0.0)
        ),
        sources: vec!["FHWA NBI 2023".into()],
        estimated: attrs.pct_bridges_poor.is_none(),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn estimated(dim: Dimension, reason: &str) -> ScoredDimension {
    ScoredDimension {
        dim,
        score: 0.0,
        justification: format!("{reason} Score not computed."),
        sources: vec![],
        estimated: true,
    }
}

fn format_pop(pop: u64) -> String {
    if pop >= 1_000_000 {
        format!("{:.1}M", pop as f64 / 1_000_000.0)
    } else {
        format!("{:.0}K", pop as f64 / 1_000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> ScoringConfig {
        ScoringConfig::default_config()
    }

    fn dims(scores: &DimensionScores) -> [&ScoredDimension; 16] {
        [
            &scores.a1, &scores.a2, &scores.a3, &scores.a4, &scores.a5, &scores.b1, &scores.b2,
            &scores.b3, &scores.b4, &scores.c1, &scores.c2, &scores.c3, &scores.c4, &scores.d1,
            &scores.d2, &scores.d3,
        ]
    }

    fn assert_close(left: f64, right: f64) {
        assert!(
            (left - right).abs() < 1e-6,
            "left {left} should be within epsilon of right {right}"
        );
    }

    #[test]
    fn dimension_registry_exposes_sixteen_stable_codes() {
        let dimensions = [
            Dimension::A1ThroughputGap,
            Dimension::A2FreightIntensity,
            Dimension::A3SpeedReliability,
            Dimension::A4InternationalTrade,
            Dimension::A5SafetyRecord,
            Dimension::B1Redundancy,
            Dimension::B2NetworkCentrality,
            Dimension::B3PortBorderAccess,
            Dimension::B4MilitaryStrategic,
            Dimension::C1PopulationReach,
            Dimension::C2RuralConnectivity,
            Dimension::C3EconomicOpportunity,
            Dimension::C4AgriculturalExport,
            Dimension::D1ClimateResilience,
            Dimension::D2MultimodalIntegration,
            Dimension::D3InfrastructureVintage,
        ];
        let codes: Vec<&str> = dimensions.iter().map(Dimension::code).collect();

        assert_eq!(
            codes,
            [
                "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1",
                "D2", "D3",
            ]
        );
        assert!(dimensions.iter().all(|d| !d.name().is_empty()));
    }

    #[test]
    fn sparse_corridor_scores_all_dimensions_with_truth_labels() {
        let scores = score_corridor(&CorridorAttributes::default(), &cfg());
        let dims = dims(&scores);

        assert_eq!(dims.len(), 16);
        assert!(dims.iter().all(|d| d.score.is_finite()));
        assert!(dims.iter().all(|d| (0.0..=10.0).contains(&d.score)));
        assert!(scores.any_estimated());

        for dim in [
            &scores.a1, &scores.a2, &scores.a3, &scores.a5, &scores.b1, &scores.b2, &scores.b3,
            &scores.c1, &scores.c2, &scores.c3, &scores.d1, &scores.d2, &scores.d3,
        ] {
            assert!(
                dim.estimated,
                "{} should be marked estimated",
                dim.dim.code()
            );
        }

        for dim in [&scores.a4, &scores.b4, &scores.c4] {
            assert!(
                !dim.estimated,
                "{} is a deterministic designation score",
                dim.dim.code()
            );
        }
    }

    #[test]
    fn populated_corridor_scores_are_bounded_and_labeled() {
        let attrs = CorridorAttributes {
            p90_aadt: Some(90_000.0),
            mean_aadt: Some(60_000.0),
            annual_freight_value_b: Some(500.0),
            mean_pct_truck: Some(0.22),
            p90_pti: Some(1.8),
            intl_trade_score: 8.0,
            fatal_crash_rate: Some(2.0),
            detour_penalty_miles: Some(250.0),
            nearest_parallel_miles: Some(70.0),
            betweenness_centrality: Some(0.8),
            nearest_top25_port_miles: Some(40.0),
            pop_within_50mi: Some(12_000_000),
            pct_rural_in_buffer: Some(0.35),
            max_rural_interchange_gap_miles: Some(45.0),
            gdp_per_capita_relative: Some(0.75),
            agricultural_export_score: 6.0,
            max_consecutive_sfha_miles: Some(18.0),
            fema_sfha_miles: Some(75.0),
            wildfire_risk: Some(4.0),
            tornado_risk: Some(5.0),
            seismic_risk: Some(2.0),
            intermodal_hub_count: 3,
            dcfc_per_100mi: Some(22.0),
            bridge_count: 100,
            pct_bridges_poor: Some(0.08),
            mean_year_built: Some(1970.0),
            military_strategic_score: 5.0,
            ..Default::default()
        };

        let scores = score_corridor(&attrs, &cfg());

        assert!(dims(&scores)
            .iter()
            .all(|d| (0.0..=10.0).contains(&d.score)));
        assert!(!scores.a1.estimated);
        assert!(scores.a2.estimated, "FAF5 zone traversal remains a proxy");
        assert!(!scores.a3.estimated, "real PTI is authoritative");
        assert!(!scores.a5.estimated);
        assert!(!scores.b1.estimated);
        assert!(!scores.b2.estimated, "provided centrality is authoritative");
        assert!(!scores.b3.estimated);
        assert!(!scores.c1.estimated);
        assert!(!scores.c2.estimated);
        assert!(!scores.c3.estimated);
        assert!(!scores.d1.estimated);
        assert!(!scores.d2.estimated);
        assert!(!scores.d3.estimated);
    }

    #[test]
    fn a3_prefers_real_pti_then_bpr_then_capped_iri() {
        let cfg = cfg();

        let real = score_corridor(
            &CorridorAttributes {
                p90_pti: Some(1.8),
                pti_bpr_estimate: Some(2.2),
                mean_iri: Some(9.0),
                ..Default::default()
            },
            &cfg,
        );
        assert!(!real.a3.estimated);
        assert_close(real.a3.score, cfg.a3.score_pti(1.8));

        let bpr = score_corridor(
            &CorridorAttributes {
                pti_bpr_estimate: Some(2.2),
                mean_iri: Some(9.0),
                ..Default::default()
            },
            &cfg,
        );
        assert!(bpr.a3.estimated);
        assert_close(bpr.a3.score, cfg.a3.score_bpr_pti(2.2));

        let iri = score_corridor(
            &CorridorAttributes {
                mean_iri: Some(30.0),
                ..Default::default()
            },
            &cfg,
        );
        assert!(iri.a3.estimated);
        assert_eq!(iri.a3.score, cfg.a3.iri_fallback_max);
    }

    #[test]
    fn b1_rail_parallel_discount_reduces_redundancy_score() {
        let cfg = cfg();
        let attrs = CorridorAttributes {
            detour_penalty_miles: Some(250.0),
            nearest_parallel_miles: Some(60.0),
            ..Default::default()
        };
        let no_rail = score_corridor(&attrs, &cfg).b1.score;
        let with_rail = score_corridor(
            &CorridorAttributes {
                rail_parallel_flag: true,
                rail_parallel_name: Some("UP Overland Route".into()),
                ..attrs
            },
            &cfg,
        )
        .b1
        .score;

        assert_eq!(with_rail, no_rail * 0.8);
    }

    #[test]
    fn stepped_designation_dimensions_keep_literal_scores() {
        let scores = score_corridor(
            &CorridorAttributes {
                intl_trade_score: 7.0,
                military_strategic_score: 5.0,
                agricultural_export_score: 6.0,
                ..Default::default()
            },
            &cfg(),
        );

        assert_eq!(scores.a4.score, 7.0);
        assert_eq!(scores.b4.score, 5.0);
        assert_eq!(scores.c4.score, 6.0);
        assert!(!scores.a4.estimated);
        assert!(!scores.b4.estimated);
        assert!(!scores.c4.estimated);
    }
}
