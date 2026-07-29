//! `T1FetchTdotSmartway` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    output: PathBuf,
    timeout_seconds: u64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    fetch_tdot_smartway_events(&output, timeout_seconds).with_context(|| {
        format!("fetching TDOT SmartWay events to {}", output.display())
    })?;
    println!("route t1-fetch-tdot-smartway");
    println!("  wrote {}", output.display());
        
    Ok(())
}

