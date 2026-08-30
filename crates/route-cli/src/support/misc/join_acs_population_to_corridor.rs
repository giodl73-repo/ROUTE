//! Helper `join_acs_population_to_corridor`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_acs_population_to_corridor(
    manifest: &route_data::Manifest,
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    verbose: bool,
) {
    if let Some(counties) = load_acs_counties_for_scoring(manifest) {
        let (pop, rural_pop) = route_network::corridor_pop_within_50mi(graph, route_id, &counties);
        if pop > 0 {
            let rural_share = rural_pop as f32 / pop as f32;
            attrs.pop_within_50mi = Some(pop);
            attrs.rural_pop_within_50mi = Some(rural_pop);
            attrs.pct_rural_in_buffer = Some(rural_share);

            // C3: compute median income relative to national median
            // Use population-weighted median HHI across counties in the 50-mile buffer
            let near_counties: Vec<_> =
                route_network::counties_within_50mi(graph, route_id, &counties);
            if !near_counties.is_empty() {
                let total_pop_w: u64 = near_counties.iter().map(|c| c.population).sum();
                if total_pop_w > 0 {
                    let weighted_hhi: f64 = near_counties
                        .iter()
                        .map(|c| c.median_hhi as f64 * c.population as f64)
                        .sum::<f64>()
                        / total_pop_w as f64;
                    if weighted_hhi > 0.0 {
                        let relative =
                            (weighted_hhi / route_data::NATIONAL_MEDIAN_HHI_2022 as f64) as f32;
                        attrs.gdp_per_capita_relative = Some(relative);
                    }
                }
            }

            if verbose {
                println!(
                    "  C1 population (50mi buffer): {:>12} ({:.1}% rural)",
                    pop,
                    rural_share * 100.0
                );
            }
        } else if verbose {
            println!("  C1: no counties found within 50mi corridor buffer for {route_id}");
        }
    }
    // If counties is None (files not cached), silently leave attrs as-is (None = not scored)
}
