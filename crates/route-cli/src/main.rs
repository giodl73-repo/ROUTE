use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "route", about = "ROUTE — Interstate 2.0 analysis pipeline", version)]
struct Cli {
    /// Path to scoring config (default: config/scoring.toml in repo root)
    #[arg(long, global = true, value_name = "FILE")]
    scoring_config: Option<PathBuf>,

    /// Path to data manifest (default: ~/.route/manifest.json)
    #[arg(long, global = true, value_name = "FILE")]
    manifest: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download all manifest sources to the cache directory
    Fetch {
        /// Re-download even if already cached
        #[arg(long)]
        force: bool,
        /// Data year (default: 2023)
        #[arg(long, default_value = "2023")]
        year: u16,
    },

    /// Parse NHS shapefile, join attributes, build and cache the HighwayGraph
    Build {
        /// Include US highways and state roads (upgrade candidates) in addition to interstates
        #[arg(long)]
        all_roads: bool,
        /// Path to HPMS CSV (from route fetch-hpms or manual download)
        #[arg(long, value_name = "FILE")]
        hpms: Option<PathBuf>,
    },

    /// Fetch HPMS traffic data from FHWA geo.dot.gov (no registration required)
    FetchHpms {
        /// Output file (default: data/cache/hpms_2018.csv)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Fetch only these states (comma-separated, e.g. CA,NV,UT)
        #[arg(long, value_name = "STATES")]
        states: Option<String>,
    },

    /// Score one corridor against the 12-dimension pool
    Score {
        /// Interstate designation, e.g. "I-80" or "I80"
        designation: String,
        /// Mark all scores as estimated (for proposed corridors)
        #[arg(long)]
        estimated: bool,
        /// Write corpus entry to proposed/ instead of existing/
        #[arg(long)]
        proposed: bool,
    },

    /// Score all corridors in the graph, compute national betweenness centrality
    ScoreAll {
        /// Number of parallel workers (default: num CPUs)
        #[arg(long)]
        workers: Option<usize>,
    },

    /// Analyze scored corpus; identify gap corridors by type
    Gap {
        /// Gap type to detect
        #[arg(long, value_enum)]
        r#type: GapType,
        /// Output gap finding to gaps/{slug}.md
        #[arg(long, value_name = "SLUG")]
        slug: Option<String>,
    },

    /// Render a corridor map to PNG
    Map {
        /// Interstate designation
        designation: String,
        /// Output file (default: maps/{designation}.png)
        #[arg(long, short, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Color corridor by dimension score
        #[arg(long, value_name = "DIM", help = "e.g. a1, b1, d1")]
        color_by: Option<String>,
    },

    /// (Re)generate corpus entry markdown from current scores
    Report {
        /// Interstate designation
        designation: String,
    },

    /// Compute max-flow capacity of a corridor and identify bottleneck segments
    Flow {
        /// Corridor designation (e.g. "I-80", "US30")
        designation: String,
    },

    /// Solve investment allocation LP — which corridors to upgrade within a budget
    Invest {
        /// Budget in billions USD (e.g. 500 for $500B)
        #[arg(long, default_value_t = 100.0)]
        budget: f64,
        /// Include US highway upgrade candidates (default: interstates only)
        #[arg(long)]
        include_upgrades: bool,
        /// Top N corridors to show (default: 20)
        #[arg(long, default_value_t = 20)]
        top: usize,
    },

    /// Compute highway network coverage — how far is anyone from an on-ramp?
    Coverage {
        /// Distance threshold in miles (default: 30)
        #[arg(long, default_value_t = 30.0)]
        threshold: f64,
        /// Grid resolution in miles — only used when no county data available (default: 10)
        #[arg(long, default_value_t = 10.0)]
        grid: f64,
        /// Restrict to T1 corridors only
        #[arg(long)]
        t1_only: bool,
        /// Show top N gap counties/locations (default: 30)
        #[arg(long, default_value_t = 30)]
        top_gaps: usize,
        /// Force geographic grid mode even if county data is available
        #[arg(long)]
        grid_mode: bool,
    },

    /// Fetch ACS county population from Census API (no auth required)
    FetchAcs,

    /// Show tier standards for a given tier
    Standards {
        /// Tier to show (1, 2, 3, or 4)
        #[arg(default_value_t = 1)]
        tier: u8,
    },

