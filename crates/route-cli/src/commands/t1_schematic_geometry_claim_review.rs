//! `T1SchematicGeometryClaimReview` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    claim_review: PathBuf,
    design_review: PathBuf,
    policy_actions: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t1-schematic-geometry-claim-review");
            let claim_rows = load_optimizer_claim_review(&claim_review)
                .with_context(|| format!("loading {}", claim_review.display()))?;
            let design_rows = load_t1_design_review(&design_review)
                .with_context(|| format!("loading {}", design_review.display()))?;
            let policy_rows = load_t1_design_policy_actions(&policy_actions)
                .with_context(|| format!("loading {}", policy_actions.display()))?;
            let rows =
                t1_schematic_geometry_claim_review_rows(&claim_rows, &design_rows, &policy_rows);
            write_t1_schematic_geometry_claim_review(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t1_schematic_geometry_claim_review_summary(&output, &rows);

            if gate {
                let failures = t1_schematic_geometry_claim_review_gate_failures(
                    &rows,
                    &claim_rows,
                    &design_rows,
                    &policy_rows,
                );
                if !failures.is_empty() {
                    println!();
                    println!("T1 schematic geometry claim review gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T1 schematic geometry claim review gate failed");
                }
                println!();
                println!("T1 schematic geometry claim review gate: PASS");
            }
        
    Ok(())
}

