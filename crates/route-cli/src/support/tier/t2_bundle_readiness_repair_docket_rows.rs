//! Helper `t2_bundle_readiness_repair_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_repair_docket_rows(
    readiness_rows: &[T2BundleReadinessDispositionRow],
) -> Vec<T2BundleReadinessRepairDocketRow> {
    let mut rows = readiness_rows
        .iter()
        .filter(|row| row.disposition == "repair-needed")
        .map(|row| {
            let repair_action = match row.readiness_class.as_str() {
                "stop-chain" => "author-stop-chain-before-bundle-pass",
                "stitched-member" => "stitch-member-segments-before-bundle-pass",
                "terminal-stop" => "author-terminal-stop-before-bundle-pass",
                _ => "manual-bundle-readiness-repair",
            };
            T2BundleReadinessRepairDocketRow {
                repair_id: format!(
                    "T2BUNDLEREADINESSREPAIR-{}",
                    stable_id_fragment(&row.disposition_id)
                ),
                disposition_id: row.disposition_id.clone(),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                readiness_class: row.readiness_class.clone(),
                repair_decision: "repair-needed".to_string(),
                repair_action: repair_action.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                required_artifact: row.required_artifact.clone(),
                next_artifact: row.next_artifact.clone(),
                blocks_claims: row.blocks_claims.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.readiness_class
            .cmp(&right.readiness_class)
            .then(left.route.cmp(&right.route))
    });
    rows
}

