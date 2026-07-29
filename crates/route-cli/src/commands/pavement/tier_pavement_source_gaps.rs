//! `TierPavementSourceGaps` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    docket: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-source-gaps");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let docket_rows = load_tier_pavement_docket(&docket)
        .with_context(|| format!("loading {}", docket.display()))?;
    let rows = tier_pavement_source_gap_rows(Some(&graph), &docket_rows);
    write_tier_pavement_source_gaps(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_source_gap_summary(&output, &rows, details);

    if gate {
        let failures = tier_pavement_source_gap_gate_failures(&rows, &docket_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement source-gap gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement source-gap gate failed");
        }
        println!();
        println!("Tier pavement source-gap gate: PASS");
    }
        
    Ok(())
}

