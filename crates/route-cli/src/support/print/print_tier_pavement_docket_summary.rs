//! Helper `print_tier_pavement_docket_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_docket_summary(
    output: &Path,
    rows: &[TierPavementDocketRow],
    details: bool,
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.pavement_status.as_str()).or_default() += 1;
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement docket rows to {}",
        rows.len(),
        output.display()
    );
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:<8} {:>7} {:>7} {:<24} {}",
            "Tier", "Route", "State", "IRI", "Max", "Status", "Repair action"
        );
        println!("{}", "-".repeat(116));
        for row in rows.iter().filter(|row| row.validation_status == "review") {
            println!(
                "{:<4} {:<8} {:<8} {:>7} {:>7} {:<24} {}",
                row.tier,
                row.route,
                row.state,
                row.iri_m_per_km,
                row.max_iri_m_per_km,
                row.pavement_status,
                truncate_for_table(&row.repair_action, 48)
            );
        }
    }
}
