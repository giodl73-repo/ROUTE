//! Helper `print_t2_graph_contact_repair_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_graph_contact_repair_summary(output: &Path, rows: &[T2GraphContactRepairRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} graph contact repair rows to {}",
        rows.len(),
        output.display()
    );
    for (repair_class, count) in counts {
        println!("  {repair_class}: {count}");
    }
}

