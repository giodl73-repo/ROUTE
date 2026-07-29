//! `TierPavementFundingEvidenceContract` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    downgrade_exclusion_decision: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-contract");
    let decision_rows =
        load_tier_pavement_downgrade_exclusion_decision(&downgrade_exclusion_decision)
            .with_context(|| {
                format!("loading {}", downgrade_exclusion_decision.display())
            })?;
    let rows = tier_pavement_funding_evidence_contract_rows(&decision_rows);
    write_tier_pavement_funding_evidence_contract(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_contract_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_contract_gate_failures(&rows, &decision_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence contract gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence contract gate failed");
        }
        println!();
        println!("Tier pavement funding evidence contract gate: PASS");
    }
        
    Ok(())
}

