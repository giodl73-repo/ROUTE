//! Helper `t4_terminal_scenario_readiness_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_scenario_readiness_rows(
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<T4TerminalScenarioReadinessRow> {
    let mut rows = contact_rows
        .iter()
        .filter(|row| matches!(row.decision.as_str(), "source-backed" | "scenario-ready"))
        .map(|row| T4TerminalScenarioReadinessRow {
            docket_id: format!("T4SCENARIO-{}", stable_id_fragment(&row.queue_id)),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district: row.terminal_district_seed.clone(),
            contact_basis: row.contact_basis.clone(),
            contact_proof_source: row.contact_proof_source.clone(),
            selected_higher_tier_attachment: row.selected_higher_tier_attachment.clone(),
            freight_access_rationale: terminal_scenario_rationale(row),
            scenario_decision: if row.decision == "scenario-ready" {
                "scenario-candidate".to_string()
            } else {
                "source-backed-review".to_string()
            },
            scenario_artifact: if row.decision == "scenario-ready" {
                "data/t4-terminal-scenario-readiness.csv".to_string()
            } else {
                String::new()
            },
            source_evidence_status: row.evidence_status.clone(),
            release_status: "held-source-review".to_string(),
            next_artifact: "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-05.md"
                .to_string(),
            validation_status: if row.decision == "scenario-ready" {
                "review".to_string()
            } else {
                "held".to_string()
            },
        })
        .collect::<Vec<_>>();

    if rows.is_empty() {
        rows.push(T4TerminalScenarioReadinessRow {
            docket_id: "__all_t4_terminal_scenarios__".to_string(),
            route: String::new(),
            zone_id: "all-terminal-contact-zones".to_string(),
            terminal_district: String::new(),
            contact_basis: "no-source-backed-terminal-contact-rows".to_string(),
            contact_proof_source: String::new(),
            selected_higher_tier_attachment: String::new(),
            freight_access_rationale:
                "scenario docket remains empty until a contact row is source-backed".to_string(),
            scenario_decision: "no-source-backed-contacts".to_string(),
            scenario_artifact: String::new(),
            source_evidence_status: "source-needed".to_string(),
            release_status: "held-source-needed".to_string(),
            next_artifact: "data/t4-terminal-contact-evidence.csv".to_string(),
            validation_status: "held".to_string(),
        });
    }

    rows.sort_by(|a, b| {
        a.scenario_decision
            .cmp(&b.scenario_decision)
            .then_with(|| a.zone_id.cmp(&b.zone_id))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}
