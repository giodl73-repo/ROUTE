//! `StopSlaSurface` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    output: PathBuf
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let csv = route_map::build_beck_stop_sla_csv();
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating {}", parent.display()))?;
            }
            std::fs::write(&output, csv)
                .with_context(|| format!("writing {}", output.display()))?;
            println!("route stop-sla-surface — wrote {}", output.display());
            println!("  source: Beck T1/T2 stops and lines; evidence_status=heuristic-planning");
        
    Ok(())
}

