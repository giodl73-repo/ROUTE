//! `T2BundleReadinessDisposition` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    targets: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-readiness-disposition");
    let target_rows = load_t2_bundle_overlay_repair_targets(&targets)
        .with_context(|| format!("loading {}", targets.display()))?;
    let rows = t2_bundle_readiness_disposition_rows(&target_rows);
    write_t2_bundle_readiness_disposition(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_readiness_disposition_summary(&output, &rows);

    if gate {
        let failures = t2_bundle_readiness_disposition_gate_failures(&rows, &target_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle readiness disposition gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle readiness disposition gate failed");
        }
        println!();
        println!("T2 bundle readiness disposition gate: PASS");
    }
        
    Ok(())
}

