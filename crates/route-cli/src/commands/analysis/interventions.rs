//! `Interventions` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    corridor: InterventionCorridorArg,
    trips: usize,
    seed: u64,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let data_dir = std::path::PathBuf::from("data");
    let c = match corridor {
        InterventionCorridorArg::NyLa => {
            route_sim::load_corridor(&data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor)
        }
        InterventionCorridorArg::HouChi => route_sim::load_corridor(&data_dir, "hou_chi_current")
            .unwrap_or_else(route_sim::hou_chi_current),
        InterventionCorridorArg::HouI69 => route_sim::load_corridor(&data_dir, "hou_chi_i69")
            .unwrap_or_else(route_sim::hou_chi_i69),
    };
    println!("route interventions — {trips} trips per scenario\n");
    let bench = route_sim::InterventionBenchmark::run(&c, trips, seed);
    print_intervention_benchmark(&bench);

    Ok(())
}
