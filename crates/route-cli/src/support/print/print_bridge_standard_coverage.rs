//! Helper `print_bridge_standard_coverage`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_bridge_standard_coverage(
    tier: &str,
    routes: &[String],
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
    details: bool,
) {
    let covered = routes
        .iter()
        .filter(|route| nbi.contains_key(*route))
        .count();
    let total_bridges: u32 = routes
        .iter()
        .filter_map(|route| nbi.get(route))
        .map(|row| row.bridge_count)
        .sum();
    let total_poor: f32 = routes
        .iter()
        .filter_map(|route| nbi.get(route))
        .map(|row| row.pct_bridges_poor * row.bridge_count as f32)
        .sum();
    let poor_pct = if total_bridges > 0 {
        total_poor / total_bridges as f32
    } else {
        0.0
    };
    let missing = bridge_standard_missing_routes(routes, nbi);

    println!("route standards-bridges");
    println!("  tier: {tier}");
    println!("  routes: {}", routes.len());
    println!("  routes with NBI coverage: {covered}");
    println!("  total bridges: {total_bridges}");
    println!("  poor/critical bridge share: {:.2}%", poor_pct * 100.0);
    println!("  missing coverage: {}", missing.len());
    println!("  note: clearance and load-posting joins remain separate source gaps");

    if details {
        println!();
        println!(
            "{:<10} {:>8} {:>9} {:>10}",
            "Route", "Bridges", "Poor %", "Mean Year"
        );
        println!("{}", "-".repeat(44));
        for route in routes {
            if let Some(row) = nbi.get(route) {
                println!(
                    "{:<10} {:>8} {:>8.2}% {:>10.0}",
                    route,
                    row.bridge_count,
                    row.pct_bridges_poor * 100.0,
                    row.mean_year_built
                );
            } else {
                println!("{:<10} {:>8} {:>9} {:>10}", route, "-", "-", "-");
            }
        }
    }
}
