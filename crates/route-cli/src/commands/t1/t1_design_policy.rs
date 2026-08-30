//! `T1DesignPolicy` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    review: PathBuf,
    policy: PathBuf,
    details: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let review_rows = load_t1_design_review(&review)
        .with_context(|| format!("loading T1 design review {}", review.display()))?;
    let policy_rows = load_t1_design_policy_actions(&policy)
        .with_context(|| format!("loading T1 design policy {}", policy.display()))?;
    print_t1_design_policy(&review_rows, &policy_rows, details);
    if gate {
        let failures = t1_design_policy_gate_failures(&review_rows, &policy_rows);
        if failures.is_empty() {
            println!("T1 design policy gate: PASS");
        } else {
            println!("T1 design policy gate: FAIL");
            for failure in failures.iter().take(10) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 design policy gate failed");
        }
    }

    Ok(())
}
