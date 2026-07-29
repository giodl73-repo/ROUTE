//! Helper `print_t1_snapshot_plan`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_snapshot_plan(rows: &[T1SnapshotPlanRow], priority: Option<&str>, details: bool) {
    let filtered: Vec<&T1SnapshotPlanRow> = rows
        .iter()
        .filter(|row| {
            priority
                .map(|priority| row.priority_band.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect();
    let mut by_cadence: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_cadence.entry(row.cadence.clone()).or_insert(0) += 1;
    }

    println!("route t1-snapshot-plan");
    println!("  feeds: {} shown / {} total", filtered.len(), rows.len());
    println!("  cadence: {}", format_count_map(&by_cadence));
    println!();
    println!(
        "{:<18} {:<14} {:<8} {:<24} {:<14} {}",
        "Site", "Intersection", "Priority", "Source", "Cadence", "Next"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<8} {:<24} {:<14} {}",
            row.site_id,
            row.intersection,
            row.priority_band,
            truncate_for_table(&row.source_name, 24),
            row.cadence,
            row.next_step
        );
        if details {
            println!("  source health: {}", row.source_health);
            println!("  fetch: {}", row.fetch_command);
            println!("  import: {}", row.import_command);
            println!("  accumulate: {}", row.accumulate_command);
            println!("  raw: {}", row.raw_output);
            println!("  normalized: {}", row.normalized_output);
            println!("  accumulated: {}", row.accumulated_output);
            println!("  gap: {}", row.blocking_gap);
        }
    }
}

