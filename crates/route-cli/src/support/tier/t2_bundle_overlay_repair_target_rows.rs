//! Helper `t2_bundle_overlay_repair_target_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_overlay_repair_target_rows(
    decision_rows: &[T2GameOpsBindingDecisionRow],
    overlay_rows: &[T2BundleOverlayRow],
) -> Vec<T2BundleOverlayRepairTargetRow> {
    let overlays_by_bundle = overlay_rows
        .iter()
        .filter(|row| !row.segment_bundle_id.trim().is_empty())
        .map(|row| (row.segment_bundle_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = decision_rows
        .iter()
        .filter(|row| row.decision != "bound")
        .map(|row| {
            let overlay = overlays_by_bundle.get(row.segment_bundle_id.as_str());
            let pavement_debt_cost_m = overlay
                .map(|overlay| overlay.pavement_debt_cost_m)
                .unwrap_or(0.0);
            let pavement_debt_class = overlay
                .map(|overlay| overlay.pavement_debt_class.clone())
                .unwrap_or_else(|| "none".to_string());
            let (repair_class, repair_action, required_artifact, next_artifact) =
                match row.bundle_status.as_str() {
                    "needs-stop-chain" => (
                        "stop-chain",
                        "author-or-demote-stop-chain-before-bundle-pass",
                        "data/national-segment-registry.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    "needs-stitched-members" => (
                        "stitched-member",
                        "stitch-member-segments-before-bundle-pass",
                        "data/tier-segment-candidates.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    "needs-terminal-stop" => (
                        "terminal-stop",
                        "author-terminal-stop-before-bundle-pass",
                        "data/t2-service-selection.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    _ if row.service_class == "unclassified" => (
                        "service-class",
                        "repair-service-class-before-game-overlay",
                        "data/beck-t2-diagnostics.csv",
                        "data/game/t2-service-overlays.csv",
                    ),
                    _ if pavement_debt_cost_m > 0.0 => (
                        "pavement-debt",
                        "preserve-pavement-debt-before-claim-promotion",
                        "data/tier-pavement-debt-budget.csv",
                        "data/tier-pavement-source-gaps.csv",
                    ),
                    _ => (
                        "manual-review",
                        "manual-overlay-review-before-claim-promotion",
                        "data/game/t2-bundle-overlays.csv",
                        "data/t2-bundle-overlay-repair-delta.csv",
                    ),
                };
            let target_status = if row.decision == "repair-needed"
                || matches!(
                    repair_class,
                    "stitched-member" | "terminal-stop" | "pavement-debt"
                )
                || (repair_class == "stop-chain" && row.service_class != "unclassified")
            {
                "repair-needed"
            } else {
                "held"
            };
            T2BundleOverlayRepairTargetRow {
                target_id: format!("T2OVERLAYREPAIR-{}", stable_id_fragment(&row.decision_id)),
                decision_id: row.decision_id.clone(),
                subject_id: row.subject_id.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                route: row.route.clone(),
                decision: row.decision.clone(),
                binding_status: row.binding_status.clone(),
                bundle_status: row.bundle_status.clone(),
                service_class: row.service_class.clone(),
                qualification_effects: row.qualification_effects.clone(),
                qualification_gate_policy: row.qualification_gate_policy.clone(),
                qualification_game_use: row.qualification_game_use.clone(),
                pavement_debt_cost_m,
                pavement_debt_class,
                blocks_claims: row.blocks_claims.clone(),
                repair_class: repair_class.to_string(),
                repair_action: repair_action.to_string(),
                required_artifact: required_artifact.to_string(),
                next_artifact: next_artifact.to_string(),
                target_status: target_status.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

