//! Helper `print_t4_terminal_access_column_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_column_summary(
    output: &Path,
    rows: &[T4TerminalAccessColumnRow],
) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision.entry(row.column_decision.as_str()).or_default() += 1;
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access column rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}
