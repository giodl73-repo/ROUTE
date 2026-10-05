//! Helper `print_lower_tier_pressure_witness_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_lower_tier_pressure_witness_summary(
    output: &Path,
    rows: &[LowerTierPressureWitnessRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.pressure_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pressure witness rows to {}",
        rows.len(),
        output.display()
    );
    for (pressure_type, count) in counts {
        println!("  {pressure_type}: {count}");
    }
}
