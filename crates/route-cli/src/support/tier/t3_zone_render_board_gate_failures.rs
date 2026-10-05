//! Helper `t3_zone_render_board_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_render_board_gate_failures(
    rows: &[T3ZoneRenderBoardRow],
    atlas_rows: &[MapAtlasRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3 zone render board rows emitted".to_string());
        return failures;
    }

    let t3_maps = t3_zone_map_ids(atlas_rows);
    let summary_maps = rows
        .iter()
        .filter(|row| row.board_layer == "zone-summary")
        .map(|row| row.map_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    for map_id in &t3_maps {
        if !summary_maps.contains(map_id) {
            failures.push(format!("{map_id} has no zone-summary render row"));
        }
    }

    for row in rows {
        if row.zone_id.trim().is_empty()
            || row.zone_name.trim().is_empty()
            || row.map_id.trim().is_empty()
            || row.map_path.trim().is_empty()
            || row.board_layer.trim().is_empty()
            || row.national_segment_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.segment_aliases.trim().is_empty()
            || row.route_status.trim().is_empty()
            || row.map_treatment.trim().is_empty()
            || row.source_artifact.trim().is_empty()
            || row.render_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} {} {} has incomplete render board fields",
                row.zone_id, row.board_layer, row.route
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} {} has invalid validation status {}",
                row.zone_id, row.board_layer, row.validation_status
            ));
        }
        if row.zone_id != "zone-assignment-needed" {
            if row.zone_id != row.map_id {
                failures.push(format!(
                    "{} render row detached from map id {}",
                    row.zone_id, row.map_id
                ));
            }
            if !t3_maps.contains(&row.map_id) {
                failures.push(format!("{} references unknown T3 map", row.zone_id));
            }
        }
        if row.board_layer == "zone-summary" && row.selected_route_count == 0 {
            failures.push(format!(
                "{} has no selected route render count",
                row.zone_id
            ));
        }
        if matches!(
            row.board_layer.as_str(),
            "selected-route" | "review-connector" | "held-gap"
        ) && row.route.trim().is_empty()
        {
            failures.push(format!(
                "{} {} is missing route",
                row.zone_id, row.board_layer
            ));
        }
        if !row.national_segment_id.starts_with("US.HWYSEG.") {
            failures.push(format!(
                "{} {} has non-hierarchical segment id {}",
                row.zone_id, row.route, row.national_segment_id
            ));
        }
        if !row.stitch_group_id.starts_with("US.HWYSTITCH.") {
            failures.push(format!(
                "{} {} has non-hierarchical stitch group {} for segment id {}",
                row.zone_id, row.route, row.stitch_group_id, row.national_segment_id
            ));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!(
                "{} {} has non-hierarchical bundle id {}",
                row.zone_id, row.route, row.segment_bundle_id
            ));
        }
        if row.board_layer == "selected-route"
            && (row.route_status != "selected" || row.map_treatment != "render-as-zone-column")
        {
            failures.push(format!(
                "{} {} selected route is not renderable",
                row.zone_id, row.route
            ));
        }
        if row.board_layer == "unassigned-gap-backlog" && row.map_treatment != "hide-until-assigned"
        {
            failures.push("unassigned backlog must stay hidden until assigned".to_string());
        }
    }

    failures
}
