//! Helper `t1_failure_event_observation_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_failure_event_observation_gate_failures(
    rows: &[T1FailureEventRow],
) -> Vec<String> {
    if rows.is_empty() {
        return vec!["event ledger has no observation rows".to_string()];
    }

    let mut failures = Vec::new();
    for row in rows {
        let label = if row.event_id.trim().is_empty() {
            format!("{}:<missing-event-id>", row.site_id)
        } else {
            format!("{}:{}", row.site_id, row.event_id)
        };

        if !t1_failure_event_has_observation_contract(row) {
            failures.push(format!(
                "{label} missing site/event/source/year/type/confidence/timing contract"
            ));
        }
    }

    if !rows.iter().any(|row| row.freight_relevant) {
        failures.push("event ledger has no freight-relevant observations".to_string());
    }

    failures
}
