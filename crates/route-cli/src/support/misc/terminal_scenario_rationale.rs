//! Helper `terminal_scenario_rationale`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_scenario_rationale(row: &T4TerminalContactEvidenceRow) -> String {
    format!(
        "source-backed contact between {} and {}; preserves T4 access while selecting scenario scope",
        row.route, row.terminal_district_seed
    )
}
