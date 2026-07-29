//! Extracted helper `print_passenger_matrix` from main.
use super::*;

pub(crate) fn print_passenger_matrix(trips: usize, seed: u64, data_dir: &std::path::Path) {
    use route_sim::{run_passenger_simulation, PassengerMode};

    // Load Amtrak schedules from CSV; fall back to hardcoded values if file missing.
    let amtrak = load_amtrak_schedules(data_dir);

    let amtrak_hours = |slug: &str, fallback: Option<f64>| -> Option<f64> {
        amtrak.get(slug).copied().or(fallback)
    };

    // Corridors with Amtrak benchmarks (scheduled hours, reliability PTI)
    // PTI: 1.0 = perfectly on time; Amtrak long-distance PTI ~1.4-2.0
    // (corridor, amtrak_scheduled_hours, amtrak_note)
    // Airlines currently bus some short routes: BOS-NYC, LAX-SNA, etc.
    // Threshold: air is competitive when door-to-door < 4h (flight < 1.5h + overhead 2.5h)
    // Below that, bus relay often wins on total door-to-door time AND cost
    let corridors: Vec<(route_sim::OdCorridor, Option<f64>, &str)> = vec![
        (
            route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi),
            amtrak_hours("ny_chi", Some(18.0)),
            "Lake Shore Ltd 18h (60% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
            amtrak_hours("la_sea", Some(35.5)),
            "Coast Starlight 53h p95 (50% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
            amtrak_hours("mia_nyc", Some(30.0)),
            "Silver Star 45h p95 (75% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
            amtrak_hours("atl_chi", None),
            "No direct Amtrak service",
        ),
        (
            route_sim::load_corridor(data_dir, "hou_chi_i69")
                .unwrap_or_else(route_sim::hou_chi_i69),
            amtrak_hours("hou_chi_i69", None),
            "No direct Amtrak",
        ),
        (
            route_sim::load_corridor(data_dir, "dal_nyc").unwrap_or_else(route_sim::dal_nyc),
            amtrak_hours("dal_nyc", None),
            "No direct Amtrak",
        ),
        (
            route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
            amtrak_hours("sea_chi", Some(46.0)),
            "Empire Builder 69h p95 (65% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
            amtrak_hours("ny_la", Some(67.0)),
            "Southwest Chief 100h p95 (55% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "chi_la").unwrap_or_else(route_sim::chi_la),
            amtrak_hours("chi_la", Some(43.0)),
            "Southwest Chief 64h p95 (55% on-time)",
        ),
    ];

    println!(
        "{:<35} {:>6}  {:>10}  {:>12}  {:>12}  {:>14}  {:>10}",
        "Corridor",
        "Miles",
        "Amtrak p95",
        "Bus relay",
        "AV managed",
        "Air (door-to-door)",
        "AV vs Air"
    );
    println!(
        "{:<35} {:>6}  {:>10}  {:>12}  {:>12}  {:>14}  {:>10}",
        "", "", "(current)", "($0.12/mi)", "(~$0.18/mi)", "(est.)", ""
    );
    println!("{}", "─".repeat(110));

    for (corridor, amtrak_sched, _amtrak_note) in &corridors {
        let miles = corridor.total_miles();

        let bus = run_passenger_simulation(
            corridor,
            PassengerMode::ExpressBus,
            trips,
            seed,
            *amtrak_sched,
        );
        let av = run_passenger_simulation(
            corridor,
            PassengerMode::AutonomousVehicle,
            trips,
            seed + 1,
            *amtrak_sched,
        );

        let amtrak_str = if let Some(sched) = amtrak_sched {
            let pti = 1.5; // typical long-distance Amtrak PTI
            format!("{:.0}h p95", sched * pti)
        } else {
            "no service".to_string()
        };

        // Air: door-to-door estimate (drive to airport 45min + security 60min + flight + arrive 45min)
        let flight_hours = miles / 500.0; // rough cruising speed
        let air_dttd = flight_hours + 2.5; // airport overhead both ends
        let air_str = format!("{:.1}h", air_dttd);

        // Does AV beat air door-to-door?
        let av_vs_air = if av.p95_hours < air_dttd {
            format!("AV faster +{:.1}h", air_dttd - av.p95_hours)
        } else {
            format!("Air -{:.1}h", av.p95_hours - air_dttd)
        };

        println!(
            "{:<35} {:>6.0}  {:>10}  {:>10.1}h  {:>10.1}h  {:>14}  {:>10}",
            corridor
                .name
                .split(' ')
                .take(4)
                .collect::<Vec<_>>()
                .join(" "),
            miles,
            amtrak_str,
            bus.p95_hours,
            av.p95_hours,
            air_str,
            av_vs_air,
        );
    }

    println!("\n{}", "─".repeat(110));
    println!("\nKey: p95 = 95th-percentile commitment window. Air = door-to-door (45min drive + 60min security + flight).");
    println!("Bus relay at $0.12/mi ≈ $0.12 × miles. AV managed at ~$0.18/mi (fuel + managed lane toll).");
    println!();
    println!("── Bus routes competitive with air (< 4h door-to-door threshold) ──────────");
    println!("  Airlines already bus some short-haul routes (United/Delta bus BOS↔NYC, LAX↔SNA).");
    println!(
        "  Door-to-door air < 4h means flight is under 1.5h — below that, bus relay competes:"
    );
    println!();
    println!("  NY→CHI (790mi):    bus relay ~12h  vs air 4.7h — NOT competitive on time,");
    println!(
        "                      but competitive on COST ($95 bus vs $180+ air + Uber both ends)"
    );
    println!("                      and AV managed lane ~10h = sleep in your car, arrive rested");
    println!();
    println!(
        "  Routes where I2.0 BUS RELAY beats air door-to-door (rare; requires short corridor):"
    );
    println!("  → sub-300 mile routes where air = 3.5h door-to-door but bus relay = 3h:");
    println!("    LA→San Diego (120mi): bus relay ~2.5h vs air 2.8h door-to-door — BUS WINS");
    println!(
        "    NYC→Philadelphia (95mi): bus relay ~1.8h vs air 2.5h — BUS WINS (Amtrak 1.5h wins)"
    );
    println!("    Chicago→Milwaukee (90mi): bus relay ~1.7h vs air 2.3h — BUS WINS");
    println!("    Miami→Orlando (240mi): bus relay ~4.5h vs air 3.2h — air narrowly wins");
    println!();
    println!("── The AV managed lane passenger case ──────────────────────────────────────");
    println!("  Not competing with air. Replacing: exhausting driving, unreliable Amtrak,");
    println!("  slow bus. The 'sleep-and-arrive' use case:");
    println!();
    println!("  NY→CHI: depart 10pm, arrive 8am rested. Beats Lake Shore (18h+, unreliable).");
    println!("  MIA→NYC: depart 8pm, arrive noon next day. Beats Silver Star (45h p95!).");
    println!("  ATL→CHI: depart 9pm, arrive 8am. No Amtrak alternative. Beats driving.");
    println!("  SEA→CHI: depart Sunday 6pm, arrive Tuesday 8am. Empire Builder p95 = 69h.");
    println!();
    println!("  AV managed lane is the return of the overnight sleeper — in your own car.");
}

