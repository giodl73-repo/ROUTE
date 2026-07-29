//! Helper `t4_terminal_contact_evidence_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_evidence_rows(
    terminal_rows: &[T4TerminalAccessColumnRow],
) -> Vec<T4TerminalContactEvidenceRow> {
    let mut rows = terminal_rows
        .iter()
        .filter(|row| row.column_decision == "terminal-review")
        .map(|row| T4TerminalContactEvidenceRow {
            queue_id: format!(
                "T4CONTACT-{}-{}",
                canonical_route_key(&row.zone_id),
                canonical_route_key(&row.route)
            ),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: terminal_district_seed_for_row(row),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: terminal_contact_basis_for_row(row),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact: terminal_contact_next_artifact(&row.zone_id),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: row.column_decision.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| a.decision.cmp(&b.decision))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}

