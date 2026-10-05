//! Helper `print_t3_t4_pressure_intake_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_t4_pressure_intake_summary(output: &Path, rows: &[T3T4PressureIntakeRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.intake_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3/T4 pressure intake rows to {}",
        rows.len(),
        output.display()
    );
    for (intake_class, count) in counts {
        println!("  {intake_class}: {count}");
    }
}
