//! Helper `t2_overlay_optimizer_action_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_overlay_optimizer_action_docket_gate_failures(
    rows: &[T2OverlayOptimizerActionDocketRow],
    delta_rows: &[T2BundleOverlayRepairDeltaRow],
) -> Vec<String> {
    let expected = delta_rows
        .iter()
        .map(|row| row.delta_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "optimizer action docket has {} rows but expected {} delta rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.action_id.trim().is_empty()
            || row.delta_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.replay_decision.trim().is_empty()
            || row.service_action.trim().is_empty()
            || row.readiness_disposition.trim().is_empty()
            || row.optimizer_action.trim().is_empty()
            || row.priority_class.trim().is_empty()
            || row.action_status.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete optimizer action fields",
                row.route
            ));
        }
        if !seen.insert(row.delta_id.clone()) {
            failures.push(format!("{} appears more than once", row.delta_id));
        }
        if !expected.contains(row.delta_id.as_str()) {
            failures.push(format!("{} is not a repair-delta row", row.delta_id));
        }
        if row.replay_decision == "bound"
            || row.action_status != "optimizer-held-known"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} promoted optimizer action prematurely",
                row.route
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.action_status != "optimizer-held-known"
        {
            failures.push(format!(
                "{} optimizer action carries qualification effects without held status",
                row.route
            ));
        }
        if row.blocked_claims_before != row.blocked_claims_after || row.blocker_delta != 0 {
            failures.push(format!("{} did not preserve claim blockers", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from optimizer action docket"
            ));
        }
    }
    failures
}
