//! `T2BundleOverlays` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    service_selection: PathBuf,
    bundles: PathBuf,
    game_overlays: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-overlays");
    let service_rows = load_t2_service_selection(&service_selection)
        .with_context(|| format!("loading {}", service_selection.display()))?;
    let bundle_rows = load_national_segment_bundles(&bundles)
        .with_context(|| format!("loading {}", bundles.display()))?;
    let overlay_rows = load_game_t2_service_overlays(&game_overlays)
        .with_context(|| format!("loading {}", game_overlays.display()))?;
    let rows = t2_bundle_overlay_rows(&service_rows, &bundle_rows, &overlay_rows);
    write_t2_bundle_overlays(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_overlay_summary(&output, &rows);

    if gate {
        let failures = t2_bundle_overlay_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle overlay gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle overlay gate failed");
        }
        println!();
        println!("T2 bundle overlay gate: PASS");
    }

    Ok(())
}
