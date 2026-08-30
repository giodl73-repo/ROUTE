//! Helper `t3_zone_stop_placement_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_stop_placement_rows(
    board_rows: &[T3ZoneRenderBoardRow],
    stop_rows: &[StopCandidateRow],
) -> Vec<T3ZoneStopPlacementRow> {
    let mut rows = board_rows
        .iter()
        .filter(|row| row.board_layer == "selected-route")
        .map(|board| {
            let stop_plan = t3_zone_stop_plan_for_route(stop_rows, &board.route, &board.zone_id);
            let transfer_grade_stop_count = stop_plan
                .iter()
                .filter(|stop| t3_transfer_grade_stop(stop))
                .count();
            let stop_chain = stop_plan
                .iter()
                .map(|stop| stop.stop_id.as_str())
                .collect::<Vec<_>>()
                .join(";");
            let stop_classes = stop_plan
                .iter()
                .map(|stop| stop.requested_class.trim())
                .filter(|class| !class.is_empty())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(";");
            let state_scope = t3_stop_state_scope(&stop_plan);
            let (placement_status, placement_action, next_artifact, validation_status) =
                t3_zone_stop_placement_decision(stop_plan.len(), transfer_grade_stop_count);

            T3ZoneStopPlacementRow {
                zone_id: board.zone_id.clone(),
                zone_name: board.zone_name.clone(),
                route: board.route.clone(),
                national_segment_id: board.national_segment_id.clone(),
                stitch_group_id: board.stitch_group_id.clone(),
                segment_bundle_id: board.segment_bundle_id.clone(),
                segment_aliases: board.segment_aliases.clone(),
                state_scope,
                stop_count: stop_plan.len(),
                transfer_grade_stop_count,
                stop_chain,
                stop_classes,
                placement_status: placement_status.to_string(),
                placement_action: placement_action.to_string(),
                source_artifact: "data/t3-zone-render-board.csv; data/tier-stop-candidates.csv"
                    .to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| a.placement_status.cmp(&b.placement_status))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}
