//! Helper `t4_terminal_contact_source_plan_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_source_plan_rows(
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<T4TerminalContactSourcePlanRow> {
    let mut rows = contact_rows
        .iter()
        .filter(|row| row.decision == "source-needed")
        .map(|row| T4TerminalContactSourcePlanRow {
            plan_id: format!("T4SOURCEPLAN-{}", stable_id_fragment(&row.queue_id)),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district: row.terminal_district_seed.clone(),
            terminal_district_seed_source: row.terminal_district_seed_source.clone(),
            contact_proof_source_family: "public-terminal-contact-proof".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.terminal_district
            .cmp(&b.terminal_district)
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}

