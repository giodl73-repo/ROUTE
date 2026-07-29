//! Helper `t4_terminal_access_proof_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_review_rows(
    artifact_rows: &[T4TerminalAccessProofArtifactRow],
) -> Vec<T4TerminalAccessProofReviewRow> {
    let mut rows = artifact_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| T4TerminalAccessProofReviewRow {
            proof_review_id: format!(
                "T4ACCESSREVIEWPROOF-{}",
                stable_id_fragment(&row.proof_artifact_id)
            ),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: row.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-optimizer-runs.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

