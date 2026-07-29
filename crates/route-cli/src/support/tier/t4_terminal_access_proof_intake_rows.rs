//! Helper `t4_terminal_access_proof_intake_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_intake_rows(
    access_rows: &[T4TerminalAccessSourceAccessRow],
) -> Vec<T4TerminalAccessProofIntakeRow> {
    let mut rows = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|row| T4TerminalAccessProofIntakeRow {
            proof_intake_id: format!(
                "T4ACCESSINTAKE-{}",
                stable_id_fragment(&row.source_access_id)
            ),
            source_access_id: row.source_access_id.clone(),
            proof_review_id: row.proof_review_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; terminal; connector"
                    .to_string(),
            required_contact_statement:
                "non-seed source statement that the route provides route-to-terminal contact"
                    .to_string(),
            proof_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been captured or reviewed"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

