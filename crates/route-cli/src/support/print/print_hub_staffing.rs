//! Helper `print_hub_staffing`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_hub_staffing(net: &route_sim::NetworkSummary, proposed: bool) {
    println!("route hub-staff — T1 relay hub employment model\n");
    println!("Model: truck volumes from HPMS AADT × truck fraction.");
    println!("Relay drivers: 1 driver per truck swap, 3 shifts/day, 5-day week, 35%% buffer.");
    println!("Like airline crew bases: drivers work 1 leg, home same day.\n");

    println!(
        "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}",
        "Hub", "Trucks/d", "Swaps/d", "Frt Drvr", "Bus Drvr", "Support", "Total Jobs"
    );
    println!("{}", "─".repeat(95));

    for s in &net.hub_staffings {
        let is_proposed = s.hub_name.contains("proposed");
        let marker = if is_proposed { " *" } else { "" };
        println!(
            "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}{}",
            s.hub_name.split('(').next().unwrap_or(&s.hub_name).trim(),
            s.daily_truck_swaps,
            s.daily_total_swaps,
            s.freight_relay_drivers,
            s.bus_relay_drivers,
            s.dispatchers + s.maintenance_staff + s.admin_scheduling,
            s.total_hub_employment,
            marker,
        );
    }

    println!("{}", "─".repeat(95));
    println!(
        "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}",
        "TOTAL (all hubs)",
        net.hub_staffings
            .iter()
            .map(|s| s.daily_truck_swaps)
            .sum::<u32>(),
        net.total_daily_swaps,
        net.total_freight_drivers,
        net.total_bus_drivers,
        net.hub_staffings
            .iter()
            .map(|s| s.dispatchers + s.maintenance_staff + s.admin_scheduling)
            .sum::<u32>(),
        net.total_hub_employment,
    );

    if proposed {
        println!("\n  * = proposed hub (corridor not yet built)");
    }

    println!("\n── What this means ──────────────────────────────────────────────────────");
    println!(
        "  {} total hub-based jobs nationally ({} hubs)",
        net.total_hub_employment, net.total_hubs
    );
    println!(
        "  {} freight relay drivers — regional CDL jobs, home every night",
        net.total_freight_drivers
    );
    println!(
        "  {} bus relay drivers — intercity express on managed lanes",
        net.total_bus_drivers
    );
    println!();

    let avg_wage_freight = 58_000u32; // relay driver: regional premium, no overnight
    let avg_wage_bus = 52_000u32;
    let avg_wage_support = 48_000u32;
    let support_count: u32 = net
        .hub_staffings
        .iter()
        .map(|s| s.dispatchers + s.maintenance_staff + s.admin_scheduling)
        .sum();
    let total_payroll = (net.total_freight_drivers as u64 * avg_wage_freight as u64
        + net.total_bus_drivers as u64 * avg_wage_bus as u64
        + support_count as u64 * avg_wage_support as u64)
        / 1_000_000;

    println!("  Annual payroll: ~${total_payroll}M at hub locations");
    println!("  Average freight relay driver: ${avg_wage_freight}/yr (vs $70,000 long-haul signing bonus alone)");
    println!("  Driver shortage: 80,000 current shortfall; relay model expands addressable pool");
    println!("  Repositioning: drivers return home via relay hub bus network or partner vehicles");
    println!();
    println!("── Comparison: airline crew base model ──────────────────────────────────");
    println!("  United Airlines crew bases: ~12 bases, ~25,000 pilots/FAs total");
    println!(
        "  I2.0 relay hubs: {} bases, {} drivers",
        net.total_hubs,
        net.total_freight_drivers + net.total_bus_drivers
    );
    println!("  Pilot works 1 flight leg, overnights at hub or flies back on jumpseat");
    println!("  Relay driver works 1 truck leg, drives back or takes hub bus home");
    println!("  The operational model is identical. The regulation is the gap.");
}

