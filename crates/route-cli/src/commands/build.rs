//! `Build` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    all_roads: bool,
    hpms_path: Option<PathBuf>,
    fpm_path: Option<PathBuf>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


            println!("route build{}", if all_roads { " --all-roads" } else { "" });
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;

            let shp_path = ensure_shapefile(&manifest)?;

            println!("  parsing road segments…");
            let segments = route_data::nhs::read_nhs_shapefile(&shp_path, all_roads)
                .map_err(|e| anyhow::anyhow!("shapefile error: {e}"))?;

            let interstate_count = segments
                .iter()
                .filter(|s| s.route_id.starts_with('I'))
                .count();
            let us_count = segments
                .iter()
                .filter(|s| s.route_id.starts_with("US"))
                .count();
            println!(
                "  segments: {} total  ({} interstate, {} US highway)",
                segments.len(),
                interstate_count,
                us_count
            );

            // Load HPMS if provided
            let hpms = if let Some(ref path) = hpms_path {
                println!("  loading HPMS: {}", path.display());
                route_data::hpms::read_hpms_csv(path)?
            } else {
                // Try default cache location
                let default = manifest.cache_dir.join("hpms_2018.csv");
                if default.exists() {
                    println!("  auto-loading HPMS: {}", default.display());
                    route_data::hpms::read_hpms_csv(&default)?
                } else {
                    println!("  no HPMS data — run `route fetch-hpms` to get traffic data");
                    Vec::new()
                }
            };

            if !hpms.is_empty() {
                println!("  HPMS records: {}", hpms.len());
            }

            let fpm = if let Some(ref path) = fpm_path {
                println!("  loading FPM reliability: {}", path.display());
                route_data::hpms::read_hpms_fpm_csv(path)?
            } else {
                load_cached_fpm(&manifest)
            };

            if !fpm.is_empty() {
                println!("  FPM reliability records: {}", fpm.len());
            }

            let (graph, report) = route_network::build_graph_with_fpm(segments, &hpms, &fpm);
            graph.print_build_report(&report);

            std::fs::create_dir_all(&manifest.cache_dir)?;
            let route_ids = graph.interstate_ids();
            let all_ids = graph.route_ids();
            let summary = serde_json::json!({
                "nodes": graph.graph.node_count(),
                "edges": graph.graph.edge_count(),
                "routes": all_ids.len(),
                "interstates": route_ids.len(),
                "interstate_ids": &route_ids,
            });
            let cache_path = manifest.cache_dir.join("graph.json");
            std::fs::write(&cache_path, serde_json::to_string_pretty(&summary)?)?;
            println!("  graph summary → {}", cache_path.display());
            println!(
                "build complete. {} interstates, {} total routes.",
                route_ids.len(),
                all_ids.len()
            );
            if !hpms.is_empty() {
                println!("  HPMS joined — A1/A2 and BPR A3 fallback will use traffic data.");
            }
            if !fpm.is_empty() {
                println!("  FPM joined — A3 will use observed PTI/TTI reliability data.");
            }
            Ok(())
}
