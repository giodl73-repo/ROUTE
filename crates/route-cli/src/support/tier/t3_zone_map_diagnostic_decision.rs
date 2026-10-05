//! Helper `t3_zone_map_diagnostic_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_map_diagnostic_decision(
    selected_route_count: usize,
    access_gap_count: usize,
    zone_assignment_gap_count: usize,
) -> (&'static str, &'static str, &'static str) {
    if selected_route_count == 0 {
        return (
            "blocked-no-selected-feeders",
            "select at least one T3 feeder before rendering zone map",
            "review",
        );
    }
    if zone_assignment_gap_count > 0 {
        return (
            "review-zone-assignment-gaps",
            "render selected feeders but keep unassigned local access hidden",
            "review",
        );
    }
    if access_gap_count > 0 {
        return (
            "review-terminal-and-feeder-gaps",
            "render selected feeders with held access-gap callouts",
            "review",
        );
    }
    (
        "ready-for-zone-render",
        "render selected T3 feeder columns on zone map",
        "pass",
    )
}
