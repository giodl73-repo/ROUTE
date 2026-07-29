//! `TierPavementDocket` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    segments: PathBuf,
    standards: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-docket");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let segment_rows = load_tier_segment_candidates(&segments)
        .with_context(|| format!("loading {}", segments.display()))?;
    let standard_rows = load_pavement_standards(&standards)
        .with_context(|| format!("loading {}", standards.display()))?;
    let rows = tier_pavement_docket_rows(&graph, &segment_rows, &standard_rows);
    write_tier_pavement_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_docket_summary(&output, &rows, details);

    if gate {
        let failures = tier_pavement_docket_gate_failures(&rows, &segment_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement docket gate failed");
        }
        println!();
        println!("Tier pavement docket gate: PASS");
    }
        
    Ok(())
}

