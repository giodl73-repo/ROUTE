//! Helper `lower_tier_pressure_witness_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn lower_tier_pressure_witness_gate_failures(rows: &[LowerTierPressureWitnessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no lower-tier pressure witnesses emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.current_tier.trim().is_empty()
            || row.pressure_type.trim().is_empty()
            || row.witness_action.trim().is_empty()
            || row.target_tier.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pressure witness contract",
                row.route
            ));
        }
    }
    failures
}

