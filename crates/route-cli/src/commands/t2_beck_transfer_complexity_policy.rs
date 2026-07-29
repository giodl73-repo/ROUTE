//! `T2BeckTransferComplexityPolicy` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    transfer_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-beck-transfer-complexity-policy");
            let review_rows = load_t2_beck_transfer_complexity_review(&transfer_review)
                .with_context(|| format!("loading {}", transfer_review.display()))?;
            let rows = t2_beck_transfer_complexity_policy_rows(&review_rows);
            write_t2_beck_transfer_complexity_policy(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_beck_transfer_complexity_policy_summary(&output, &rows);

            if gate {
                let failures =
                    t2_beck_transfer_complexity_policy_gate_failures(&rows, &review_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 Beck transfer complexity policy gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T2 Beck transfer complexity policy gate failed");
                }
                println!();
                println!("T2 Beck transfer complexity policy gate: PASS");
            }
        
    Ok(())
}

