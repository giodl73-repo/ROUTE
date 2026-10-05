//! Helper `t2_bundle_overlay_repair_target_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_repair_target_gate_failures(
    rows: &[T2BundleOverlayRepairTargetRow],
    decision_rows: &[T2GameOpsBindingDecisionRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = decision_rows
        .iter()
        .filter(|row| row.decision != "bound")
        .map(|row| row.decision_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.is_empty() {
        failures.push("no T2 bundle overlay repair targets emitted".to_string());
        return failures;
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 bundle overlay repair targets have {} rows but expected {} residual decisions",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.target_id.trim().is_empty()
            || row.decision_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.repair_class.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.target_status.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete repair target fields", row.route));
        }
        if !seen.insert(row.decision_id.clone()) {
            failures.push(format!("{} appears more than once", row.decision_id));
        }
        if !expected.contains(row.decision_id.as_str()) {
            failures.push(format!(
                "{} is not a residual game/ops binding decision",
                row.decision_id
            ));
        }
        if !matches!(
            row.repair_class.as_str(),
            "service-class"
                | "stop-chain"
                | "stitched-member"
                | "terminal-stop"
                | "pavement-debt"
                | "local-zone"
                | "manual-review"
        ) {
            failures.push(format!(
                "{} has invalid repair class {}",
                row.route, row.repair_class
            ));
        }
        if !matches!(
            row.target_status.as_str(),
            "pass-candidate" | "repair-needed" | "demote" | "held"
        ) {
            failures.push(format!(
                "{} has invalid target status {}",
                row.route, row.target_status
            ));
        }
        if row.target_status == "pass-candidate" {
            failures.push(format!(
                "{} cannot be a pass candidate in repair intake",
                row.route
            ));
        }
        if row.target_status == "repair-needed"
            && row.binding_status == "bundle-bound-review"
            && (row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty())
        {
            failures.push(format!(
                "{} repair target missing qualification semantics",
                row.route
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.qualification_gate_policy.trim().is_empty()
            && row.qualification_game_use.trim().is_empty()
        {
            failures.push(format!(
                "{} repair target drops qualification contract",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from repair targets"));
        }
    }
    failures
}
