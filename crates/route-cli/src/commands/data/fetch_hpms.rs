//! `FetchHpms` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    output: Option<PathBuf>,
    states: Option<String>,
    functional_systems: String,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let out = output.unwrap_or_else(|| PathBuf::from("data/cache/hpms_2018.csv"));
    println!("route fetch-hpms → {}", out.display());
    println!("  source: FHWA geo.dot.gov ArcGIS REST (2018 HPMS, no registration)");
    let hpms_functional_systems = parse_hpms_functional_systems(&functional_systems)?;
    println!(
        "  functional systems: {}",
        hpms_functional_systems
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );

    std::fs::create_dir_all(out.parent().unwrap_or(std::path::Path::new(".")))?;

    if let Some(state_filter) = states {
        // Fetch only specified states
        let filter = state_filter
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_ascii_uppercase())
            .collect::<std::collections::BTreeSet<_>>();
        let mut fetched: Vec<route_data::HpmsRecord> = Vec::new();
        for (abbr, name) in route_data::STATE_CODES {
            if filter.contains(*abbr) {
                print!("  [hpms] {abbr}… ");
                match route_data::hpms_fetch::fetch_state_hpms_with_systems(
                    abbr,
                    name,
                    &hpms_functional_systems,
                ) {
                    Ok(recs) => {
                        println!("{} segments", recs.len());
                        let state_out = out
                            .parent()
                            .unwrap_or(std::path::Path::new("."))
                            .join(format!("hpms_{}.csv", abbr.to_ascii_lowercase()));
                        write_hpms_records(&state_out, &recs)?;
                        fetched.extend(recs);
                    }
                    Err(e) => println!("FAILED — {e}"),
                }
            }
        }
        if fetched.is_empty() {
            anyhow::bail!(
                "HPMS fetch returned zero records; preserving existing cache at {}",
                out.display()
            );
        }
        let existing = if out.exists() {
            route_data::hpms::read_hpms_csv(&out)?
        } else {
            Vec::new()
        };
        let merged = merge_hpms_state_records(existing, fetched, &filter);
        write_hpms_records(&out, &merged)?;
        println!("  merged {} records into {}", merged.len(), out.display());
        for state in &filter {
            println!(
                "  state cache: {}",
                out.parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join(format!("hpms_{}.csv", state.to_ascii_lowercase()))
                    .display()
            );
        }
    } else {
        if hpms_functional_systems != [1] {
            anyhow::bail!(
                "non-default HPMS functional-system scope requires --states for scoped cache mutation"
            );
        }
        route_data::fetch_all_hpms(&out)?;
    }
    println!("fetch-hpms complete. Run `route build` to join.");

    Ok(())
}
