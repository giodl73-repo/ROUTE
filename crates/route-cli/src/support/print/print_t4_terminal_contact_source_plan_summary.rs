//! Helper `print_t4_terminal_contact_source_plan_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t4_terminal_contact_source_plan_summary(
    output: &Path,
    rows: &[T4TerminalContactSourcePlanRow],
) {
    let mut by_district = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_district
            .entry(row.terminal_district.as_str())
            .or_default() += 1;
        *by_status
            .entry(row.acquisition_status.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal contact source plan rows to {}",
        rows.len(),
        output.display()
    );
    for (district, count) in by_district {
        println!("  {district}: {count}");
    }
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

