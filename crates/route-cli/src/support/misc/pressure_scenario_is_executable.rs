//! Helper `pressure_scenario_is_executable`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_scenario_is_executable(row: &PressureScenarioRow) -> bool {
    matches!(
        row.current_status.trim().to_ascii_lowercase().as_str(),
        "implemented" | "heuristic"
    )
}

