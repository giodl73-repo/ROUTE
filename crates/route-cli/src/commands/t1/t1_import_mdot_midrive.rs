//! `T1ImportMdotMidrive` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    input: PathBuf,
    output: PathBuf,
    site_id: String,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    observation_year: Option<u16>,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let json = std::fs::read_to_string(&input)
        .with_context(|| format!("reading MDOT Mi Drive JSON {}", input.display()))?;
    let rows = parse_mdot_midrive_events(
        &json,
        &site_id,
        lat,
        lon,
        radius_miles,
        observation_year.unwrap_or_else(current_utc_year),
    )
    .with_context(|| format!("normalizing MDOT Mi Drive JSON {}", input.display()))?;
    write_t1_failure_events(&output, &rows)
        .with_context(|| format!("writing normalized events {}", output.display()))?;
    println!("route t1-import-mdot-midrive");
    println!("  rows: {}", rows.len());
    println!("  wrote {}", output.display());

    Ok(())
}
