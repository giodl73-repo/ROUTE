//! Helper `t2_game_ops_bundle_required_evidence`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_required_evidence(
    row: &T2GameOpsBundleEvidenceReviewRow,
) -> &'static str {
    match row.repair_class.as_str() {
        "service-class" if row.service_repair_class == "local-zone" => {
            "accepted-local-zone-overlay-handoff"
        }
        "service-class" => "accepted-service-overlay-classification",
        "stitched-member" => "accepted-stitched-member-proof-review",
        "stop-chain" => "accepted-stop-chain-repair-or-demotion",
        "terminal-stop" => "accepted-terminal-stop-repair",
        _ => "accepted-bundle-binding-repair-evidence",
    }
}
