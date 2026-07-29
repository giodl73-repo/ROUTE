//! `Sim` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    mode: SimMode
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


    match mode {
        SimMode::List => {
            println!("Available scenarios:");
            for name in route_sim::scenarios::available_scenarios() {
                let status = route_sim::scenarios::load_scenario(name)
                    .and_then(|toml| toml::from_str::<route_sim::Scenario>(toml).ok())
                    .map(|scenario| {
                        if route_sim::scenario_validation_warnings(&scenario).is_empty() {
                            "ready"
                        } else {
                            "needs edge bindings"
                        }
                    })
                    .unwrap_or("parse error");
                println!("  {name:<20} {status}");
            }
            println!("\nUsage: route sim scenario <name> [--intervention]");
            println!("       route sim bind --route I80 --lat 39.32 --lon -120.33");
            println!("       route sim chaos [--iterations N] [--seed S] [--t1-only]");
        }

        SimMode::Bind {
            route,
            lat,
            lon,
            radius,
            top,
        } => {
            let norm = normalise_designation(&route);
            println!(
                "route sim bind --route {norm} --lat {lat:.5} --lon {lon:.5} --radius {radius:.1}"
            );

            let manifest =
                route_data::Manifest::load(&manifest_path).with_context(|| {
                    format!("loading manifest from {}", manifest_path.display())
                })?;
            let graph = load_graph(&manifest)?;

            let candidates = scenario_edge_candidates(&graph, &norm, lat, lon, radius, top);
            if candidates.is_empty() {
                println!("  no {norm} edges found within {radius:.1} miles");
                println!(
                    "  tip: increase --radius or verify the route exists in the graph"
                );
            } else {
                println!(
                    "  {} candidate edge IDs for scenario affected_edges:",
                    candidates.len()
                );
                println!(
                    "  {:>12}  {:>7}  {:>7}  {:>8}  {:>5}  {:>8}  midpoint",
                    "edge_id", "dist", "length", "aadt", "lanes", "state"
                );
                for c in candidates {
                    println!(
                        "  {:>12}  {:>6.2}m  {:>6.2}m  {:>8}  {:>5}  {:>8}  {:.5},{:.5}",
                        c.edge_id,
                        c.distance_miles,
                        c.length_miles,
                        c.aadt
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "-".to_string()),
                        c.lanes
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "-".to_string()),
                        c.state,
                        c.mid_lat,
                        c.mid_lon
                    );
                }
            }
        }

        SimMode::Scenario { name, intervention } => {
            println!(
                "route sim scenario {name}{}",
                if intervention { " --intervention" } else { "" }
            );

            let toml_str = route_sim::scenarios::load_scenario(&name)
                .ok_or_else(|| anyhow::anyhow!(
                    "Unknown scenario '{}'. Run `route sim list` to see available scenarios.", name
                ))?;

            let mut scenario: route_sim::Scenario = toml::from_str(toml_str)
                .with_context(|| format!("parsing scenario {name}"))?;

            if !intervention {
                scenario.intervention = None;
            }

            let warnings = route_sim::scenario_validation_warnings(&scenario);
            if !warnings.is_empty() {
                println!("  scenario warnings:");
                for warning in warnings {
                    println!("  - {warning}");
                }
                println!();
            }

            let manifest =
                route_data::Manifest::load(&manifest_path).with_context(|| {
                    format!("loading manifest from {}", manifest_path.display())
                })?;
            let graph = load_graph(&manifest)?;

            // Use AADT-based demand proxy (FAF5 not yet joined)
            let demand = build_demand_from_graph(&graph);
            println!("  demand pairs: {}", demand.len());

            println!("  running Wardrop equilibrium (Frank-Wolfe)…");
            let result = route_sim::run_scenario(&graph, &demand, &scenario);

            print_scenario_result(&result);
        }

        SimMode::Chaos {
            iterations,
            seed,
            t1_only,
        } => {
            println!(
                "route sim chaos --iterations {iterations} --seed {seed}{}",
                if t1_only { " --t1-only" } else { "" }
            );

            let manifest =
                route_data::Manifest::load(&manifest_path).with_context(|| {
                    format!("loading manifest from {}", manifest_path.display())
                })?;
            let graph = load_graph(&manifest)?;
            let demand = build_demand_from_graph(&graph);

            let config = route_sim::ChaosConfig {
                seed,
                iterations,
                t1_only,
                ..Default::default()
            };

            println!("  running {iterations} chaos iterations…");
            let result = route_sim::run_chaos(&graph, &demand, &config);
            print_chaos_result(&result);
        }
    }
    Ok(())
}
