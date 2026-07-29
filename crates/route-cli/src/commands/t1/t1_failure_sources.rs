//! `T1FailureSources` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    lookup_needed: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_failure_source_plan(&ledger)
        .with_context(|| format!("loading T1 failure source plan {}", ledger.display()))?;
    print_t1_failure_sources(&rows, lookup_needed);
        
    Ok(())
}

