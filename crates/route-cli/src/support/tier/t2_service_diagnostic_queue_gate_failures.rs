//! Helper `t2_service_diagnostic_queue_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_diagnostic_queue_gate_failures(rows: &[T2ServiceDiagnosticQueueRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 service diagnostic queue rows emitted".to_string());
        return failures;
    }
    if rows.len() == 1 && rows[0].route == "__all_t2_service_diagnostics__" {
        let row = &rows[0];
        if row.diagnostic_status != "service-diagnostic-clear" || row.validation_status != "pass" {
            failures
                .push("service diagnostic clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty() {
            failures.push("service diagnostic queue row missing route".to_string());
        }
        if row.region_id.trim().is_empty() {
            failures.push(format!("{} missing region_id", row.route));
        }
        if row.segment_bundle_id.trim().is_empty() {
            failures.push(format!("{} missing segment_bundle_id", row.route));
        }
        if row.bundle_status != "bundle-ready" {
            failures.push(format!(
                "{} service diagnostic row is not bundle-ready ({})",
                row.route, row.bundle_status
            ));
        }
        if !matches!(
            row.diagnostic_status.as_str(),
            "beck-diagnostic-missing"
                | "beck-diagnostic-review"
                | "route-family-diagnostic-split-needed"
                | "local-relief-map-review"
        ) {
            failures.push(format!(
                "{} unexpected diagnostic_status {}",
                row.route, row.diagnostic_status
            ));
        }
        if row.service_diagnostic_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!("{} missing diagnostic action artifacts", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && !row.optimizer_effect.contains("qualification")
        {
            failures.push(format!(
                "{} diagnostic row drops qualification effects",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} diagnostic queue row must remain review",
                row.route
            ));
        }
    }
    failures
}

