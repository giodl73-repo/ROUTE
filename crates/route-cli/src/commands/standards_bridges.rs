//! `StandardsBridges` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    tier_table: PathBuf,
    tier: String,
    details: bool,
    gate_l1: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let routes = load_tier_routes(&tier_table, &tier)
                .with_context(|| format!("loading tier table {}", tier_table.display()))?;
            let nbi = load_nbi_bridges();
            print_bridge_standard_coverage(&tier, &routes, &nbi, details);
            if gate_l1 {
                let missing = bridge_standard_missing_routes(&routes, &nbi);
                if routes.is_empty() || nbi.is_empty() || !missing.is_empty() {
                    println!();
                    println!("Bridge standards L1 gate: FAIL");
                    if routes.is_empty() {
                        println!("  no routes found for tier {tier}");
                    }
                    if nbi.is_empty() {
                        println!("  no cached NBI bridge summary rows loaded");
                    }
                    if !missing.is_empty() {
                        println!("  missing NBI coverage: {}", missing.join(", "));
                    }
                    anyhow::bail!("bridge standards L1 gate failed");
                }
                println!();
                println!("Bridge standards L1 gate: PASS");
            }
        
    Ok(())
}

