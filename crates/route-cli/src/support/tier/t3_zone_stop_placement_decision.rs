//! Helper `t3_zone_stop_placement_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_stop_placement_decision(
    stop_count: usize,
    transfer_grade_stop_count: usize,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if stop_count >= 2 && transfer_grade_stop_count >= 1 {
        (
            "ready-for-stop-layout",
            "place route on zone schematic using ordered stop chain",
            "maps/t3-zone",
            "pass",
        )
    } else if stop_count == 1 {
        (
            "needs-terminal-stop",
            "author one additional transfer or regional terminal stop before geometry",
            "data/tier-stop-candidates.csv",
            "review",
        )
    } else {
        (
            "needs-stop-chain",
            "author at least two visible T3 zone stops before geometry",
            "data/tier-stop-candidates.csv",
            "review",
        )
    }
}

