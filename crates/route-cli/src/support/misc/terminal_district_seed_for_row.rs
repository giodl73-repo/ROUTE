//! Helper `terminal_district_seed_for_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_district_seed_for_row(row: &T4TerminalAccessColumnRow) -> String {
    terminal_seed_for_zone_route(&row.zone_id, &row.route)
        .unwrap_or_else(|| terminal_district_seed(&row.terminal_obligation))
}
