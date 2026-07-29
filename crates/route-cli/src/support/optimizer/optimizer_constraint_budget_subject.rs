//! Helper `optimizer_constraint_budget_subject`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_budget_subject(row: &OptimizerConstraintLedgerRow) -> (String, String) {
    if !row.segment_bundle_id.trim().is_empty() {
        ("bundle".to_string(), row.segment_bundle_id.clone())
    } else if !row.route.trim().is_empty() {
        ("route".to_string(), row.route.clone())
    } else {
        (row.constraint_scope.clone(), row.subject_id.clone())
    }
}

