//! Helper `t2_contact_resolution_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_contact_resolution_rows(
    rows: &[TierContactWitnessInputRow],
    exceptions: &[EndpointExceptionRow],
) -> Vec<T2ContactResolutionRow> {
    rows.iter()
        .filter(|row| row.tier.eq_ignore_ascii_case("T2"))
        .map(|row| {
            let route_exceptions = endpoint_exceptions_for_route(exceptions, &row.route, "T2");
            let exception = route_exceptions.first().copied();
            let (resolution_action, resolution_basis, next_artifact, validation_status) =
                t2_contact_resolution_decision(row, &route_exceptions);
            T2ContactResolutionRow {
                route: row.route.clone(),
                witness_type: row.witness_type.clone(),
                node_class: row.node_class.clone(),
                repair_action: row.repair_action.clone(),
                required_artifact: row.required_artifact.clone(),
                exception_type: exception
                    .map(|exception| exception.exception_type.clone())
                    .unwrap_or_default(),
                exception_evidence_level: exception
                    .map(|exception| exception.evidence_level.clone())
                    .unwrap_or_default(),
                resolution_action: resolution_action.to_string(),
                resolution_basis: resolution_basis.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}

