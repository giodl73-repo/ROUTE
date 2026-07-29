//! Helper `print_tier_connectivity`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_connectivity(
    tier: &str,
    rows: &[route_network::TierConnectivityRow],
    exceptions: &[EndpointExceptionRow],
    details: bool,
) {
    let mut by_class = std::collections::BTreeMap::new();
    for row in rows {
        *by_class
            .entry(row.classification.as_str().to_string())
            .or_insert(0usize) += 1;
    }
    let failures = tier_connectivity_gate_failures_with_exceptions(rows, exceptions, tier);

    println!("  tier: {tier}");
    println!("  routes analyzed: {}", rows.len());
    println!("  node class mix: {}", format_count_map(&by_class));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<10} {:>7} {:>8} {:>7} {:<18} {:<28} T1 trunks",
        "Route", "Miles", "T1 nodes", "T1s", "Class", "Endpoint exception"
    );
    println!("{}", "-".repeat(124));
    for row in rows {
        println!(
            "{:<10} {:>7.0} {:>8} {:>7} {:<18} {:<28} {}",
            row.route,
            row.route_miles,
            row.t1_node_count,
            row.t1_routes.len(),
            row.classification.as_str(),
            truncate_for_table(
                &endpoint_exception_summary(exceptions, &row.route, tier),
                28
            ),
            if row.t1_routes.is_empty() {
                "-".to_string()
            } else {
                row.t1_routes.join(",")
            }
        );
        if details {
            for touch in &row.touch_nodes {
                println!(
                    "  node {:>8}: {:>8.3},{:>7.3} via {}",
                    touch.node_id,
                    touch.lon,
                    touch.lat,
                    touch.t1_routes.join(",")
                );
            }
        }
    }

    if !failures.is_empty() {
        println!();
        println!(
            "  interpretation: {} rows look like one-ended feeders, local spurs, or missing graph data.",
            failures.len()
        );
        println!(
            "  use these as demotion candidates, graph-contact fixes, or terminal-worthy exception records."
        );
    }
}

