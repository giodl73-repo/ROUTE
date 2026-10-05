//! Helper `tier_pavement_acquisition_plan_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_acquisition_plan_gate_failures(
    rows: &[TierPavementAcquisitionPlanRow],
    gap_rows: &[TierPavementSourceGapRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !gap_rows.is_empty() && rows.is_empty() {
        failures.push("pavement source gaps exist but acquisition plan is empty".to_string());
        return failures;
    }
    let source_states = gap_rows
        .iter()
        .flat_map(|row| row.affected_states.split(';'))
        .map(str::trim)
        .filter(|state| !state.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let plan_states = rows
        .iter()
        .map(|row| row.state.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for state in source_states {
        if !plan_states.contains(state) {
            failures.push(format!("{state} missing from pavement acquisition plan"));
        }
    }
    for row in rows {
        if row.state.trim().is_empty()
            || row.tier.trim().is_empty()
            || row.source_family.trim().is_empty()
            || row.affected_routes.trim().is_empty()
            || row.affected_bundles.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.acquisition_action.trim().is_empty()
            || row.required_fields.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pavement acquisition row",
                row.state
            ));
        }
        if row.route_count == 0 || row.bundle_count == 0 || row.blocked_member_count == 0 {
            failures.push(format!("{} has zero acquisition coverage", row.state));
        }
        if !matches!(row.source_priority.as_str(), "A" | "B" | "C") {
            failures.push(format!(
                "{} has invalid source priority {}",
                row.state, row.source_priority
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.state, row.validation_status
            ));
        }
    }
    failures
}