    /// Run rubric calibration pass — compute variance stats, flag retirement candidates
    Calibrate,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum GapType {
    MissingLink,
    Bottleneck,
    Resilience,
    Intermodal,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load scoring config
    let scoring_cfg = {
        let path = cli.scoring_config
            .clone()
            .unwrap_or_else(|| PathBuf::from("config/scoring.toml"));
        if path.exists() {
            route_score::ScoringConfig::load(&path)
                .context("loading scoring config")?
        } else {
            eprintln!("note: config/scoring.toml not found — using built-in defaults");
            route_score::ScoringConfig::default_config()
        }
    };

    // Load manifest — check data/manifest.json in project root first, then ~/.route/manifest.json
    let manifest_path = cli.manifest.clone().unwrap_or_else(|| {
        let local = std::path::PathBuf::from("data/manifest.json");
        if local.exists() { local } else { route_data::Manifest::default_path() }
    });

    match cli.command {
        Commands::Fetch { force, year: _ } => {
            println!("route fetch");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            println!("  manifest: {} sources", manifest.sources.len());
            route_data::fetch::fetch_all(&manifest, force)?;
            println!("fetch complete.");
        }

        Commands::Build { all_roads, hpms: hpms_path } => {
            println!("route build{}", if all_roads { " --all-roads" } else { "" });
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;

            let shp_path = ensure_shapefile(&manifest)?;

            println!("  parsing road segments…");
            let segments = route_data::nhs::read_nhs_shapefile(&shp_path, all_roads)
                .map_err(|e| anyhow::anyhow!("shapefile error: {e}"))?;

            let interstate_count = segments.iter().filter(|s| s.route_id.starts_with('I')).count();
            let us_count = segments.iter().filter(|s| s.route_id.starts_with("US")).count();
            println!("  segments: {} total  ({} interstate, {} US highway)",
                segments.len(), interstate_count, us_count);

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

            let (graph, report) = route_network::build_graph(segments, &hpms);
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
            println!("build complete. {} interstates, {} total routes.", route_ids.len(), all_ids.len());
            if !hpms.is_empty() {
                println!("  HPMS joined — A1/A2/A3 scores will use real traffic data.");
            }
        }

        Commands::FetchHpms { output, states } => {
            let out = output.unwrap_or_else(|| PathBuf::from("data/cache/hpms_2018.csv"));
            println!("route fetch-hpms → {}", out.display());
            println!("  source: FHWA geo.dot.gov ArcGIS REST (2018 HPMS, no registration)");

            std::fs::create_dir_all(out.parent().unwrap_or(std::path::Path::new(".")))?;

            if let Some(state_filter) = states {
                // Fetch only specified states
                let filter: Vec<&str> = state_filter.split(',').map(str::trim).collect();
                let mut all: Vec<route_data::HpmsRecord> = Vec::new();
                for (abbr, name) in route_data::STATE_CODES {
                    if filter.iter().any(|f| f.eq_ignore_ascii_case(abbr)) {
                        print!("  [hpms] {abbr}… ");
                        match route_data::hpms_fetch::fetch_state_hpms(abbr, name) {
                            Ok(recs) => { println!("{} segments", recs.len()); all.extend(recs); }
                            Err(e)   => println!("FAILED — {e}"),
                        }
                    }
                }
                // Write subset CSV
                let mut wtr = csv::Writer::from_path(&out)?;
                wtr.write_record(["STATE","ROUTE_ID","AADT","PCT_TRUCK","LANE_COUNT","IRI"])?;
                for r in &all {
                    wtr.write_record(&[
                        r.state.clone(), r.route_id.clone(),
                        r.aadt.map(|v|v.to_string()).unwrap_or_default(),
                        r.pct_truck.map(|v|format!("{v:.4}")).unwrap_or_default(),
                        r.lane_count.map(|v|v.to_string()).unwrap_or_default(),
                        r.iri.map(|v|format!("{v:.1}")).unwrap_or_default(),
                        String::new(), // speed_limit
                    ])?;
                }
                wtr.flush()?;
                println!("  wrote {} records", all.len());
            } else {
                route_data::fetch_all_hpms(&out)?;
            }
            println!("fetch-hpms complete. Run `route build` to join.");
        }

        Commands::Score { designation, estimated, proposed } => {
            let norm = normalise_designation(&designation);
            println!("route score {}", norm);

            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;

            // Build graph from cached shapefile
            let graph = load_graph(&manifest)?;
            println!("  graph: {} edges, {} interstates", graph.graph.edge_count(), graph.interstate_ids().len());

            // Extract corridor
            let mut corridor = route_network::aggregate_corridor(&graph, &norm)
                .ok_or_else(|| anyhow::anyhow!(
                    "Route '{}' not found in graph. Available: {:?}",
                    norm, &graph.interstate_ids()[..graph.interstate_ids().len().min(20)]
                ))?;

            println!("  corridor: {} ({:.0} miles, {} segments)",
                corridor.designation, corridor.total_miles, corridor.edge_count);

            // Score
            let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
            // Print score table
            print_score_table(&corridor.designation, &scores, estimated);

            // Write corpus entry
            let slug = norm.to_lowercase();
            let corpus_dir = if proposed { "corpus/proposed" } else { "corpus/existing" };
            let output_path = PathBuf::from(format!("{corpus_dir}/{slug}.md"));
            route_report::write_corpus_entry(&corridor, &scores, &output_path)?;
            println!("\n  corpus entry → {}", output_path.display());

            if scores.any_estimated() {
                println!("  † Some scores are estimated — see justifications above.");
                println!("    Run `route build` with HPMS data joined to improve accuracy.");
            }
        }

        Commands::ScoreAll { workers } => {
            println!("route score-all");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let mut graph = load_graph(&manifest)?;

            // Compute betweenness centrality on the full graph
            let w = workers.unwrap_or_else(num_cpus);
            println!("  computing betweenness centrality ({w} workers)…");
            let bc = route_network::centrality::compute_edge_betweenness(&graph);
            println!("  centrality: {} edges scored", bc.len());
            graph.edge_betweenness = Some(bc);

            // Score all interstates
            let ids = graph.interstate_ids();
            println!("  scoring {} corridors…", ids.len());

            let mut all_scores = Vec::new();
            for id in &ids {
                if let Some(corridor) = route_network::aggregate_corridor(&graph, id) {
                    let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
                    println!("  {}: {:.1}/120{}", corridor.designation, scores.total(),
                        if scores.any_estimated() { "†" } else { "" });
                    all_scores.push(scores);
                }
            }

            println!("score-all complete: {} corridors scored.", all_scores.len());
        }

        Commands::Gap { r#type, slug } => {
            println!("route gap --type {:?}", r#type);
            let out_slug = slug.unwrap_or_else(|| format!("{:?}", r#type).to_lowercase());
            println!("  [stub] gap analysis output → gaps/{out_slug}.md");
        }

        Commands::Map { designation, output, color_by } => {
            let out = output.unwrap_or_else(|| {
                let slug = designation.to_lowercase().replace('-', "");
                PathBuf::from(format!("maps/{slug}.png"))
            });
            println!("route map {designation} → {}", out.display());
            println!("  [stub] renderer wired — graph load required first.");
            let _ = color_by;
        }

        Commands::Report { designation } => {
            println!("route report {designation}");
            println!("  [stub] corpus entry regeneration — scores required first.");
        }

        Commands::Flow { designation } => {
            let norm = normalise_designation(&designation);
            println!("route flow {norm}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;

            let result = route_network::corridor_max_flow(&graph, &norm)
                .ok_or_else(|| anyhow::anyhow!("Route '{}' not found in graph", norm))?;

            let lane_note = if result.has_lane_data { "" } else { "† (default 2-lane assumed — no HPMS data)" };
            println!("\n┌──────────────────────────────────────────────────────┐");
            println!("│  {} — Corridor Flow Capacity", norm);
            println!("├──────────────────────────────────────────────────────┤");
            println!("│  Binding throughput (min segment):  {:>10.0} vpd  │", result.max_flow_vpd);
            println!("│  Mean corridor capacity:            {:>10.0} vpd  │", result.mean_capacity_vpd);
            println!("│  Segments analyzed:                 {:>10}      │", result.augmenting_paths);
            println!("│  Bottleneck count:                  {:>10}      │", result.bottleneck_edges.len());
            println!("└──────────────────────────────────────────────────────┘");

            for (i, &ei) in result.bottleneck_edges.iter().enumerate() {
                let edge = &graph.graph[ei];
                let cap = result.bottleneck_capacities.get(i).cloned().unwrap_or(0.0);
                let gain = result.lane_addition_gain.get(i).cloned().unwrap_or(0.0);
                let lanes = edge.lane_count.map(|l| l.to_string()).unwrap_or("?".into());
                println!("\n  Binding bottleneck:");
                println!("    Route: {}  State: {}  Lanes: {}",
                    edge.route_id, if edge.state.is_empty() { "—" } else { &edge.state }, lanes);
                println!("    Capacity: {:.0} vpd  |  +1 lane adds: +{:.0} vpd", cap, gain);
            }
            println!("\n  {} vpd = vehicles per day", "vpd");
            if !lane_note.is_empty() {
                println!("  {lane_note}");
                println!("  Run `route fetch-hpms --states <state>` then `route build` for real lane counts.");
            }
        }

        Commands::Invest { budget, include_upgrades, top } => {
            println!("route invest --budget ${budget}B{}", if include_upgrades { " --include-upgrades" } else { "" });
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;

            // Build candidate list from graph
            let route_ids: Vec<String> = if include_upgrades {
                graph.route_ids()
            } else {
                graph.interstate_ids()
            };

            let candidates: Vec<route_network::InvestmentCandidate> = route_ids
                .iter()
                .filter_map(|id| {
                    route_network::aggregate_corridor(&graph, id).map(|c| {
                        route_network::InvestmentCandidate::from_corridor(
                            id,
                            &c.designation,
                            c.total_miles,
                            c.attributes.is_upgrade_candidate,
                        )
                    })
                })
                .collect();

            println!("  {} corridors in candidate pool", candidates.len());

            let plan = route_network::allocate_investment(&candidates, budget);

            println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
            println!("│  Investment Plan — ${:.0}B budget", budget);
            println!("├─────────────────────────────────────────────────────────────────────────┤");
            println!("│  Allocated: ${:.1}B of ${:.0}B budget", plan.allocated_b, plan.budget_b);
            println!("│  Total throughput gain: {:.0} vehicles/day", plan.total_throughput_gain_vpd);
            println!("│  Corridors funded: {}", plan.items.len());
            println!("├──────┬───────────────┬───────────┬──────────────┬───────────────────────┤");
            println!("│ Rank │ Corridor      │    Miles  │   Cost ($B)  │ Gain (vpd)  | Type    │");
            println!("├──────┼───────────────┼───────────┼──────────────┼───────────────────────┤");

            for (i, item) in plan.items.iter().take(top).enumerate() {
                let type_label = match item.upgrade_type {
                    route_network::UpgradeType::InterstateWidening      => "widen  ",
                    route_network::UpgradeType::UsHighwayToInterstate   => "US→Int ",
                    route_network::UpgradeType::StateHighwayToInterstate=> "SR→Int ",
                    route_network::UpgradeType::Greenfield              => "new    ",
                };
                let alloc_pct = if item.allocation < 0.999 {
                    format!("{:.0}%", item.allocation * 100.0)
                } else {
                    "100%".to_string()
                };
                println!("│ {:>4} │ {:<13} │ {:>6.0} mi │ {:>8.1} {} │ {:>10.0}  │ {} │",
                    i + 1, item.designation, item.miles,
                    item.cost_b, alloc_pct, item.throughput_gain_vpd, type_label);
            }
            println!("└──────┴───────────────┴───────────┴──────────────┴───────────────────────┘");
            println!("\n  Costs: widen=$10M/mi, US→Int=$30M/mi, SR→Int=$40M/mi, new=$75M/mi (rough FHWA ranges)");
            println!("  † Upgrade costs and throughput gains are order-of-magnitude estimates.");
            println!("  † Run `route score-all` to improve gain estimates with real AADT data.");
        }

        Commands::FetchAcs => {
            println!("route fetch-acs — Census ACS 5-year county population");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            std::fs::create_dir_all(&manifest.cache_dir)?;
            let out = manifest.cache_dir.join("acs_county_pop_2022.csv");
            route_data::fetch_acs_population(&out)?;
            println!("  saved → {}", out.display());
            println!("  run `route fetch` to get county gazetteer, then `route coverage` for population-weighted analysis.");
        }

        Commands::Coverage { threshold, grid, t1_only, top_gaps, grid_mode } => {
            println!("route coverage --threshold {threshold}mi{}{}",
                if t1_only { " --t1-only" } else { "" },
                if grid_mode { " --grid-mode" } else { "" });
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;
            let t1_ids = ["I5","I10","I35","I40","I75","I80","I90","I95"];
            let filter: Option<&[&str]> = if t1_only { Some(&t1_ids) } else { None };
            let interchange_count = graph.graph.node_indices()
                .filter(|&ni| graph.graph[ni].is_interchange).count();

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
                std::fs::read_dir(&manifest.cache_dir).ok()
                    .and_then(|entries| {
                        entries.filter_map(|e| e.ok())
                            .find(|e| e.file_name().to_string_lossy().ends_with("counties_national.txt"))
                            .map(|e| e.path())
                    })
            } else { None };

            if let Some(ref path) = county_path {
                // Population-weighted county centroid analysis
                println!("  mode: county centroids ({interchange_count} interchange nodes)");
                let mut counties = route_data::read_county_gazetteer(path)
                    .context("reading county gazetteer")?;
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

                let result = route_network::compute_pop_coverage(&graph, &counties, filter, threshold);
                let tc = result.total_counties as f64;
                let tp = result.total_population as f64;
                let tl = result.total_land_sqmi;

                println!("\n┌──────────────────────────────────────────────────────────────┐");
                println!("│  Coverage — {}mi threshold{}  [county centroid mode]",
                    threshold, if t1_only { " T1-only" } else { "" });
                println!("├──────────────────────────────────────────────────────────────┤");
                println!("│  Counties analyzed:  {:>8} total                          │", result.total_counties);
                println!("│                      {:>8} within 20mi  ({:.1}%)          │",
                    result.counties_within_20mi, result.counties_within_20mi as f64/tc*100.0);
                println!("│                      {:>8} within 30mi  ({:.1}%)          │",
                    result.counties_within_30mi, result.counties_within_30mi as f64/tc*100.0);
                println!("│                      {:>8} within 50mi  ({:.1}%)          │",
                    result.counties_within_50mi, result.counties_within_50mi as f64/tc*100.0);
                if has_pop && tp > 0.0 {
                    println!("│  Population:                                                 │");
                    println!("│    Within 20mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_20mi, result.pop_within_20mi as f64/tp*100.0);
                    println!("│    Within 30mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_30mi, result.pop_within_30mi as f64/tp*100.0);
                    println!("│    Within 50mi:  {:>12} ({:.1}% of US)               │",
                        result.pop_within_50mi, result.pop_within_50mi as f64/tp*100.0);
                }
                println!("│  Land area within 30mi:  {:>9.0} sq mi  ({:.1}% of US)  │",
                    result.land_within_30mi, result.land_within_30mi/tl*100.0);
                println!("│  Gap counties (>{}mi): {:>8}                           │",
                    threshold, result.gap_counties.len());
                println!("│  Worst gap:          {:>9.1} miles  ({}, {})           │",
                    result.max_gap_miles,
                    result.gap_counties.first().map(|g| g.name.as_str()).unwrap_or("—"),
                    result.gap_counties.first().map(|g| g.state.as_str()).unwrap_or("—"));
                println!("└──────────────────────────────────────────────────────────────┘");

                if !result.gap_counties.is_empty() {
                    let label = if t1_only { "T1" } else { "any interstate" };
                    println!("\n  Top {} counties >{}mi from {} on-ramp:", top_gaps, threshold, label);
                    println!("  {:>6}  {:<28} {:>5}  {:>8}  {:>10}",
                        "Miles", "County", "State", "Pop", "Land(sqmi)");
                    println!("  {}", "─".repeat(66));
                    for gap in result.gap_counties.iter().take(top_gaps) {
                        println!("  {:>5.1}mi  {:<28} {:>5}  {:>8}  {:>10.0}",
                            gap.nearest_miles, gap.name, gap.state,
                            gap.population, gap.aland_sqmi);
                    }

                    // Save gap list to CSV for paper B.1
                    let gap_csv = std::path::PathBuf::from("data/coverage-gaps.csv");
                    if let Ok(mut wtr) = csv::Writer::from_path(&gap_csv) {
                        let _ = wtr.write_record(["GEOID","NAME","STATE","LAT","LON","NEAREST_MI","POPULATION","LAND_SQMI"]);
                        for g in &result.gap_counties {
                            let _ = wtr.write_record(&[
                                g.geoid.clone(), g.name.clone(), g.state.clone(),
                                format!("{:.4}", g.lat), format!("{:.4}", g.lon),
                                format!("{:.1}", g.nearest_miles),
                                g.population.to_string(),
                                format!("{:.0}", g.aland_sqmi),
                            ]);
                        }
                        println!("\n  gap list saved → {}", gap_csv.display());
                    }

                    println!("\n  I2.0 target: 99% of counties within 30mi via T2+T3 combined");
                    println!("  T3 rural spurs / new T3 designations needed: {}", result.gap_counties.len());
                }
            } else {
                // Fallback: geometric grid mode
                println!("  mode: geographic grid ({}mi resolution) — run `route fetch` for county data",
                    grid);
                println!("  NOTE: includes ocean cells; county centroid mode is more accurate");
                let result = route_network::coverage::compute_coverage(&graph, filter, grid, threshold);
                println!("  cells: {} total, {:.1}% within 30mi, max gap {:.1}mi",
                    result.total_cells, result.pct_within_30mi, result.max_gap_miles);
                println!("  For accurate results: run `route fetch` to download county gazetteer,");
                println!("  then `route fetch-acs` for population, then `route coverage` again.");
            }
        }

        Commands::Standards { tier } => {
            match tier {
                1 => {
                    println!("=== TIER 1 — Primary Arteries ===");
                    println!("PTI target:           ≤ 1.15 (freight lanes) / ≤ 1.30 (GP)");
                    println!("Express freight lanes: 2 per direction, physically separated");
                    println!("Design speed:         65 mph sustained");
                    println!("EV charging:          ≥150kW DC fast, every 50 miles, 8+ chargers");
                    println!("Truck EV:             ≥350kW at freight terminals");
                    println!("Rest areas:           Every 100 miles, 50+ truck spaces, full service");
                    println!("Transit hub:          8 platforms, 2,000 parking at T1/T1 diamonds");
                    println!("Bus frequency:        ≤ 2 hours per direction");
                    println!("Resilience spurs:     Every 50 miles (rural)");
                    println!("Diamond k-connect:    k ≥ 3 at all T1/T1 intersections");
                    println!("Climate hardening:    Full SFHA protection");
                    println!("Intermodal spurs:     1 per state traversed");
                    println!("Bridge target:        All fair+ by 2030");
                    println!("C-D roads:            Required in all metros >500k");
                }
                2 => {
                    println!("=== TIER 2 — Major Connectors ===");
                    println!("PTI target:           ≤ 1.30");
                    println!("Freight lanes:        None — truck-friendly design, no dedicated lanes");
                    println!("Design speed:         65 mph");
                    println!("EV charging:          ≥100kW DC fast, every 75 miles, 4+ chargers");
                    println!("Truck EV:             ≥150kW at fuel stops");
                    println!("Rest areas:           Every 150 miles, 20+ truck spaces, enhanced");
                    println!("Transit stops:        4 platforms, 500 parking at T1/T2 interchanges");
                    println!("Bus frequency:        ≤ 4 hours per direction");
                    println!("Resilience spurs:     Every 75 miles (rural)");
                    println!("Diamond k-connect:    k ≥ 2 at T2/T2 intersections");
                    println!("Bridge target:        All fair+ by 2035");
                    println!("Capacity expansion:   Only where V/C > 0.90 at peak");
                }
                3 => {
                    println!("=== TIER 3 — Regional Feeders ===");
                    println!("PTI target:           ≤ 1.50 (functional reliability)");
                    println!("Freight lanes:        None — standard lanes, no corridor restrictions");
                    println!("Design speed:         65 mph (55 mph acceptable mountainous)");
                    println!("EV charging:          ≥50kW DC fast, every 100 miles, 2+ chargers");
                    println!("Rest areas:           Every 200 miles, 10 truck spaces, basic");
                    println!("Transit nodes:        Shelter + demand-responsive, 50-100 parking");
                    println!("Bus:                  Demand-responsive, min 2 round trips/day");
                    println!("Resilience spurs:     Every 100 miles (rural)");
                    println!("Rural access spurs:   ≤10mi, for communities >5k pop >30mi from T1/T2/T3");
                    println!("Bridge target:        All fair+ by 2040");
                    println!("Coverage role:        Fill 30-mile coverage gaps");
                }
                4 => {
                    println!("=== TIER 4 — Local Access ===");
                    println!("Standard:             Maintenance and safety only. No expansion.");
                    println!("Pavement:             IRI ≤ 170 (fair) by 2040");
                    println!("Bridges:              All fair+ by 2045");
                    println!("Safety:               Standard signing, guardrails, interchange lighting");
                    println!("EV:                   Preserve rest area sites for future; no new requirement");
                    println!("Transit:              None required");
                    println!("Freight:              Posted restrictions only where bridge-specific");
                }
                _ => println!("Error: tier must be 1, 2, 3, or 4"),
            }
        }

        Commands::Calibrate => {
            println!("route calibrate");
            println!("  [stub] reads scored corpus from personas/axis-pool.md ledger.");
            println!("  runs after route score-all produces a full ledger.");
        }
    }

    Ok(())
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Normalise user input to internal route ID: "I-80" → "I80", "i80" → "I80"
fn normalise_designation(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

/// Ensure the TIGER shapefile is extracted; return path to .shp file.
fn ensure_shapefile(manifest: &route_data::Manifest) -> Result<std::path::PathBuf> {
    let extract_dir = manifest.cache_dir.join("tiger-primary-roads");
    let shp_path = extract_dir.join("tl_2023_us_primaryroads.shp");
    if shp_path.exists() {
        return Ok(shp_path);
    }
    let zip_path = manifest.cache_path("tiger-primary-roads");
    if !zip_path.exists() {
        anyhow::bail!("TIGER primary roads not cached — run `route fetch` first.");
    }
    println!("  extracting shapefile…");
    route_data::fetch::extract_shp(&zip_path, &extract_dir)
}

/// Load the HighwayGraph from cached TIGER + optional HPMS.
fn load_graph(manifest: &route_data::Manifest) -> Result<route_network::HighwayGraph> {
    let shp_path = ensure_shapefile(manifest)?;
    // Always load all road classes — US highways needed for upgrade-candidate scoring
    let segments = route_data::nhs::read_nhs_shapefile(&shp_path, true)
        .map_err(|e| anyhow::anyhow!("shapefile error: {e}"))?;

    // Auto-load HPMS if cached
    let hpms_path = manifest.cache_dir.join("hpms_2018.csv");
    let hpms = if hpms_path.exists() {
        route_data::hpms::read_hpms_csv(&hpms_path).unwrap_or_default()
    } else {
        Vec::new()
    };

    let (graph, _) = route_network::build_graph(segments, &hpms);
    Ok(graph)
}

/// Print a formatted score table to stdout.
fn print_score_table(designation: &str, scores: &route_score::DimensionScores, all_estimated: bool) {
    println!("\n┌─────────────────────────────────────────────────────────────────────┐");
    println!("│  {} — Dimension Scores (rubric {})", designation, scores.rubric_version);
    println!("├──────┬──────────────────────────────┬───────┬────────────────────────┤");
    println!("│ Dim  │ Name                         │ Score │ Est │");
    println!("├──────┼──────────────────────────────┼───────┼─────┤");

    let all = [
        &scores.a1, &scores.a2, &scores.a3,
        &scores.b1, &scores.b2, &scores.b3,
        &scores.c1, &scores.c2, &scores.c3,
        &scores.d1, &scores.d2, &scores.d3,
    ];

    for sd in all {
        let est = if sd.estimated || all_estimated { "†" } else { " " };
        println!("│ {:4} │ {:<28} │ {:>5.1} │  {}  │",
            sd.dim.code(), sd.dim.name(), sd.score, est);
    }

    println!("├──────┴──────────────────────────────┼───────┼─────┤");
    println!("│ Band A (Flow)                        │ {:>5.1} │     │", scores.band_a());
    println!("│ Band B (Network)                     │ {:>5.1} │     │", scores.band_b());
    println!("│ Band C (People)                      │ {:>5.1} │     │", scores.band_c());
    println!("│ Band D (Future)                      │ {:>5.1} │     │", scores.band_d());
    println!("│ TOTAL                                │ {:>5.1} │ /120│", scores.total());
    println!("└──────────────────────────────────────┴───────┴─────┘");
}
