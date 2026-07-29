//! Helper `print_hub_outage`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_hub_outage(summary: &route_sim::HubOutageSummary, config: route_sim::HubOutageConfig) {
    println!("route hub-outage — relay hub outage sensitivity\n");
    println!(
        "Model: {:.1}h outage, {:.0}% reserve-driver absorption, {:.0}% adjacent-hub absorption of remaining swaps.\n",
        config.outage_hours,
        config.reserve_driver_fraction * 100.0,
        config.adjacent_absorption_fraction * 100.0
    );

    println!(
        "{:<35} {:>10}  {:>10}  {:>10}  {:>10}  {:>9}",
        "Hub", "Affected", "Reserve", "Adjacent", "Missed", "Retain"
    );
    println!("{}", "─".repeat(95));

    for result in &summary.results {
        println!(
            "{:<35} {:>10.1}  {:>10.1}  {:>10.1}  {:>10.1}  {:>8.1}%",
            result
                .hub_name
                .split('(')
                .next()
                .unwrap_or(&result.hub_name)
                .trim(),
            result.affected_swaps,
            result.reserve_absorbed_swaps,
            result.adjacent_absorbed_swaps,
            result.missed_swaps,
            result.throughput_retention * 100.0
        );
    }

    println!("{}", "─".repeat(95));
    println!(
        "{:<35} {:>10.1}  {:>10}  {:>10}  {:>10.1}  {:>8.1}%",
        "NETWORK",
        summary.total_affected_swaps,
        "",
        "",
        summary.total_missed_swaps,
        summary.network_throughput_retention * 100.0
    );
    println!(
        "\n  Worst-hub retention: {:.1}%",
        summary.worst_hub_throughput_retention * 100.0
    );
    println!(
        "  Gate interpretation: this is a heuristic L2 outage bound; publication-grade proof still needs actual adjacent-hub capacity, driver reserve rosters, and dispatch recovery rules."
    );
}

