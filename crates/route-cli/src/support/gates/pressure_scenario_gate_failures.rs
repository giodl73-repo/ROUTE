//! Helper `pressure_scenario_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_scenario_gate_failures(rows: &[PressureScenarioRow]) -> Vec<&PressureScenarioRow> {
    rows.iter()
        .filter(|row| !pressure_scenario_has_bounded_contract(row))
        .collect()
}

