//! Helper `t4_terminal_scenario_readiness_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_scenario_readiness_gate_failures(
    rows: &[T4TerminalScenarioReadinessRow],
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T4 terminal scenario readiness rows emitted".to_string());
        return failures;
    }

    let source_backed_count = contact_rows
        .iter()
        .filter(|row| matches!(row.decision.as_str(), "source-backed" | "scenario-ready"))
        .count();
    if source_backed_count == 0
        && !rows.iter().any(|row| {
            row.docket_id == "__all_t4_terminal_scenarios__"
                && row.scenario_decision == "no-source-backed-contacts"
                && row.release_status == "held-source-needed"
        })
    {
        failures.push("empty scenario docket lacks held clear row".to_string());
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.docket_id.trim().is_empty()
            || row.zone_id.trim().is_empty()
            || row.contact_basis.trim().is_empty()
            || row.freight_access_rationale.trim().is_empty()
            || row.scenario_decision.trim().is_empty()
            || row.source_evidence_status.trim().is_empty()
            || row.release_status.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete scenario docket fields",
                row.docket_id
            ));
        }
        if !seen.insert(row.docket_id.clone()) {
            failures.push(format!("{} is duplicated", row.docket_id));
        }
        if matches!(
            row.scenario_decision.as_str(),
            "scenario-candidate" | "source-backed-review"
        ) && (row.route.trim().is_empty()
            || row.terminal_district.trim().is_empty()
            || row.contact_proof_source.trim().is_empty()
            || row.selected_higher_tier_attachment.trim().is_empty()
            || row.source_evidence_status != "accepted")
        {
            failures.push(format!(
                "{} scenario row lacks proof, terminal, attachment, or accepted evidence",
                row.docket_id
            ));
        }
        if row.scenario_decision == "scenario-candidate" && row.scenario_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} scenario candidate lacks artifact",
                row.docket_id
            ));
        }
        if row.release_status != "held-source-review" && row.release_status != "held-source-needed"
        {
            failures.push(format!(
                "{} has invalid release status {}",
                row.docket_id, row.release_status
            ));
        }
        if !matches!(row.validation_status.as_str(), "review" | "held") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.docket_id, row.validation_status
            ));
        }
    }
    failures
}

