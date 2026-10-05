//! Helper `print_ev_rest_outage`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_ev_rest_outage(
    data_dir: &std::path::Path,
    config: route_sim::EvRestOutageConfig,
) {
    let corridors = vec![
        route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi),
        route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
        route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
        route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
        route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
        route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
    ];
    let evs = load_ev_profiles(data_dir);

    println!("route ev-rest-outage — EV/rest-area outage sensitivity\n");
    println!(
        "Model: T1 charging every {:.0}mi, {:.0}% station outage, {:.0}% backup power/mobile charging, {:.0}min queue penalty.\n",
        config.station_spacing_miles,
        config.outage_station_fraction * 100.0,
        config.backup_power_fraction * 100.0,
        config.queue_delay_minutes
    );
    println!(
        "{:<28} {:<24} {:>5}  {:>5}  {:>6}  {:>7}  {:>8}  {}",
        "Corridor", "Vehicle", "Stops", "Out", "Backup", "Delay", "Retain", "Viable"
    );
    println!("{}", "─".repeat(106));

    let mut worst_retention = 1.0_f64;
    let mut failing = 0usize;
    for corridor in &corridors {
        for ev in &evs {
            let result = route_sim::analyze_ev_rest_outage(corridor, ev, config);
            worst_retention = worst_retention.min(result.throughput_retention);
            if !result.viable {
                failing += 1;
            }
            println!(
                "{:<28} {:<24} {:>5}  {:>5}  {:>6}  {:>6.1}h  {:>7.1}%  {}",
                truncate_for_table(&result.corridor_name, 28),
                truncate_for_table(&result.ev_name, 24),
                result.planned_stops,
                result.disrupted_stops,
                result.backup_absorbed_stops,
                result.queue_delay_hours,
                result.throughput_retention * 100.0,
                if result.viable { "yes" } else { "no" }
            );
        }
    }

    println!("{}", "─".repeat(106));
    println!("  Worst retention: {:.1}%", worst_retention * 100.0);
    println!("  Non-viable corridor/vehicle pairs: {failing}");
    println!(
        "  Gate interpretation: this is a heuristic L2 outage bound; publication-grade proof still needs station inventory, grid outage distributions, and observed queue data."
    );
}
