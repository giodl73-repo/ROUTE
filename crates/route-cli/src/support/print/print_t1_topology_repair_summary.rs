//! Helper `print_t1_topology_repair_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_topology_repair_summary(output: &Path, rows: &[T1TopologyRepairRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_type.as_str()).or_default() += 1;
    }
    println!("  wrote {} repair rows to {}", rows.len(), output.display());
    for (repair_type, count) in counts {
        println!("  {repair_type}: {count}");
    }
}

