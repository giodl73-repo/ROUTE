//! Helper `print_stop_sla_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_stop_sla_summary(rows: &[StopSlaRow], top: usize) {
    let mut gap_status = std::collections::BTreeMap::<String, usize>::new();
    let mut sla_windows = std::collections::BTreeMap::<String, usize>::new();
    let mut evidence = std::collections::BTreeMap::<String, usize>::new();
    let mut air = std::collections::BTreeMap::<String, usize>::new();
    let mut rail = std::collections::BTreeMap::<String, usize>::new();
    let mut max_gap = 0.0_f64;
    let mut total_miles = 0.0_f64;

    for row in rows {
        *gap_status.entry(row.stop_gap_status.clone()).or_default() += 1;
        *sla_windows
            .entry(row.freight_sla_window.clone())
            .or_default() += 1;
        *evidence.entry(row.evidence_status.clone()).or_default() += 1;
        *air.entry(row.passenger_competitive_with_air.clone())
            .or_default() += 1;
        *rail.entry(row.rail_competition_note.clone()).or_default() += 1;
        max_gap = max_gap.max(row.max_stop_gap_miles);
        total_miles += row.network_miles;
    }

    println!("route stop-sla-summary");
    println!("  stop pairs: {}", rows.len());
    println!(
        "  average network miles: {:.0}",
        total_miles / rows.len().max(1) as f64
    );
    println!("  max stop gap: {:.0} mi", max_gap);
    println!("  gap status: {}", format_count_map(&gap_status));
    println!("  freight SLA windows: {}", format_count_map(&sla_windows));
    println!("  passenger air comparison: {}", format_count_map(&air));
    println!("  rail competition notes: {}", format_count_map(&rail));
    println!("  evidence: {}", format_count_map(&evidence));

    let mut worst = rows.iter().collect::<Vec<_>>();
    worst.sort_by(|a, b| b.max_stop_gap_miles.total_cmp(&a.max_stop_gap_miles));
    println!();
    println!(
        "  {:<9} {:<9} {:>7} {:>7}  {:<28} {}",
        "Origin", "Dest", "Miles", "Gap", "Routes", "Stops"
    );
    println!("  {}", "-".repeat(96));
    for row in worst.into_iter().take(top) {
        println!(
            "  {:<9} {:<9} {:>7.0} {:>7.0}  {:<28} {}",
            row.origin_id,
            row.dest_id,
            row.network_miles,
            row.max_stop_gap_miles,
            truncate_for_table(&row.route_path, 28),
            truncate_for_table(&row.stop_path, 42)
        );
    }

    let recurring_gaps = recurring_stop_gaps(rows);
    println!();
    println!(
        "  {:<25} {:>7} {:>7}  {}",
        "Recurring Segment", "Miles", "Rows", "Labels"
    );
    println!("  {}", "-".repeat(74));
    for gap in recurring_gaps.into_iter().take(top) {
        println!(
            "  {:<25} {:>7.0} {:>7}  {}",
            gap.segment_id,
            gap.miles,
            gap.row_count,
            truncate_for_table(&gap.labels, 34)
        );
    }
}

