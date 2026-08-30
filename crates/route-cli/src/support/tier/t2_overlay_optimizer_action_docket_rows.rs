//! Helper `t2_overlay_optimizer_action_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_overlay_optimizer_action_docket_rows(
    delta_rows: &[T2BundleOverlayRepairDeltaRow],
) -> Vec<T2OverlayOptimizerActionDocketRow> {
    let mut rows = delta_rows
        .iter()
        .map(|delta| {
            let optimizer_action =
                if delta.service_action == "hold-local-relief-below-national-game-overlay" {
                    "local-zone-overlay-review"
                } else if delta.readiness_disposition == "repair-needed" {
                    "bundle-readiness-repair-review"
                } else if delta.service_action == "repair-service-overlay-before-game-ops-binding" {
                    "service-overlay-diagnostic-review"
                } else if delta.previous_decision == "repair-needed" {
                    "bundle-repair-replay-review"
                } else {
                    "optimizer-held-replay-review"
                };
            let priority_class = match optimizer_action {
                "bundle-readiness-repair-review" => "P1-structural-readiness",
                "service-overlay-diagnostic-review" => "P2-service-overlay",
                "local-zone-overlay-review" => "P3-local-zone-overlay",
                _ => "P4-held-replay",
            };
            T2OverlayOptimizerActionDocketRow {
                action_id: format!("T2OVERLAYACTION-{}", stable_id_fragment(&delta.delta_id)),
                delta_id: delta.delta_id.clone(),
                route: delta.route.clone(),
                segment_bundle_id: delta.segment_bundle_id.clone(),
                replay_decision: delta.replay_decision.clone(),
                service_action: delta.service_action.clone(),
                readiness_disposition: delta.readiness_disposition.clone(),
                optimizer_action: optimizer_action.to_string(),
                priority_class: priority_class.to_string(),
                action_status: "optimizer-held-known".to_string(),
                qualification_effects: delta.qualification_effects.clone(),
                blocked_claims_before: delta.blocked_claims_after.clone(),
                blocked_claims_after: delta.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact: delta.next_artifact.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.priority_class
            .cmp(&right.priority_class)
            .then(left.route.cmp(&right.route))
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}
