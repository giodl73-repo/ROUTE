//! Helper `t4_terminal_contact_proof_docket_gate_failures` (support::tier).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_proof_docket_gate_failures(
    rows: &[T4TerminalContactProofDocketRow],
    plan_rows: &[T4TerminalContactSourcePlanRow],
    catalog_rows: &[T4TerminalContactSourceCatalogRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    let expected_ids = plan_rows
        .iter()
        .map(|row| row.queue_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let catalog_districts = catalog_rows
        .iter()
        .map(|row| row.terminal_district.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    if rows.is_empty() {
        failures.push("no terminal contact proof docket rows emitted".to_string());
        return failures;
    }
    if rows.len() != expected_ids.len() {
        failures.push(format!(
            "proof docket has {} rows but expected {} source-plan rows",
            rows.len(),
            expected_ids.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.task_id.trim().is_empty()
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
            || row.scenario_effect.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete proof docket fields",
                row.task_id
            ));
        }
        if !seen.insert(row.queue_id.clone()) {
            failures.push(format!(
                "{} appears more than once in proof docket",
                row.queue_id
            ));
        }
        if !expected_ids.contains(row.queue_id.as_str()) {
            failures.push(format!(
                "{} does not appear in the source plan",
                row.queue_id
            ));
        }
        if !catalog_districts.contains(row.terminal_district.as_str()) {
            failures.push(format!(
                "{} lacks district source-family catalog row",
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
        if !matches!(
            row.proof_status.as_str(),
            "source-needed" | "source-backed" | "blocked"
        ) {
            failures.push(format!(
                "{} has invalid proof status {}",
                row.queue_id, row.proof_status
            ));
        }
        if row.proof_status == "source-needed" {
            if row.contact_proof_source_artifact != "source-needed"
                || row.validation_status != "review"
            {
                failures.push(format!(
                    "{} source-needed proof task must keep source-needed proof artifact and review status",
                    row.queue_id
                ));
            }
            if !row.scenario_effect.contains("no scenario-readiness") {
                failures.push(format!(
                    "{} source-needed proof task must block scenario-readiness",
                    row.queue_id
                ));
            }
        }
        if row.proof_status == "source-backed"
            && row.contact_proof_source_artifact == "source-needed"
        {
            failures.push(format!(
                "{} source-backed proof task lacks proof artifact",
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
            failures.push(format!("{expected_id} is missing from proof docket"));
        }
    }

    failures
}

