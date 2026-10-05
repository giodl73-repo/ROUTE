//! `T2StitchedMemberEvidenceContract` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    selection_docket: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-evidence-contract");
    let selection_rows = load_t2_stitched_member_selection_docket(&selection_docket)
        .with_context(|| format!("loading {}", selection_docket.display()))?;
    let rows = t2_stitched_member_evidence_contract_rows(&selection_rows);
    write_t2_stitched_member_evidence_contract(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_evidence_contract_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_evidence_contract_gate_failures(&rows, &selection_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member evidence contract gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member evidence contract gate failed");
        }
        println!("T2 stitched member evidence contract gate: PASS");
    }

    Ok(())
}
