//! `T2OverlayP1StructuralReadinessReview` command handler extracted from main.
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

    println!("route t2-overlay-p1-structural-readiness-review");
    let action_rows = load_t2_overlay_optimizer_action_docket(&action_docket)
        .with_context(|| format!("loading {}", action_docket.display()))?;
    let rows = t2_overlay_p1_structural_readiness_review_rows(&action_rows);
    write_t2_overlay_p1_structural_readiness_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_overlay_p1_structural_readiness_review_summary(&output, &rows);

    if gate {
        let failures =
            t2_overlay_p1_structural_readiness_review_gate_failures(&rows, &action_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 overlay P1 structural readiness review gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 overlay P1 structural readiness review gate failed");
        }
        println!("T2 overlay P1 structural readiness review gate: PASS");
    }
        
    Ok(())
}

