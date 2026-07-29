//! `Report` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    designation: String,
    output: Option<PathBuf>,
    allow_partial: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


            let norm = normalise_designation(&designation);
            println!("route report {norm}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let canonical_output =
                PathBuf::from(format!("corpus/existing/{}.md", norm.to_lowercase()));
            let output_path = output.unwrap_or_else(|| canonical_output.clone());
            ensure_reviewed_report_sources(
                &canonical_output,
                &manifest.cache_dir,
                std::path::Path::new("."),
                allow_partial,
            )?;
            let mut graph = load_graph(&manifest)?;
            let bc_raw = route_network::centrality::compute_edge_betweenness(&graph);
            let mut vals_sorted: Vec<f64> =
                bc_raw.values().copied().filter(|v| v.is_finite()).collect();
            vals_sorted.sort_by(f64::total_cmp);
            let p95_idx = ((vals_sorted.len() as f64 * 0.95) as usize)
                .min(vals_sorted.len().saturating_sub(1));
            let bc_norm = vals_sorted.get(p95_idx).cloned().unwrap_or(1.0).max(1.0);
            let bc = bc_raw
                .into_iter()
                .map(|(k, v)| (k, (v / bc_norm).min(1.0)))
                .collect();
            graph.edge_betweenness = Some(bc);

            let mut corridor =
                route_network::aggregate_corridor(&graph, &norm).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Route '{}' not found in graph. Available: {:?}",
                        norm,
                        &graph.interstate_ids()[..graph.interstate_ids().len().min(20)]
                    )
                })?;

            let excluded_sources = if norm == "I80" {
                load_excluded_i80_sources(Path::new("data/i80-report-source-contract.csv"))?
            } else {
                std::collections::BTreeSet::new()
            };
            let acs_counties = load_acs_counties_for_scoring(&manifest);
            let ports = load_ports();
            let dcfc = if excluded_sources.contains("SRC-I80-DCFC") {
                Vec::new()
            } else {
                load_dcfc_stations()
            };
            let intermodal = load_intermodal_terminals();
            let fema_tiles = if excluded_sources.contains("SRC-I80-FEMA") {
                Vec::new()
            } else {
                load_fema_tiles()
            };
            let nbi = if excluded_sources.contains("SRC-I80-NBI") {
                std::collections::HashMap::new()
            } else {
                load_nbi_bridges()
            };
            let fars_safety = if excluded_sources.contains("SRC-I80-FARS") {
                std::collections::HashMap::new()
            } else {
                load_fars_safety()
            };
            let railroad_parallels = load_railroad_parallels();
            let hazard_zones = load_hazard_zones();

            if acs_counties.is_some() {
                join_acs_population_to_corridor(
                    &manifest,
                    &graph,
                    &norm,
                    &mut corridor.attributes,
                    false,
                );
            }
            if !ports.is_empty() {
                join_port_access_to_corridor(&graph, &norm, &mut corridor.attributes, &ports);
            }
            if !dcfc.is_empty() {
                join_dcfc_to_corridor(
                    &graph,
                    &norm,
                    corridor.total_miles,
                    &mut corridor.attributes,
                    &dcfc,
                );
            }
            if !intermodal.is_empty() {
                join_intermodal_to_corridor(&graph, &norm, &mut corridor.attributes, &intermodal);
            }
            if !fema_tiles.is_empty() {
                join_fema_d1_to_corridor(&graph, &norm, &mut corridor.attributes, &fema_tiles);
            }
            if !nbi.is_empty() {
                join_nbi_to_corridor(&norm, &mut corridor.attributes, &nbi);
            }
            join_d3_iri_proxy(&mut corridor.attributes);
            if let Some(&rate) = fars_safety.get(&norm) {
                corridor.attributes.fatal_crash_rate = Some(rate);
            }
            if let Some(railroad) = railroad_parallels.get(&norm) {
                corridor.attributes.rail_parallel_flag = true;
                corridor.attributes.rail_parallel_name = Some(railroad.clone());
            }
            if let Some(zone) = hazard_zones.get(&norm) {
                corridor.attributes.wildfire_risk = Some(zone.wildfire);
                corridor.attributes.tornado_risk = Some(zone.tornado);
                corridor.attributes.seismic_risk = Some(zone.seismic);
            }
            join_a2_freight_proxy(&mut corridor.attributes, corridor.total_miles);

            let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
            let provenance = route_report::CorpusProvenance {
                command: format!("route report {norm}"),
                manifest_version: manifest.version.clone(),
                manifest_path: manifest_path.display().to_string(),
                scoring_config_path: scoring_config_path.display().to_string(),
            };
            let annotation_path =
                PathBuf::from(format!("corpus/annotations/{}.toml", norm.to_lowercase()));
            route_report::write_corpus_entry_with_provenance_and_annotations(
                &corridor,
                &scores,
                &output_path,
                &provenance,
                &annotation_path,
            )?;

            println!(
                "  regenerated: {} ({:.1}/160{})",
                output_path.display(),
                scores.total(),
                if scores.any_estimated() { "†" } else { "" }
            );
            if scores.any_estimated() {
                println!("  † Some scores are estimated — see report justifications.");
            }
            Ok(())
}
