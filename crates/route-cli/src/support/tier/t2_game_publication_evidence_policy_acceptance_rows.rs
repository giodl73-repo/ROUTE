//! Helper `t2_game_publication_evidence_policy_acceptance_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_evidence_policy_acceptance_rows(
    policy_rows: &[T2GamePublicationEvidencePolicyRow],
) -> Vec<T2GamePublicationEvidencePolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "publication-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidencePolicyAcceptanceRow {
            acceptance_id: format!("T2GAMEACCEPT-{}", stable_id_fragment(&row.scenario_id)),
            policy_id: row.policy_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            accepted_required_evidence: row.required_evidence.clone(),
            accepted_policy_treatment: row.policy_treatment.clone(),
            acceptance_decision: "publication-evidence-policy-accepted".to_string(),
            publication_treatment: row.publication_treatment.clone(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

