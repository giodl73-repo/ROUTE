//! Helper `print_t1_source_health`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_source_health(rows: &[T1SourceHealthRow], blockers: bool, details: bool) {
    let blocked = t1_source_health_blockers(rows);
    let filtered = if blockers {
        blocked.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_access: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut by_ingestion: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_access.entry(row.access_health.clone()).or_insert(0) += 1;
        *by_ingestion
            .entry(row.ingestion_status.clone())
            .or_insert(0) += 1;
    }

    println!("route t1-source-health");
    println!("  sources: {} shown / {} total", filtered.len(), rows.len());
    println!("  access: {}", format_count_map(&by_access));
    println!("  ingestion: {}", format_count_map(&by_ingestion));
    println!();
    println!(
        "{:<18} {:<24} {:<16} {:<14} {:<14} {}",
        "Site", "Source", "Access", "Ingestion", "History", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<24} {:<16} {:<14} {:<14} {}",
            row.site_id,
            truncate_for_table(&row.source_name, 24),
            row.access_health,
            row.ingestion_status,
            row.history_status,
            row.blocking_gap
        );
        if details {
            println!("  kind: {}", row.source_kind);
            println!("  last checked: {}", row.last_checked);
            println!("  url: {}", row.source_url);
            println!("  next: {}", row.next_step);
        }
    }
}

