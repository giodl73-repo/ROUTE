//! Helper `t2_graph_contact_repair_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_graph_contact_repair_rows(
    rows: &[T2HeldContactActionRow],
) -> Vec<T2GraphContactRepairRow> {
    let mut repairs = rows
        .iter()
        .filter(|row| row.held_action_type == "graph-contact-repair")
        .map(|row| {
            let (repair_class, repair_action, required_evidence, next_artifact, optimizer_effect) =
                t2_graph_contact_repair_contract(row);
            T2GraphContactRepairRow {
                route: row.route.clone(),
                repair_class: repair_class.to_string(),
                source_exception_type: row.exception_type.clone(),
                repair_action: repair_action.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if repairs.is_empty() {
        repairs.push(T2GraphContactRepairRow {
            route: "__all_t2_graph_contact_repairs__".to_string(),
            repair_class: "graph-contact-repair-clear".to_string(),
            source_exception_type: String::new(),
            repair_action: "graph-contact-repair-clear".to_string(),
            required_evidence: "no graph-contact repair blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "graph-contact repair lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    repairs
}
