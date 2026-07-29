//! Helper `pressure_standard_scenario_refs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_standard_scenario_refs(
    scenarios: &[PressureScenarioRow],
) -> std::collections::BTreeMap<&str, Vec<String>> {
    let mut refs = std::collections::BTreeMap::new();
    for row in scenarios {
        for standard_id in row
            .standards_tested
            .split(';')
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            refs.entry(standard_id)
                .or_insert_with(Vec::new)
                .push(row.scenario_id.clone());
        }
    }
    refs
}

