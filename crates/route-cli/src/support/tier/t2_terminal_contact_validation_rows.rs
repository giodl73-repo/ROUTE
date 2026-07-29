//! Helper `t2_terminal_contact_validation_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_terminal_contact_validation_rows(
    held_rows: &[T2HeldContactActionRow],
    exception_rows: &[EndpointExceptionRow],
    witness_rows: &[TierContactWitnessInputRow],
) -> Vec<T2TerminalContactValidationRow> {
    let witness_by_route = witness_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();

    let mut rows = held_rows
        .iter()
        .filter(|row| {
            matches!(
                row.held_action_type.as_str(),
                "terminal-contact-validation" | "terminal-exception-review"
            )
        })
        .map(|row| {
            let route_exceptions = endpoint_exceptions_for_route(exception_rows, &row.route, "T2");
            let exception = route_exceptions.first().copied();
            let terminal_worthy = route_exceptions
                .iter()
                .any(|exception| endpoint_exception_is_terminal_worthy(exception));
            let witness = witness_by_route.get(&canonical_route_key(&row.route));
            let observed_t1_node_count = witness
                .map(|witness| witness.observed_t1_node_count)
                .unwrap_or_default();
            let observed_dual_contacts = witness
                .map(|witness| witness.observed_dual_contacts)
                .unwrap_or_default();
            let has_graph_contact = observed_t1_node_count > 0 || observed_dual_contacts > 0;
            let (terminal_action, required_evidence, next_artifact, optimizer_effect) =
                if !terminal_worthy {
                    (
                        "prove-terminal-exception-or-demote",
                        "terminal-worthy endpoint exception under T2 endpoint standard",
                        "data/tier-node-exceptions.csv",
                        "blocked from T2 unless endpoint exception is upgraded or route demotes",
                    )
                } else if !has_graph_contact {
                    (
                        "prove-terminal-contact-or-demote",
                        "terminal endpoint plus at least one T1/T2 contact chain",
                        "data/tier-contact-witnesses.csv",
                        "blocked from T2 until graph contact validates",
                    )
                } else {
                    (
                        "accept-terminal-contact",
                        "terminal-worthy endpoint and graph contact observed",
                        "data/tier-candidate-columns.csv",
                        "eligible for terminal service-column review",
                    )
                };

            T2TerminalContactValidationRow {
                route: row.route.clone(),
                held_action_type: row.held_action_type.clone(),
                endpoint_name: exception
                    .map(|exception| exception.endpoint_name.clone())
                    .unwrap_or_default(),
                endpoint_role: exception
                    .map(|exception| exception.endpoint_role.clone())
                    .unwrap_or_default(),
                exception_type: exception
                    .map(|exception| exception.exception_type.clone())
                    .unwrap_or_else(|| row.exception_type.clone()),
                terminal_worthy,
                observed_t1_node_count,
                observed_dual_contacts,
                terminal_action: terminal_action.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        rows.push(T2TerminalContactValidationRow {
            route: "__all_t2_terminal_contacts__".to_string(),
            held_action_type: "terminal-contact-clear".to_string(),
            endpoint_name: String::new(),
            endpoint_role: String::new(),
            exception_type: String::new(),
            terminal_worthy: false,
            observed_t1_node_count: 0,
            observed_dual_contacts: 0,
            terminal_action: "terminal-contact-clear".to_string(),
            required_evidence: "no terminal-contact validation blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "terminal-contact validation lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

