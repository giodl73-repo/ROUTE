//! Helper `tier_optimizer_run_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_optimizer_run_gate_failures(
    all_tiers: bool,
    rows: &[TierOptimizerRunRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !all_tiers {
        failures.push("tier-optimize bundle gate requires --all-tiers".to_string());
    }
    failures.extend(optimizer_manifest_gate_failures(rows));
    failures
}
