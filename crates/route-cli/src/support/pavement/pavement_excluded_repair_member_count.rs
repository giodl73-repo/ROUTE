//! Helper `pavement_excluded_repair_member_count`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_excluded_repair_member_count(
    join_row: &TierPavementUnmatchedJoinReviewRow,
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
) -> usize {
    let repair_routes = semicolon_values(&join_row.repair_required_routes);
    exclusion_rows
        .iter()
        .filter(|row| {
            row.validation_status == "pass"
                && row.exclusion_status == "route-state-not-supported"
                && row.state == join_row.state
                && repair_routes
                    .iter()
                    .any(|route| route_display_key(route) == route_display_key(&row.route))
        })
        .map(|row| row.excluded_member_count)
        .sum()
}

