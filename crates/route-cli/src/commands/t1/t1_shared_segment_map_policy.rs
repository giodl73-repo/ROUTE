//! `T1SharedSegmentMapPolicy` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    schematic_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-shared-segment-map-policy");
    let schematic_rows = load_t1_schematic_geometry_claim_review(&schematic_review)
        .with_context(|| format!("loading {}", schematic_review.display()))?;
    let rows = t1_shared_segment_map_policy_rows(&schematic_rows);
    write_t1_shared_segment_map_policy(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_shared_segment_map_policy_summary(&output, &rows);

    if gate {
        let failures = t1_shared_segment_map_policy_gate_failures(&rows, &schematic_rows);
        if !failures.is_empty() {
            println!();
            println!("T1 shared segment map policy gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 shared segment map policy gate failed");
        }
        println!();
        println!("T1 shared segment map policy gate: PASS");
    }
        
    Ok(())
}

