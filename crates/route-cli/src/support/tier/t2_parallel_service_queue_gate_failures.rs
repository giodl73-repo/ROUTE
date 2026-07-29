//! Helper `t2_parallel_service_queue_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_parallel_service_queue_gate_failures(rows: &[T2ParallelServiceQueueRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 parallel service queue rows emitted".to_string());
        return failures;
    }
    if rows.len() == 1 && rows[0].route == "__all_t2_parallel_services__" {
        if rows[0].validation_status != "pass" {
            failures.push("parallel service clearance row must pass".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty() {
            failures.push("parallel service row missing route".to_string());
        }
        if row.region_id.trim().is_empty() {
            failures.push(format!("{} missing region_id", row.route));
        }
        if row.beck_corridor.trim().is_empty() {
            failures.push(format!("{} missing beck_corridor", row.route));
        }
        if row.close_parallel_count == 0 || row.close_parallel_corridors.trim().is_empty() {
            failures.push(format!("{} missing close-parallel evidence", row.route));
        }
        if row.selection_action != "split-parallel-service" {
            failures.push(format!(
                "{} unexpected selection_action {}",
                row.route, row.selection_action
            ));
        }
        if row.parallel_action.trim().is_empty()
            || row.required_artifact.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!("{} missing parallel action artifacts", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && !row.optimizer_effect.contains("qualification_effects=")
        {
            failures.push(format!(
                "{} parallel queue drops qualification effects",
                row.route
            ));
        }
        if row.validation_status != "review" {
            failures.push(format!(
                "{} parallel service row must remain review",
                row.route
            ));
        }
    }
    failures
}

