//! `T1FetchMdotMidrive` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    output: PathBuf
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    fetch_mdot_midrive_events(&output).with_context(|| {
        format!("fetching MDOT Mi Drive events to {}", output.display())
    })?;
    println!("route t1-fetch-mdot-midrive");
    println!("  wrote {}", output.display());
        
    Ok(())
}

