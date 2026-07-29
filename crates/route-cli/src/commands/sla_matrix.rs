//! `SlaMatrix` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    trips: usize,
    seed: u64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route sla-matrix — national SLA commitment windows ({trips} trips)\n");
            let data_dir = std::path::PathBuf::from("data");
            print_sla_matrix(trips, seed, &data_dir);
        
    Ok(())
}

