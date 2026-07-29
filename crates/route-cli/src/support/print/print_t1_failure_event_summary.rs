//! Helper `print_t1_failure_event_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_failure_event_summary(rows: &[T1FailureEventRow]) {
    let summaries = summarize_t1_failure_events(rows);
    let freight_rows = rows.iter().filter(|row| row.freight_relevant).count();
    let full_closures = rows.iter().filter(|row| row.full_closure).count();
    let lane_rows = rows.iter().filter(|row| row.lanes_closed.is_some()).count();
    let source_id_rows = rows
        .iter()
        .filter(|row| !row.source_event_id.trim().is_empty())
        .count();
    let timed_rows = rows
        .iter()
        .filter(|row| !row.start_time.trim().is_empty() && !row.end_time.trim().is_empty())
        .count();
    let noted_rows = rows
        .iter()
        .filter(|row| !row.notes.trim().is_empty())
        .count();
    let sources = rows
        .iter()
        .map(|row| row.source.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let event_types = rows
        .iter()
        .map(|row| row.event_type.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let confidence_labels = rows
        .iter()
        .map(|row| row.confidence.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    println!("route t1-failure-events");
    println!("  events: {} raw rows", rows.len());
    println!("  freight-relevant rows: {freight_rows}");
    println!(
        "  sites with freight-relevant observations: {}",
        summaries.len()
    );
    if rows.is_empty() {
        println!("  no observations loaded yet; populate data/t1-failure-events.csv from source plan records");
        return;
    }
    println!("  full closures: {full_closures}");
    println!("  rows with lane counts: {lane_rows}");
    println!("  rows with source event ids: {source_id_rows}");
    println!("  rows with start/end times: {timed_rows}");
    println!("  rows with notes: {noted_rows}");
    println!("  sources: {}", join_set(&sources));
    println!("  event types: {}", join_set(&event_types));
    println!("  confidence labels: {}", join_set(&confidence_labels));
    println!();
    println!(
        "{:<18} {:>6} {:>7} {:>8} {:>8} {:>8} {:>8}",
        "Site", "Years", "Events", "Rate/Yr", "P_ann", "P50 h", "P95 h"
    );
    println!("{}", "-".repeat(78));
    for summary in summaries {
        println!(
            "{:<18} {:>6} {:>7} {:>8.3} {:>8.3} {:>8} {:>8}",
            summary.site_id,
            summary.observed_years,
            summary.event_count,
            summary.annual_rate,
            summary.annual_probability,
            fmt_opt(summary.duration_p50_hours),
            fmt_opt(summary.duration_p95_hours)
        );
    }
}

