//! `T1SnapshotPlan` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    priority: Option<String>,
    details: bool,
    script: bool,
    gate_plan: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_snapshot_plan(&ledger)
        .with_context(|| format!("loading T1 snapshot plan {}", ledger.display()))?;
    if script {
        print_t1_snapshot_script(&rows, priority.as_deref());
    } else {
        print_t1_snapshot_plan(&rows, priority.as_deref(), details);
    }

    if gate_plan {
        let failures = t1_snapshot_plan_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T1/T1 snapshot plan gate: FAIL");
            println!("  {} snapshot rows lack executable plans.", failures.len());
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{}]: {}",
                    row.site_id, row.source_name, row.blocking_gap
                );
            }
            anyhow::bail!("T1/T1 snapshot plan gate failed");
        }
        println!();
        println!("T1/T1 snapshot plan gate: PASS");
    }
        
    Ok(())
}

