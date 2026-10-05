//! Helper `tier_contact_witness_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_contact_witness_gate_failures(rows: &[TierContactWitnessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no contact witness rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if tier_contact_witness_is_unresolved_blocker(
            row.witness_type.as_str(),
            row.required_artifact.as_str(),
            row.validation_status.as_str(),
        ) {
            failures.push(format!(
                "{} requires {} via {}",
                row.route, row.witness_type, row.required_artifact
            ));
        }
    }
    failures
}
