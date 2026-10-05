//! Helper `print_t2_parent_contact_validation_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_parent_contact_validation_summary(
    output: &Path,
    rows: &[T2ParentContactValidationRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.validation_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} parent contact validation rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}
