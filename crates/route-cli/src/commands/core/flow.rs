//! `Flow` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    designation: String
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let norm = normalise_designation(&designation);
    println!("route flow {norm}");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;

    let result = route_network::corridor_max_flow(&graph, &norm)
        .ok_or_else(|| anyhow::anyhow!("Route '{}' not found in graph", norm))?;

    let lane_note = if result.has_lane_data {
        ""
    } else {
        "† (default 2-lane assumed — no HPMS data)"
    };
    println!("\n┌──────────────────────────────────────────────────────┐");
    println!("│  {} — Corridor Flow Capacity", norm);
    println!("├──────────────────────────────────────────────────────┤");
    println!(
        "│  Binding throughput (min segment):  {:>10.0} vpd  │",
        result.max_flow_vpd
    );
    println!(
        "│  Mean corridor capacity:            {:>10.0} vpd  │",
        result.mean_capacity_vpd
    );
    println!(
        "│  Segments analyzed:                 {:>10}      │",
        result.augmenting_paths
    );
    println!(
        "│  Bottleneck count:                  {:>10}      │",
        result.bottleneck_edges.len()
    );
    println!("└──────────────────────────────────────────────────────┘");

    for (i, &ei) in result.bottleneck_edges.iter().enumerate() {
        let edge = &graph.graph[ei];
        let cap = result.bottleneck_capacities.get(i).cloned().unwrap_or(0.0);
        let gain = result.lane_addition_gain.get(i).cloned().unwrap_or(0.0);
        let lanes = edge.lane_count.map(|l| l.to_string()).unwrap_or("?".into());
        println!("\n  Binding bottleneck:");
        println!(
            "    Route: {}  State: {}  Lanes: {}",
            edge.route_id,
            if edge.state.is_empty() {
                "—"
            } else {
                &edge.state
            },
            lanes
        );
        println!(
            "    Capacity: {:.0} vpd  |  +1 lane adds: +{:.0} vpd",
            cap, gain
        );
    }
    println!("\n  {} vpd = vehicles per day", "vpd");
    if !lane_note.is_empty() {
        println!("  {lane_note}");
        println!("  Run `route fetch-hpms --states <state>` then `route build` for real lane counts.");
    }
        
    Ok(())
}

