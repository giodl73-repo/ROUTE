//! Helper `endpoint_exception_has_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn endpoint_exception_has_contract(row: &EndpointExceptionRow) -> bool {
    !row.endpoint_name.trim().is_empty()
        && !row.endpoint_role.trim().is_empty()
        && !row.exception_type.trim().is_empty()
        && !row.artifact.trim().is_empty()
        && !row.next_step.trim().is_empty()
        && valid_endpoint_evidence_level(&row.evidence_level)
}
