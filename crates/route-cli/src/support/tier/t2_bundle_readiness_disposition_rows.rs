//! Helper `t2_bundle_readiness_disposition_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_disposition_rows(
    target_rows: &[T2BundleOverlayRepairTargetRow],
) -> Vec<T2BundleReadinessDispositionRow> {
    let mut rows = target_rows
        .iter()
        .filter(|row| {
            matches!(
                row.bundle_status.as_str(),
                "needs-stop-chain" | "needs-stitched-members" | "needs-terminal-stop"
            ) || row.binding_status == "bundle-bound-review"
        })
        .map(|row| {
            let (readiness_class, disposition, action, required_artifact, next_artifact) =
                match row.bundle_status.as_str() {
                    "needs-stop-chain" if row.service_class == "unclassified" => (
                        "stop-chain",
                        "held",
                        "repair-service-class-before-stop-chain-pass",
                        "data/game/t2-service-overlays.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    "needs-stop-chain" => (
                        "stop-chain",
                        "repair-needed",
                        "author-stop-chain-before-bundle-pass",
                        "data/national-segment-registry.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    "needs-stitched-members" => (
                        "stitched-member",
                        "repair-needed",
                        "stitch-member-segments-before-bundle-pass",
                        "data/tier-segment-candidates.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    "needs-terminal-stop" => (
                        "terminal-stop",
                        "repair-needed",
                        "author-terminal-stop-before-bundle-pass",
                        "data/t2-service-selection.csv",
                        "data/national-segment-bundles.csv",
                    ),
                    _ => (
                        "manual-review",
                        "held",
                        "manual-bundle-readiness-review",
                        "data/game/t2-bundle-overlays.csv",
                        "data/t2-bundle-overlay-repair-delta.csv",
                    ),
                };
            T2BundleReadinessDispositionRow {
                disposition_id: format!("T2BUNDLEREADINESS-{}", stable_id_fragment(&row.target_id)),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                bundle_status: row.bundle_status.clone(),
                service_class: row.service_class.clone(),
                readiness_class: readiness_class.to_string(),
                disposition: disposition.to_string(),
                disposition_action: action.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                required_artifact: required_artifact.to_string(),
                next_artifact: next_artifact.to_string(),
                blocks_claims: row.blocks_claims.clone(),
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

