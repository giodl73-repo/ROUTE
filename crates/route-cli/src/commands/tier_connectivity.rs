//! `TierConnectivity` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    tier_table: PathBuf,
    exceptions: PathBuf,
    tier: String,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-connectivity --tier {tier}");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let t1_routes = load_tier_routes(&tier_table, "T1")
        .with_context(|| format!("loading T1 routes from {}", tier_table.display()))?;
    let tier_routes = load_tier_routes(&tier_table, &tier)
        .with_context(|| format!("loading {tier} routes from {}", tier_table.display()))?;
    let rows = route_network::analyze_tier_connectivity(&graph, &tier_routes, &t1_routes);
    let exception_rows = load_endpoint_exceptions(&exceptions)
        .with_context(|| format!("loading endpoint exceptions {}", exceptions.display()))?;
    print_tier_connectivity(&tier, &rows, &exception_rows, details);

    if gate {
        let failures =
            tier_connectivity_gate_failures_with_exceptions(&rows, &exception_rows, &tier);
        if !failures.is_empty() {
            println!();
            println!("{tier} connectivity gate: FAIL");
            println!(
                "  {} routes remain blocked after endpoint exception review.",
                failures.len()
            );
            for failure in failures.iter().take(12) {
                println!(
                    "  - {}: {} T1 nodes, {} T1 trunks, {:.0} mi ({}) — {}",
                    failure.row.route,
                    failure.row.t1_node_count,
                    failure.row.t1_routes.len(),
                    failure.row.route_miles,
                    failure.row.classification.as_str(),
                    failure.reason
                );
            }
            anyhow::bail!("{tier} connectivity gate failed");
        }
        println!();
        println!("{tier} connectivity gate: PASS");
    }
        
    Ok(())
}

