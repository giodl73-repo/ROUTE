//! Helper `t2_bundle_readiness_disposition_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_disposition_gate_failures(
    rows: &[T2BundleReadinessDispositionRow],
    target_rows: &[T2BundleOverlayRepairTargetRow],
) -> Vec<String> {
    let expected = target_rows
        .iter()
        .filter(|row| {
            matches!(
                row.bundle_status.as_str(),
                "needs-stop-chain" | "needs-stitched-members" | "needs-terminal-stop"
            ) || row.binding_status == "bundle-bound-review"
        })
        .map(|row| row.target_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if rows.len() != expected.len() {
        failures.push(format!(
            "bundle readiness disposition has {} rows but expected {} readiness targets",
            rows.len(),
            expected.len()
        ));
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.disposition_id.trim().is_empty()
            || row.target_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.readiness_class.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.disposition_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.blocks_claims.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete readiness fields", row.route));
        }
        if !seen.insert(row.target_id.clone()) {
            failures.push(format!("{} appears more than once", row.target_id));
        }
        if !expected.contains(row.target_id.as_str()) {
            failures.push(format!("{} is not a readiness target", row.target_id));
        }
        if !matches!(
            row.disposition.as_str(),
            "repair-needed" | "demote" | "held"
        ) {
            failures.push(format!(
                "{} has invalid readiness disposition {}",
                row.route, row.disposition
            ));
        }
        if row.route == "I37" && row.disposition != "repair-needed" {
            failures.push("I37 bundle-bound-review must remain repair-needed".to_string());
        }
        if !row.qualification_effects.trim().is_empty() && row.disposition.trim().is_empty() {
            failures.push(format!(
                "{} readiness disposition has qualification effects without disposition",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} readiness disposition must remain review",
                row.route
            ));
        }
    }
    for expected_id in expected {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} missing from readiness disposition"));
        }
    }
    failures
}
