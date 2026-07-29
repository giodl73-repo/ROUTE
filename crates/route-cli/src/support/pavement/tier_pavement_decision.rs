//! Helper `tier_pavement_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_decision(
    segment: &TierSegmentCandidateRow,
    standard: Option<&PavementStandardRow>,
    edge: Option<&route_network::HighwayEdge>,
    iri: Option<f32>,
) -> (String, String, String, String) {
    let Some(standard) = standard else {
        return (
            "missing-tier-standard".to_string(),
            format!("author pavement standard for {}", segment.tier),
            "data/tier-pavement-standards.csv".to_string(),
            "review".to_string(),
        );
    };
    if edge.is_none() {
        return (
            "missing-graph-edge".to_string(),
            format!("rebuild segment candidate edge {}", segment.edge_id),
            "data/tier-segment-candidates.csv".to_string(),
            "review".to_string(),
        );
    }
    let Some(iri) = iri else {
        return (
            "pavement-source-needed".to_string(),
            format!(
                "record pavement evidence debt for {} edge {} before SLA or transit readiness claim",
                segment.route, segment.edge_id
            ),
            "data/standards-l1-inventory.csv".to_string(),
            "review".to_string(),
        );
    };
    if f64::from(iri) <= standard.max_iri_m_per_km {
        (
            "pavement-floor-pass".to_string(),
            "no pavement debt payment required for this member".to_string(),
            "data/national-segment-registry.csv".to_string(),
            "pass".to_string(),
        )
    } else {
        (
            "pavement-repair-required".to_string(),
            standard.repair_trigger.clone(),
            "data/tier-pavement-docket.csv".to_string(),
            "review".to_string(),
        )
    }
}

