//! Helper `tier_region_next_artifact`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_region_next_artifact(repair_action: &str) -> &'static str {
    match repair_action {
        "keep-for-regionalizer" => "data/tier-candidate-columns.csv",
        "keep-with-parent-region-review" => "data/tier-candidate-columns.csv",
        "add-dual-contact-witness" => "data/tier-contact-witnesses.csv",
        "add-parent-contact-or-demote" => "data/tier-contact-witnesses.csv",
        "terminal-exception-or-demote" => "data/tier-node-exceptions.csv",
        "demote-to-t3-t4" => "data/tier-table.csv",
        "fix-graph-contact-or-demote" => "data/tier-contact-witnesses.csv",
        _ => "data/tier-region-repairs.csv",
    }
}
