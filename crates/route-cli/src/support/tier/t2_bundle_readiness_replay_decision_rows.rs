//! Helper `t2_bundle_readiness_replay_decision_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_replay_decision_rows(
    evidence_rows: &[T2BundleReadinessRepairEvidenceRow],
    delta_rows: &[T2BundleOverlayRepairDeltaRow],
) -> Vec<T2BundleReadinessReplayDecisionRow> {
    let deltas_by_bundle = delta_rows
        .iter()
        .map(|row| (row.segment_bundle_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = evidence_rows
        .iter()
        .filter_map(|evidence| {
            let delta = deltas_by_bundle.get(evidence.segment_bundle_id.as_str())?;
            let replay_action = match delta.replay_decision.as_str() {
                "repair-needed" => "rerun-after-structural-artifact-update",
                "held" => "keep-held-until-repair-delta-mutates",
                other => {
                    if other.trim().is_empty() {
                        "keep-held-until-repair-delta-mutates"
                    } else {
                        "manual-readiness-replay-review"
                    }
                }
            };
            Some(T2BundleReadinessReplayDecisionRow {
                replay_id: format!(
                    "T2BUNDLEREADINESSREPLAY-{}",
                    stable_id_fragment(&evidence.evidence_id)
                ),
                evidence_id: evidence.evidence_id.clone(),
                delta_id: delta.delta_id.clone(),
                route: evidence.route.clone(),
                segment_bundle_id: evidence.segment_bundle_id.clone(),
                readiness_class: evidence.readiness_class.clone(),
                evidence_status: evidence.evidence_status.clone(),
                delta_replay_decision: delta.replay_decision.clone(),
                replay_decision: "held-for-bundle-replay".to_string(),
                replay_action: replay_action.to_string(),
                qualification_effects: merge_qualification_effects(
                    &evidence.qualification_effects,
                    &delta.qualification_effects,
                ),
                blocked_claims_before: evidence.blocks_claims.clone(),
                blocked_claims_after: delta.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: delta.next_artifact.clone(),
                validation_status: "review".to_string(),
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.readiness_class
            .cmp(&right.readiness_class)
            .then(left.route.cmp(&right.route))
    });
    rows
}
