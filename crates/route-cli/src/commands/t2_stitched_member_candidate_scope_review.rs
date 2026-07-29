//! `T2StitchedMemberCandidateScopeReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    handoff: PathBuf,
    segment_candidates: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-candidate-scope-review");
    let handoff_rows = load_t2_stitched_member_registry_handoff(&handoff)
        .with_context(|| format!("loading {}", handoff.display()))?;
    let candidate_rows = load_tier_segment_candidates(&segment_candidates)
        .with_context(|| format!("loading {}", segment_candidates.display()))?;
    let rows =
        t2_stitched_member_candidate_scope_review_rows(&handoff_rows, &candidate_rows);
    write_t2_stitched_member_candidate_scope_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_candidate_scope_review_summary(&output, &rows);

    if gate {
        let failures =
            t2_stitched_member_candidate_scope_review_gate_failures(&rows, &handoff_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member candidate scope review gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member candidate scope review gate failed");
        }
        println!("T2 stitched member candidate scope review gate: PASS");
    }
        
    Ok(())
}

