//! Helper `print_ev_analysis`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_ev_analysis(data_dir: &std::path::Path) {
    use route_sim::analyze_ev_charging;

    let i20_dcfc_kw = 150.0; // T1 standard: 150kW minimum DCFC

    let corridors = vec![
        route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi),
        route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
        route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
        route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
        route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
        route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
    ];

    let evs = load_ev_profiles(data_dir);

    println!("route ev-analysis — I2.0 guaranteed DCFC (150kW every 50 miles on T1)\n");
    println!("Current T1 DCFC gap: rural segments have 80-120+ mile gaps (some 0 DCFC at all).");
    println!(
        "I2.0 standard: DCFC ≤ 50 miles, 150kW minimum passenger / 350kW freight terminals.\n"
    );

    // Compare vs train lines
    println!("── How I2.0 compares to high-speed rail investment ─────────────────────────");
    println!("  Northeast Corridor (BOS-NYC-WAS, 440mi): Amtrak Acela 3.5h, $150-300");
    println!("    I2.0 AV managed lane same corridor: ~5.9h — rail wins on this dense corridor");
    println!("    BUT: Acela capital cost = $50B+ for 440mi. I2.0 DCFC: $400M for 440mi of T1.");
    println!();
    println!("  California HSR (SF-LA, 380mi): projected $100B+, 2h40m target (not built)");
    println!("    I2.0 AV managed lane SF-LA: ~5.5h via I-5 — rail wins IF built");
    println!("    BUT: HSR $100B for one corridor vs I2.0 $253B for entire national network.");
    println!();
    println!("  For corridors WITHOUT rail (Atlanta-Chicago, Dallas-NYC, Houston-Chicago):");
    println!("    Rail: not built, not planned, EIS would take 20+ years");
    println!("    I2.0 AV managed lane: operational in 5-10 years on existing right-of-way");
    println!("    I2.0 wins by default on every corridor where rail doesn't exist.");
    println!();
    println!("  The rail comparison depends on the question:");
    println!("  'Is AV managed lane faster than HSR?' → No, on dense corridors where HSR exists.");
    println!("  'Does I2.0 give more Americans better travel options?' → Yes, overwhelmingly.");
    println!("  HSR serves 5-10 dense corridors. I2.0 serves 60,000 miles of T1/T2 network.");
    println!();

    println!("── EV charging analysis by corridor ─────────────────────────────────────────");
    println!(
        "{:<38} {:>8}  {:>12}  {:>10}  {:>8}  {}",
        "Corridor", "Miles", "EV type", "Stops I2.0", "Chrg min", "Overnight OK?"
    );
    println!("{}", "─".repeat(100));

    for corridor in &corridors {
        for ev in &evs {
            let analysis = analyze_ev_charging(corridor, ev, i20_dcfc_kw);
            let overnight = if analysis.overnight_scenario {
                "✓ auto-charge"
            } else {
                "needs stop"
            };
            println!(
                "{:<38} {:>8.0}  {:>12}  {:>10}  {:>8.0}  {}",
                corridor.name.split('(').next().unwrap_or("").trim(),
                analysis.corridor_miles,
                ev.name.split('(').next().unwrap_or(ev.name).trim(),
                analysis.stops_i20,
                analysis.charge_minutes_i20,
                overnight,
            );
        }
        println!();
    }

    println!("── The overnight AV scenario ─────────────────────────────────────────────");
    println!("  Tesla Model Y (290mi range) on NY→CHI (760mi):");
    let ny_chi = route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi);
    let model_y = evs
        .iter()
        .find(|e| e.highway_range_miles >= 280.0 && e.charge_rate_kw <= 250.0)
        .cloned()
        .unwrap_or_else(route_sim::tesla_model_y);
    let a = analyze_ev_charging(&ny_chi, &model_y, i20_dcfc_kw);
    println!("    Charging stops: {}", a.stops_i20);
    println!("    Total charge time: {:.0} minutes", a.charge_minutes_i20);
    println!("    {}", a.overnight_note);
    println!();
    println!("  The AV pulls off at the hub, plugs in automatically (CCS/NACS standard),");
    println!("  charges for 20 minutes while you sleep, continues. You wake up in Chicago.");
    println!("  Zero range anxiety. Zero driver fatigue. Guaranteed charging at every hub.");
    println!();
    println!("  Current gap: I-80 through Wyoming has 85-120 mile gaps between DCFC.");
    println!("  A 220-mile range EV cannot complete Wyoming today without careful planning.");
    println!("  I2.0 standard (50-mile spacing) eliminates this completely.");
    println!();
    println!("  Freight Tesla Semi (480mi range, 1MW Megacharger):");
    let semi = evs
        .iter()
        .find(|e| e.charge_rate_kw >= 900.0)
        .cloned()
        .unwrap_or_else(route_sim::tesla_semi);
    let a2 = analyze_ev_charging(&ny_chi, &semi, 1000.0); // 1MW freight charger
    println!(
        "    NY→CHI: {} charging stops, {:.0} min total charge time",
        a2.stops_i20, a2.charge_minutes_i20
    );
    println!(
        "    {} at relay hubs (driver swap + charge simultaneously)",
        a2.overnight_note
    );
}
