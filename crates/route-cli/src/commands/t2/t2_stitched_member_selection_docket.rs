//! `T2StitchedMemberSelectionDocket` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    split_plan: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-selection-docket");
    let split_rows = load_t2_stitched_member_split_plan(&split_plan)
        .with_context(|| format!("loading {}", split_plan.display()))?;
    let rows = t2_stitched_member_selection_docket_rows(&split_rows);
    write_t2_stitched_member_selection_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_selection_docket_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_selection_docket_gate_failures(&rows, &split_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member selection docket gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member selection docket gate failed");
        }
        println!("T2 stitched member selection docket gate: PASS");
    }

    Ok(())
}
