//! `T2StitchedMemberProofSourceCapture` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    proof_intake: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-proof-source-capture");
    let intake_rows = load_t2_stitched_member_proof_intake(&proof_intake)
        .with_context(|| format!("loading {}", proof_intake.display()))?;
    let rows = t2_stitched_member_proof_source_capture_rows(&intake_rows);
    write_t2_stitched_member_proof_source_capture(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_proof_source_capture_summary(&output, &rows);

    if gate {
        let failures =
            t2_stitched_member_proof_source_capture_gate_failures(&rows, &intake_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member proof source capture gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member proof source capture gate failed");
        }
        println!("T2 stitched member proof source capture gate: PASS");
    }
        
    Ok(())
}

