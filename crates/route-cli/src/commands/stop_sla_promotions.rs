//! `StopSlaPromotions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    input: PathBuf,
    output: PathBuf,
    include_ledger: bool,
    include_alternates: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route stop-sla-promotions");
    let file = std::fs::File::open(&input)
        .with_context(|| format!("opening SLA candidate docket {}", input.display()))?;
    let docket = parse_stop_sla_candidate_docket(file)
        .with_context(|| format!("parsing SLA candidate docket {}", input.display()))?;
    let promotions = stop_sla_promotion_rows(&docket, include_ledger, include_alternates);
    write_stop_sla_promotions(&output, &promotions)
        .with_context(|| format!("writing {}", output.display()))?;
    println!("  source docket: {}", input.display());
    println!("  promotion rows: {}", promotions.len());
    println!("  wrote promotion docket: {}", output.display());

    if gate {
        let refs = promotions.iter().collect::<Vec<_>>();
        let failures = stop_candidate_gate_failures(&refs);
        if failures.is_empty() {
            println!("stop SLA promotion gate: PASS");
        } else {
            println!("stop SLA promotion gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("stop SLA promotion gate failed");
        }
    }
        
    Ok(())
}

