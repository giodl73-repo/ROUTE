//! `OptimizerManifest` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    manifest: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route optimizer-manifest");
    let rows = load_tier_optimizer_runs(&manifest)
        .with_context(|| format!("loading {}", manifest.display()))?;
    print_optimizer_manifest_summary(&manifest, &rows);

    if gate {
        let failures = optimizer_manifest_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("optimizer manifest gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("optimizer manifest gate failed");
        }
        println!();
        println!("optimizer manifest gate: PASS");
    }
        
    Ok(())
}

