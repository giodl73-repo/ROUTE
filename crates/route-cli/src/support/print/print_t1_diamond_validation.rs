//! Helper `print_t1_diamond_validation`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_diamond_validation(
    rows: &[T1DiamondValidationRow],
    blockers: bool,
    priority: Option<&str>,
    details: bool,
) {
    let filtered: Vec<&T1DiamondValidationRow> = rows
        .iter()
        .filter(|row| !blockers || !row.validation_status.eq_ignore_ascii_case("validated"))
        .filter(|row| {
            priority
                .map(|priority| row.priority_band.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect();

    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.validation_status.clone()).or_insert(0) += 1;
    }

    println!("route t1-diamond-validation");
    println!("  sites: {} shown / {} total", filtered.len(), rows.len());
    println!("  validation: {}", format_count_map(&by_status));
    println!();
    println!(
        "{:<18} {:<14} {:<20} {:<8} {:<12} {:<12} {}",
        "Site", "Intersection", "Location", "Priority", "Analyzer", "Validation", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<20} {:<8} {:<12} {:<12} {}",
            row.site_id,
            row.intersection,
            truncate_for_table(&row.location, 20),
            row.priority_band,
            row.analyzer_status,
            row.validation_status,
            row.blocking_gap
        );
        if details {
            println!("  anchor: {:.3}, {:.3}", row.anchor_lon, row.anchor_lat);
            println!("  geometry: {}", row.manual_geometry_status);
            println!("  alternate capacity: {}", row.alternate_capacity_status);
            println!("  observed failure: {}", row.observed_failure_status);
            println!("  artifact: {}", row.current_artifact);
            println!("  next: {}", row.next_validation_step);
        }
    }
}

