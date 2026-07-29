//! `map` command handler (same contract as `build` exemplar).
//! See `commands/build.rs` for the reference shape.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    designation: String,
    output: Option<PathBuf>,
    color_by: Option<String>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();


    let norm = normalise_designation(&designation);
    let out = output.unwrap_or_else(|| {
        let slug = match norm.as_str() {
            "ALL" => "all-tiers".to_string(),
            "BECK" => "beck-schematic".to_string(),
            "BECKT2" => "beck-schematic-t2".to_string(),
            "BECKT2ONLY" => "beck-schematic-t2-only".to_string(),
            "T3ZGREATLAKES" => "t3-great-lakes".to_string(),
            "T3ZSOUTHEAST" => "t3-southeast".to_string(),
            "T3ZTEXASBORDER" => "t3-texas-border".to_string(),
            "T3ZMOUNTAINWEST" => "t3-mountain-west".to_string(),
            "T3ZMIDSOUTH" => "t3-mid-south".to_string(),
            _ => norm.to_lowercase(),
        };
        PathBuf::from(format!("maps/{slug}.png"))
    });
    println!("route map {norm} → {}", out.display());

    // Beck schematic — topological relay network (0°/45°/90° geometry, no geography)
    if norm == "BECK" {
        std::fs::create_dir_all("maps")?;
        let svg = route_map::build_beck_svg();
        route_map::svg_to_png(&svg, &out, 2400, 1350)?;
        println!("  rendered Beck schematic: {} (2400×1350)", out.display());
        println!("  T1 relay network topology · 0°/45°/90° · inspired by Beck 1933");
        return Ok(());
    }

    if norm == "BECKT2" {
        std::fs::create_dir_all("maps")?;
        let svg = route_map::build_beck_t2_svg();
        route_map::svg_to_png(&svg, &out, 2400, 1350)?;
        println!(
            "  rendered Beck schematic with T2 connectors: {} (2400×1350)",
            out.display()
        );
        println!("  T1 trunks bold · T2 connectors thin and split-tinted to parent trunks");
        return Ok(());
    }

    if norm == "BECKT2ONLY" {
        std::fs::create_dir_all("maps")?;
        let svg = route_map::build_beck_t2_only_svg();
        route_map::svg_to_png(&svg, &out, 2400, 1350)?;
        println!(
            "  rendered Beck T2-only schematic: {} (2400×1350)",
            out.display()
        );
        println!("  T2 service connectors only · T1 trunk layer suppressed");
        return Ok(());
    }

    if norm.starts_with("T3Z") {
        let manifest = route_data::Manifest::load(&manifest_path).with_context(|| {
            format!("loading manifest from {}", manifest_path.display())
        })?;
        let graph = load_graph(&manifest)?;
        let scores =
            route_map::load_tier_scores(std::path::Path::new("data/scores-all.csv"))
                .into_iter()
                .map(|(route, score)| (route, score as f32))
                .collect::<std::collections::HashMap<_, _>>();
        let stop_file = std::fs::File::open("data/tier-stop-candidates.csv")
            .context("reading data/tier-stop-candidates.csv")?;
        let stop_rows = parse_stop_candidates(stop_file)?;
        let stops = stop_rows
            .iter()
            .filter_map(|row| {
                Some(route_map::t3_zone::T3Stop {
                    id: row.stop_id.clone(),
                    name: row.name.clone(),
                    class_name: row.requested_class.clone(),
                    lat: parse_coord(&row.lat)?,
                    lon: parse_coord(&row.lon)?,
                })
            })
            .collect::<Vec<_>>();
        let svg = route_map::build_t3_zone_svg(&graph, &norm, &stops, &scores)?;
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        route_map::svg_to_png(&svg, &out, 1800, 1000)?;
        let board_csv = route_map::build_t3_zone_board_csv(&norm)?;
        let board_out = out.with_extension("board.csv");
        std::fs::write(&board_out, board_csv).with_context(|| {
            format!("writing T3 board manifest {}", board_out.display())
        })?;
        println!(
            "  rendered T3 Beck zone schematic: {} (1800x1000)",
            out.display()
        );
        println!(
            "  regional Beck schematic · stops define endpoints, bends, and transfers"
        );
        println!("  wrote T3 game board manifest: {}", board_out.display());
        return Ok(());
    }

    // Mega-map: all tiers at once
    if norm == "ALL" {
        let manifest = route_data::Manifest::load(&manifest_path).with_context(|| {
            format!("loading manifest from {}", manifest_path.display())
        })?;
        let graph = load_graph(&manifest)?;
        let scores =
            route_map::load_tier_scores(std::path::Path::new("data/scores-all.csv"));
        println!(
            "  building tier mega-map ({} routes, {} score entries)…",
            graph.route_ids().len(),
            scores.len()
        );
        let svg = route_map::build_megamap_svg(&graph, &scores)?;
        std::fs::create_dir_all("maps")?;
        route_map::svg_to_png(&svg, &out, 2400, 1350)?;
        println!("  rendered mega-map: {} (2400×1350)", out.display());
        println!("  T1 red · T2 orange · T3 gold · T4 gray");
        return Ok(());
    }

    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;

    // T1 primary corridors get a regional map showing T2/T3/T4 feeders.
    if route_network::T1_BACKBONE_ROUTES.contains(&norm.as_str()) {
        let tier_scores =
            route_map::load_tier_scores(std::path::Path::new("data/scores-all.csv"));
        // Convert f64 scores to f32 for the T1 corridor map API.
        let scores_f32: std::collections::HashMap<String, f32> = tier_scores
            .iter()
            .map(|(k, &v)| (k.clone(), v as f32))
            .collect();
        println!(
            "  building T1 regional map for {norm} ({} score entries)…",
            scores_f32.len()
        );

        // Load relay hubs and resolve coordinates for the map.
        // t1_hub_coordinates() returns the canonical lat/lon table; we join
        // against load_hubs() so only hubs that actually exist in the TOML
        // (or built-in defaults) are shown.
        let data_dir = std::path::PathBuf::from("data");
        let hubs = route_sim::load_hubs(&data_dir, false);
        let coord_table = route_map::t1_hub_coordinates();
        // Build owned (lat, lon, name) tuples for hubs that have coordinates.
        let hub_pts: Vec<(f64, f64, String)> = hubs
            .iter()
            .filter_map(|hub| {
                // Match hub name against the coordinate table (TOML name is the
                // prefix before any parenthetical suffix in hub.rs defaults).
                coord_table
                    .iter()
                    .find(|(_, _, table_name, _)| {
                        hub.name.starts_with(table_name.as_str())
                            || table_name.starts_with(hub.name.as_str())
                    })
                    .map(|(lat, lon, _, _)| (*lat, *lon, hub.name.clone()))
            })
            .collect();
        // Build the &str slice expected by build_t1_corridor_svg.
        let hub_slice: Vec<(f64, f64, &str)> = hub_pts
            .iter()
            .map(|(lat, lon, name)| (*lat, *lon, name.as_str()))
            .collect();
        let hub_arg = if hub_slice.is_empty() {
            None
        } else {
            Some(hub_slice.as_slice())
        };
        println!("  relay hubs loaded: {}", hub_slice.len());

        let svg = route_map::build_t1_corridor_svg(&graph, &norm, &scores_f32, hub_arg)?;
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        route_map::svg_to_png(&svg, &out, 1800, 1000)?;
        println!("  rendered T1 regional map: {} (1800×1000)", out.display());
        println!("  {norm} bold · surrounding T2/T3/T4 visible in region");
        return Ok(());
    }

    let corridor = route_network::aggregate_corridor(&graph, &norm)
        .ok_or_else(|| anyhow::anyhow!("Route '{}' not found in graph", norm))?;

    // Score for color-by
    let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);

    // Build SVG
    let svg = route_map::build_svg(&corridor, &graph, Some(&scores), color_by.as_deref())?;

    // Create output directory
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }

    route_map::svg_to_png(&svg, &out, 1600, 900)?;
    println!(
        "  rendered: {} ({} segments, {:.0} miles)",
        out.display(),
        corridor.edge_count,
        corridor.total_miles
    );
    println!(
        "  score: {:.1}/160  A3: {:.2}",
        scores.total(),
        scores.a3.score
    );
    Ok(())
}
