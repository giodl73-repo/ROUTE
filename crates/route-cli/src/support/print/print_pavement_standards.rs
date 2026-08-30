//! Helper `print_pavement_standards`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_pavement_standards(
    rows: &[PavementStandardRow],
    blockers: bool,
    details: bool,
) {
    let failures = pavement_standard_gate_failures(rows);
    let failure_tiers = failures
        .iter()
        .filter_map(|failure| failure.split_whitespace().next())
        .collect::<std::collections::BTreeSet<_>>();
    let filtered = if blockers {
        rows.iter()
            .filter(|row| failure_tiers.contains(row.tier.as_str()))
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    println!("route standards-pavement");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<4} {:<24} {:>7} {:<12} {}",
        "Tier", "Role", "IRI", "Condition", "Repair trigger"
    );
    println!("{}", "-".repeat(112));
    for row in filtered {
        println!(
            "{:<4} {:<24} {:>7.2} {:<12} {}",
            row.tier,
            truncate_for_table(&row.road_role, 24),
            row.max_iri_m_per_km,
            row.target_pavement_condition,
            row.repair_trigger
        );
        if details {
            println!("  freight: {}", row.freight_ride_requirement);
            println!("  transit: {}", row.transit_ride_requirement);
            println!("  inspection: {} months", row.inspection_interval_months);
            println!("  exception: {}", row.allowed_exception);
            println!("  source: {}", row.source_contract);
            println!("  status: {}", row.validation_status);
        }
    }
}
