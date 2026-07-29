//! Helper `print_blueprint_cost_ranges`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_blueprint_cost_ranges(rows: &[BlueprintCostRow], blockers: bool, details: bool) {
    let filtered = if blockers {
        rows.iter()
            .filter(|row| {
                matches!(
                    row.source_status.trim().to_ascii_lowercase().as_str(),
                    "source_needed" | "corridor_specific" | "planning_range"
                )
            })
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.source_status.clone()).or_insert(0) += 1;
    }

    println!("route blueprint-costs");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  source status: {}", format_count_map(&by_status));
    println!();
    println!(
        "{:<24} {:<18} {:<24} {:<16} {}",
        "Package", "Source", "Capital range", "Claim", "Cost basis"
    );
    println!("{}", "-".repeat(126));
    for row in filtered {
        println!(
            "{:<24} {:<18} {:<24} {:<16} {}",
            row.package_id,
            row.source_status,
            row.capital_range_2026_usd,
            row.cost_claim_status,
            row.cost_basis
        );
        if details {
            println!("  lifecycle_burden: {}", row.lifecycle_burden);
            println!("  source_artifact: {}", row.source_artifact);
            println!("  risk: {}", row.risk_note);
            println!("  next: {}", row.next_cost_step);
        }
    }
}

