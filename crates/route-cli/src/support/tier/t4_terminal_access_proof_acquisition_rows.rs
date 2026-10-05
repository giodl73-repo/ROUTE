//! Helper `t4_terminal_access_proof_acquisition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_access_proof_acquisition_rows(
    review_rows: &[T4TerminalAccessEvidenceReviewRow],
) -> Vec<T4TerminalAccessProofAcquisitionRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-source-needed")
        .map(|row| T4TerminalAccessProofAcquisitionRow {
            acquisition_id: format!("T4ACCESSACQ-{}", stable_id_fragment(&row.review_id)),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            required_proof:
                "non-seed route-to-terminal contact source with route, terminal, connector, and date"
                    .to_string(),
            prohibited_seed_source: row.terminal_district_seed_source.clone(),
            acquisition_status: "source-needed".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            proof_artifact_status: "not-attached".to_string(),
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
