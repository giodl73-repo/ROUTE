//! `score_all` command handler (same contract as `build` exemplar).
//! See `commands/build.rs` for the reference shape.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, workers: Option<usize>) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route score-all");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let mut graph = load_graph(&manifest)?;

    // Compute betweenness centrality on the full graph
    let w = workers.unwrap_or_else(num_cpus);
    println!("  computing betweenness centrality ({w} workers)…");
    let bc_raw = route_network::centrality::compute_edge_betweenness(&graph);
    println!("  centrality: {} edges scored", bc_raw.len());
    // Normalize using P95 to prevent outlier junction edges from compressing the distribution
    let mut vals_sorted: Vec<f64> = bc_raw.values().copied().filter(|v| v.is_finite()).collect();
    vals_sorted.sort_by(f64::total_cmp);
    let p95_idx =
        ((vals_sorted.len() as f64 * 0.95) as usize).min(vals_sorted.len().saturating_sub(1));
    let bc_norm = vals_sorted.get(p95_idx).cloned().unwrap_or(1.0).max(1.0);
    let bc = bc_raw
        .into_iter()
        .map(|(k, v)| (k, (v / bc_norm).min(1.0)))
        .collect();
    graph.edge_betweenness = Some(bc);

    // Load all data sources (mirrors calibrate command)
    let acs_counties = load_acs_counties_for_scoring(&manifest);
    if acs_counties.is_some() {
        println!("  ACS population loaded");
    }
    let ports = load_ports();
    if !ports.is_empty() {
        println!("  {} port/border locations loaded", ports.len());
    }
    let dcfc = load_dcfc_stations();
    if !dcfc.is_empty() {
        println!("  {} DCFC stations loaded", dcfc.len());
    }
    let intermodal = load_intermodal_terminals();
    if !intermodal.is_empty() {
        println!("  {} intermodal terminals loaded", intermodal.len());
    }
    let fema_tiles = load_fema_tiles();
    if !fema_tiles.is_empty() {
        println!("  {} FEMA tiles loaded", fema_tiles.len());
    }
    let nbi = load_nbi_bridges();
    if !nbi.is_empty() {
        println!("  {} NBI bridge records loaded", nbi.len());
    }
    let fars_safety = load_fars_safety();
    if !fars_safety.is_empty() {
        println!("  {} FARS route records loaded", fars_safety.len());
    }
    let railroad_parallels = load_railroad_parallels();
    if !railroad_parallels.is_empty() {
        println!("  {} railroad parallels loaded", railroad_parallels.len());
    }
    let hazard_zones = load_hazard_zones();
    if !hazard_zones.is_empty() {
        println!("  {} hazard zone records loaded", hazard_zones.len());
    }

    // Score interstates plus US highway upgrade candidates when present.
    let ids = atlas_candidate_ids(&graph);
    println!("  scoring {} corridors…", ids.len());

    let mut all_scores = Vec::new();
    let mut score_rows: Vec<ScoreAllRow> = Vec::new();
    for id in &ids {
        if let Some(mut corridor) = route_network::aggregate_corridor(&graph, id) {
            // Apply all data joins (same as calibrate)
            if acs_counties.is_some() {
                join_acs_population_to_corridor(
                    &manifest,
                    &graph,
                    id,
                    &mut corridor.attributes,
                    false,
                );
            }
            if !ports.is_empty() {
                join_port_access_to_corridor(&graph, id, &mut corridor.attributes, &ports);
            }
            if !dcfc.is_empty() {
                join_dcfc_to_corridor(
                    &graph,
                    id,
                    corridor.total_miles,
                    &mut corridor.attributes,
                    &dcfc,
                );
            }
            if !intermodal.is_empty() {
                join_intermodal_to_corridor(&graph, id, &mut corridor.attributes, &intermodal);
            }
            if !fema_tiles.is_empty() {
                join_fema_d1_to_corridor(&graph, id, &mut corridor.attributes, &fema_tiles);
            }
            if !nbi.is_empty() {
                join_nbi_to_corridor(id, &mut corridor.attributes, &nbi);
            }
            join_d3_iri_proxy(&mut corridor.attributes);
            // A5: join FARS fatal crash rate
            if let Some(&rate) = fars_safety.get(id) {
                corridor.attributes.fatal_crash_rate = Some(rate);
            }
            // B1: join railroad parallel flag
            if let Some(railroad) = railroad_parallels.get(id) {
                corridor.attributes.rail_parallel_flag = true;
                corridor.attributes.rail_parallel_name = Some(railroad.clone());
            }
            // D1: join multi-hazard zone data
            if let Some(zone) = hazard_zones.get(id) {
                corridor.attributes.wildfire_risk = Some(zone.wildfire);
                corridor.attributes.tornado_risk = Some(zone.tornado);
                corridor.attributes.seismic_risk = Some(zone.seismic);
            }
            join_a2_freight_proxy(&mut corridor.attributes, corridor.total_miles);
            let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
            println!(
                "  {}: {:.1}/160{}",
                corridor.designation,
                scores.total(),
                if scores.any_estimated() { "†" } else { "" }
            );
            let total = rounded_score(scores.total());
            let tier = tier_for_score(total);
            score_rows.push(ScoreAllRow {
                route: corridor.designation.clone(),
                score: total,
                tier,
                rubric_version: scores.rubric_version.clone(),
                estimated: scores.any_estimated(),
                confidence: scores.mean_confidence(),
                score_confidence: scores.score_weighted_confidence(),
                dimensions: dimension_score_values(&scores),
                dimension_confidences: dimension_confidence_values(&scores),
            });
            all_scores.push(scores);
        }
    }

    std::fs::create_dir_all("data")?;
    let out = PathBuf::from("data/scores-all.csv");
    let mut wtr = csv::Writer::from_path(&out)?;
    let mut header = vec![
        "route",
        "score",
        "tier",
        "rubric_version",
        "estimated",
        "confidence",
        "score_confidence",
        "confidence_label",
        "score_confidence_label",
    ];
    header.extend(DIMENSION_CODES);
    header.extend([
        "A1_conf", "A2_conf", "A3_conf", "A4_conf", "A5_conf", "B1_conf", "B2_conf", "B3_conf",
        "B4_conf", "C1_conf", "C2_conf", "C3_conf", "C4_conf", "D1_conf", "D2_conf", "D3_conf",
    ]);
    wtr.write_record(header)?;
    for row in &score_rows {
        let mut csv_row = vec![
            row.route.clone(),
            format!("{:.1}", row.score),
            row.tier.to_string(),
            row.rubric_version.clone(),
            row.estimated.to_string(),
            format!("{:.2}", row.confidence),
            format!("{:.2}", row.score_confidence),
            route_score::confidence_label(row.confidence).to_string(),
            route_score::confidence_label(row.score_confidence).to_string(),
        ];
        csv_row.extend(row.dimensions.iter().map(|value| format!("{value:.1}")));
        csv_row.extend(
            row.dimension_confidences
                .iter()
                .map(|value| format!("{value:.2}")),
        );
        wtr.write_record(csv_row)?;
    }
    wtr.flush()?;
    println!(
        "  wrote {} score rows → {}",
        score_rows.len(),
        out.display()
    );
    write_tier_artifacts(&score_rows)?;
    println!("score-all complete: {} corridors scored.", all_scores.len());
    Ok(())
}
