//! Helper `t4_terminal_contact_source_plan_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_source_plan_gate_failures(
    rows: &[T4TerminalContactSourcePlanRow],
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = contact_rows
        .iter()
        .filter(|row| row.decision == "source-needed")
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    if rows.is_empty() {
        failures.push("no terminal contact source plan rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "source plan has {} rows but expected {} source-needed rows",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.plan_id.trim().is_empty()
            || row.queue_id.trim().is_empty()
            || row.route.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.terminal_district_seed_source.trim().is_empty()
            || row.contact_proof_source_family.trim().is_empty()
            || row.required_proof_fields.trim().is_empty()
            || row.acquisition_status.trim().is_empty()
            || row.proof_blocker.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete source plan fields", row.plan_id));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in source plan",
                row.queue_id
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} is not a source-needed contact row",
                row.queue_id
            ));
        }
        if row.contact_proof_source_artifact == row.terminal_district_seed_source {
            failures.push(format!(
                "{} uses terminal district seed source as contact proof",
                row.queue_id
            ));
        }
        if !matches!(
            row.acquisition_status.as_str(),
            "source-needed" | "source-backed" | "blocked"
        ) {
            failures.push(format!(
                "{} has invalid acquisition status {}",
                row.queue_id, row.acquisition_status
            ));
        }
        if row.acquisition_status == "source-needed" {
            if row.contact_proof_source_artifact != "source-needed"
                || row.validation_status != "review"
            {
                failures.push(format!(
                    "{} source-needed row must keep source-needed proof artifact and review status",
                    row.queue_id
                ));
            }
            if !row.proof_blocker.contains("seed is not") {
                failures.push(format!(
                    "{} source-needed row lacks seed/proof blocker",
                    row.queue_id
                ));
            }
        }
        if row.acquisition_status == "source-backed"
            && row.contact_proof_source_artifact == "source-needed"
        {
            failures.push(format!(
                "{} source-backed row lacks contact proof artifact",
                row.queue_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review" | "held") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.queue_id, row.validation_status
            ));
        }
    }

    for expected_id in expected_ids {
        if !seen.contains(expected_id) {
            failures.push(format!("{expected_id} is missing from source plan"));
        }
    }

    failures
}

