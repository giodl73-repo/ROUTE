//! Extracted helper `t3_zone_render_board_rows` from main.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_render_board_rows(
    diagnostic_rows: &[T3ZoneMapDiagnosticRow],
    route_rows: &[T3ZoneRouteColumnRow],
    gap_rows: &[T3T4AccessGapRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<T3ZoneRenderBoardRow> {
    let atlas_by_id = atlas_rows
        .iter()
        .filter(|row| row.map_type == "t3-zone")
        .map(|row| (row.map_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = Vec::new();

    for diagnostic in diagnostic_rows {
        let map_path = atlas_by_id
            .get(diagnostic.map_id.as_str())
            .map(|row| row.path.as_str())
            .unwrap_or(diagnostic.map_path.as_str());
        rows.push(T3ZoneRenderBoardRow {
            zone_id: diagnostic.zone_id.clone(),
            zone_name: diagnostic.zone_name.clone(),
            map_id: diagnostic.map_id.clone(),
            map_path: map_path.to_string(),
            board_layer: "zone-summary".to_string(),
            route: String::new(),
            national_segment_id: t3_national_segment_id(&diagnostic.zone_id, ""),
            stitch_group_id: t3_stitch_group_id(&diagnostic.zone_id, ""),
            segment_bundle_id: t3_segment_bundle_id(&diagnostic.zone_id, ""),
            segment_aliases: t3_segment_aliases(&diagnostic.zone_id, "", "zone-summary"),
            route_status: diagnostic.map_readiness.clone(),
            map_treatment: "render-zone-summary".to_string(),
            selected_route_count: diagnostic.selected_route_count,
            access_gap_count: diagnostic.access_gap_count,
            source_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            render_action: diagnostic.diagnostic_action.clone(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: diagnostic.validation_status.clone(),
        });

        let mut zone_routes = route_rows
            .iter()
            .filter(|row| row.zone_id == diagnostic.zone_id)
            .collect::<Vec<_>>();
        zone_routes.sort_by(|a, b| {
            route_layer_rank(&a.column_decision)
                .cmp(&route_layer_rank(&b.column_decision))
                .then_with(|| b.current_score.total_cmp(&a.current_score))
                .then_with(|| a.route.cmp(&b.route))
        });

        for route in zone_routes {
            let board_layer = match route.column_decision.as_str() {
                "selected" => "selected-route",
                _ => "review-connector",
            };
            rows.push(T3ZoneRenderBoardRow {
                zone_id: diagnostic.zone_id.clone(),
                zone_name: diagnostic.zone_name.clone(),
                map_id: diagnostic.map_id.clone(),
                map_path: map_path.to_string(),
                board_layer: board_layer.to_string(),
                route: route.route.clone(),
                national_segment_id: t3_national_segment_id(&diagnostic.zone_id, &route.route),
                stitch_group_id: t3_stitch_group_id(&diagnostic.zone_id, &route.route),
                segment_bundle_id: t3_segment_bundle_id(&diagnostic.zone_id, &route.route),
                segment_aliases: t3_segment_aliases(&diagnostic.zone_id, &route.route, board_layer),
                route_status: route.column_decision.clone(),
                map_treatment: route.map_treatment.clone(),
                selected_route_count: diagnostic.selected_route_count,
                access_gap_count: diagnostic.access_gap_count,
                source_artifact: "data/t3-zone-route-columns.csv".to_string(),
                render_action: route_render_action(route),
                next_artifact: "maps/t3-zone".to_string(),
                validation_status: route.validation_status.clone(),
            });
        }

        let mut zone_gaps = gap_rows
            .iter()
            .filter(|row| row.zone_id == diagnostic.zone_id)
            .collect::<Vec<_>>();
        zone_gaps.sort_by(|a, b| {
            a.gap_class
                .cmp(&b.gap_class)
                .then_with(|| b.current_score.total_cmp(&a.current_score))
                .then_with(|| a.route.cmp(&b.route))
        });
        for gap in zone_gaps {
            rows.push(T3ZoneRenderBoardRow {
                zone_id: diagnostic.zone_id.clone(),
                zone_name: diagnostic.zone_name.clone(),
                map_id: diagnostic.map_id.clone(),
                map_path: map_path.to_string(),
                board_layer: "held-gap".to_string(),
                route: gap.route.clone(),
                national_segment_id: t3_national_segment_id(&diagnostic.zone_id, &gap.route),
                stitch_group_id: t3_stitch_group_id(&diagnostic.zone_id, &gap.route),
                segment_bundle_id: t3_segment_bundle_id(&diagnostic.zone_id, &gap.route),
                segment_aliases: t3_segment_aliases(&diagnostic.zone_id, &gap.route, "held-gap"),
                route_status: gap.gap_class.clone(),
                map_treatment: "render-gap-callout".to_string(),
                selected_route_count: diagnostic.selected_route_count,
                access_gap_count: diagnostic.access_gap_count,
                source_artifact: gap.source_surface.clone(),
                render_action: gap.repair_action.clone(),
                next_artifact: gap.next_artifact.clone(),
                validation_status: gap.validation_status.clone(),
            });
        }
    }

    let unassigned_gaps = gap_rows
        .iter()
        .filter(|row| row.zone_id == "zone-assignment-needed")
        .collect::<Vec<_>>();
    if !unassigned_gaps.is_empty() {
        rows.push(T3ZoneRenderBoardRow {
            zone_id: "zone-assignment-needed".to_string(),
            zone_name: "Unassigned Terminal / Local Access".to_string(),
            map_id: "zone-assignment-needed".to_string(),
            map_path: "data/t3-t4-access-gaps.csv".to_string(),
            board_layer: "unassigned-gap-backlog".to_string(),
            route: String::new(),
            national_segment_id: t3_national_segment_id("zone-assignment-needed", ""),
            stitch_group_id: t3_stitch_group_id("zone-assignment-needed", ""),
            segment_bundle_id: t3_segment_bundle_id("zone-assignment-needed", ""),
            segment_aliases: t3_segment_aliases(
                "zone-assignment-needed",
                "",
                "unassigned-gap-backlog",
            ),
            route_status: "zone-assignment-needed".to_string(),
            map_treatment: "hide-until-assigned".to_string(),
            selected_route_count: 0,
            access_gap_count: unassigned_gaps.len(),
            source_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            render_action: "assign-zone-or-terminal-district-before-render".to_string(),
            next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| board_layer_rank(&a.board_layer).cmp(&board_layer_rank(&b.board_layer)))
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.route_status.cmp(&b.route_status))
    });
    rows
}
