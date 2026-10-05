//! `T1FetchIndotTrafficwise` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    output: PathBuf,
    north: f64,
    south: f64,
    east: f64,
    west: f64,
    zoom: u8,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    fetch_indot_trafficwise_events(&output, north, south, east, west, zoom)
        .with_context(|| format!("fetching INDOT TrafficWise events to {}", output.display()))?;
    println!("route t1-fetch-indot-trafficwise");
    println!("  wrote {}", output.display());

    Ok(())
}
