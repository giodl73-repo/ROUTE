//! Helper `pavement_funded_repair_member_count`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_funded_repair_member_count(
    join_row: &TierPavementUnmatchedJoinReviewRow,
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> usize {
    let repair_routes = semicolon_values(&join_row.repair_required_routes);
    funding_rows
        .iter()
        .filter(|row| {
            row.validation_status == "pass"
                && row.acceptance_status == "accepted-full-cost-repair-funding"
                && row.state == join_row.state
                && repair_routes
                    .iter()
                    .any(|route| route_display_key(route) == route_display_key(&row.route))
                && row.committed_amount_m + 1e-6 >= row.covered_repair_cost_m
                && row.covered_repair_cost_m > 0.0
        })
        .map(|row| (row.covered_repair_cost_m / PAVEMENT_REPAIR_COST_PER_MEMBER_M).round() as usize)
        .sum()
}

