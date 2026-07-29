//! Helper `print_t2_service_selection_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_service_selection_summary(output: &Path, rows: &[T2ServiceSelectionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.selection_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} service selection rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

