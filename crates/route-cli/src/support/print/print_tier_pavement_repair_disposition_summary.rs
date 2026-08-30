//! Helper `print_tier_pavement_repair_disposition_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_repair_disposition_summary(
    output: &Path,
    rows: &[TierPavementRepairDispositionRow],
) {
    println!(
        "  wrote {} pavement repair disposition rows to {}",
        rows.len(),
        output.display()
    );
    let total_cost = rows
        .iter()
        .map(|row| row.estimated_repair_cost_m)
        .sum::<f64>();
    println!("  repair funding required: ${total_cost:.2}M");
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.disposition, row.relief_eligibility
        );
    }
}
