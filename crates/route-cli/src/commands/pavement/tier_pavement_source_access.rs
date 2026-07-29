//! `TierPavementSourceAccess` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    acquisition_docket: PathBuf,
    output: PathBuf,
    priority: String,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-source-access");
    let docket_rows = load_tier_pavement_acquisition_docket(&acquisition_docket)
        .with_context(|| format!("loading {}", acquisition_docket.display()))?;
    let rows = tier_pavement_source_access_rows(&docket_rows, &priority);
    write_tier_pavement_source_access(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_source_access_summary(&output, &rows, &priority);

    if gate {
        let failures =
            tier_pavement_source_access_gate_failures(&rows, &docket_rows, &priority);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement source access gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement source access gate failed");
        }
        println!();
        println!("Tier pavement source access gate: PASS");
    }
        
    Ok(())
}

