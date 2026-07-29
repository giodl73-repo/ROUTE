//! Helper `t2_game_ops_bundle_policy_treatment`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_ops_bundle_policy_treatment(row: &T2GameOpsBundleEvidenceReviewRow) -> &'static str {
    match row.repair_class.as_str() {
        "service-class" if row.service_repair_class == "local-zone" => {
            "hold game/ops claims until local-zone overlay handoff is accepted or explicitly carried"
        }
        "service-class" => {
            "hold game/ops claims until service-overlay classification is accepted"
        }
        "stitched-member" => {
            "hold game/ops claims until stitched-member proof is accepted or explicitly carried"
        }
        "stop-chain" => "hold game/ops claims until stop-chain repair is accepted or demoted",
        "terminal-stop" => "hold game/ops claims until terminal-stop repair is accepted",
        _ => "hold game/ops claims until bundle-binding repair evidence is accepted",
    }
}

