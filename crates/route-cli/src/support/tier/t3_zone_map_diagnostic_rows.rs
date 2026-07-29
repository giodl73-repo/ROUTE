//! Helper `t3_zone_map_diagnostic_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_map_diagnostic_rows(
    route_rows: &[T3ZoneRouteColumnRow],
    gap_rows: &[T3T4AccessGapRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<T3ZoneMapDiagnosticRow> {
    let mut rows = Vec::new();

    for atlas in atlas_rows.iter().filter(|row| row.map_type == "t3-zone") {
        let zone_id = atlas.map_id.clone();
        let zone_name = t3_zone_catalog_entry(&zone_id)
            .map(|(_, name)| name.to_string())
            .unwrap_or_else(|| atlas.tier_role.clone());

        let selected_routes = route_rows
            .iter()
            .filter(|row| row.zone_id == zone_id && row.column_decision == "selected")
            .map(|row| row.route.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let review_connectors = route_rows
            .iter()
            .filter(|row| row.zone_id == zone_id && row.column_decision != "selected")
            .map(|row| row.route.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let zone_gaps = gap_rows
            .iter()
            .filter(|row| row.zone_id == zone_id)
            .collect::<Vec<_>>();
        let below_threshold_feeder_count = zone_gaps
            .iter()
            .filter(|row| row.gap_class == "below-threshold-feeder")
            .count();
        let terminal_evidence_gap_count = zone_gaps
            .iter()
            .filter(|row| row.gap_class == "terminal-evidence-needed")
            .count();
        let zone_assignment_gap_count = zone_gaps
            .iter()
            .filter(|row| row.gap_class == "zone-assignment-needed")
            .count();

        let (map_readiness, diagnostic_action, validation_status) = t3_zone_map_diagnostic_decision(
            selected_routes.len(),
            zone_gaps.len(),
            zone_assignment_gap_count,
        );

        rows.push(T3ZoneMapDiagnosticRow {
            zone_id: zone_id.clone(),
            zone_name,
            map_id: atlas.map_id.clone(),
            map_path: atlas.path.clone(),
            selected_route_count: selected_routes.len(),
            selected_routes: selected_routes.into_iter().collect::<Vec<_>>().join(";"),
            review_connector_count: review_connectors.len(),
            review_connectors: review_connectors.into_iter().collect::<Vec<_>>().join(";"),
            access_gap_count: zone_gaps.len(),
            below_threshold_feeder_count,
            terminal_evidence_gap_count,
            zone_assignment_gap_count,
            map_readiness: map_readiness.to_string(),
            diagnostic_action: diagnostic_action.to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: validation_status.to_string(),
        });
    }

    rows.sort_by(|a, b| a.zone_id.cmp(&b.zone_id));
    rows
}

