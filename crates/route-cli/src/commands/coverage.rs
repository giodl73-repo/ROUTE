//! `coverage` command handler (same contract as `build` exemplar).
//! See `commands/build.rs` for the reference shape.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    threshold: f64,
    grid: f64,
    t1_only: bool,
    top_gaps: usize,
    grid_mode: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


            println!(
                "route coverage --threshold {threshold}mi{}{}",
                if t1_only { " --t1-only" } else { "" },
                if grid_mode { " --grid-mode" } else { "" }
            );
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;
            let filter: Option<&[&str]> = if t1_only {
                Some(route_network::T1_BACKBONE_ROUTES)
            } else {
                None
            };
            let interchange_count = graph
                .graph
                .node_indices()
                .filter(|&ni| graph.graph[ni].is_interchange)
                .count();

            // Try county centroid mode first (population-weighted, no ocean problem)
            let gaz_path = manifest.cache_dir.join("2023_Gaz_counties_national.txt");
            let gaz_zip = manifest.cache_path("census-gazetteer-counties");

            let county_path = if !grid_mode && (gaz_path.exists() || gaz_zip.exists()) {
                // Extract if needed
                if !gaz_path.exists() && gaz_zip.exists() {
                    println!("  extracting gazetteer…");
                    route_data::fetch::extract_shp(&gaz_zip, &manifest.cache_dir).ok();
                }
                // Find the .txt file
                std::fs::read_dir(&manifest.cache_dir)
                    .ok()
                    .and_then(|entries| {
                        entries
                            .filter_map(|e| e.ok())
                            .find(|e| {
                                e.file_name()
                                    .to_string_lossy()
                                    .ends_with("counties_national.txt")
                            })
                            .map(|e| e.path())
                    })
            } else {
                None
            };

            if let Some(ref path) = county_path {
                // Population-weighted county centroid analysis
                println!("  mode: county centroids ({interchange_count} interchange nodes)");
                let mut counties =
                    route_data::read_county_gazetteer(path).context("reading county gazetteer")?;
                println!("  counties loaded: {}", counties.len());

                // Join population if available
                let pop_path = manifest.cache_dir.join("acs_county_pop_2022.csv");
                let has_pop = if pop_path.exists() {
                    let joined = route_data::join_population(&mut counties, &pop_path)?;
                    println!("  population joined: {} counties", joined);
                    true
                } else {
                    println!("  no population data — run `route fetch-acs` for weighted stats");
                    false
                };

                let result =
                    route_network::compute_pop_coverage(&graph, &counties, filter, threshold);
                let tc = result.total_counties as f64;
                let tp = result.total_population as f64;
                let tl = result.total_land_sqmi;

                println!("\n┌──────────────────────────────────────────────────────────────┐");
                println!(
                    "│  Coverage — {}mi threshold{}  [county centroid mode]",
                    threshold,
                    if t1_only { " T1-only" } else { "" }
                );
                println!("├──────────────────────────────────────────────────────────────┤");
                println!(
                    "│  Counties analyzed:  {:>8} total                          │",
                    result.total_counties
                );
                println!(
                    "│                      {:>8} within 20mi  ({:.1}%)          │",
                    result.counties_within_20mi,
                    result.counties_within_20mi as f64 / tc * 100.0
                );
                println!(
                    "│                      {:>8} within 30mi  ({:.1}%)          │",
                    result.counties_within_30mi,
                    result.counties_within_30mi as f64 / tc * 100.0
                );
                println!(
                    "│                      {:>8} within 50mi  ({:.1}%)          │",
                    result.counties_within_50mi,
                    result.counties_within_50mi as f64 / tc * 100.0
                );
                if has_pop && tp > 0.0 {
                    println!("│  Population:                                                 │");
                    println!(
                        "│    Within 20mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_20mi,
                        result.pop_within_20mi as f64 / tp * 100.0
                    );
                    println!(
                        "│    Within 30mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_30mi,
                        result.pop_within_30mi as f64 / tp * 100.0
                    );
                    println!(
                        "│    Within 50mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_50mi,
                        result.pop_within_50mi as f64 / tp * 100.0
                    );
                }
                println!(
                    "│  Land area within 30mi:  {:>9.0} sq mi  ({:.1}% of US)  │",
                    result.land_within_30mi,
                    result.land_within_30mi / tl * 100.0
                );
                println!(
                    "│  Gap counties (>{}mi): {:>8}                           │",
                    threshold,
                    result.gap_counties.len()
                );
                let candidate_gaps = result
                    .gap_counties
                    .iter()
                    .filter(|g| g.gap_class == "candidate_access_gap")
                    .count();
                let centroid_risks = result
                    .gap_counties
                    .iter()
                    .filter(|g| g.gap_class == "centroid_artifact_risk")
                    .count();
                let non_conus = result
                    .gap_counties
                    .iter()
                    .filter(|g| g.gap_class == "non_conus")
                    .count();
                println!(
                    "│    Candidate access: {:>8}  centroid-risk: {:>5}       │",
                    candidate_gaps, centroid_risks
                );
                println!(
                    "│    Non-CONUS/model: {:>8}                              │",
                    non_conus
                );
                println!(
                    "│  Worst gap:          {:>9.1} miles  ({}, {})           │",
                    result.max_gap_miles,
                    result
                        .gap_counties
                        .first()
                        .map(|g| g.name.as_str())
                        .unwrap_or("—"),
                    result
                        .gap_counties
                        .first()
                        .map(|g| g.state.as_str())
                        .unwrap_or("—")
                );
                println!("└──────────────────────────────────────────────────────────────┘");

                if !result.gap_counties.is_empty() {
                    let label = if t1_only { "T1" } else { "any interstate" };
                    println!(
                        "\n  Top {} counties >{}mi from {} on-ramp:",
                        top_gaps, threshold, label
                    );
                    println!(
                        "  {:>6}  {:<28} {:>5}  {:>8}  {:>10}",
                        "Miles", "County", "State", "Pop", "Land(sqmi)"
                    );
                    println!("  {}", "─".repeat(66));
                    for gap in result.gap_counties.iter().take(top_gaps) {
                        println!(
                            "  {:>5.1}mi  {:<28} {:>5}  {:>8}  {:>10.0}",
                            gap.nearest_miles, gap.name, gap.state, gap.population, gap.aland_sqmi
                        );
                    }

                    // Save gap list to CSV for paper B.1
                    let gap_csv = std::path::PathBuf::from("data/coverage-gaps.csv");
                    if let Ok(mut wtr) = csv::Writer::from_path(&gap_csv) {
                        let _ = wtr.write_record([
                            "GEOID",
                            "NAME",
                            "STATE",
                            "LAT",
                            "LON",
                            "NEAREST_MI",
                            "POPULATION",
                            "LAND_SQMI",
                            "GAP_CLASS",
                            "ARTIFACT_REASON",
                        ]);
                        for g in &result.gap_counties {
                            let _ = wtr.write_record(&[
                                g.geoid.clone(),
                                g.name.clone(),
                                g.state.clone(),
                                format!("{:.4}", g.lat),
                                format!("{:.4}", g.lon),
                                format!("{:.1}", g.nearest_miles),
                                g.population.to_string(),
                                format!("{:.0}", g.aland_sqmi),
                                g.gap_class.clone(),
                                g.artifact_reason.clone(),
                            ]);
                        }
                        println!("\n  gap list saved → {}", gap_csv.display());
                    }

                    println!("\n  I2.0 target: 99% of counties within 30mi via T2+T3 combined");
                    println!(
                        "  T3 rural spurs / new T3 designations needed: {}",
                        result.gap_counties.len()
                    );
                }
            } else {
                // Fallback: geometric grid mode
                println!(
                    "  mode: geographic grid ({}mi resolution) — run `route fetch` for county data",
                    grid
                );
                println!("  NOTE: includes ocean cells; county centroid mode is more accurate");
                let result =
                    route_network::coverage::compute_coverage(&graph, filter, grid, threshold);
                println!(
                    "  cells: {} total, {:.1}% within 30mi, max gap {:.1}mi",
                    result.total_cells, result.pct_within_30mi, result.max_gap_miles
                );
                println!("  For accurate results: run `route fetch` to download county gazetteer,");
                println!("  then `route fetch-acs` for population, then `route coverage` again.");
            }
            Ok(())
}
