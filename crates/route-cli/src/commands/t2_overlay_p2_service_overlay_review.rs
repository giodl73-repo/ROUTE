//! `T2OverlayP2ServiceOverlayReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    action_docket: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-overlay-p2-service-overlay-review");
            let action_rows = load_t2_overlay_optimizer_action_docket(&action_docket)
                .with_context(|| format!("loading {}", action_docket.display()))?;
            let rows = t2_overlay_p2_service_overlay_review_rows(&action_rows);
            write_t2_overlay_p2_service_overlay_review(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_overlay_p2_service_overlay_review_summary(&output, &rows);

            if gate {
                let failures =
                    t2_overlay_p2_service_overlay_review_gate_failures(&rows, &action_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 overlay P2 service overlay review gate: FAIL");
                    for failure in failures {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("t2 overlay P2 service overlay review gate failed");
                }
                println!("T2 overlay P2 service overlay review gate: PASS");
            }
        
    Ok(())
}

