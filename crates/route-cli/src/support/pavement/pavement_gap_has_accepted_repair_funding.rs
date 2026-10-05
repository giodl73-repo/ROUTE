//! Helper `pavement_gap_has_accepted_repair_funding`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_gap_has_accepted_repair_funding(
    gap_row: &TierPavementSourceGapRow,
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> bool {
    if !gap_row
        .blocker_statuses
        .contains("pavement-repair-required")
    {
        return false;
    }
    let estimated_repair_cost_m =
        round_cost_m(gap_row.blocker_count as f64 * PAVEMENT_REPAIR_COST_PER_MEMBER_M);
    funding_rows.iter().any(|funding| {
        funding.validation_status == "pass"
            && funding.acceptance_status == "accepted-full-cost-repair-funding"
            && funding.tier == gap_row.tier
            && route_display_key(&funding.route) == route_display_key(&gap_row.route)
            && funding.segment_bundle_id == gap_row.segment_bundle_id
            && semicolon_values(&gap_row.affected_states)
                .iter()
                .any(|state| state == &funding.state)
            && funding.committed_amount_m + 1e-6 >= estimated_repair_cost_m
            && funding.covered_repair_cost_m + 1e-6 >= estimated_repair_cost_m
    })
}
