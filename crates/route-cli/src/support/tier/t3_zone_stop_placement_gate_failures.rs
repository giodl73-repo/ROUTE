//! Helper `t3_zone_stop_placement_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_stop_placement_gate_failures(
    rows: &[T3ZoneStopPlacementRow],
    board_rows: &[T3ZoneRenderBoardRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let selected_routes = board_rows
        .iter()
        .filter(|row| row.board_layer == "selected-route")
        .map(|row| {
            (
                row.zone_id.clone(),
                normalise_designation(row.route.as_str()),
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    if selected_routes.is_empty() {
        failures
            .push("no selected T3 render-board routes available for stop placement".to_string());
        return failures;
    }
    let placement_routes = rows
        .iter()
        .map(|row| {
            (
                row.zone_id.clone(),
                normalise_designation(row.route.as_str()),
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    for route in &selected_routes {
        if !placement_routes.contains(route) {
            failures.push(format!(
                "{} {} missing stop placement row",
                route.0, route.1
            ));
        }
    }

    let mut seen = std::collections::BTreeSet::<(String, String)>::new();
    for row in rows {
        if row.zone_id.trim().is_empty()
            || row.zone_name.trim().is_empty()
            || row.route.trim().is_empty()
            || row.national_segment_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.segment_aliases.trim().is_empty()
            || row.placement_status.trim().is_empty()
            || row.placement_action.trim().is_empty()
            || row.source_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} {} has incomplete stop placement fields",
                row.zone_id, row.route
            ));
        }
        let key = (
            row.zone_id.clone(),
            normalise_designation(row.route.as_str()),
        );
        if !seen.insert(key) {
            failures.push(format!(
                "{} {} has duplicate stop placement row",
                row.zone_id, row.route
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} {} has invalid validation status {}",
                row.zone_id, row.route, row.validation_status
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
        if row.placement_status == "ready-for-stop-layout" {
            if row.stop_count < 2
                || row.transfer_grade_stop_count == 0
                || row.stop_chain.trim().is_empty()
                || row.state_scope.trim().is_empty()
            {
                failures.push(format!(
                    "{} {} is marked ready without a viable T3 stop chain",
                    row.zone_id, row.route
                ));
            }
            if row.validation_status != "pass" {
                failures.push(format!("{} {} ready row must pass", row.zone_id, row.route));
            }
        } else {
            if row.validation_status != "review" {
                failures.push(format!(
                    "{} {} gap row must be review",
                    row.zone_id, row.route
                ));
            }
            if row.next_artifact != "data/tier-stop-candidates.csv" {
                failures.push(format!(
                    "{} {} stop gap must return to tier stop candidates",
                    row.zone_id, row.route
                ));
            }
        }
    }

    failures
}
