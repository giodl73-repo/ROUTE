//! Helper `pressure_scenario_unknown_standard_refs`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_scenario_unknown_standard_refs(
    standards: &[StandardsProofRow],
    scenarios: &[PressureScenarioRow],
) -> Vec<String> {
    let known = standards
        .iter()
        .map(|row| row.standard_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut unknown = pressure_standard_scenario_refs(scenarios)
        .keys()
        .filter(|standard_id| !known.contains(**standard_id))
        .map(|standard_id| (*standard_id).to_string())
        .collect::<Vec<_>>();
    unknown.sort();
    unknown
}
