//! Helper `tier_pavement_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_docket_rows(
    graph: &route_network::HighwayGraph,
    segment_rows: &[TierSegmentCandidateRow],
    standard_rows: &[PavementStandardRow],
) -> Vec<TierPavementDocketRow> {
    let standards = standard_rows
        .iter()
        .map(|row| (row.tier.trim().to_string(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let edge_by_id = graph
        .graph
        .edge_indices()
        .map(|idx| (graph.graph[idx].id, &graph.graph[idx]))
        .collect::<std::collections::BTreeMap<_, _>>();

    segment_rows
        .iter()
        .map(|segment| {
            let standard = standards.get(segment.tier.trim());
            let edge = edge_by_id.get(&segment.edge_id).copied();
            let iri = edge.and_then(|edge| normalized_iri_m_per_km(edge.iri));
            let max_iri = standard.map(|row| row.max_iri_m_per_km);
            let (pavement_status, repair_action, next_artifact, validation_status) =
                tier_pavement_decision(segment, standard.copied(), edge, iri);

            TierPavementDocketRow {
                tier: segment.tier.clone(),
                source_selector: segment.source_selector.clone(),
                region_id: segment.region_id.clone(),
                route: segment.route.clone(),
                segment_bundle_id: segment.segment_bundle_id.clone(),
                stitch_group_id: segment.stitch_group_id.clone(),
                national_segment_id: segment.national_segment_id.clone(),
                edge_id: segment.edge_id,
                edge_sequence: segment.edge_sequence,
                state: segment.state.clone(),
                length_miles: segment.length_miles,
                iri_m_per_km: iri
                    .map(|value| format!("{:.2}", value))
                    .unwrap_or_else(|| "unknown".to_string()),
                max_iri_m_per_km: max_iri
                    .map(|value| format!("{value:.2}"))
                    .unwrap_or_else(|| "unknown".to_string()),
                pavement_status,
                repair_action,
                freight_ride_requirement: standard
                    .map(|row| row.freight_ride_requirement.clone())
                    .unwrap_or_else(|| "tier pavement standard missing".to_string()),
                transit_ride_requirement: standard
                    .map(|row| row.transit_ride_requirement.clone())
                    .unwrap_or_else(|| "tier pavement standard missing".to_string()),
                source_contract: standard
                    .map(|row| row.source_contract.clone())
                    .unwrap_or_else(|| "data/tier-pavement-standards.csv".to_string()),
                qualification_effects: segment.qualification_effects.clone(),
                next_artifact,
                validation_status,
            }
        })
        .collect()
}

