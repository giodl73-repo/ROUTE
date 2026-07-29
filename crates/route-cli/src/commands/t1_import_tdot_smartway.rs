//! `T1ImportTdotSmartway` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    input: PathBuf,
    output: PathBuf,
    site_id: String,
    lat: f64,
    lon: f64,
    radius_miles: f64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let json = std::fs::read_to_string(&input)
        .with_context(|| format!("reading TDOT SmartWay JSON {}", input.display()))?;
    let rows = parse_tdot_smartway_events(&json, &site_id, lat, lon, radius_miles)
        .with_context(|| format!("normalizing TDOT SmartWay JSON {}", input.display()))?;
    write_t1_failure_events(&output, &rows)
        .with_context(|| format!("writing normalized events {}", output.display()))?;
    println!("route t1-import-tdot-smartway");
    println!("  rows: {}", rows.len());
    println!("  wrote {}", output.display());
        
    Ok(())
}

