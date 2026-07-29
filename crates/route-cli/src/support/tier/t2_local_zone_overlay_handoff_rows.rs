//! Helper `t2_local_zone_overlay_handoff_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_local_zone_overlay_handoff_rows(
    docket_rows: &[T2ServiceClassRepairDocketRow],
    route_rows: &[T3ZoneRouteColumnRow],
    board_rows: &[T3ZoneRenderBoardRow],
) -> Vec<T2LocalZoneOverlayHandoffRow> {
    let selected_routes = route_rows
        .iter()
        .filter(|row| row.column_decision == "selected")
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let render_rows = board_rows
        .iter()
        .filter(|row| row.board_layer == "selected-route")
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = docket_rows
        .iter()
        .filter(|row| row.service_repair_class == "local-zone")
        .map(|row| {
            let route_key = canonical_route_key(&row.route);
            let zone_route = selected_routes.get(&route_key);
            let render_row = render_rows.get(&route_key);
            let zone_id = zone_route
                .map(|row| row.zone_id.clone())
                .or_else(|| render_row.map(|row| row.zone_id.clone()))
                .unwrap_or_else(|| "missing-zone".to_string());
            let zone_name = zone_route
                .map(|row| row.zone_name.clone())
                .or_else(|| render_row.map(|row| row.zone_name.clone()))
                .unwrap_or_else(|| "missing zone context".to_string());
            let zone_role = zone_route
                .map(|row| row.zone_role.clone())
                .unwrap_or_else(|| "missing-zone-role".to_string());
            let column_decision = zone_route
                .map(|row| row.column_decision.clone())
                .unwrap_or_else(|| "held".to_string());
            let map_treatment = render_row
                .map(|row| row.map_treatment.clone())
                .or_else(|| zone_route.map(|row| row.map_treatment.clone()))
                .unwrap_or_else(|| "missing-map-treatment".to_string());
            let (handoff_decision, handoff_reason, required_artifact, next_artifact) =
                if zone_route.is_some() && render_row.is_some() {
                    (
                        "held-local-zone",
                        "local relief is represented as a T3 zone role and remains below national T2 game overlay",
                        "data/t3-zone-render-board.csv",
                        "data/t3-zone-stop-placement.csv",
                    )
                } else {
                    (
                        "held-missing-zone-context",
                        "local relief lacks complete T3 zone role or map treatment context",
                        "data/t3-zone-route-columns.csv",
                        "data/t3-zone-render-board.csv",
                    )
                };
            T2LocalZoneOverlayHandoffRow {
                handoff_id: format!("T2LOCALZONE-{}", stable_id_fragment(&row.docket_id)),
                docket_id: row.docket_id.clone(),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                zone_id,
                zone_name,
                zone_role,
                column_decision,
                map_treatment,
                handoff_decision: handoff_decision.to_string(),
                handoff_reason: handoff_reason.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                blocks_claims: "game;incident;publication;upgrade".to_string(),
                required_artifact: required_artifact.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.zone_id
            .cmp(&right.zone_id)
            .then(left.route.cmp(&right.route))
    });
    rows
}

