//! `T1SourceHealth` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate_ingestion: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_source_health(&ledger)
        .with_context(|| format!("loading T1 source health {}", ledger.display()))?;
    print_t1_source_health(&rows, blockers, details);
    if gate_ingestion {
        let blocked = t1_source_health_blockers(&rows);
        if !blocked.is_empty() {
            anyhow::bail!(
                "{} T1 source-health blocker(s) remain; run `route t1-source-health --blockers --details`",
                blocked.len()
            );
        }
    }
        
    Ok(())
}

