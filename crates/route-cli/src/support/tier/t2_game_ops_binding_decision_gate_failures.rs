//! Helper `t2_game_ops_binding_decision_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_binding_decision_gate_failures(
    rows: &[T2GameOpsBindingDecisionRow],
    intake_rows: &[T2GameOpsBindingIntakeRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = intake_rows
        .iter()
        .map(|row| row.intake_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if rows.is_empty() {
        failures.push("no T2 game/ops binding decision rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected.len() {
        failures.push(format!(
            "T2 game/ops binding decisions have {} rows but expected {} intake rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.decision_id.trim().is_empty()
            || row.intake_id.trim().is_empty()
            || row.subject_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.bundle_status.trim().is_empty()
            || row.binding_status.trim().is_empty()
            || row.decision.trim().is_empty()
            || row.decision_reason.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete decision fields",
                row.decision_id
            ));
        }
        if !seen.insert(row.intake_id.clone()) {
            failures.push(format!("{} appears more than once", row.intake_id));
        }
        if !expected.contains(row.intake_id.as_str()) {
            failures.push(format!("{} does not appear in intake", row.intake_id));
        }
        if !matches!(
            row.decision.as_str(),
            "bound" | "repair-needed" | "demote" | "held"
        ) {
            failures.push(format!(
                "{} has invalid decision {}",
                row.route, row.decision
            ));
        }
        if row.decision == "bound" {
            if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.")
                || row.service_class == "unclassified"
                || row.binding_status != "bundle-bound"
                || row.validation_status != "pass"
                || !row.blocks_claims.trim().is_empty()
            {
                failures.push(format!(
                    "{} bound decision lacks passing bundle binding",
                    row.route
                ));
            }
            if row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty()
            {
                failures.push(format!(
                    "{} bound decision missing qualification semantics",
                    row.route
                ));
            }
            if !row.qualification_effects.trim().is_empty()
                && row.qualification_gate_policy.trim().is_empty()
                && row.qualification_game_use.trim().is_empty()
            {
                failures.push(format!(
                    "{} bound decision drops qualification contract",
                    row.route
                ));
            }
        } else if row.decision == "repair-needed" && row.binding_status == "bundle-bound-review" {
            if row.qualification_gate_policy.trim().is_empty()
                || row.qualification_game_use.trim().is_empty()
            {
                failures.push(format!(
                    "{} repair decision missing qualification semantics",
                    row.route
                ));
            }
        } else if row.validation_status != "review" || row.blocks_claims.trim().is_empty() {
            failures.push(format!(
                "{} residual decision must remain review with blocked claims",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} missing from T2 game/ops binding decisions"
            ));
        }
    }
    failures
}
