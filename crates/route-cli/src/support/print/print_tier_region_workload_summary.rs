//! Helper `print_tier_region_workload_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_region_workload_summary(
    tier: &str,
    requested_regions: usize,
    output: &Path,
    repairs: &Path,
    rows: &[TierRegionWorkloadRow],
) {
    let mut route_counts = vec![0usize; requested_regions];
    let mut weight_counts = vec![0i32; requested_regions];
    for row in rows {
        route_counts[row.region_id] += 1;
        weight_counts[row.region_id] += row.route_weight;
    }
    println!(
        "  wrote {} {tier} route workload rows to {}",
        rows.len(),
        output.display()
    );
    for region in 0..requested_regions {
        println!(
            "  region {region}: {} routes, {} weighted miles",
            route_counts[region], weight_counts[region]
        );
    }
    if let Some(status) = rows.first().map(|row| row.component_status.as_str()) {
        println!("  graph status: {status}");
    }
    println!("  wrote repair docket: {}", repairs.display());
}
