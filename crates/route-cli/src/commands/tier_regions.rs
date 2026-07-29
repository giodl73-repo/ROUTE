//! `TierRegions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    tier_table: PathBuf,
    tier: String,
    regions: usize,
    graph: TierRegionGraphArg,
    output: PathBuf,
    repairs: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route tier-regions --tier {tier} --regions {regions}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let highway_graph = load_graph(&manifest)?;
            let t1_routes = load_tier_routes(&tier_table, "T1")
                .with_context(|| format!("loading T1 routes from {}", tier_table.display()))?;
            let tier_routes = load_tier_routes(&tier_table, &tier)
                .with_context(|| format!("loading {tier} routes from {}", tier_table.display()))?;
            let rows = tier_region_workload_rows(
                &highway_graph,
                &tier,
                &tier_routes,
                &t1_routes,
                graph.service_graph_kind(),
                regions,
            )?;
            write_tier_region_workloads(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            let repair_rows = tier_region_repair_rows(&rows);
            write_tier_region_repairs(&repairs, &repair_rows)
                .with_context(|| format!("writing {}", repairs.display()))?;
            print_tier_region_workload_summary(&tier, regions, &output, &repairs, &rows);

            if gate {
                let failures = tier_region_gate_failures(&rows, regions);
                if !failures.is_empty() {
                    println!();
                    println!("tier region gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("tier region gate failed");
                }
                println!();
                println!("tier region gate: PASS");
            }
        
    Ok(())
}

