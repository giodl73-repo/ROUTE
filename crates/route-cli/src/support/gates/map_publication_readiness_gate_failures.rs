//! Helper `map_publication_readiness_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn map_publication_readiness_gate_failures(rows: &[MapPublicationReadinessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("map publication readiness has no rows".to_string());
        return failures;
    }
    for row in rows {
        if row.map_count == 0 {
            failures.push(format!("{} has no map atlas rows", row.readiness_id));
        }
        if row.render_gate_status != "pass" {
            failures.push(format!("{} render gate is not pass", row.readiness_id));
        }
        if row.scope_decision_status != "pass" {
            failures.push(format!(
                "{} scope decision status is {}",
                row.readiness_id, row.scope_decision_status
            ));
        }
        if row.publication_blocker_count > 0 {
            failures.push(format!(
                "{} still has {} publication blockers ({})",
                row.readiness_id, row.publication_blocker_count, row.publication_blocker_families
            ));
        }
        if row.validation_status != "pass" {
            failures.push(format!("{} validation is not pass", row.readiness_id));
        }
        if split_claim_tokens(&row.held_claims)
            .iter()
            .any(|claim| *claim == "publication")
        {
            failures.push(format!("{} still holds publication", row.readiness_id));
        }
    }
    failures
}

