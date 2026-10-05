//! Helper `t2_overlay_p3_local_zone_overlay_review_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_overlay_p3_local_zone_overlay_review_gate_failures(
    rows: &[T2OverlayP3LocalZoneOverlayReviewRow],
    action_rows: &[T2OverlayOptimizerActionDocketRow],
) -> Vec<String> {
    let expected = action_rows
        .iter()
        .filter(|row| row.priority_class == "P3-local-zone-overlay")
        .map(|row| row.action_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if expected.is_empty() {
        failures.push("P3 local-zone overlay review has no P3 optimizer actions".to_string());
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "P3 local-zone overlay review has {} rows but expected {} P3 action rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.p3_review_id.trim().is_empty()
            || row.action_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.optimizer_action.trim().is_empty()
            || row.priority_class.trim().is_empty()
            || row.local_zone_decision.trim().is_empty()
            || row.local_zone_reason.trim().is_empty()
            || row.downstream_action.trim().is_empty()
            || row.action_status.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete P3 local-zone fields", row.route));
        }
        if !seen.insert(row.action_id.clone()) {
            failures.push(format!("{} appears more than once", row.action_id));
        }
        if !expected.contains(row.action_id.as_str()) {
            failures.push(format!(
                "{} is not a P3 optimizer action row",
                row.action_id
            ));
        }
        if row.priority_class != "P3-local-zone-overlay"
            || row.local_zone_decision != "held-local-zone-overlay-review-needed"
            || row.downstream_action != "route-to-local-zone-overlay-review"
            || row.action_status != "optimizer-held-known"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} P3 local-zone review promoted action",
                row.route
            ));
        }
        if row.blocked_claims_before != row.blocked_claims_after || row.blocker_delta != 0 {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.action_status != "optimizer-held-known"
        {
            failures.push(format!(
                "{} P3 local-zone review carries qualification effects without held status",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from P3 local-zone review"));
        }
    }
    failures
}
