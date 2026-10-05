//! `Od` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, corridor: OdCorridorCmd, month: Option<u8>) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let data_dir = std::path::PathBuf::from("data");
    let (corridors, trips, seed): (Vec<route_sim::OdCorridor>, usize, u64) = match corridor {
        OdCorridorCmd::NyLa { trips, seed } => {
            let c = route_sim::load_corridor(&data_dir, "ny_la")
                .unwrap_or_else(route_sim::ny_la_corridor);
            (vec![c], trips, seed)
        }
        OdCorridorCmd::HouChi { trips, seed } => {
            let c = route_sim::load_corridor(&data_dir, "hou_chi_current")
                .unwrap_or_else(route_sim::hou_chi_current);
            (vec![c], trips, seed)
        }
        OdCorridorCmd::HouChiI69 { trips, seed } => {
            let c = route_sim::load_corridor(&data_dir, "hou_chi_i69")
                .unwrap_or_else(route_sim::hou_chi_i69);
            (vec![c], trips, seed)
        }
        OdCorridorCmd::All { trips, seed } => {
            let ny_la = route_sim::load_corridor(&data_dir, "ny_la")
                .unwrap_or_else(route_sim::ny_la_corridor);
            let hou_chi = route_sim::load_corridor(&data_dir, "hou_chi_current")
                .unwrap_or_else(route_sim::hou_chi_current);
            let hou_i69 = route_sim::load_corridor(&data_dir, "hou_chi_i69")
                .unwrap_or_else(route_sim::hou_chi_i69);
            (vec![ny_la, hou_chi, hou_i69], trips, seed)
        }
    };

    // Apply seasonal modifiers if month specified
    let corridors: Vec<route_sim::OdCorridor> = if let Some(m) = month {
        corridors
            .into_iter()
            .map(|c| route_sim::apply_seasonal(&c, m))
            .collect()
    } else {
        corridors
    };

    let month_names = [
        "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let season_note = match month {
        Some(m @ 1..=12) => {
            let is_winter = matches!(m, 11 | 12 | 1 | 2 | 3 | 4);
            let is_holiday = matches!(m, 10 | 11 | 12);
            let mut notes = vec![month_names[m as usize]];
            if is_winter {
                notes.push("WINTER: mountain pass closures 2.4× baseline");
            }
            if is_holiday {
                notes.push("HOLIDAY: urban freight surge +20% V/C");
            }
            format!(" — seasonal: {}", notes.join(" | "))
        }
        _ => " — annual average (use --month 1..12 for seasonal SLA)".to_string(),
    };

    println!("route od — transit time Monte Carlo ({trips} trips{season_note})\n");
    println!("Driver modes compared:");
    println!("  Solo / GP:     current infrastructure, 1 driver, mandatory 10h rest stops");
    println!("  Solo / I2.0:   managed lanes, 1 driver, mandatory rest stops");
    println!("  Team / I2.0:   managed lanes, 2 drivers, co-driver sleeps in berth");
    println!("  Relay / I2.0:  managed lanes, fresh driver at each T1 hub (~500mi legs)");
    println!("  Relay / GP:    current infrastructure with relay network only\n");

    for corridor in &corridors {
        let cmp = route_sim::OdComparison::run(corridor, trips, seed);
        print_od_comparison(&cmp);
        println!();
    }

    Ok(())
}
