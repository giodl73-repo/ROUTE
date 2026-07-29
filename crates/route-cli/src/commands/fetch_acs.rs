//! `FetchAcs` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route fetch-acs — Census ACS 5-year county population");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    std::fs::create_dir_all(&manifest.cache_dir)?;
    let out = manifest.cache_dir.join("acs_county_pop_2022.csv");
    let api_key = census_api_key()?;
    route_data::fetch_acs_population(&out, &api_key)?;
    println!("  saved → {}", out.display());
    println!("  run `route fetch` to get county gazetteer, then `route coverage` for population-weighted analysis.");
        
    Ok(())
}

