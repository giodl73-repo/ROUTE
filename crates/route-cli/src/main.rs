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
