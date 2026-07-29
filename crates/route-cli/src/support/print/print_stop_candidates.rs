//! Helper `print_stop_candidates`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_stop_candidates(rows: &[&StopCandidateRow], details: bool) {
    let mut by_class = std::collections::BTreeMap::new();
    let mut by_status = std::collections::BTreeMap::new();
    for row in rows {
        *by_class
            .entry(row.requested_class.trim().to_ascii_uppercase())
            .or_insert(0usize) += 1;
        *by_status
            .entry(row.evidence_status.trim().to_ascii_lowercase())
            .or_insert(0usize) += 1;
    }

    println!("  candidates: {}", rows.len());
    println!("  class mix: {}", format_count_map(&by_class));
    println!("  evidence mix: {}", format_count_map(&by_status));
    println!();
    println!(
        "{:<16} {:<24} {:<6} {:<5} {:<28} Routes",
        "Stop", "Name", "State", "Class", "Role"
    );
    println!("{}", "-".repeat(104));
    for row in rows {
        println!(
            "{:<16} {:<24} {:<6} {:<5} {:<28} {}",
            row.stop_id,
            truncate_for_table(&row.name, 24),
            row.state,
            row.requested_class,
            truncate_for_table(&row.stop_role, 28),
            row.route_refs
        );
        if details {
            println!("  location: {},{}", row.lat, row.lon);
            println!(
                "  values: transfer={} freight={} spacing={} resilience={} energy={} land={} equity={}",
                row.transfer_value,
                row.freight_volume,
                row.spacing_need,
                row.resilience_value,
                row.energy_service,
                row.land_ops_feasibility,
                row.equity_community
            );
            println!(
                "  evidence: {} via {}",
                row.evidence_status, row.source_artifact
            );
            println!("  next: {}", row.next_step);
        }
    }
}

