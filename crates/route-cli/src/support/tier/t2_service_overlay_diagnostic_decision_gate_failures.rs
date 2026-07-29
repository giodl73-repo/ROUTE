//! Helper `t2_service_overlay_diagnostic_decision_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_overlay_diagnostic_decision_gate_failures(
    rows: &[T2ServiceOverlayDiagnosticDecisionRow],
    docket_rows: &[T2ServiceClassRepairDocketRow],
) -> Vec<String> {
    let expected = docket_rows
        .iter()
        .filter(|row| row.service_repair_class == "service-overlay")
        .map(|row| row.docket_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "service overlay diagnostic decisions have {} rows but expected {} service-overlay repair rows",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.decision_id.trim().is_empty()
            || row.docket_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.current_service_class.trim().is_empty()
            || row.diagnostic_status.trim().is_empty()
            || row.diagnostic_action.trim().is_empty()
            || row.overlay_decision.trim().is_empty()
            || row.decision_reason.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete diagnostic fields", row.route));
        }
        if !seen.insert(row.docket_id.clone()) {
            failures.push(format!("{} appears more than once", row.docket_id));
        }
        if !expected.contains(row.docket_id.as_str()) {
            failures.push(format!(
                "{} is not a service-overlay repair row",
                row.docket_id
            ));
        }
        if row.current_service_class == "unclassified" && row.overlay_decision != "held" {
            failures.push(format!("{} promoted without a service class", row.route));
        }
        if row.overlay_decision == "bound" {
            failures.push(format!(
                "{} cannot bind from diagnostic decision surface",
                row.route
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && row.current_service_class.trim().is_empty()
        {
            failures.push(format!(
                "{} diagnostic decision has qualification effects without service class context",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} diagnostic decision must remain review",
                row.route
            ));
        }
        if row.current_service_class == "unclassified"
            && row.required_artifact != "data/beck-t2-diagnostics.csv"
        {
            failures.push(format!(
                "{} unclassified row must point to Beck diagnostics",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from diagnostic decisions"));
        }
    }
    failures
}

