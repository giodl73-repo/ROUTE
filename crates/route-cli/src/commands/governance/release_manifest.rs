//! `ReleaseManifest` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    manifest: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_release_manifest(&manifest)
        .with_context(|| format!("loading release manifest {}", manifest.display()))?;
    print_release_manifest(&manifest, &rows, blockers, details);

    if gate {
        let failures = release_manifest_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("release manifest gate: FAIL");
            println!("  {} release rows lack complete contracts.", failures.len());
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("release manifest gate failed");
        }
        println!();
        println!("release manifest gate: PASS");
    }
        
    Ok(())
}

