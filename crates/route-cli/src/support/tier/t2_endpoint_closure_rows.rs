//! Helper `t2_endpoint_closure_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_endpoint_closure_rows(
    closure_rows: &[T2BlockerClosureRow],
    exception_rows: &[EndpointExceptionRow],
) -> Vec<T2EndpointClosureRow> {
    let mut rows = closure_rows
        .iter()
        .filter(|row| row.blocker_class == "endpoint-exception-upgrade")
        .map(|row| {
            let exception = endpoint_exceptions_for_route(exception_rows, &row.route, "T2")
                .into_iter()
                .next();
            let terminal_worthy = exception
                .map(endpoint_exception_is_terminal_worthy)
                .unwrap_or_default();
            let (endpoint_action, disposition, required_evidence, next_artifact, optimizer_effect) =
                if terminal_worthy {
                    (
                        "accept-terminal-worthy-exception",
                        "candidate-review",
                        "terminal-worthy endpoint exception",
                        "data/tier-candidate-columns.csv",
                        "eligible for T2 candidate-column review",
                    )
                } else {
                    (
                        "upgrade-endpoint-exception-or-demote",
                        "lower-tier-pressure",
                        "terminal-worthy endpoint role and exception type",
                        "data/lower-tier-pressure-witnesses.csv",
                        "kept out of T2 until endpoint exception is upgraded",
                    )
                };

            T2EndpointClosureRow {
                route: row.route.clone(),
                endpoint_name: exception
                    .map(|exception| exception.endpoint_name.clone())
                    .unwrap_or_default(),
                endpoint_role: exception
                    .map(|exception| exception.endpoint_role.clone())
                    .unwrap_or_default(),
                exception_type: exception
                    .map(|exception| exception.exception_type.clone())
                    .unwrap_or_default(),
                evidence_level: exception
                    .map(|exception| exception.evidence_level.clone())
                    .unwrap_or_default(),
                terminal_worthy,
                endpoint_action: endpoint_action.to_string(),
                disposition: disposition.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        rows.push(T2EndpointClosureRow {
            route: "__all_t2_endpoint_closures__".to_string(),
            endpoint_name: String::new(),
            endpoint_role: String::new(),
            exception_type: String::new(),
            evidence_level: String::new(),
            terminal_worthy: false,
            endpoint_action: "endpoint-closure-clear".to_string(),
            disposition: "clear".to_string(),
            required_evidence: "no endpoint-exception closure blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "endpoint closure lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}
