//! `StandardsProof` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    tier: Option<String>,
    family: Option<String>,
    details: bool,
    gate_blueprint: bool,
    gate_pressure: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_standards_proof_ledger(&ledger)
        .with_context(|| format!("loading standards proof ledger {}", ledger.display()))?;
    print_standards_proof(&rows, tier.as_deref(), family.as_deref(), details);

    if gate_pressure {
        let failures = standards_pressure_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Pressure proof-record gate: FAIL");
            println!(
                "  {} standards lack complete Milepost 4 proof records.",
                failures.len()
            );
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{} {}]: evidence={}, artifact={}, next={}",
                    row.standard_id,
                    row.tier,
                    row.standard_family,
                    row.evidence_level,
                    row.current_artifact,
                    row.next_command_or_test
                );
            }
            anyhow::bail!("standards pressure proof-record gate failed");
        }
        println!();
        println!("Pressure proof-record gate: PASS");
    }

    if gate_blueprint {
        let failures = standards_blueprint_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Blueprint gate: FAIL");
            println!(
                "  {} standards still have unresolved proof gaps.",
                failures.len()
            );
            println!("  First unresolved standards:");
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{} {}]: {}",
                    row.standard_id, row.tier, row.evidence_level, row.blocking_gap
                );
            }
            anyhow::bail!("standards proof gate failed");
        }
        println!();
        println!("Blueprint gate: PASS");
    }

    Ok(())
}
