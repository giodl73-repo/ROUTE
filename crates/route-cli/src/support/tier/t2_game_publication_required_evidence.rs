//! Helper `t2_game_publication_required_evidence`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_required_evidence(evidence_hold: &str) -> &'static str {
    let hold = evidence_hold.to_ascii_lowercase();
    if hold.contains("port") || hold.contains("flood") {
        "port-surge-demand-and-flood-closure-evidence"
    } else if hold.contains("managed-lane") || hold.contains("spillback") {
        "managed-lane-merge-and-spillback-validation"
    } else {
        "standards-proof-and-scenario-promotion-record"
    }
}
