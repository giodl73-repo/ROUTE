//! Helper `t2_game_ops_bundle_evidence_policy_acceptance_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_policy_acceptance_rows(
    policy_rows: &[T2GameOpsBundleEvidencePolicyRow],
) -> Vec<T2GameOpsBundleEvidencePolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "bundle-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GameOpsBundleEvidencePolicyAcceptanceRow {
            acceptance_id: format!(
                "T2GAMEOPSBUNDLEACCEPT-{}",
                stable_id_fragment(&row.policy_id)
            ),
            policy_id: row.policy_id.clone(),
            review_id: row.review_id.clone(),
            decision_id: row.decision_id.clone(),
            target_id: row.target_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            accepted_required_evidence: row.required_evidence.clone(),
            accepted_policy_treatment: row.policy_treatment.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
            acceptance_decision: "bundle-evidence-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

