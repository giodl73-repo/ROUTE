//! Helper `t2_overlay_p2_service_overlay_review_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_overlay_p2_service_overlay_review_rows(
    action_rows: &[T2OverlayOptimizerActionDocketRow],
) -> Vec<T2OverlayP2ServiceOverlayReviewRow> {
    let mut rows = action_rows
        .iter()
        .filter(|row| row.priority_class == "P2-service-overlay")
        .map(|action| T2OverlayP2ServiceOverlayReviewRow {
            p2_review_id: format!("T2OVERLAYP2-{}", stable_id_fragment(&action.action_id)),
            action_id: action.action_id.clone(),
            route: action.route.clone(),
            segment_bundle_id: action.segment_bundle_id.clone(),
            optimizer_action: action.optimizer_action.clone(),
            priority_class: action.priority_class.clone(),
            service_overlay_decision: "held-service-overlay-diagnostic-needed".to_string(),
            service_overlay_reason:
                "service overlay remains diagnostic-only; no sourced evidence supports blocker reduction"
                    .to_string(),
            downstream_action: "route-to-service-overlay-diagnostic-review".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects: action.qualification_effects.clone(),
            blocked_claims_before: action.blocked_claims_after.clone(),
            blocked_claims_after: action.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: action.next_artifact.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

