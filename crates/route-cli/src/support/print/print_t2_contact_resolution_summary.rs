//! Helper `print_t2_contact_resolution_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_contact_resolution_summary(output: &Path, rows: &[T2ContactResolutionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.resolution_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} contact resolution rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}
