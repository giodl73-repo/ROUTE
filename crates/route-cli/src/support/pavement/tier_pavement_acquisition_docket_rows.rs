//! Helper `tier_pavement_acquisition_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_acquisition_docket_rows(
    plan_rows: &[TierPavementAcquisitionPlanRow],
) -> Vec<TierPavementAcquisitionDocketRow> {
    let mut rows = plan_rows
        .iter()
        .map(|row| {
            let task_id = format!(
                "PAVEMENT-{}-{}",
                row.source_priority,
                row.state.to_ascii_uppercase()
            );
            TierPavementAcquisitionDocketRow {
                task_id,
                state: row.state.clone(),
                source_priority: row.source_priority.clone(),
                affected_routes: row.affected_routes.clone(),
                affected_bundles: row.affected_bundles.clone(),
                blocked_member_count: row.blocked_member_count,
                fetch_command: format!("route fetch-hpms --states {}", row.state),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: row.required_fields.clone(),
                next_artifact: row.next_artifact.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        acquisition_priority_rank(&a.source_priority)
            .cmp(&acquisition_priority_rank(&b.source_priority))
            .then_with(|| b.blocked_member_count.cmp(&a.blocked_member_count))
            .then_with(|| a.state.cmp(&b.state))
    });
    rows
}
