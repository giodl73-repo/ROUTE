//! `T2BundleRepairQueue` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    candidates: PathBuf,
    blocker_closure: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-repair-queue");
    let candidate_rows = load_tier_candidate_columns(&candidates)
        .with_context(|| format!("loading {}", candidates.display()))?;
    let blocker_rows = load_t2_blocker_closure(&blocker_closure)
        .with_context(|| format!("loading {}", blocker_closure.display()))?;
    let rows = t2_bundle_repair_queue_rows(&candidate_rows, &blocker_rows);
    write_t2_bundle_repair_queue(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_repair_queue_summary(&output, &rows);

    if gate {
        let failures = t2_bundle_repair_queue_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle repair queue gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle repair queue gate failed");
        }
        println!();
        println!("T2 bundle repair queue gate: PASS");
    }

    Ok(())
}
