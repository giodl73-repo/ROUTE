//! Helper `t4_terminal_access_proof_source_capture_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_source_capture_rows(
    intake_rows: &[T4TerminalAccessProofIntakeRow],
) -> Vec<T4TerminalAccessProofSourceCaptureRow> {
    let mut rows = intake_rows
        .iter()
        .filter(|row| row.proof_artifact == "source-needed")
        .map(|row| T4TerminalAccessProofSourceCaptureRow {
            source_capture_id: format!(
                "T4ACCESSCAPTURE-{}",
                stable_id_fragment(&row.proof_intake_id)
            ),
            proof_intake_id: row.proof_intake_id.clone(),
            source_access_id: row.source_access_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-terminal-access-proof".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached non-seed terminal-access source artifact has not been attached"
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
