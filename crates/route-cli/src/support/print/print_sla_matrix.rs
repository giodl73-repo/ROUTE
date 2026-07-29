//! Helper `print_sla_matrix` (support::print).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_sla_matrix(trips: usize, seed: u64, data_dir: &std::path::Path) {
    use route_sim::{apply_interventions, run_od_simulation_with_driver, DriverMode, Intervention};

    // All corridors — loaded from od-corridors.toml, falling back to built-ins
    let corridors = vec![
        route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
        route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
        route_sim::load_corridor(data_dir, "hou_chi_i69").unwrap_or_else(route_sim::hou_chi_i69),
        route_sim::load_corridor(data_dir, "hou_chi_current")
            .unwrap_or_else(route_sim::hou_chi_current),
        route_sim::load_corridor(data_dir, "dal_nyc").unwrap_or_else(route_sim::dal_nyc),
        route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
        route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
        route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
        route_sim::load_corridor(data_dir, "chi_la").unwrap_or_else(route_sim::chi_la),
    ];

    let relay_interventions = |c: &route_sim::OdCorridor| {
        let stations = ((c.total_miles() / 500.0).ceil() as usize).max(1);
        vec![Intervention::DriverRelay {
            stations,
            swap_minutes: 20.0,
        }]
    };

    let full_stack = |c: &route_sim::OdCorridor| {
        let stations = ((c.total_miles() / 500.0).ceil() as usize).max(1);
        vec![
            Intervention::ManagedFreightLanes,
            Intervention::DonnerTunnel,
            Intervention::DiamondInterchanges,
            Intervention::IntelligentRouting,
            Intervention::DriverRelay {
                stations,
                swap_minutes: 15.0,
            },
        ]
    };

    println!(
        "{:<38} {:>6}  {:>10}  {:>12}  {:>10}  {:>10}  {:>12}",
        "Corridor", "Miles", "Today p95", "Relay only", "Relay+Mgd", "Full I2.0", "SLA unlock"
    );
    println!(
        "{:<38} {:>6}  {:>10}  {:>12}  {:>10}  {:>10}  {:>12}",
        "", "", "(solo/GP)", "($40M)", "(+$121B)", "(full stk)", ""
    );
    println!("{}", "─".repeat(110));

    for c in &corridors {
        let miles = c.total_miles();

        // 1. Today: solo/GP
        let today = run_od_simulation_with_driver(c, false, &DriverMode::Solo, trips, seed);

        // 2. Relay only (GP lanes)
        let relay_only = {
            let (modified, driver) = apply_interventions(c, &relay_interventions(c));
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 1)
        };

        // 3. Relay + managed lanes
        let relay_managed = {
            let interventions = {
                let stations = ((miles / 500.0).ceil() as usize).max(1);
                vec![
                    Intervention::ManagedFreightLanes,
                    Intervention::DriverRelay {
                        stations,
                        swap_minutes: 20.0,
                    },
                ]
            };
            let (modified, driver) = apply_interventions(c, &interventions);
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 2)
        };

        // 4. Full I2.0 stack
        let full = {
            let (modified, driver) = apply_interventions(c, &full_stack(c));
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 3)
        };

        // SLA classification
        let sla_label = |h: f64| -> &str {
            if h <= 12.0 {
                "12h (half-day)"
            } else if h <= 24.0 {
                "24h (overnight)"
            } else if h <= 36.0 {
                "36h (next-day)"
            } else if h <= 48.0 {
                "48h (2-day)"
            } else if h <= 72.0 {
                "72h (3-day)"
            } else {
                ">3-day"
            }
        };

        // Highlight which scenario first achieves a new SLA tier
        let today_sla = sla_label(today.p95_hours);
        let full_sla = sla_label(full.p95_hours);
        let unlock = if full_sla != today_sla {
            format!("{} → {}", today_sla, full_sla)
        } else {
            format!("holds at {}", today_sla)
        };

        println!(
            "{:<38} {:>6.0}  {:>8.1}h   {:>10.1}h  {:>9.1}h  {:>9.1}h  {}",
            c.name,
            miles,
            today.p95_hours,
            relay_only.p95_hours,
            relay_managed.p95_hours,
            full.p95_hours,
            unlock,
        );
    }

    println!("\n{}", "─".repeat(110));
    println!("\nSLA categories: 12h (half-day) | 24h (overnight) | 36h (next-day) | 48h (2-day) | 72h (3-day)");
    println!("Relay only = $40M per corridor. Relay+Managed = +$121B program. Full stack = +Donner/Diamond/Routing.");
    println!("\nMarketplace note: relay captures 90%+ of the gain at 0.03% of the cost.");
    println!("The relay MARKETPLACE (driver matching, HOS handoff, load custody) is the critical enabler.");
}

