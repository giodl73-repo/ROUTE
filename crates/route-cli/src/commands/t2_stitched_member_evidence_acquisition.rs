//! `T2StitchedMemberEvidenceAcquisition` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    evidence_contract: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-stitched-member-evidence-acquisition");
            let contract_rows = load_t2_stitched_member_evidence_contract(&evidence_contract)
                .with_context(|| format!("loading {}", evidence_contract.display()))?;
            let rows = t2_stitched_member_evidence_acquisition_rows(&contract_rows);
            write_t2_stitched_member_evidence_acquisition(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_stitched_member_evidence_acquisition_summary(&output, &rows);

            if gate {
                let failures =
                    t2_stitched_member_evidence_acquisition_gate_failures(&rows, &contract_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T2 stitched member evidence acquisition gate: FAIL");
                    for failure in failures {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("t2 stitched member evidence acquisition gate failed");
                }
                println!("T2 stitched member evidence acquisition gate: PASS");
            }
        
    Ok(())
}

