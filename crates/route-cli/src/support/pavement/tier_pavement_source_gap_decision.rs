//! Helper `tier_pavement_source_gap_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_source_gap_decision(
    blocker_statuses: &str,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if blocker_statuses.contains("pavement-repair-required") {
        return (
            "price pavement repair debt for failing member segments",
            "data/tier-pavement-docket.csv",
            "bundle remains service-addressable while pavement repair debt is priced and paid before SLA or transit readiness claims",
            "review",
        );
    }
    if blocker_statuses.contains("pavement-source-needed") {
        return (
            "price pavement evidence debt for affected member edges",
            "data/standards-l1-inventory.csv",
            "bundle remains service-addressable while pavement source debt is acquired and converted to pass or repair debt",
            "review",
        );
    }
    (
        "review pavement debt status",
        "data/tier-pavement-docket.csv",
        "bundle remains service-addressable while pavement debt is classified",
        "review",
    )
}
