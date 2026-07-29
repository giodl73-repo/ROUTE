//! `StandardsInventory` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    standards_ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool,
    gate_planned: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_standards_inventory(&ledger)
        .with_context(|| format!("loading standards inventory {}", ledger.display()))?;
    print_standards_inventory(&rows, blockers, details);

    let standards_rows = if gate_planned {
        Some(
            load_standards_proof_ledger(&standards_ledger).with_context(|| {
                format!(
                    "loading standards proof ledger {}",
                    standards_ledger.display()
                )
            })?,
        )
    } else {
        None
    };

    if gate {
        let failures = standards_inventory_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Standards inventory gate: FAIL");
            println!(
                "  {} inventory rows lack L1 source contracts.",
                failures.len()
            );
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{}]: {}",
                    row.standard_id, row.source_status, row.blocking_gap
                );
            }
            anyhow::bail!("standards inventory gate failed");
        }
        println!();
        println!("Standards inventory gate: PASS");
    }
    if let Some(standards_rows) = standards_rows.as_ref() {
        let missing = planned_standard_inventory_missing(standards_rows, &rows);
        if !missing.is_empty() {
            println!();
            println!("Planned standard inventory gate: FAIL");
            println!("  {} Planned standards lack inventory rows.", missing.len());
            for row in missing.iter().take(10) {
                println!(
                    "  - {} [{}]: {}",
                    row.standard_id, row.standard_family, row.blocking_gap
                );
            }
            anyhow::bail!("planned standard inventory gate failed");
        }
        println!();
        println!("Planned standard inventory gate: PASS");
    }
        
    Ok(())
}

