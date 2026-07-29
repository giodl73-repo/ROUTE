//! `T2BeckLabelDensityPolicyAcceptance` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    policy: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-beck-label-density-policy-acceptance");
            let policy_rows = load_t2_beck_label_density_policy(&policy)
                .with_context(|| format!("loading {}", policy.display()))?;
            let rows = t2_beck_label_density_policy_acceptance_rows(&policy_rows);
            write_t2_beck_label_density_policy_acceptance(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_beck_label_density_policy_acceptance_summary(&output, &rows);

            if gate {
                let failures =
                    t2_beck_label_density_policy_acceptance_gate_failures(&rows, &policy_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 Beck label density policy acceptance gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T2 Beck label density policy acceptance gate failed");
                }
                println!();
                println!("T2 Beck label density policy acceptance gate: PASS");
            }
        
    Ok(())
}

