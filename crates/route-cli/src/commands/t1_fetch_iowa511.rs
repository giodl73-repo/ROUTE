//! `T1FetchIowa511` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    output: PathBuf
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            fetch_iowa511_events(&output)
                .with_context(|| format!("fetching Iowa 511 events to {}", output.display()))?;
            println!("route t1-fetch-iowa511");
            println!("  wrote {}", output.display());
        
    Ok(())
}

