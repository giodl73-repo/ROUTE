//! Helper `t2_bundle_overlay_repair_delta_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_repair_delta_gate_failures(
    rows: &[T2BundleOverlayRepairDeltaRow],
    decision_rows: &[T2GameOpsBindingDecisionRow],
) -> Vec<String> {
    let expected = decision_rows
        .iter()
        .filter(|row| row.decision != "bound")
        .map(|row| row.decision_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "repair delta has {} rows but expected {} residual decisions",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.delta_id.trim().is_empty()
            || row.decision_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.previous_decision.trim().is_empty()
            || row.target_status.trim().is_empty()
            || row.service_action.trim().is_empty()
            || row.readiness_disposition.trim().is_empty()
            || row.replay_decision.trim().is_empty()
            || row.blocked_claims_before.trim().is_empty()
            || row.blocked_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete delta fields", row.route));
        }
        if !seen.insert(row.decision_id.clone()) {
            failures.push(format!("{} appears more than once", row.decision_id));
        }
        if !expected.contains(row.decision_id.as_str()) {
            failures.push(format!("{} is not a residual decision", row.decision_id));
        }
        if row.replay_decision == "bound" {
            failures.push(format!(
                "{} replayed as bound without overlay pass mutation",
                row.route
            ));
        }
        if row.blocked_claims_before != row.blocked_claims_after || row.blocker_delta != 0 {
            failures.push(format!("{} lost residual blocked claims", row.route));
        }
        if !row.qualification_effects.trim().is_empty() && row.replay_decision.trim().is_empty() {
            failures.push(format!(
                "{} repair delta has qualification effects without replay decision",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} delta must remain review", row.route));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from repair delta"));
        }
    }
    failures
}

