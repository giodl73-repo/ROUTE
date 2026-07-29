//! Helper `print_standards_inventory`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_standards_inventory(rows: &[StandardsInventoryRow], blockers: bool, details: bool) {
    let failures = standards_inventory_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.source_status.clone()).or_insert(0) += 1;
    }

    println!("route standards-inventory");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  source status: {}", format_count_map(&by_status));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<24} {:<22} {:<14} {:<18} {}",
        "Standard", "Inventory", "Status", "Source", "Gap"
    );
    println!("{}", "-".repeat(122));
    for row in filtered {
        println!(
            "{:<24} {:<22} {:<14} {:<18} {}",
            row.standard_id,
            truncate_for_table(&row.inventory_name, 22),
            row.source_status,
            truncate_for_table(&row.source_kind, 18),
            row.blocking_gap
        );
        if details {
            println!("  artifact: {}", row.current_artifact);
            println!("  scope: {}", row.coverage_scope);
            println!("  next: {}", row.next_step);
        }
    }
}

