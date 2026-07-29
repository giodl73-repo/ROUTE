//! Helper `t4_terminal_access_proof_artifact_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_artifact_rows(
    acquisition_rows: &[T4TerminalAccessProofAcquisitionRow],
) -> Vec<T4TerminalAccessProofArtifactRow> {
    let mut rows = acquisition_rows
        .iter()
        .filter(|row| row.proof_artifact_status == "not-attached")
        .map(|row| T4TerminalAccessProofArtifactRow {
            proof_artifact_id: format!(
                "T4ACCESSARTIFACT-{}",
                stable_id_fragment(&row.acquisition_id)
            ),
            acquisition_id: row.acquisition_id.clone(),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            required_proof: row.required_proof.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-review.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

