//! Helper `print_stop_plan`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_stop_plan(route: &str, stops: &[&StopCandidateRow], details: bool) {
    let mut by_class = std::collections::BTreeMap::new();
    for stop in stops {
        *by_class
            .entry(stop.requested_class.trim().to_ascii_uppercase())
            .or_insert(0usize) += 1;
    }

    println!("  stops: {}", stops.len());
    println!("  class mix: {}", format_count_map(&by_class));
    println!();
    println!(
        "{:<4} {:<24} {:<5} {:<26} Evidence",
        "#", "Stop", "Class", "Role"
    );
    println!("{}", "-".repeat(86));
    for (idx, stop) in stops.iter().enumerate() {
        println!(
            "{:<4} {:<24} {:<5} {:<26} {}",
            idx + 1,
            truncate_for_table(&stop.name, 24),
            stop.requested_class,
            truncate_for_table(&stop.stop_role, 26),
            stop.evidence_status
        );
        if details {
            println!(
                "  id: {}  location: {},{}",
                stop.stop_id, stop.lat, stop.lon
            );
            println!("  routes: {}", stop.route_refs);
            println!(
                "  values: transfer={} freight={} spacing={} resilience={} energy={}",
                stop.transfer_value,
                stop.freight_volume,
                stop.spacing_need,
                stop.resilience_value,
                stop.energy_service
            );
            println!("  artifact: {}", stop.source_artifact);
            println!("  next: {}", stop.next_step);
        }
    }

    if !stops.is_empty() {
        println!();
        println!(
            "  schematic chain: {}",
            stops
                .iter()
                .map(|stop| stop.name.as_str())
                .collect::<Vec<_>>()
                .join(" -> ")
        );
    } else {
        println!("  no stop candidates touch {route}");
    }
}
