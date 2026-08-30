//! Helper `optimizer_manifest_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_manifest_gate_failures(rows: &[TierOptimizerRunRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no tier optimizer run rows emitted".to_string());
        return failures;
    }
    let mut previous_step = 0usize;
    for row in rows {
        if row.step == 0 || row.step <= previous_step {
            failures.push(format!(
                "{} has non-increasing manifest step {}",
                row.optimizer_stage, row.step
            ));
        }
        previous_step = row.step;
        if row.optimizer_stage.trim().is_empty()
            || row.command.trim().is_empty()
            || row.artifact.trim().is_empty()
            || row.gate_status.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("step {} has empty manifest fields", row.step));
        }
        if !row.command.starts_with("route ")
            || !row.command.split_whitespace().any(|part| part == "--gate")
        {
            failures.push(format!(
                "{} has non-gate optimizer command {}",
                row.optimizer_stage, row.command
            ));
        }
        if row.row_count == 0 {
            failures.push(format!("{} has missing or empty artifact", row.artifact));
        }
        match csv_record_count(&repo_relative_artifact_path(&row.artifact)) {
            Ok(actual_count) if row.row_count != actual_count => {
                failures.push(format!(
                    "{} row_count {} does not match current artifact count {}",
                    row.artifact, row.row_count, actual_count
                ));
            }
            Err(error) => {
                failures.push(format!(
                    "{} row count could not be verified: {error}",
                    row.artifact
                ));
            }
            _ => {}
        }
        if !matches!(row.gate_status.as_str(), "pass" | "held-known" | "review") {
            failures.push(format!(
                "{} has unexpected gate status {}",
                row.optimizer_stage, row.gate_status
            ));
        }
        if row.gate_status == "pass" && row.validation_status != "pass" {
            failures.push(format!("{} did not validate as pass", row.optimizer_stage));
        }
        if row.gate_status == "held-known" && row.validation_status != "held" {
            failures.push(format!(
                "{} held row did not validate as held",
                row.optimizer_stage
            ));
        }
        if row.gate_status == "held-known" && row.blocker_count == 0 {
            failures.push(format!(
                "{} held without blocker count",
                row.optimizer_stage
            ));
        }
        if matches!(row.gate_status.as_str(), "held-known" | "review")
            && row.blocker_summary.trim().is_empty()
        {
            failures.push(format!(
                "{} held without blocker summary",
                row.optimizer_stage
            ));
        }
        if row.gate_status == "fail" || row.validation_status == "missing-or-empty" {
            failures.push(format!(
                "{} has non-committable manifest status {}/{}",
                row.optimizer_stage, row.gate_status, row.validation_status
            ));
        }
    }
    failures
}
