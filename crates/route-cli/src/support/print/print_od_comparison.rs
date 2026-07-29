//! Helper `print_od_comparison`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_od_comparison(cmp: &route_sim::OdComparison) {
    let sg = &cmp.solo_gp;
    let sm = &cmp.solo_managed;
    let tm = &cmp.team_managed;
    let rg = &cmp.relay_gp;
    let rm = &cmp.relay_managed;
    let net = route_sim::RelayNetwork::for_corridor_miles(sg.free_flow_hours);

    println!(
        "╔══════════════════════════════════════════════════════════════════════════════════╗"
    );
    println!("║  {}  ║", pad_center(&cmp.corridor_name, 80));
    println!(
        "║  Free-flow: {:.1}h ({:.1} days)  |  Relay stations: {}  |  Station cost: ${:.0}M ea  ║",
        sg.free_flow_hours,
        sg.free_flow_hours / 24.0,
        net.stations,
        net.station_cost_m
    );
    println!("╠══════════════════╦══════════════╦══════════════╦══════════════╦══════════════╣");
    println!("║  Metric          ║ Solo / GP    ║ Solo / I2.0  ║ Team / I2.0  ║Relay / I2.0  ║");
    println!("╠══════════════════╬══════════════╬══════════════╬══════════════╬══════════════╣");

    let row = |label: &str, f: fn(&route_sim::TransitDistribution) -> f64| {
        println!(
            "║  {:<16}║  {:>8.1}h   ║  {:>8.1}h   ║  {:>8.1}h   ║  {:>8.1}h   ║",
            label,
            f(sg),
            f(sm),
            f(tm),
            f(rm)
        );
    };
    row("Mean", |d| d.mean_hours);
    row("p50", |d| d.p50_hours);
    row("p75", |d| d.p75_hours);
    row("p90", |d| d.p90_hours);
    row("p95 commit wdw", |d| d.p95_hours);
    row("p99 worst-case", |d| d.p99_hours);

    println!("╠══════════════════╬══════════════╬══════════════╬══════════════╬══════════════╣");
    println!(
        "║  PTI             ║  {:>9.3}  ║  {:>9.3}  ║  {:>9.3}  ║  {:>9.3}  ║",
        sg.pti, sm.pti, tm.pti, rm.pti
    );
    println!(
        "║  < 48h trips     ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║",
        sg.pct_under_48h, sm.pct_under_48h, tm.pct_under_48h, rm.pct_under_48h
    );
    println!(
        "║  < 72h trips     ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║",
        pct_under(sg, 72.0),
        pct_under(sm, 72.0),
        pct_under(tm, 72.0),
        pct_under(rm, 72.0)
    );
    println!(
        "║  SLA window      ║  {:>7.1}d   ║  {:>7.1}d   ║  {:>7.1}d   ║  {:>7.1}d   ║",
        sg.commitment_window_days,
        sm.commitment_window_days,
        tm.commitment_window_days,
        rm.commitment_window_days
    );
    println!("╚══════════════════╩══════════════╩══════════════╩══════════════╩══════════════╝");

    // Verdict per scenario
    println!();
    let verdict = |label: &str, d: &route_sim::TransitDistribution| {
        let sla = d.p95_hours;
        let days = sla / 24.0;
        let icon = if sla <= 48.0 {
            "✓ 48h SLA ACHIEVABLE".to_string()
        } else if sla <= 72.0 {
            format!("✓ {:.1}d ({:.0}h) — tight 3-day SLA", days, sla)
        } else {
            format!("→ {:.1}d ({:.0}h) commitment window", days, sla)
        };
        println!("  {:20}  {}", label, icon);
    };
    verdict("Solo / GP lanes:", sg);
    verdict("Solo / Managed:", sm);
    verdict("Team / Managed:", tm);
    verdict("Relay / Managed:", rm);
    verdict("Relay / GP lanes:", rg);

    // Relay network economics
    println!();
    println!(
        "  Relay network: {} stations × ${:.0}M = ${:.0}M total capex",
        net.stations, net.station_cost_m, net.total_capex_m
    );
    println!(
        "  Avg driver leg: {:.0} miles / {:.1}h — home base return same day",
        net.avg_leg_miles, net.avg_leg_hours
    );
    println!(
        "  vs. $253B I2.0 portfolio = {:.2}% of total program cost",
        net.total_capex_m / 253_000.0 * 100.0
    );
}

