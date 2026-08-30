//! Helper `print_optimizer_map_hook_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_optimizer_map_hook_summary(output: &Path, rows: &[OptimizerMapHookRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.consumer_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer map hook rows to {}",
        rows.len(),
        output.display()
    );
    for (consumer_type, count) in counts {
        println!("  {consumer_type}: {count}");
    }
}
