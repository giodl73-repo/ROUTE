//! `TierPavementAcquisitionPlan` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_gaps: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-acquisition-plan");
    let gap_rows = load_tier_pavement_source_gaps(&source_gaps)
        .with_context(|| format!("loading {}", source_gaps.display()))?;
    let rows = tier_pavement_acquisition_plan_rows(&gap_rows);
    write_tier_pavement_acquisition_plan(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_acquisition_plan_summary(&output, &rows, details);

    if gate {
        let failures = tier_pavement_acquisition_plan_gate_failures(&rows, &gap_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement acquisition plan gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement acquisition plan gate failed");
        }
        println!();
        println!("Tier pavement acquisition plan gate: PASS");
    }
        
    Ok(())
}

