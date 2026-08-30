//! Helper `t2_bundle_repair_queue_action`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_repair_queue_action(bundle_status: &str) -> (&'static str, &'static str) {
    match bundle_status {
        "bundle-missing" => (
            "add-or-split-segment-bundle-before-regionalizer",
            "data/national-segment-bundles.csv",
        ),
        "needs-stop-chain" => (
            "author-stop-chain-before-regionalizer",
            "data/tier-stop-candidates.csv",
        ),
        "needs-terminal-stop" => (
            "complete-terminal-stop-before-regionalizer",
            "data/tier-stop-candidates.csv",
        ),
        _ => (
            "resolve-bundle-readiness-before-regionalizer",
            "data/t2-blocker-closure.csv",
        ),
    }
}
