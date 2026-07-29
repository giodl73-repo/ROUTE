//! Helper `t2_game_ops_bundle_evidence_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_evidence_review_rows(
    decision_rows: &[T2GameOpsBindingDecisionRow],
    target_rows: &[T2BundleOverlayRepairTargetRow],
    service_docket_rows: &[T2ServiceClassRepairDocketRow],
) -> Vec<T2GameOpsBundleEvidenceReviewRow> {
    let targets_by_decision = target_rows
        .iter()
        .map(|row| (row.decision_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let service_by_target = service_docket_rows
        .iter()
        .map(|row| (row.target_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = decision_rows
        .iter()
        .filter_map(|decision| {
            let target = targets_by_decision.get(decision.decision_id.as_str())?;
            let service = service_by_target.get(target.target_id.as_str());
            let evidence_artifact = service
                .map(|row| row.required_artifact.as_str())
                .unwrap_or(target.required_artifact.as_str());
            let service_repair_class = service
                .map(|row| row.service_repair_class.as_str())
                .unwrap_or("not-service-class");
            let blocker_count = semicolon_values(&decision.blocks_claims).len();
            Some(T2GameOpsBundleEvidenceReviewRow {
                review_id: format!(
                    "T2GAMEOPSBUNDLEEVIDENCEREVIEW-{}",
                    stable_id_fragment(&decision.decision_id)
                ),
                decision_id: decision.decision_id.clone(),
                target_id: target.target_id.clone(),
                route: decision.route.clone(),
                segment_bundle_id: decision.segment_bundle_id.clone(),
                decision: decision.decision.clone(),
                binding_status: decision.binding_status.clone(),
                bundle_status: decision.bundle_status.clone(),
                service_class: decision.service_class.clone(),
                repair_class: target.repair_class.clone(),
                repair_action: target.repair_action.clone(),
                qualification_effects: service
                    .map(|row| row.qualification_effects.clone())
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| target.qualification_effects.clone()),
                qualification_gate_policy: target.qualification_gate_policy.clone(),
                qualification_game_use: target.qualification_game_use.clone(),
                evidence_artifact: evidence_artifact.to_string(),
                service_repair_class: service_repair_class.to_string(),
                evidence_status: "downstream-evidence-bound-blocker-preserved".to_string(),
                blocker_claims_before: decision.blocks_claims.clone(),
                blocker_claims_after: decision.blocks_claims.clone(),
                blocker_count_before: blocker_count,
                blocker_count_after: blocker_count,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-game-ops-bundle-evidence-policy.csv".to_string(),
                validation_status: "review".to_string(),
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

