//! Helper `t2_parallel_service_queue_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_parallel_service_queue_rows(
    service_rows: &[T2ServiceSelectionRow],
) -> Vec<T2ParallelServiceQueueRow> {
    let mut rows = service_rows
        .iter()
        .filter(|row| {
            row.selection_action == "split-parallel-service" || row.close_parallel_count > 0
        })
        .map(|row| T2ParallelServiceQueueRow {
            route: row.route.clone(),
            region_id: row.region_id.clone(),
            beck_corridor: row.beck_corridor.clone(),
            service_class: row.beck_service_class.clone(),
            close_parallel_count: row.close_parallel_count,
            close_parallel_corridors: row.close_parallel_corridors.clone(),
            selection_action: row.selection_action.clone(),
            selection_basis: row.selection_basis.clone(),
            parallel_action: "review-spacing-or-split-service-before-promotion".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "docs/t2-regional-treatment.md".to_string(),
            optimizer_effect: service_diagnostic_optimizer_effect(
                "keeps close-parallel T2 line visible but below automatic keep/promotion",
                &row.qualification_effects,
            ),
            qualification_effects: row.qualification_effects.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    if rows.is_empty() {
        rows.push(T2ParallelServiceQueueRow {
            route: "__all_t2_parallel_services__".to_string(),
            region_id: String::new(),
            beck_corridor: String::new(),
            service_class: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            selection_action: "clear".to_string(),
            selection_basis: "no-close-parallel-t2-services".to_string(),
            parallel_action: "no-parallel-service-work-needed".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/game/t2-bundle-overlays.csv".to_string(),
            optimizer_effect: "all T2 service rows clear close-parallel review".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

