//! Helper `pressure_scenario_has_bounded_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_scenario_has_bounded_contract(row: &PressureScenarioRow) -> bool {
    let has_identity = row.scenario_id.starts_with("S-L2-")
        && !row.scenario_name.trim().is_empty()
        && !row.adversity_class.trim().is_empty();
    let has_test_scope = !row.standards_tested.trim().is_empty()
        && row
            .standards_tested
            .split(';')
            .any(|value| value.trim().starts_with('T'));
    let has_artifact = !row.existing_artifact.trim().is_empty();
    let has_next_step = !row.next_evidence_step.trim().is_empty();
    let status = row.current_status.to_ascii_lowercase();
    let status_is_labeled = matches!(
        status.as_str(),
        "implemented" | "heuristic" | "planned" | "stub" | "deprecated"
    );

    has_identity && has_test_scope && has_artifact && has_next_step && status_is_labeled
}

