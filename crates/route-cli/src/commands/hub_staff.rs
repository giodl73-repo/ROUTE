//! `HubStaff` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    include_proposed: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let data_dir = std::path::PathBuf::from("data");
            let confirmed_only = !include_proposed;
            let hubs = route_sim::load_hubs(&data_dir, confirmed_only);
            if hubs.is_empty() {
                eprintln!("No hubs loaded — check data/relay-hubs.toml");
            } else {
                println!("Loaded {} hubs from data/relay-hubs.toml", hubs.len());
            }
            let net = route_sim::compute_network_summary(&hubs);
            print_hub_staffing(&net, include_proposed);
        
    Ok(())
}

