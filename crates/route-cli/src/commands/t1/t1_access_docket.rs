//! `T1AccessDocket` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    category: Option<String>,
    details: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_t1_source_health(&ledger)
        .with_context(|| format!("loading T1 source health {}", ledger.display()))?;
    print_t1_access_docket(&rows, category.as_deref(), details);
        
    Ok(())
}

