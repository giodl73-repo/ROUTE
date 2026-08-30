//! Helper `t2_overlay_p1_structural_readiness_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_overlay_p1_structural_readiness_review_rows(
    action_rows: &[T2OverlayOptimizerActionDocketRow],
) -> Vec<T2OverlayP1StructuralReadinessReviewRow> {
    let mut rows = action_rows
        .iter()
        .filter(|row| row.priority_class == "P1-structural-readiness")
        .map(|action| {
            let readiness_decision = if action.route == "I295" {
                "held-stitched-proof-returned"
            } else {
                "held-readiness-repair-needed"
            };
            let downstream_action = if action.route == "I295" {
                "do-not-advance-until-stitched-member-proof-exists"
            } else {
                "route-to-bundle-readiness-repair-review"
            };
            let readiness_reason = if action.route == "I295" {
                "stitched-member proof review returned source-needed rows to optimizer held-known"
            } else {
                "bundle readiness remains repair-needed and requires structural repair review"
            };
            T2OverlayP1StructuralReadinessReviewRow {
                p1_review_id: format!("T2OVERLAYP1-{}", stable_id_fragment(&action.action_id)),
                action_id: action.action_id.clone(),
                route: action.route.clone(),
                segment_bundle_id: action.segment_bundle_id.clone(),
                optimizer_action: action.optimizer_action.clone(),
                priority_class: action.priority_class.clone(),
                readiness_decision: readiness_decision.to_string(),
                readiness_reason: readiness_reason.to_string(),
                downstream_action: downstream_action.to_string(),
                action_status: "optimizer-held-known".to_string(),
                qualification_effects: action.qualification_effects.clone(),
                blocked_claims_before: action.blocked_claims_after.clone(),
                blocked_claims_after: action.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: action.next_artifact.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}
