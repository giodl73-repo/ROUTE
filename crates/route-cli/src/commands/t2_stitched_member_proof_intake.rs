//! `T2StitchedMemberProofIntake` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    source_access: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-proof-intake");
    let access_rows = load_t2_stitched_member_source_access_policy(&source_access)
        .with_context(|| format!("loading {}", source_access.display()))?;
    let rows = t2_stitched_member_proof_intake_rows(&access_rows);
    write_t2_stitched_member_proof_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_proof_intake_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_proof_intake_gate_failures(&rows, &access_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member proof intake gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member proof intake gate failed");
        }
        println!("T2 stitched member proof intake gate: PASS");
    }
        
    Ok(())
}

