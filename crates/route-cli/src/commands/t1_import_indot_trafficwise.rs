//! `T1ImportIndotTrafficwise` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    input: PathBuf,
    output: PathBuf,
    site_id: String,
    observation_year: Option<u16>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("reading INDOT TrafficWise JSON {}", input.display()))?;
            let rows = parse_indot_trafficwise_events(
                &json,
                &site_id,
                observation_year.unwrap_or_else(current_utc_year),
            )
            .with_context(|| format!("normalizing INDOT TrafficWise JSON {}", input.display()))?;
            write_t1_failure_events(&output, &rows)
                .with_context(|| format!("writing normalized events {}", output.display()))?;
            println!("route t1-import-indot-trafficwise");
            println!("  rows: {}", rows.len());
            println!("  wrote {}", output.display());
        
    Ok(())
}

