//! `TierPavementAcquisitionDocket` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    acquisition_plan: PathBuf,
    output: PathBuf,
    priority: Option<String>,
    script: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-acquisition-docket");
    let plan_rows = load_tier_pavement_acquisition_plan(&acquisition_plan)
        .with_context(|| format!("loading {}", acquisition_plan.display()))?;
    let rows = tier_pavement_acquisition_docket_rows(&plan_rows);
    write_tier_pavement_acquisition_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_acquisition_docket_summary(
        &output,
        &rows,
        priority.as_deref(),
        script,
    );

    if gate {
        let failures = tier_pavement_acquisition_docket_gate_failures(&rows, &plan_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement acquisition docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement acquisition docket gate failed");
        }
        println!();
        println!("Tier pavement acquisition docket gate: PASS");
    }
        
    Ok(())
}

