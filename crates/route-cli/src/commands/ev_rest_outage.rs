//! `EvRestOutage` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    outage_station_fraction: f64,
    backup_power_fraction: f64,
    queue_delay_minutes: f64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let data_dir = std::path::PathBuf::from("data");
    let config = route_sim::EvRestOutageConfig {
        station_spacing_miles: 50.0,
        outage_station_fraction,
        backup_power_fraction,
        queue_delay_minutes,
    };
    print_ev_rest_outage(&data_dir, config);
        
    Ok(())
}

