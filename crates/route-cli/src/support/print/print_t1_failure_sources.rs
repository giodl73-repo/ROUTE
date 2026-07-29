//! Helper `print_t1_failure_sources`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_failure_sources(rows: &[T1FailureSourceRow], lookup_needed: bool) {
    let filtered: Vec<&T1FailureSourceRow> = rows
        .iter()
        .filter(|row| !lookup_needed || row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .collect();
    let identified = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("identified"))
        .count();
    let lookup = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .count();

    println!("route t1-failure-sources");
    println!("  sources: {} shown / {} total", filtered.len(), rows.len());
    println!("  access: identified {identified}, lookup_needed {lookup}");
    println!();
    println!(
        "{:<18} {:<14} {:<18} {:<14} {}",
        "Site", "Intersection", "Location", "Access", "Primary sources"
    );
    println!("{}", "-".repeat(120));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<18} {:<14} {}",
            row.site_id,
            row.intersection,
            row.location,
            row.access_status,
            row.primary_state_sources
        );
        println!("  fields: {}", row.fields_to_populate);
        println!("  national: {}", row.national_sources);
        if !row.source_url.trim().is_empty() {
            println!("  url: {}", row.source_url);
        }
        println!("  notes: {}", row.notes);
    }
}

