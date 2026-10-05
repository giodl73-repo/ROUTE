//! Helper `t2_bundle_overlay_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_gate_failures(rows: &[T2BundleOverlayRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 bundle overlay rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.service_class.trim().is_empty()
            || row.binding_status.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.source_artifacts.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete bundle overlay fields",
                row.route
            ));
        }
        if !matches!(
            row.binding_status.as_str(),
            "bundle-bound"
                | "bundle-bound-review"
                | "bundle-binding-pending"
                | "service-class-overlay-pending"
                | "service-class-held-known"
        ) {
            failures.push(format!(
                "{} has unknown binding status {}",
                row.route, row.binding_status
            ));
        }
        if row.binding_status.starts_with("bundle-bound")
            && !row.segment_bundle_id.starts_with("US.HWYBUNDLE.")
        {
            failures.push(format!(
                "{} claims bundle binding without a bundle id",
                row.route
            ));
        }
        if row.binding_status == "bundle-bound" && row.validation_status != "pass" {
            failures.push(format!("{} bound bundle did not pass", row.route));
        }
        if row.pavement_debt_cost_m < 0.0 {
            failures.push(format!("{} has negative pavement debt cost", row.route));
        }
        if row.pavement_debt_cost_m > 0.0
            && (row.pavement_debt_class.trim().is_empty()
                || row.pavement_debt_basis.trim().is_empty())
        {
            failures.push(format!(
                "{} has pavement debt cost without debt class and basis",
                row.route
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.route, row.validation_status
            ));
        }
        if matches!(
            row.binding_status.as_str(),
            "bundle-bound" | "bundle-bound-review"
        ) && (row.qualification_map_treatment.trim().is_empty()
            || row.qualification_gate_policy.trim().is_empty()
            || row.qualification_game_use.trim().is_empty())
        {
            failures.push(format!(
                "{} bound overlay missing qualification action semantics",
                row.route
            ));
        }
    }
    failures
}
