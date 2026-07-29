//! Helper `print_t4_terminal_access_source_access_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_access_source_access_summary(
    output: &Path,
    rows: &[T4TerminalAccessSourceAccessRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.access_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access source access rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in counts {
        println!("  {mode}: {count}");
    }
}

