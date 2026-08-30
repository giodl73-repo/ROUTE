//! Helper `t4_terminal_columbus_proof_attempt_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_proof_attempt_rows(
    source_access_rows: &[T4TerminalColumbusSourceAccessRow],
) -> Vec<T4TerminalColumbusProofAttemptRow> {
    let mut rows = source_access_rows
        .iter()
        .map(|row| T4TerminalColumbusProofAttemptRow {
            attempt_id: format!("T4COLUMBUSATTEMPT-{}", stable_id_fragment(&row.queue_id)),
            access_id: row.access_id.clone(),
            intake_id: row.intake_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
            source_artifact: row.contact_proof_source_artifact.clone(),
            capture_status: "not-attempted-live-fetch-unsupported".to_string(),
            contact_statement_status: "source-needed".to_string(),
            selected_higher_tier_attachment_status: "source-needed".to_string(),
            proof_attempt_status: "blocked".to_string(),
            proof_decision: "source-needed".to_string(),
            proof_blocker: row.source_access_blocker.clone(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-04.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
