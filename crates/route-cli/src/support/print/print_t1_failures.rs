//! Helper `print_t1_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_failures(rows: &[T1FailureRow], needs_sources: bool, details: bool) {
    let filtered: Vec<&T1FailureRow> = rows
        .iter()
        .filter(|row| !needs_sources || row.source_status.eq_ignore_ascii_case("source_needed"))
        .collect();

    let empirical = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("empirical"))
        .count();
    let modeled = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("modeled"))
        .count();
    let source_needed = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("source_needed"))
        .count();

    println!("route t1-failures");
    println!("  sites: {} shown / {} total", filtered.len(), rows.len());
    println!("  evidence: empirical {empirical}, modeled {modeled}, source_needed {source_needed}");
    println!();
    println!(
        "{:<18} {:<14} {:<18} {:<13} {:>8} {:>8} {:>8} {}",
        "Site", "Intersection", "Location", "Source", "P_fail", "KeepNow", "KeepI2", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<18} {:<13} {:>8} {:>8} {:>8} {}",
            row.site_id,
            row.intersection,
            row.location,
            row.source_status,
            fmt_opt(row.annual_probability),
            fmt_opt(row.throughput_retention_current),
            fmt_opt(row.throughput_retention_i2),
            row.blocking_gap
        );
        if details {
            println!("  failure mode: {}", row.failure_mode);
            println!(
                "  duration p50/p95: {} / {} h",
                fmt_opt(row.duration_p50_hours),
                fmt_opt(row.duration_p95_hours)
            );
            println!(
                "  reroute p50/p95: {} / {} h",
                fmt_opt(row.reroute_time_p50_hours),
                fmt_opt(row.reroute_time_p95_hours)
            );
            println!("  confidence: {}", row.confidence);
            println!("  artifact: {}", row.current_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}
