//! Helper `optimizer_map_hook_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_map_hook_gate_failures(rows: &[OptimizerMapHookRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no optimizer map hook rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.hook_id.trim().is_empty()
            || row.optimizer_artifact.trim().is_empty()
            || row.consumer_artifact.trim().is_empty()
            || row.consumer_type.trim().is_empty()
            || row.gate_command.trim().is_empty()
            || row.link_basis.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete optimizer hook fields",
                row.hook_id
            ));
        }
        if !row.gate_command.starts_with("route ")
            || !row
                .gate_command
                .split_whitespace()
                .any(|part| part == "--gate")
        {
            failures.push(format!(
                "{} has non-gate consumer command {}",
                row.hook_id, row.gate_command
            ));
        }
        if !artifact_has_content(&row.optimizer_artifact) {
            failures.push(format!(
                "{} optimizer artifact missing or empty",
                row.hook_id
            ));
        }
        if !artifact_has_content(&row.consumer_artifact) {
            failures.push(format!(
                "{} consumer artifact missing or empty",
                row.hook_id
            ));
        }
        if row.validation_status != "pass" {
            failures.push(format!(
                "{} has non-pass validation status {}",
                row.hook_id, row.validation_status
            ));
        }
    }
    failures
}
