//! Helper `t2_service_class_repair_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_class_repair_docket_gate_failures(
    rows: &[T2ServiceClassRepairDocketRow],
    target_rows: &[T2BundleOverlayRepairTargetRow],
) -> Vec<String> {
    let expected = target_rows
        .iter()
        .filter(|row| row.service_class == "unclassified" || row.repair_class == "service-class")
        .map(|row| row.target_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "service-class repair docket has {} rows but expected {} service-class-held targets",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.docket_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.service_repair_class.trim().is_empty()
            || row.service_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete service repair fields",
                row.route
            ));
        }
        if !seen.insert(row.target_id.clone()) {
            failures.push(format!("{} appears more than once", row.target_id));
        }
        if !expected.contains(row.target_id.as_str()) {
            failures.push(format!("{} is not a service-class target", row.target_id));
        }
        if !matches!(
            row.service_repair_class.as_str(),
            "beck-diagnostic" | "local-zone" | "service-overlay"
        ) {
            failures.push(format!(
                "{} has invalid service repair class {}",
                row.route, row.service_repair_class
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} service repair must remain review", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && !row.optimizer_effect.contains("qualification")
        {
            failures.push(format!(
                "{} service repair drops qualification effects",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from service repair docket"));
        }
    }
    failures
}

