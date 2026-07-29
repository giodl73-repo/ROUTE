//! `HubOutage` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    include_proposed: bool,
    outage_hours: f64,
    reserve_driver_fraction: f64,
    adjacent_absorption_fraction: f64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let data_dir = std::path::PathBuf::from("data");
    let confirmed_only = !include_proposed;
    let hubs = route_sim::load_hubs(&data_dir, confirmed_only);
    if hubs.is_empty() {
        eprintln!("No hubs loaded — check data/relay-hubs.toml");
    }
    let net = route_sim::compute_network_summary(&hubs);
    let config = route_sim::HubOutageConfig {
        outage_hours,
        reserve_driver_fraction,
        adjacent_absorption_fraction,
    };
    let summary = route_sim::run_hub_outage_sensitivity(&net.hub_staffings, config);
    print_hub_outage(&summary, config);
        
    Ok(())
}

