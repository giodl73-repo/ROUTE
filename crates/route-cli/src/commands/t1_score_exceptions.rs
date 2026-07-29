//! `T1ScoreExceptions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    review: PathBuf,
    exceptions: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let review_rows = load_t1_design_review(&review)
                .with_context(|| format!("loading T1 design review {}", review.display()))?;
            let exception_rows = load_t1_score_exceptions(&exceptions)
                .with_context(|| format!("loading T1 score exceptions {}", exceptions.display()))?;
            print_t1_score_exceptions(&review_rows, &exception_rows, details);
            if gate {
                let failures = t1_score_exception_gate_failures(&review_rows, &exception_rows);
                if failures.is_empty() {
                    println!("T1 score exception gate: PASS");
                } else {
                    println!("T1 score exception gate: FAIL");
                    for failure in failures.iter().take(10) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T1 score exception gate failed");
                }
            }
        
    Ok(())
}

