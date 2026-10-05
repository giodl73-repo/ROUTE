//! Helper `print_t2_held_contact_action_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_held_contact_action_summary(output: &Path, rows: &[T2HeldContactActionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.held_action_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} held contact action rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}
