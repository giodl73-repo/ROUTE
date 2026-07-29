//! Helper `print_optimizer_manifest_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_manifest_summary(path: &Path, rows: &[TierOptimizerRunRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.gate_status.as_str()).or_default() += 1;
    }
    println!(
        "  read {} optimizer manifest rows from {}",
        rows.len(),
        path.display()
    );
    for (gate_status, count) in counts {
        println!("  {gate_status}: {count}");
    }
}

