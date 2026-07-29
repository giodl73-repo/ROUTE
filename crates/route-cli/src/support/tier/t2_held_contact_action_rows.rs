//! Helper `t2_held_contact_action_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_held_contact_action_rows(rows: &[T2ContactResolutionRow]) -> Vec<T2HeldContactActionRow> {
    rows.iter()
        .filter(|row| row.validation_status == "review")
        .map(|row| {
            let (held_action_type, required_evidence, next_artifact, optimizer_effect) =
                t2_held_contact_action_contract(row);
            T2HeldContactActionRow {
                route: row.route.clone(),
                held_action_type: held_action_type.to_string(),
                source_resolution_action: row.resolution_action.clone(),
                exception_type: row.exception_type.clone(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}

