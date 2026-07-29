//! Helper `tier_pavement_acquisition_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_acquisition_docket_gate_failures(
    rows: &[TierPavementAcquisitionDocketRow],
    plan_rows: &[TierPavementAcquisitionPlanRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !plan_rows.is_empty() && rows.len() != plan_rows.len() {
        failures.push(format!(
            "acquisition docket rows {} do not match plan rows {}",
            rows.len(),
            plan_rows.len()
        ));
    }
    for row in rows {
        if row.task_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.source_priority.trim().is_empty()
            || row.affected_routes.trim().is_empty()
            || row.affected_bundles.trim().is_empty()
            || row.fetch_command.trim().is_empty()
            || row.rebuild_command.trim().is_empty()
            || row.verify_command.trim().is_empty()
            || row.source_contract.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete acquisition docket row",
                row.task_id
            ));
        }
        if !row.fetch_command.starts_with("route fetch-hpms --states ") {
            failures.push(format!("{} has invalid fetch command", row.task_id));
        }
        if row.rebuild_command != "route build --all-roads" {
            failures.push(format!("{} has invalid rebuild command", row.task_id));
        }
        if !row
            .verify_command
            .contains("route tier-pavement-docket --gate")
            || !row
                .verify_command
                .contains("route tier-pavement-source-gaps --gate")
        {
            failures.push(format!("{} has invalid verify command", row.task_id));
        }
        if !matches!(row.source_priority.as_str(), "A" | "B" | "C") {
            failures.push(format!(
                "{} has invalid source priority {}",
                row.task_id, row.source_priority
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.task_id, row.validation_status
            ));
        }
    }
    failures
}

