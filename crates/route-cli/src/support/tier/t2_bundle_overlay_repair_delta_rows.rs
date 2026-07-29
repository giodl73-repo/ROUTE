//! Helper `t2_bundle_overlay_repair_delta_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_repair_delta_rows(
    decision_rows: &[T2GameOpsBindingDecisionRow],
    target_rows: &[T2BundleOverlayRepairTargetRow],
    service_rows: &[T2ServiceClassRepairDocketRow],
    readiness_rows: &[T2BundleReadinessDispositionRow],
) -> Vec<T2BundleOverlayRepairDeltaRow> {
    let targets = target_rows
        .iter()
        .map(|row| (row.decision_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let service_by_target = service_rows
        .iter()
        .map(|row| (row.target_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let readiness_by_target = readiness_rows
        .iter()
        .map(|row| (row.target_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = decision_rows
        .iter()
        .filter(|row| row.decision != "bound")
        .filter_map(|decision| {
            let target = targets.get(decision.decision_id.as_str())?;
            let service_action = service_by_target
                .get(target.target_id.as_str())
                .map(|row| row.service_action.clone())
                .unwrap_or_else(|| "no-service-class-action-required".to_string());
            let readiness = readiness_by_target
                .get(target.target_id.as_str())
                .map(|row| row.disposition.clone())
                .unwrap_or_else(|| "no-readiness-disposition-required".to_string());
            let next_artifact = readiness_by_target
                .get(target.target_id.as_str())
                .map(|row| row.next_artifact.clone())
                .or_else(|| {
                    service_by_target
                        .get(target.target_id.as_str())
                        .map(|row| row.next_artifact.clone())
                })
                .unwrap_or_else(|| target.next_artifact.clone());
            Some(T2BundleOverlayRepairDeltaRow {
                delta_id: format!(
                    "T2OVERLAYDELTA-{}",
                    stable_id_fragment(&decision.decision_id)
                ),
                decision_id: decision.decision_id.clone(),
                route: decision.route.clone(),
                segment_bundle_id: decision.segment_bundle_id.clone(),
                previous_decision: decision.decision.clone(),
                target_status: target.target_status.clone(),
                service_action,
                readiness_disposition: readiness,
                replay_decision: decision.decision.clone(),
                qualification_effects: merge_qualification_effects(
                    &decision.qualification_effects,
                    &target.qualification_effects,
                ),
                blocked_claims_before: decision.blocks_claims.clone(),
                blocked_claims_after: decision.blocks_claims.clone(),
                blocker_delta: 0,
                next_artifact,
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

