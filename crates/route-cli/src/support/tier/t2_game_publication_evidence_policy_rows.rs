//! Helper `t2_game_publication_evidence_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_policy_rows(
    review_rows: &[T2GamePublicationEvidenceReviewRow],
) -> Vec<T2GamePublicationEvidencePolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "publication-evidence-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidencePolicyRow {
            policy_id: format!("T2GAMEPOLICY-{}", stable_id_fragment(&row.scenario_id)),
            game_review_id: row.game_review_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            evidence_policy_basis: row.evidence_hold.clone(),
            required_evidence: row.required_evidence.clone(),
            evidence_policy_decision: "publication-evidence-policy-authored-review".to_string(),
            policy_treatment: t2_game_publication_policy_treatment(&row.required_evidence)
                .to_string(),
            publication_treatment:
                "hold game publication until accepted evidence policy is replayed".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

