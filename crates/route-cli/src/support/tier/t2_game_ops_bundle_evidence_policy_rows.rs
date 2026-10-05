//! Helper `t2_game_ops_bundle_evidence_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_policy_rows(
    review_rows: &[T2GameOpsBundleEvidenceReviewRow],
) -> Vec<T2GameOpsBundleEvidencePolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.claim_blocker_delta == 0 && row.blocker_count_after > 0)
        .map(|row| T2GameOpsBundleEvidencePolicyRow {
            policy_id: format!(
                "T2GAMEOPSBUNDLEPOLICY-{}",
                stable_id_fragment(&row.review_id)
            ),
            review_id: row.review_id.clone(),
            decision_id: row.decision_id.clone(),
            target_id: row.target_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            repair_class: row.repair_class.clone(),
            service_repair_class: row.service_repair_class.clone(),
            evidence_artifact: row.evidence_artifact.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
            required_evidence: t2_game_ops_bundle_required_evidence(row).to_string(),
            evidence_policy_decision: "bundle-evidence-policy-authored-review".to_string(),
            policy_treatment: t2_game_ops_bundle_policy_treatment(row).to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-policy-acceptance.csv".to_string(),
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
