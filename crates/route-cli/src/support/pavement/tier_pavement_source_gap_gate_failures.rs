//! Helper `tier_pavement_source_gap_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_gap_gate_failures(
    rows: &[TierPavementSourceGapRow],
    docket_rows: &[TierPavementDocketRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_blocker_bundles = docket_rows
        .iter()
        .filter(|row| {
            row.validation_status == "review" || row.pavement_status != "pavement-floor-pass"
        })
        .map(|row| row.segment_bundle_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.len() != expected_blocker_bundles.len() {
        failures.push(format!(
            "source-gap rows {} do not match blocker bundle count {}",
            rows.len(),
            expected_blocker_bundles.len()
        ));
    }
    for row in rows {
        if row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.blocker_statuses.trim().is_empty()
            || row.affected_edge_ids.trim().is_empty()
            || row.source_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pavement source-gap row",
                row.route
            ));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!("{} is not a bundle id", row.segment_bundle_id));
        }
        if !row.stitch_group_id.starts_with("US.HWYSTITCH.") {
            failures.push(format!("{} is not a stitch id", row.stitch_group_id));
        }
        if row.blocker_count == 0 || row.member_count < row.blocker_count {
            failures.push(format!(
                "{} has invalid pavement blocker count {}/{}",
                row.route, row.blocker_count, row.member_count
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.route, row.validation_status
            ));
        }
    }
    failures
}

