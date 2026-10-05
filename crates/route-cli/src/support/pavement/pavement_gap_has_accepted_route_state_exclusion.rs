//! Helper `pavement_gap_has_accepted_route_state_exclusion`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pavement_gap_has_accepted_route_state_exclusion(
    gap_row: &TierPavementSourceGapRow,
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
) -> bool {
    exclusion_rows.iter().any(|exclusion| {
        exclusion.validation_status == "pass"
            && exclusion.exclusion_status == "route-state-not-supported"
            && exclusion.tier == gap_row.tier
            && route_display_key(&exclusion.route) == route_display_key(&gap_row.route)
            && exclusion.segment_bundle_id == gap_row.segment_bundle_id
            && semicolon_values(&gap_row.affected_states)
                .iter()
                .any(|state| state == &exclusion.state)
    })
}
