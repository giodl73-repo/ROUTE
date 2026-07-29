//! `T2StitchedMemberDecisionDocket` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    scope_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-decision-docket");
    let scope_rows = load_t2_stitched_member_candidate_scope_review(&scope_review)
        .with_context(|| format!("loading {}", scope_review.display()))?;
    let rows = t2_stitched_member_decision_docket_rows(&scope_rows);
    write_t2_stitched_member_decision_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_decision_docket_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_decision_docket_gate_failures(&rows, &scope_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member decision docket gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member decision docket gate failed");
        }
        println!("T2 stitched member decision docket gate: PASS");
    }
        
    Ok(())
}

