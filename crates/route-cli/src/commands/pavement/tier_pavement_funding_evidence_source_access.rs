//! `TierPavementFundingEvidenceSourceAccess` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    evidence_acquisition: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-funding-evidence-source-access");
    let acquisition_rows =
        load_tier_pavement_funding_evidence_acquisition(&evidence_acquisition)
            .with_context(|| format!("loading {}", evidence_acquisition.display()))?;
    let rows = tier_pavement_funding_evidence_source_access_rows(&acquisition_rows);
    write_tier_pavement_funding_evidence_source_access(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_funding_evidence_source_access_summary(&output, &rows);

    if gate {
        let failures =
            tier_pavement_funding_evidence_source_access_gate_failures(&rows, &acquisition_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement funding evidence source-access gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement funding evidence source-access gate failed");
        }
        println!();
        println!("Tier pavement funding evidence source-access gate: PASS");
    }

    Ok(())
}
