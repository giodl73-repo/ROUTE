//! Helper `t4_terminal_columbus_proof_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_proof_intake_rows(
    proof_rows: &[T4TerminalContactProofDocketRow],
) -> Vec<T4TerminalColumbusProofIntakeRow> {
    let mut rows = proof_rows
        .iter()
        .filter(|row| {
            row.terminal_district == "Columbus South" && row.proof_status == "source-needed"
        })
        .map(|row| T4TerminalColumbusProofIntakeRow {
            intake_id: format!("T4COLUMBUS-{}", stable_id_fragment(&row.queue_id)),
            task_id: row.task_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
            required_proof_field: row.required_proof_field.clone(),
            selected_higher_tier_attachment_requirement: row
                .selected_higher_tier_attachment_requirement
                .clone(),
            contact_proof_source_artifact: row.contact_proof_source_artifact.clone(),
            proof_status: row.proof_status.clone(),
            proof_blocker: row.proof_blocker.clone(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.route
            .cmp(&b.route)
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}

