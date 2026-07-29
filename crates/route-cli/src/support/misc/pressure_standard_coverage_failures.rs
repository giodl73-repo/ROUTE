//! Helper `pressure_standard_coverage_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_standard_coverage_failures<'a>(
    standards: &'a [StandardsProofRow],
    scenarios: &[PressureScenarioRow],
) -> Vec<&'a StandardsProofRow> {
    let scenario_refs = pressure_standard_scenario_refs(scenarios);
    pressure_standard_coverage_focus(standards)
        .into_iter()
        .filter(|row| !scenario_refs.contains_key(row.standard_id.as_str()))
        .collect()
}

