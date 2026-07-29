//! `T2StitchedMemberSourceAccessPolicy` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    evidence_acquisition: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-stitched-member-source-access-policy");
            let acquisition_rows =
                load_t2_stitched_member_evidence_acquisition(&evidence_acquisition)
                    .with_context(|| format!("loading {}", evidence_acquisition.display()))?;
            let rows = t2_stitched_member_source_access_policy_rows(&acquisition_rows);
            write_t2_stitched_member_source_access_policy(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_stitched_member_source_access_policy_summary(&output, &rows);

            if gate {
                let failures =
                    t2_stitched_member_source_access_policy_gate_failures(&rows, &acquisition_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 stitched member source access policy gate: FAIL");
                    for failure in failures {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("t2 stitched member source access policy gate failed");
                }
                println!("T2 stitched member source access policy gate: PASS");
            }
        
    Ok(())
}

