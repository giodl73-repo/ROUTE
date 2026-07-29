//! Helper `t2_bundle_repair_queue_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_repair_queue_gate_failures(rows: &[T2BundleRepairQueueRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 bundle repair queue rows emitted".to_string());
        return failures;
    }
    if rows.len() == 1 && rows[0].route == "__all_t2_bundle_repairs__" {
        let row = &rows[0];
        if row.bundle_status != "bundle-repair-clear"
            || row.contact_evidence_status != "no-closure-bundle-pending"
            || row.candidate_decision != "clear"
            || row.validation_status != "pass"
        {
            failures.push("bundle repair clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty() {
            failures.push("bundle repair queue row missing route".to_string());
        }
        if row.bundle_status.trim().is_empty() {
            failures.push(format!("{} missing bundle_status", row.route));
        }
        if row.bundle_status == "bundle-ready" {
            failures.push(format!(
                "{} is bundle-ready but remains in repair queue",
                row.route
            ));
        }
        if row.bundle_action.trim().is_empty() {
            failures.push(format!("{} missing bundle_action", row.route));
        }
        if row.contact_evidence_status != "closure-bundle-pending" {
            failures.push(format!(
                "{} repair queue row is not closure-bundle-pending",
                row.route
            ));
        }
        if row.candidate_decision != "blocked" {
            failures.push(format!(
                "{} bundle-pending candidate is not blocked",
                row.route
            ));
        }
        if row.repair_class.trim().is_empty() {
            failures.push(format!("{} missing repair_class", row.route));
        }
        if row.repair_action.trim().is_empty() {
            failures.push(format!("{} missing repair_action", row.route));
        }
        if row.required_artifact.trim().is_empty() {
            failures.push(format!("{} missing required_artifact", row.route));
        }
        if row.next_artifact.trim().is_empty() {
            failures.push(format!("{} missing next_artifact", row.route));
        }
        if row.optimizer_effect.trim().is_empty() {
            failures.push(format!("{} missing optimizer_effect", row.route));
        }
        if !row.qualification_effects.trim().is_empty() && row.repair_action.trim().is_empty() {
            failures.push(format!(
                "{} has qualification effects without repair action",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!("{} repair queue row must remain review", row.route));
        }
    }
    failures
}

