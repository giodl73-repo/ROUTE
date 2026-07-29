//! Helper `print_tier_pavement_repair_funding_package_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_repair_funding_package_summary(
    output: &Path,
    rows: &[TierPavementRepairFundingPackageRow],
) {
    println!(
        "  wrote {} pavement repair funding package rows to {}",
        rows.len(),
        output.display()
    );
    let total_cost = rows
        .iter()
        .map(|row| row.estimated_repair_cost_m)
        .sum::<f64>();
    println!("  unfunded repair package total: ${total_cost:.2}M");
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.funding_package_status, row.funding_commitment_status
        );
    }
}

