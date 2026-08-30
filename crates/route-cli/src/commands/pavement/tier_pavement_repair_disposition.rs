//! `TierPavementRepairDisposition` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    repair_debt_review: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-repair-disposition");
    let repair_rows = load_tier_pavement_repair_debt_review(&repair_debt_review)
        .with_context(|| format!("loading {}", repair_debt_review.display()))?;
    let rows = tier_pavement_repair_disposition_rows(&repair_rows);
    write_tier_pavement_repair_disposition(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_repair_disposition_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_repair_disposition_gate_failures(&rows, &repair_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement repair disposition gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement repair disposition gate failed");
        }
        println!();
        println!("Tier pavement repair disposition gate: PASS");
    }

    Ok(())
}
