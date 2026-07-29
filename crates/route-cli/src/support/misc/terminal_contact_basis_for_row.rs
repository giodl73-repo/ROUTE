//! Helper `terminal_contact_basis_for_row`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_contact_basis_for_row(row: &T4TerminalAccessColumnRow) -> String {
    if terminal_seed_for_zone_route(&row.zone_id, &row.route).is_some() {
        "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
            .to_string()
    } else {
        "source-needed-route-to-terminal-contact".to_string()
    }
}

