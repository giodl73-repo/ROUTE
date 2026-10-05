//! `T1SharedSegmentPolicyAcceptance` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, policy: PathBuf, output: PathBuf, gate: bool) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-shared-segment-policy-acceptance");
    let policy_rows = load_t1_shared_segment_map_policy(&policy)
        .with_context(|| format!("loading {}", policy.display()))?;
    let rows = t1_shared_segment_policy_acceptance_rows(&policy_rows);
    write_t1_shared_segment_policy_acceptance(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_shared_segment_policy_acceptance_summary(&output, &rows);

    if gate {
        let failures = t1_shared_segment_policy_acceptance_gate_failures(&rows, &policy_rows);
        if !failures.is_empty() {
            println!();
            println!("T1 shared segment policy acceptance gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 shared segment policy acceptance gate failed");
        }
        println!();
        println!("T1 shared segment policy acceptance gate: PASS");
    }

    Ok(())
}
