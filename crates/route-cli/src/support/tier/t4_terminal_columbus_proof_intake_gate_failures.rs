//! Helper `t4_terminal_columbus_proof_intake_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_proof_intake_gate_failures(
    rows: &[T4TerminalColumbusProofIntakeRow],
    proof_rows: &[T4TerminalContactProofDocketRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = proof_rows
        .iter()
        .filter(|row| {
            row.terminal_district == "Columbus South" && row.proof_status == "source-needed"
        })
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    if rows.is_empty() {
        failures.push("no Columbus South proof intake rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "Columbus proof intake has {} rows but expected {} source-needed rows",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.intake_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.required_proof_field.trim().is_empty()
            || row
                .selected_higher_tier_attachment_requirement
                .trim()
                .is_empty()
            || row.proof_status.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete intake fields", row.intake_id));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in Columbus proof intake",
                row.queue_id
            ));
        }
        if row.terminal_district != "Columbus South" {
            failures.push(format!(
                "{} is not a Columbus South proof task",
                row.queue_id
            ));
        }
        if row.zone_id != "t3-great-lakes" {
            failures.push(format!("{} is not a Great Lakes proof task", row.queue_id));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in the Columbus South source-needed proof docket",
                row.queue_id
            ));
        }
        if row.required_proof_field != "route-to-terminal contact statement" {
            failures.push(format!(
                "{} has invalid proof field {}",
                row.queue_id, row.required_proof_field
            ));
        }
        if !row
            .selected_higher_tier_attachment_requirement
            .contains("selected higher-tier attachment")
        {
            failures.push(format!(
                "{} lacks higher-tier attachment requirement",
                row.queue_id
            ));
        }
        if row.contact_proof_source_artifact != "source-needed"
            || row.proof_status != "source-needed"
            || row.validation_status != "review"
        {
            failures.push(format!(
                "{} intake row must remain source-needed/review until proof exists",
                row.queue_id
            ));
        }
    }

    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!(
                "{expected_id} is missing from Columbus proof intake"
            ));
        }
    }

    failures
}

