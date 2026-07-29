//! `T1DiamondValidation` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    priority: Option<String>,
    docket: bool,
    with_access: bool,
    source_health: PathBuf,
    details: bool,
    gate_catalog: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let rows = load_t1_diamond_validation(&ledger).with_context(|| {
                format!("loading T1 diamond validation ledger {}", ledger.display())
            })?;
            if docket {
                let source_rows = if with_access {
                    Some(load_t1_source_health(&source_health).with_context(|| {
                        format!("loading T1 source health {}", source_health.display())
                    })?)
                } else {
                    None
                };
                print_t1_diamond_validation_docket(
                    &rows,
                    priority.as_deref(),
                    source_rows.as_deref(),
                    details,
                );
            } else {
                print_t1_diamond_validation(&rows, blockers, priority.as_deref(), details);
            }

            if gate_catalog {
                let failures = t1_diamond_validation_gate_failures(&rows);
                let missing = t1_diamond_validation_missing_sites(&rows);
                if !failures.is_empty() || !missing.is_empty() {
                    println!();
                    println!("T1/T1 diamond validation catalog gate: FAIL");
                    println!(
                        "  {} validation rows lack required catalog contracts.",
                        failures.len()
                    );
                    if !missing.is_empty() {
                        println!("  missing sites: {}", missing.join(", "));
                    }
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{}]: {}",
                            row.site_id, row.validation_status, row.blocking_gap
                        );
                    }
                    anyhow::bail!("T1/T1 diamond validation catalog gate failed");
                }
                println!();
                println!("T1/T1 diamond validation catalog gate: PASS");
            }
        
    Ok(())
}

