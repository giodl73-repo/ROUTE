//! Helper `print_tier_optimizer_run_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_optimizer_run_summary(output: &Path, rows: &[TierOptimizerRunRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.gate_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer run rows to {}",
        rows.len(),
        output.display()
    );
    for (gate_status, count) in counts {
        println!("  {gate_status}: {count}");
    }
}
