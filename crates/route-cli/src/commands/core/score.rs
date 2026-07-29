//! `Score` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    designation: String,
    estimated: bool,
    proposed: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


    let norm = normalise_designation(&designation);
    println!("route score {}", norm);

    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;

    // Build graph from cached shapefile
    let graph = load_graph(&manifest)?;
    println!(
        "  graph: {} edges, {} interstates",
        graph.graph.edge_count(),
        graph.interstate_ids().len()
    );

    // Extract corridor
    let mut corridor =
        route_network::aggregate_corridor(&graph, &norm).ok_or_else(|| {
            anyhow::anyhow!(
                "Route '{}' not found in graph. Available: {:?}",
                norm,
                &graph.interstate_ids()[..graph.interstate_ids().len().min(20)]
            )
        })?;

    println!(
        "  corridor: {} ({:.0} miles, {} segments)",
        corridor.designation, corridor.total_miles, corridor.edge_count
    );

    // Join ACS population for C1/C3 dimensions (if cached data is available)
    join_acs_population_to_corridor(
        &manifest,
        &graph,
        &norm,
        &mut corridor.attributes,
        true,
    );
    let ports = load_ports();
    if !ports.is_empty() {
        join_port_access_to_corridor(&graph, &norm, &mut corridor.attributes, &ports);
    }
    let dcfc = load_dcfc_stations();
    if !dcfc.is_empty() {
        join_dcfc_to_corridor(
            &graph,
            &norm,
            corridor.total_miles,
            &mut corridor.attributes,
            &dcfc,
        );
    }
    let intermodal = load_intermodal_terminals();
    if !intermodal.is_empty() {
        join_intermodal_to_corridor(&graph, &norm, &mut corridor.attributes, &intermodal);
    }
    let fema_tiles = load_fema_tiles();
    if !fema_tiles.is_empty() {
        join_fema_d1_to_corridor(&graph, &norm, &mut corridor.attributes, &fema_tiles);
    }
    let nbi = load_nbi_bridges();
    if !nbi.is_empty() {
        join_nbi_to_corridor(&norm, &mut corridor.attributes, &nbi);
    }
    join_d3_iri_proxy(&mut corridor.attributes);
    let fars_safety = load_fars_safety();
    if let Some(&rate) = fars_safety.get(&norm) {
        corridor.attributes.fatal_crash_rate = Some(rate);
    }
    let railroad_parallels = load_railroad_parallels();
    if let Some(railroad) = railroad_parallels.get(&norm) {
        corridor.attributes.rail_parallel_flag = true;
        corridor.attributes.rail_parallel_name = Some(railroad.clone());
    }
    let hazard_zones = load_hazard_zones();
    if let Some(zone) = hazard_zones.get(&norm) {
        corridor.attributes.wildfire_risk = Some(zone.wildfire);
        corridor.attributes.tornado_risk = Some(zone.tornado);
        corridor.attributes.seismic_risk = Some(zone.seismic);
    }
    join_a2_freight_proxy(&mut corridor.attributes, corridor.total_miles);

    // Score
    let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
    // Print score table
    print_score_table(&corridor.designation, &scores, estimated);

    // Write corpus entry
    let slug = norm.to_lowercase();
    let corpus_dir = if proposed {
        "corpus/proposed"
    } else {
        "corpus/existing"
    };
    let output_path = PathBuf::from(format!("{corpus_dir}/{slug}.md"));
    let provenance = route_report::CorpusProvenance {
        command: format!("route score {norm}"),
        manifest_version: manifest.version.clone(),
        manifest_path: manifest_path.display().to_string(),
        scoring_config_path: scoring_config_path.display().to_string(),
    };
    route_report::write_corpus_entry_with_provenance(
        &corridor,
        &scores,
        &output_path,
        &provenance,
    )?;
    println!("\n  corpus entry → {}", output_path.display());

    if scores.any_estimated() {
        println!("  † Some scores are estimated — see justifications above.");
        println!("    Run `route score-all` for authoritative national B2 centrality.");
    }
    Ok(())
}
