//! `MapPublicationReadiness` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    map_atlas: PathBuf,
    backlog: PathBuf,
    scope_decision: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route map-publication-readiness");
            let atlas_rows = load_map_atlas(&map_atlas)
                .with_context(|| format!("loading map atlas {}", map_atlas.display()))?;
            let backlog_rows = load_optimizer_residual_blocker_backlog(&backlog)
                .with_context(|| format!("loading {}", backlog.display()))?;
            let scope_rows = load_map_publication_scope_decision(&scope_decision)
                .with_context(|| format!("loading {}", scope_decision.display()))?;
            let rows = map_publication_readiness_rows(
                &atlas_rows,
                &backlog_rows,
                &scope_rows,
                &scope_decision,
                &backlog,
            );
            write_map_publication_readiness(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_map_publication_readiness_summary(&output, &rows, details);
            if gate {
                let failures = map_publication_readiness_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("map publication readiness gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("map publication readiness gate failed");
                }
                println!();
                println!("map publication readiness gate: PASS");
            }
        
    Ok(())
}

