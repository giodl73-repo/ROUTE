//! `T2StitchedMemberSplitPlan` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    decision_docket: PathBuf,
    segment_candidates: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-split-plan");
    let decision_rows = load_t2_stitched_member_decision_docket(&decision_docket)
        .with_context(|| format!("loading {}", decision_docket.display()))?;
    let candidate_rows = load_tier_segment_candidates(&segment_candidates)
        .with_context(|| format!("loading {}", segment_candidates.display()))?;
    let rows = t2_stitched_member_split_plan_rows(&decision_rows, &candidate_rows);
    write_t2_stitched_member_split_plan(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_split_plan_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_split_plan_gate_failures(&rows, &decision_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member split plan gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member split plan gate failed");
        }
        println!("T2 stitched member split plan gate: PASS");
    }
        
    Ok(())
}

