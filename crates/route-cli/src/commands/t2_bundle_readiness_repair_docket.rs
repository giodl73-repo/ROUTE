//! `T2BundleReadinessRepairDocket` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    readiness: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-readiness-repair-docket");
    let readiness_rows = load_t2_bundle_readiness_disposition(&readiness)
        .with_context(|| format!("loading {}", readiness.display()))?;
    let rows = t2_bundle_readiness_repair_docket_rows(&readiness_rows);
    write_t2_bundle_readiness_repair_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_readiness_repair_docket_summary(&output, &rows);

    if gate {
        let failures =
            t2_bundle_readiness_repair_docket_gate_failures(&rows, &readiness_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle readiness repair docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle readiness repair docket gate failed");
        }
        println!();
        println!("T2 bundle readiness repair docket gate: PASS");
    }
        
    Ok(())
}

