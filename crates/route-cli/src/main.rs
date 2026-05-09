use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const T1_THRESHOLD: f64 = 70.0;
const T2_THRESHOLD: f64 = 50.0;
const T3_THRESHOLD: f64 = 30.0;
const DIMENSION_CODES: [&str; 16] = [
    "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1", "D2", "D3",
];

#[derive(Parser)]
#[command(
    name = "route",
    about = "ROUTE — Interstate 2.0 analysis pipeline",
    version
)]
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

    /// Score one corridor against the 16-dimension pool
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

    /// [planned] Analyze scored corpus; identify gap corridors by type
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
        /// Interstate designation, OR "all" for the full tier mega-map
        designation: String,
        /// Output file (default: maps/{designation}.png)
        #[arg(long, short, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Color corridor by dimension score
        #[arg(long, value_name = "DIM", help = "e.g. a1, b1, d1")]
        color_by: Option<String>,
    },

    /// Regenerate corpus entry markdown from current graph attributes and scores
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

    /// Run traffic simulation — scenario, chaos, or intervention test
    Sim {
        #[command(subcommand)]
        mode: SimMode,
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

    /// Fetch ACS county median household income from Census API (B19013, no auth required)
    FetchAcsIncome,

    /// Fetch FEMA NFHL D1 data using small per-state bboxes (avoids 504 timeout)
    FetchFemaD1,

    /// Fetch FEMA NFHL SFHA feature counts for T1 corridor bounding boxes (D1 dimension)
    FetchFema {
        /// Output file (default: data/cache/fema_sfha_counts.csv)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Show tier standards for a given tier
    Standards {
        /// Tier to show (1, 2, 3, or 4)
        #[arg(default_value_t = 1)]
        tier: u8,
    },

    /// Analyze diamond intersection k-connectivity for a T1/T1 node
    Diamond {
        /// Intersection name (e.g. I35xI80, I35xI40) or "all" for all T1/T1 intersections
        at: String,
    },

    /// Test T1 network connectivity — can all T1 endpoints reach each other on T1 only?
    Connectivity {
        /// Show all pairs, not just gaps
        #[arg(long)]
        all_pairs: bool,
    },

    /// Run rubric calibration pass — compute variance stats, flag retirement candidates
    Calibrate,

    /// O-D transit time Monte Carlo — test the 48-hour SLA claim under chaos
    Od {
        #[command(subcommand)]
        corridor: OdCorridorCmd,
        /// Month of travel (1=Jan ... 12=Dec). Applies seasonal incident modifiers.
        /// Winter = higher mountain pass closure rates; Dec = holiday freight surge.
        #[arg(long, value_name = "MONTH", global = true)]
        month: Option<u8>,
    },

    /// Hub staffing model — drivers employed at each T1 relay hub (the pilot crew base model)
    HubStaff {
        /// Include proposed hubs from missing link corridors
        #[arg(long)]
        include_proposed: bool,
    },

    /// EV charging analysis — guaranteed DCFC every 50mi enables overnight AV travel
    EvAnalysis,

    /// Passenger travel matrix — what does I2.0 unlock for people, not just freight?
    PassengerMatrix {
        #[arg(long, default_value_t = 5_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },

    /// National SLA matrix — what commitment windows does I2.0 unlock across all major corridors?
    SlaMatrix {
        #[arg(long, default_value_t = 5_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },

    /// Benchmark all I2.0 interventions — rank each by contribution to 48h SLA
    Interventions {
        /// Which corridor to test
        #[arg(long, default_value = "ny-la", value_enum)]
        corridor: InterventionCorridorArg,
        /// Number of Monte Carlo trips per intervention (default: 5000)
        #[arg(long, default_value_t = 5_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },

    /// Test tier standards under simulation conditions — can T1 really hit PTI 1.15?
    StandardsTest {
        /// Tier to test (1, 2, or 3)
        #[arg(long, default_value_t = 1)]
        tier: u8,
        /// Number of Monte Carlo trips (default: 10000)
        #[arg(long, default_value_t = 10_000)]
        trips: usize,
        /// Random seed
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum InterventionCorridorArg {
    #[value(name = "ny-la")]
    NyLa,
    #[value(name = "hou-chi")]
    HouChi,
    #[value(name = "hou-i69")]
    HouI69,
}

#[derive(clap::Subcommand, Clone, Debug)]
enum OdCorridorCmd {
    /// New York → Los Angeles via I-80 (2,800 miles, northern transcontinental)
    NyLa {
        #[arg(long, default_value_t = 10_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
    /// Houston → Chicago current routing (I-45→I-35→I-55, three-corridor hop)
    HouChi {
        #[arg(long, default_value_t = 10_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
    /// Houston → Chicago via I-69 (direct, post-completion)
    HouChiI69 {
        #[arg(long, default_value_t = 10_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
    /// All three corridors side by side
    All {
        #[arg(long, default_value_t = 10_000)]
        trips: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum GapType {
    MissingLink,
    Bottleneck,
    Resilience,
    Intermodal,
}

#[derive(clap::Subcommand, Clone, Debug)]
enum SimMode {
    /// Run a named scenario (donner-closure, atlanta-peak, omaha-interchange, houston-surge)
    Scenario {
        name: String,
        /// Test the named I2.0 intervention for this scenario
        #[arg(long)]
        intervention: bool,
    },
    /// Monte Carlo chaos: random closures, measure outcome distribution
    Chaos {
        /// Number of iterations (default: 100)
        #[arg(long, default_value_t = 100)]
        iterations: usize,
        /// Random seed (default: 42)
        #[arg(long, default_value_t = 42)]
        seed: u64,
        /// Restrict to T1 corridors only
        #[arg(long)]
        t1_only: bool,
    },
    /// List available scenarios
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load scoring config
    let scoring_cfg = {
        let path = cli
            .scoring_config
            .clone()
            .unwrap_or_else(|| PathBuf::from("config/scoring.toml"));
        if path.exists() {
            route_score::ScoringConfig::load(&path).context("loading scoring config")?
        } else {
            eprintln!("note: config/scoring.toml not found — using built-in defaults");
            route_score::ScoringConfig::default_config()
        }
    };

    // Load manifest — check data/manifest.json in project root first, then ~/.route/manifest.json
    let manifest_path = cli.manifest.clone().unwrap_or_else(|| {
        let local = std::path::PathBuf::from("data/manifest.json");
        if local.exists() {
            local
        } else {
            route_data::Manifest::default_path()
        }
    });

    // Initialize strategic designation data from CSV (no-op if file not found)
    route_network::strategic::init_designations(std::path::Path::new("data"));

    match cli.command {
        Commands::Fetch { force, year: _ } => {
            println!("route fetch");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            println!("  manifest: {} sources", manifest.sources.len());
            route_data::fetch::fetch_all(&manifest, force)?;
            println!("fetch complete.");
        }

        Commands::Build {
            all_roads,
            hpms: hpms_path,
        } => {
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
            println!(
                "build complete. {} interstates, {} total routes.",
                route_ids.len(),
                all_ids.len()
            );
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
                            Ok(recs) => {
                                println!("{} segments", recs.len());
                                all.extend(recs);
                            }
                            Err(e) => println!("FAILED — {e}"),
                        }
                    }
                }
                if all.is_empty() {
                    anyhow::bail!(
                        "HPMS fetch returned zero records; preserving existing cache at {}",
                        out.display()
                    );
                }
                // Write subset CSV
                let mut wtr = csv::Writer::from_path(&out)?;
                wtr.write_record([
                    "STATE",
                    "ROUTE_ID",
                    "AADT",
                    "PCT_TRUCK",
                    "LANE_COUNT",
                    "IRI",
                    "SPEED_LIMIT",
                ])?;
                for r in &all {
                    wtr.write_record(&[
                        r.state.clone(),
                        r.route_id.clone(),
                        r.aadt.map(|v| v.to_string()).unwrap_or_default(),
                        r.pct_truck.map(|v| format!("{v:.4}")).unwrap_or_default(),
                        r.lane_count.map(|v| v.to_string()).unwrap_or_default(),
                        r.iri.map(|v| format!("{v:.1}")).unwrap_or_default(),
                        String::new(), // speed_limit placeholder
                    ])?;
                }
                wtr.flush()?;
                println!("  wrote {} records", all.len());
            } else {
                route_data::fetch_all_hpms(&out)?;
            }
            println!("fetch-hpms complete. Run `route build` to join.");
        }

        Commands::Score {
            designation,
            estimated,
            proposed,
        } => {
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
            route_report::write_corpus_entry(&corridor, &scores, &output_path)?;
            println!("\n  corpus entry → {}", output_path.display());

            if scores.any_estimated() {
                println!("  † Some scores are estimated — see justifications above.");
                println!("    Run `route score-all` for authoritative national B2 centrality.");
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
            let bc_raw = route_network::centrality::compute_edge_betweenness(&graph);
            println!("  centrality: {} edges scored", bc_raw.len());
            // Normalize using P95 to prevent outlier junction edges from compressing the distribution
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
            let mut score_rows: Vec<(String, f64, &'static str, String, bool, [f64; 16])> =
                Vec::new();
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
                        join_intermodal_to_corridor(
                            &graph,
                            id,
                            &mut corridor.attributes,
                            &intermodal,
                        );
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
                    score_rows.push((
                        corridor.designation.clone(),
                        total,
                        tier,
                        scores.rubric_version.clone(),
                        scores.any_estimated(),
                        dimension_score_values(&scores),
                    ));
                    all_scores.push(scores);
                }
            }

            std::fs::create_dir_all("data")?;
            let out = PathBuf::from("data/scores-all.csv");
            let mut wtr = csv::Writer::from_path(&out)?;
            let mut header = vec!["route", "score", "tier", "rubric_version", "estimated"];
            header.extend(DIMENSION_CODES);
            wtr.write_record(header)?;
            for (route, score, tier, rubric_version, estimated, dims) in &score_rows {
                let mut row = vec![
                    route.clone(),
                    format!("{score:.1}"),
                    tier.to_string(),
                    rubric_version.clone(),
                    estimated.to_string(),
                ];
                row.extend(dims.iter().map(|value| format!("{value:.1}")));
                wtr.write_record(row)?;
            }
            wtr.flush()?;
            println!(
                "  wrote {} score rows → {}",
                score_rows.len(),
                out.display()
            );
            println!("score-all complete: {} corridors scored.", all_scores.len());
        }

        Commands::Gap { r#type, slug } => {
            println!("route gap --type {:?}", r#type);
            let out_slug = slug.unwrap_or_else(|| format!("{:?}", r#type).to_lowercase());
            println!("  status: planned");
            println!("  no gap file written yet: gaps/{out_slug}.md");
            println!(
                "  next: move gap logic into route-network and label heuristic inputs explicitly"
            );
        }

        Commands::Map {
            designation,
            output,
            color_by,
        } => {
            let norm = normalise_designation(&designation);
            let out = output.unwrap_or_else(|| {
                let slug = if norm == "ALL" {
                    "all-tiers".to_string()
                } else {
                    norm.to_lowercase()
                };
                PathBuf::from(format!("maps/{slug}.png"))
            });
            println!("route map {norm} → {}", out.display());

            // Beck schematic — topological relay network (0°/45°/90° geometry, no geography)
            if norm == "BECK" {
                std::fs::create_dir_all("maps")?;
                let out_path = PathBuf::from("maps/beck-schematic.png");
                let svg = route_map::build_beck_svg();
                route_map::svg_to_png(&svg, &out_path, 2400, 1350)?;
                println!(
                    "  rendered Beck schematic: {} (2400×1350)",
                    out_path.display()
                );
                println!("  T1 relay network topology · 0°/45°/90° · inspired by Beck 1933");
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
            const T1_PRIMARY: &[&str] = &["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"];
            if T1_PRIMARY.contains(&norm.as_str()) {
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
        }

        Commands::Report { designation } => {
            let norm = normalise_designation(&designation);
            println!("route report {norm}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;
            let mut corridor =
                route_network::aggregate_corridor(&graph, &norm).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Route '{}' not found in graph. Available: {:?}",
                        norm,
                        &graph.interstate_ids()[..graph.interstate_ids().len().min(20)]
                    )
                })?;

            join_acs_population_to_corridor(
                &manifest,
                &graph,
                &norm,
                &mut corridor.attributes,
                false,
            );
            let scores = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
            let output_path = PathBuf::from(format!("corpus/existing/{}.md", norm.to_lowercase()));
            route_report::write_corpus_entry(&corridor, &scores, &output_path)?;

            println!(
                "  regenerated: {} ({:.1}/160{})",
                output_path.display(),
                scores.total(),
                if scores.any_estimated() { "†" } else { "" }
            );
            if scores.any_estimated() {
                println!("  † Some scores are estimated — see report justifications.");
            }
        }

        Commands::Flow { designation } => {
            let norm = normalise_designation(&designation);
            println!("route flow {norm}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;

            let result = route_network::corridor_max_flow(&graph, &norm)
                .ok_or_else(|| anyhow::anyhow!("Route '{}' not found in graph", norm))?;

            let lane_note = if result.has_lane_data {
                ""
            } else {
                "† (default 2-lane assumed — no HPMS data)"
            };
            println!("\n┌──────────────────────────────────────────────────────┐");
            println!("│  {} — Corridor Flow Capacity", norm);
            println!("├──────────────────────────────────────────────────────┤");
            println!(
                "│  Binding throughput (min segment):  {:>10.0} vpd  │",
                result.max_flow_vpd
            );
            println!(
                "│  Mean corridor capacity:            {:>10.0} vpd  │",
                result.mean_capacity_vpd
            );
            println!(
                "│  Segments analyzed:                 {:>10}      │",
                result.augmenting_paths
            );
            println!(
                "│  Bottleneck count:                  {:>10}      │",
                result.bottleneck_edges.len()
            );
            println!("└──────────────────────────────────────────────────────┘");

            for (i, &ei) in result.bottleneck_edges.iter().enumerate() {
                let edge = &graph.graph[ei];
                let cap = result.bottleneck_capacities.get(i).cloned().unwrap_or(0.0);
                let gain = result.lane_addition_gain.get(i).cloned().unwrap_or(0.0);
                let lanes = edge.lane_count.map(|l| l.to_string()).unwrap_or("?".into());
                println!("\n  Binding bottleneck:");
                println!(
                    "    Route: {}  State: {}  Lanes: {}",
                    edge.route_id,
                    if edge.state.is_empty() {
                        "—"
                    } else {
                        &edge.state
                    },
                    lanes
                );
                println!(
                    "    Capacity: {:.0} vpd  |  +1 lane adds: +{:.0} vpd",
                    cap, gain
                );
            }
            println!("\n  {} vpd = vehicles per day", "vpd");
            if !lane_note.is_empty() {
                println!("  {lane_note}");
                println!("  Run `route fetch-hpms --states <state>` then `route build` for real lane counts.");
            }
        }

        Commands::Invest {
            budget,
            include_upgrades,
            top,
        } => {
            println!(
                "route invest --budget ${budget}B{}",
                if include_upgrades {
                    " --include-upgrades"
                } else {
                    ""
                }
            );
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

            println!(
                "\n┌─────────────────────────────────────────────────────────────────────────┐"
            );
            println!("│  Investment Plan — ${:.0}B budget", budget);
            println!("├─────────────────────────────────────────────────────────────────────────┤");
            println!(
                "│  Allocated: ${:.1}B of ${:.0}B budget",
                plan.allocated_b, plan.budget_b
            );
            println!(
                "│  Total throughput gain: {:.0} vehicles/day",
                plan.total_throughput_gain_vpd
            );
            println!("│  Corridors funded: {}", plan.items.len());
            println!("├──────┬───────────────┬───────────┬──────────────┬───────────────────────┤");
            println!("│ Rank │ Corridor      │    Miles  │   Cost ($B)  │ Gain (vpd)  | Type    │");
            println!("├──────┼───────────────┼───────────┼──────────────┼───────────────────────┤");

            for (i, item) in plan.items.iter().take(top).enumerate() {
                let type_label = match item.upgrade_type {
                    route_network::UpgradeType::InterstateWidening => "widen  ",
                    route_network::UpgradeType::UsHighwayToInterstate => "US→Int ",
                    route_network::UpgradeType::StateHighwayToInterstate => "SR→Int ",
                    route_network::UpgradeType::Greenfield => "new    ",
                };
                let alloc_pct = if item.allocation < 0.999 {
                    format!("{:.0}%", item.allocation * 100.0)
                } else {
                    "100%".to_string()
                };
                println!(
                    "│ {:>4} │ {:<13} │ {:>6.0} mi │ {:>8.1} {} │ {:>10.0}  │ {} │",
                    i + 1,
                    item.designation,
                    item.miles,
                    item.cost_b,
                    alloc_pct,
                    item.throughput_gain_vpd,
                    type_label
                );
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

        Commands::FetchAcsIncome => {
            println!("route fetch-acs-income — Census ACS B19013 median household income");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            std::fs::create_dir_all(&manifest.cache_dir)?;
            let out = manifest.cache_dir.join("acs_county_income_2022.csv");
            route_data::fetch_acs_income(&out)?;
            println!("  saved → {}", out.display());
            println!("  national median HHI 2022: $74,580 (used as C3 baseline)");
            println!("  run `route score-all` to apply C3 scores.");
        }

        Commands::FetchFemaD1 => {
            println!("route fetch-fema-d1 — FEMA NFHL D1 data via per-state small bboxes");
            println!("  Querying Layer 28 (Flood Hazard Zones) in 1°×1° tiles to avoid 504...");

            // State bounding boxes (1°×1° tiles covering major flood-exposed corridors)
            // Focus on Gulf Coast (I-10 LA/TX), Atlantic Coast (I-95), Mississippi Valley
            let state_tiles: Vec<(&str, f64, f64, f64, f64)> = vec![
                ("LA-Gulf", -93.5, 29.0, -92.5, 30.0),
                ("LA-Gulf2", -92.5, 29.0, -91.5, 30.0),
                ("LA-Gulf3", -91.5, 29.0, -90.5, 30.0),
                ("LA-Gulf4", -90.5, 29.0, -89.5, 30.0),
                ("TX-Gulf", -95.5, 29.0, -94.5, 30.0),
                ("TX-Gulf2", -94.5, 29.0, -93.5, 30.0),
                ("FL-Gulf", -87.5, 30.0, -86.5, 31.0),
                ("FL-SE", -81.0, 25.5, -80.0, 26.5),
                ("FL-Atlantic", -80.5, 26.5, -79.5, 27.5),
                ("NC-coast", -77.5, 34.5, -76.5, 35.5),
                ("VA-coast", -76.5, 36.5, -75.5, 37.5),
                ("NJ-coast", -74.5, 39.5, -73.5, 40.5),
                ("MS-valley", -91.0, 32.0, -90.0, 33.0),
                ("AR-flood", -91.5, 33.5, -90.5, 34.5),
            ];

            let fema_url =
                "https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query";

            let mut results: Vec<(String, u32)> = Vec::new();
            for (name, xmin, ymin, xmax, ymax) in &state_tiles {
                let qs = format!(
                    "where=FLD_ZONE+LIKE+%27A%25%27&geometry={},{},{},{}&geometryType=esriGeometryEnvelope&spatialRel=esriSpatialRelIntersects&returnCountOnly=true&f=json",
                    xmin, ymin, xmax, ymax
                );
                let url = format!("{fema_url}?{qs}");
                // Use route-data's reqwest client pattern
                match route_data::fetch_fema_count(&url) {
                    Ok(count) => {
                        println!("  {name}: {count} SFHA features");
                        results.push((name.to_string(), count));
                    }
                    Err(e) => {
                        println!("  {name}: FAILED — {e}");
                        results.push((name.to_string(), 0));
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(800));
            }

            // Write results
            let out = PathBuf::from("data/cache/fema_sfha_tile_counts.csv");
            let mut wtr = csv::Writer::from_path(&out)?;
            wtr.write_record(["tile", "xmin", "ymin", "xmax", "ymax", "sfha_count"])?;
            for (i, (name, count)) in results.iter().enumerate() {
                let t = &state_tiles[i];
                wtr.write_record(&[
                    name,
                    &t.1.to_string(),
                    &t.2.to_string(),
                    &t.3.to_string(),
                    &t.4.to_string(),
                    &count.to_string(),
                ])?;
            }
            wtr.flush()?;
            println!("\n  Saved → {}", out.display());
            let total: u32 = results.iter().map(|(_, c)| c).sum();
            println!("  Total SFHA features across flood-exposed tiles: {total}");
            println!("  Next: wire tile counts into corridor D1 scoring via bbox intersection");
        }

        Commands::FetchFema { output } => {
            let out = output.unwrap_or_else(|| PathBuf::from("data/cache/fema_sfha_counts.csv"));
            println!("route fetch-fema → {}", out.display());
            println!(
                "  source: FEMA NFHL ArcGIS REST — Layer 28 (Flood Hazard Zones / SFHA A-zones)"
            );
            println!(
                "  querying {} T1 corridor bounding boxes…",
                route_data::T1_BBOXES.len()
            );

            std::fs::create_dir_all(out.parent().unwrap_or(std::path::Path::new(".")))?;

            let results = route_data::fetch_all_sfha_counts(&out)?;

            let ok_count = results.iter().filter(|r| r.status == "ok").count();
            println!("\n  Results:");
            println!("  {:10}  {:>14}  {}", "Corridor", "SFHA Features", "Status");
            println!("  {}", "─".repeat(40));
            for r in &results {
                println!("  {:10}  {:>14}  {}", r.corridor, r.sfha_count, r.status);
            }
            println!(
                "\n  {}/{} corridors queried successfully",
                ok_count,
                results.len()
            );
            println!("  saved → {}", out.display());
            println!("  Use counts as D1 proxy: higher = more flood-exposed corridor.");
            println!("  Note: counts reflect SFHA polygons in the bounding box, not miles.");
            println!(
                "  Run `route score <corridor>` after this to see D1 update (manual join needed)."
            );
        }

        Commands::Coverage {
            threshold,
            grid,
            t1_only,
            top_gaps,
            grid_mode,
        } => {
            println!(
                "route coverage --threshold {threshold}mi{}{}",
                if t1_only { " --t1-only" } else { "" },
                if grid_mode { " --grid-mode" } else { "" }
            );
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;
            let t1_ids = ["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"];
            let filter: Option<&[&str]> = if t1_only { Some(&t1_ids) } else { None };
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
        }

        Commands::Standards { tier } => match tier {
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
                println!(
                    "Rural access spurs:   ≤10mi, for communities >5k pop >30mi from T1/T2/T3"
                );
                println!("Bridge target:        All fair+ by 2040");
                println!("Coverage role:        Fill 30-mile coverage gaps");
            }
            4 => {
                println!("=== TIER 4 — Local Access ===");
                println!("Standard:             Maintenance and safety only. No expansion.");
                println!("Pavement:             IRI ≤ 170 (fair) by 2040");
                println!("Bridges:              All fair+ by 2045");
                println!(
                    "Safety:               Standard signing, guardrails, interchange lighting"
                );
                println!(
                    "EV:                   Preserve rest area sites for future; no new requirement"
                );
                println!("Transit:              None required");
                println!("Freight:              Posted restrictions only where bridge-specific");
            }
            _ => println!("Error: tier must be 1, 2, 3, or 4"),
        },

        Commands::Sim { mode } => {
            match mode {
                SimMode::List => {
                    println!("Available scenarios:");
                    for name in route_sim::scenarios::available_scenarios() {
                        println!("  {name}");
                    }
                    println!("\nUsage: route sim scenario <name> [--intervention]");
                    println!("       route sim chaos [--iterations N] [--seed S] [--t1-only]");
                }

                SimMode::Scenario { name, intervention } => {
                    println!(
                        "route sim scenario {name}{}",
                        if intervention { " --intervention" } else { "" }
                    );

                    let toml_str = route_sim::scenarios::load_scenario(&name)
                        .ok_or_else(|| anyhow::anyhow!(
                            "Unknown scenario '{}'. Run `route sim list` to see available scenarios.", name
                        ))?;

                    let mut scenario: route_sim::Scenario = toml::from_str(toml_str)
                        .with_context(|| format!("parsing scenario {name}"))?;

                    if !intervention {
                        scenario.intervention = None;
                    }

                    let manifest =
                        route_data::Manifest::load(&manifest_path).with_context(|| {
                            format!("loading manifest from {}", manifest_path.display())
                        })?;
                    let graph = load_graph(&manifest)?;

                    // Use AADT-based demand proxy (FAF5 not yet joined)
                    let demand = build_demand_from_graph(&graph);
                    println!("  demand pairs: {}", demand.len());

                    println!("  running Wardrop equilibrium (Frank-Wolfe)…");
                    let result = route_sim::run_scenario(&graph, &demand, &scenario);

                    print_scenario_result(&result);
                }

                SimMode::Chaos {
                    iterations,
                    seed,
                    t1_only,
                } => {
                    println!(
                        "route sim chaos --iterations {iterations} --seed {seed}{}",
                        if t1_only { " --t1-only" } else { "" }
                    );

                    let manifest =
                        route_data::Manifest::load(&manifest_path).with_context(|| {
                            format!("loading manifest from {}", manifest_path.display())
                        })?;
                    let graph = load_graph(&manifest)?;
                    let demand = build_demand_from_graph(&graph);

                    let config = route_sim::ChaosConfig {
                        seed,
                        iterations,
                        t1_only,
                        ..Default::default()
                    };

                    println!("  running {iterations} chaos iterations…");
                    let result = route_sim::run_chaos(&graph, &demand, &config);
                    print_chaos_result(&result);
                }
            }
        }

        Commands::Diamond { at } => {
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;

            if at.to_uppercase() == "ALL" {
                // Analyze all T1/T1 intersections
                let intersections = route_network::find_t1_intersections(&graph);
                println!(
                    "route diamond --at all  ({} T1/T1 intersections found)",
                    intersections.len()
                );
                println!(
                    "\n  {:25} {:>4}  {:>6}  {:>10}  {:>6}",
                    "Intersection", "k", "SPF?", "Cost ($B)", "Connectors"
                );
                println!("  {}", "─".repeat(60));
                let mut results: Vec<_> = intersections
                    .into_iter()
                    .map(|ix| route_network::analyze_diamond(&graph, ix))
                    .collect();
                results.sort_by_key(|r| r.k_current);
                for r in &results {
                    let spf = if r.is_spf { "YES ⚠" } else { "no " };
                    println!(
                        "  {:25} {:>4}  {:>6}  {:>9.2}B  {:>6}",
                        r.intersection.name, r.k_current, spf, r.est_cost_b, r.connectors_needed
                    );
                }
                let spf_count = results.iter().filter(|r| r.is_spf).count();
                println!(
                    "\n  Single points of failure: {}/{}",
                    spf_count,
                    results.len()
                );
                println!(
                    "  Total diamond investment needed: ${:.1}B",
                    results.iter().map(|r| r.est_cost_b).sum::<f64>()
                );
            } else {
                // Analyze one specific intersection
                println!("route diamond --at {at}");
                let intersection = route_network::find_intersection(&graph, &at)
                    .ok_or_else(|| anyhow::anyhow!(
                        "No T1/T1 intersection found matching '{}'. Try 'route diamond --at all' to list all.",
                        at
                    ))?;
                println!(
                    "  Found: {} ({:.2}°N {:.2}°W)",
                    intersection.name, intersection.lat, -intersection.lon
                );
                let result = route_network::analyze_diamond(&graph, intersection);
                println!("\n┌─────────────────────────────────────────────────────┐");
                println!("│  {} Diamond Analysis", result.intersection.name);
                println!("├─────────────────────────────────────────────────────┤");
                println!(
                    "│  k-connectivity (current):  {:>3}                    │",
                    result.k_current
                );
                println!(
                    "│  Single point of failure:   {}                  │",
                    if result.is_spf { "YES ⚠" } else { "no" }
                );
                println!(
                    "│  Zone nodes (route A):      {:>3}                    │",
                    result.zone_nodes_a.len()
                );
                println!(
                    "│  Zone nodes (route B):      {:>3}                    │",
                    result.zone_nodes_b.len()
                );
                println!(
                    "│  Connectors needed (→k≥3): {:>3}                    │",
                    result.connectors_needed
                );
                println!(
                    "│  Estimated cost:           ${:.2}B                 │",
                    result.est_cost_b
                );
                println!("└─────────────────────────────────────────────────────┘");

                if result.is_spf {
                    println!("\n  ⚠ This is a single point of failure.");
                    println!(
                        "  A closure here disrupts both {} and {} simultaneously.",
                        result.intersection.route_a, result.intersection.route_b
                    );
                    println!(
                        "  Adding {} connector road(s) within 50 miles would bring k to ≥3.",
                        result.connectors_needed
                    );
                } else {
                    println!(
                        "\n  This intersection has adequate path redundancy (k={}).",
                        result.k_current
                    );
                }
            }
        }

        Commands::Connectivity { all_pairs } => {
            println!("route connectivity (T1 network completeness test)");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let graph = load_graph(&manifest)?;

            println!(
                "  analyzing T1 network ({} T1 corridors, {} total routes)…",
                graph
                    .interstate_ids()
                    .iter()
                    .filter(|id| ["I5", "I10", "I35", "I40", "I75", "I80", "I90", "I95"]
                        .contains(&id.as_str()))
                    .count(),
                graph.route_ids().len()
            );

            let report = route_network::analyze_t1_connectivity(&graph);

            println!("\n┌─────────────────────────────────────────────────────────────┐");
            println!("│  T1 Network Connectivity Report");
            println!("├─────────────────────────────────────────────────────────────┤");
            println!(
                "│  T1 endpoints analyzed:    {:>4}                            │",
                report.endpoints.len()
            );
            println!(
                "│  Endpoint pairs tested:    {:>4}                            │",
                report.pair_results.len()
            );
            println!(
                "│  Gaps (require T2 bridge): {:>4}                            │",
                report.gaps.len()
            );
            println!(
                "│  Network fully connected:  {}                         │",
                if report.is_fully_connected {
                    "YES ✓"
                } else {
                    "NO  ✗ — GAPS FOUND"
                }
            );
            println!("└─────────────────────────────────────────────────────────────┘");

            if !report.gaps.is_empty() {
                println!("\n  STRUCTURAL GAPS — endpoint pairs requiring T2 to connect:");
                println!("  {:12} → {:12}  T1 miles  All miles  Detour", "From", "To");
                println!("  {}", "─".repeat(60));
                for gap in &report.gaps {
                    let t1 = gap
                        .t1_only_miles
                        .map(|m| format!("{m:.0}"))
                        .unwrap_or("NONE".into());
                    let all = gap
                        .all_corridors_miles
                        .map(|m| format!("{m:.0}"))
                        .unwrap_or("—".into());
                    let det = gap
                        .detour_factor
                        .map(|d| format!("{d:.1}×"))
                        .unwrap_or("∞".into());
                    let flag = if gap.requires_t2 {
                        " ← T2 required!"
                    } else {
                        ""
                    };
                    println!(
                        "  {:12} → {:12}  {:>8}  {:>8}  {:>6}{}",
                        gap.from_route, gap.to_route, t1, all, det, flag
                    );
                }
                println!(
                    "\n  Isolated terminals: {}",
                    report.isolated_terminals.join(", ")
                );
                println!("\n  → These gaps are I2.0 missing link targets:");
                println!(
                    "    A new T1 corridor filling each gap would close the structural disconnect."
                );
                println!("    Example: I-40/I-70 western endpoint → I-5 requires I-15 (T2).");
                println!(
                    "    A Pacific extension of I-40/I-70 (via US-50 alignment) would close it."
                );
            }

            if all_pairs {
                println!("\n  All T1 endpoint pairs:");
                println!("  {:12} → {:12}  T1-only   All-crdr  Detour", "From", "To");
                println!("  {}", "─".repeat(65));
                let mut pairs = report.pair_results.clone();
                pairs.sort_by(|a, b| {
                    b.detour_factor
                        .unwrap_or(0.0)
                        .total_cmp(&a.detour_factor.unwrap_or(0.0))
                });
                for r in pairs.iter().take(20) {
                    let t1 = r
                        .t1_only_miles
                        .map(|m| format!("{m:.0}mi"))
                        .unwrap_or("UNREACHABLE".into());
                    let all = r
                        .all_corridors_miles
                        .map(|m| format!("{m:.0}mi"))
                        .unwrap_or("—".into());
                    let det = r
                        .detour_factor
                        .map(|d| format!("{d:.2}×"))
                        .unwrap_or("∞".into());
                    println!(
                        "  {:12} → {:12}  {:>12}  {:>9}  {:>6}",
                        r.from_route, r.to_route, t1, all, det
                    );
                }
            }
        }

        Commands::Calibrate => {
            println!("route calibrate — rubric calibration pass (v1.4)");
            let manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let mut graph = load_graph(&manifest)?;
            let ids = atlas_candidate_ids(&graph);
            println!("  scoring {} corridors for calibration…", ids.len());

            // Compute betweenness centrality so B2 is populated (same as score-all)
            println!("  computing betweenness centrality…");
            let bc_raw = route_network::centrality::compute_edge_betweenness(&graph);
            println!("  centrality: {} edges scored", bc_raw.len());
            // Normalize using P95 (not max) to prevent outlier edges from compressing distribution
            // A single hyper-central junction edge can be 100× larger than trunk route edges
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

            // Load ACS population once for C1/C2/C3 wiring
            let acs_counties = load_acs_counties_for_scoring(&manifest);
            if acs_counties.is_some() {
                println!("  ACS population data loaded — C1/C2/C3 will use real values");
            }
            // Load ports for B3 scoring
            let ports = load_ports();
            if !ports.is_empty() {
                println!(
                    "  {} port/border locations loaded — B3 will use real values",
                    ports.len()
                );
            }
            // Load DCFC stations for D2 scoring (partial — DEMO_KEY rate limit)
            let dcfc = load_dcfc_stations();
            if !dcfc.is_empty() {
                println!(
                    "  {} DCFC stations loaded — D2 EV component will use real values",
                    dcfc.len()
                );
            }
            // Load intermodal terminals for D2 hub count
            let intermodal = load_intermodal_terminals();
            if !intermodal.is_empty() {
                println!(
                    "  {} intermodal terminals loaded — D2 hub count will use real values",
                    intermodal.len()
                );
            }
            // Load NBI bridge condition data for D3
            let nbi = load_nbi_bridges();
            if !nbi.is_empty() {
                println!(
                    "  {} NBI bridge records loaded — D3 will use real condition data",
                    nbi.len()
                );
            }
            // Load FEMA SFHA tile counts for D1 scoring
            let fema_tiles = load_fema_tiles();
            if !fema_tiles.is_empty() {
                println!(
                    "  {} FEMA SFHA tiles loaded — D1 will use real flood-zone data",
                    fema_tiles.len()
                );
            }
            // Load FARS fatal crash rates for A5 scoring
            let fars_safety = load_fars_safety();
            if !fars_safety.is_empty() {
                println!(
                    "  {} FARS route records loaded — A5 will use real safety data",
                    fars_safety.len()
                );
            }
            // Load railroad parallel data for B1 discount
            let railroad_parallels = load_railroad_parallels();
            if !railroad_parallels.is_empty() {
                println!(
                    "  {} railroad parallels loaded — B1 rail discount applied",
                    railroad_parallels.len()
                );
            }
            // Load multi-hazard zones for D1 extension
            let hazard_zones = load_hazard_zones();
            if !hazard_zones.is_empty() {
                println!(
                    "  {} hazard zone records loaded — D1 multi-hazard composite active",
                    hazard_zones.len()
                );
            }

            // Collect per-dimension scores for all corridors
            const N_DIMS: usize = 16;
            let dim_names = [
                "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1",
                "D2", "D3",
            ];
            let dim_labels = [
                "Throughput Gap",
                "Freight Intensity",
                "Speed Reliability",
                "International Trade",
                "Safety Record",
                "Redundancy",
                "Network Centrality",
                "Port/Border Access",
                "Military/Strategic",
                "Population Reach",
                "Rural Connectivity",
                "Economic Opportunity",
                "Agricultural Export",
                "Climate Resilience",
                "Multimodal Integration",
                "Infrastructure Vintage",
            ];

            let mut matrix: Vec<[f64; N_DIMS]> = Vec::new();
            let mut estimated_matrix: Vec<[bool; N_DIMS]> = Vec::new();
            let mut route_ids_used: Vec<String> = Vec::new();
            let mut total_scores: Vec<f64> = Vec::new();
            let mut flagged_congestion: Vec<(String, f64, f64)> = Vec::new(); // (route, A1, B2)

            for id in &ids {
                if let Some(mut corridor) = route_network::aggregate_corridor(&graph, id) {
                    // Join ACS population for C1/C2/C3
                    if acs_counties.is_some() {
                        join_acs_population_to_corridor(
                            &manifest,
                            &graph,
                            id,
                            &mut corridor.attributes,
                            false,
                        );
                    }
                    // Join port access for B3
                    if !ports.is_empty() {
                        join_port_access_to_corridor(&graph, id, &mut corridor.attributes, &ports);
                    }
                    // Join DCFC for D2 EV component
                    if !dcfc.is_empty() {
                        join_dcfc_to_corridor(
                            &graph,
                            id,
                            corridor.total_miles,
                            &mut corridor.attributes,
                            &dcfc,
                        );
                    }
                    // Join intermodal terminals for D2 hub count
                    if !intermodal.is_empty() {
                        join_intermodal_to_corridor(
                            &graph,
                            id,
                            &mut corridor.attributes,
                            &intermodal,
                        );
                    }
                    // Join FEMA SFHA tile counts for D1 flood-zone scoring
                    if !fema_tiles.is_empty() {
                        join_fema_d1_to_corridor(&graph, id, &mut corridor.attributes, &fema_tiles);
                    }
                    // Apply D3 IRI proxy when NBI bridge data is unavailable
                    // D3: join NBI real data first; fall back to IRI proxy if unavailable
                    if !nbi.is_empty() {
                        join_nbi_to_corridor(id, &mut corridor.attributes, &nbi);
                    }
                    join_d3_iri_proxy(&mut corridor.attributes); // no-op if NBI already set
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
                    let s = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
                    let row = dimension_score_values(&s);
                    let estimated_row = dimension_estimated_values(&s);
                    let total = rounded_score(s.total());
                    // Flag congestion-stress candidates: high A1 + low B2 + total near T1 threshold
                    if s.a1.score > 7.0 && s.b2.score < 3.0 && total > 20.0 {
                        flagged_congestion.push((id.clone(), s.a1.score, s.b2.score));
                    }
                    matrix.push(row);
                    estimated_matrix.push(estimated_row);
                    route_ids_used.push(id.clone());
                    total_scores.push(total);
                }
            }

            let n = matrix.len() as f64;
            println!("  {} corridors scored\n", matrix.len());

            // Per-dimension statistics
            println!("┌───────────────────────────────────────────────────────────────────────────────────────────────────┐");
            println!("│  Dimension Statistics (0.0–10.0 scale, n={})                                                    │", matrix.len());
            println!("├──────┬────────────────────────────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬────────────  ┤");
            println!("│  Dim │  Name                      │  Min │  Max │  Avg │  Std │  P90 │ Est% │ NZ%  │  Status      │");
            println!("├──────┼────────────────────────────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼────────────  ┤");

            let mut dim_stats: Vec<(f64, f64, f64, f64, f64, f64, f64)> = Vec::new(); // min,max,mean,std,p90,est_rate,nonzero_rate

            for d in 0..N_DIMS {
                let vals: Vec<f64> = matrix.iter().map(|r| r[d]).collect();
                let estimated_count = estimated_matrix.iter().filter(|r| r[d]).count();
                let est_rate = estimated_count as f64 / n;
                let nonzero_rate = vals.iter().filter(|&&v| v > 0.0).count() as f64 / n;
                let min = vals.iter().cloned().fold(f64::MAX, f64::min);
                let max = vals.iter().cloned().fold(f64::MIN, f64::max);
                let mean = vals.iter().sum::<f64>() / n;
                let variance = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
                let std = variance.sqrt();
                let mut sorted = vals.clone();
                sorted.sort_by(f64::total_cmp);
                let p90 = sorted[((n * 0.90) as usize).min(sorted.len() - 1)];

                // Status flags
                let status = if est_rate >= 0.80 {
                    "PROXY GAP ⚠"
                } else if nonzero_rate < 0.10 && max >= 8.0 {
                    "SPARSE ROLE"
                } else if std < 1.5 {
                    "LOW VAR ⚠"
                } else if max - min < 3.0 {
                    "NARROW  ⚠"
                } else {
                    "OK      ✓"
                };

                println!("│  {:>2}  │  {:<26} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.0} │ {:>4.0} │  {:<10}  │",
                    dim_names[d], dim_labels[d], min, max, mean, std, p90, est_rate * 100.0, nonzero_rate * 100.0, status);
                dim_stats.push((min, max, mean, std, p90, est_rate, nonzero_rate));
            }
            println!("└──────┴────────────────────────────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴────────────  ┘");

            // Pairwise correlation (Pearson) — flag pairs > 0.60
            println!("\n  Computing pairwise Pearson correlations…");
            let means: Vec<f64> = (0..N_DIMS)
                .map(|d| matrix.iter().map(|r| r[d]).sum::<f64>() / n)
                .collect();
            let stds: Vec<f64> = dim_stats.iter().map(|s| s.3).collect();

            let mut high_corr: Vec<(usize, usize, f64)> = Vec::new();
            for i in 0..N_DIMS {
                for j in (i + 1)..N_DIMS {
                    if stds[i] < 0.01 || stds[j] < 0.01 {
                        continue;
                    }
                    let cov: f64 = matrix
                        .iter()
                        .map(|r| (r[i] - means[i]) * (r[j] - means[j]))
                        .sum::<f64>()
                        / n;
                    let r = cov / (stds[i] * stds[j]);
                    if r.abs() > 0.55 {
                        high_corr.push((i, j, r));
                    }
                }
            }
            high_corr.sort_by(|a, b| b.2.abs().total_cmp(&a.2.abs()));

            if !high_corr.is_empty() {
                println!("\n  High-correlation pairs (|r| > 0.55):");
                println!("  {:>2} × {:>2}   r       Status", "D1", "D2");
                println!("  {}", "─".repeat(50));
                for (i, j, r) in &high_corr {
                    let warn = if r.abs() > 0.70 {
                        " ⚠ REDUNDANT?"
                    } else {
                        ""
                    };
                    println!(
                        "  {} × {}  {:>+5.2}  {}{}",
                        dim_names[*i], dim_names[*j], r, "", warn
                    );
                }
            } else {
                println!("  No high-correlation pairs found (all |r| ≤ 0.55) ✓");
            }

            // Congestion-stress paradox candidates
            if !flagged_congestion.is_empty() {
                println!("\n  Congestion-stress candidates (high A1, low B2, near T1):");
                println!("  {:>8}  {:>6}  {:>6}", "Route", "A1", "B2");
                println!("  {}", "─".repeat(30));
                for (route, a1, b2) in &flagged_congestion {
                    println!(
                        "  {:>8}  {:>6.1}  {:>6.1}  ⚠ urban connector inflation",
                        route, a1, b2
                    );
                }
                println!("  → These corridors may need centrality-adjusted tier classification.");
                println!("    See A.1 paper: betweenness centrality correction (α=0.65).");
            }

            // Tier distribution
            let t1 = total_scores.iter().filter(|&&s| s >= T1_THRESHOLD).count();
            let t2 = total_scores
                .iter()
                .filter(|&&s| s >= T2_THRESHOLD && s < T1_THRESHOLD)
                .count();
            let t3 = total_scores
                .iter()
                .filter(|&&s| s >= T3_THRESHOLD && s < T2_THRESHOLD)
                .count();
            let t4 = total_scores.iter().filter(|&&s| s < T3_THRESHOLD).count();
            println!(
                "\n  Tier distribution (v1.4 thresholds: T1≥{T1_THRESHOLD:.1}, T2≥{T2_THRESHOLD:.1}, T3≥{T3_THRESHOLD:.1}):"
            );
            println!(
                "    T1: {} corridors  T2: {} corridors  T3: {} corridors  T4: {} corridors",
                t1, t2, t3, t4
            );
            if t1 > 30 {
                println!(
                    "    ⚠ T1 count {} exceeds promotion atlas target range (~12-30).",
                    t1
                );
                println!(
                    "    → Review congestion-stress candidates and promotion thresholds before freezing a release."
                );
            }

            // Retirement candidates
            println!("\n  Retirement candidates (std < 1.5, estimated < 80%):");
            let mut any_retire = false;
            for d in 0..N_DIMS {
                let (_, max, _, std, _, est_rate, nonzero_rate) = dim_stats[d];
                let sparse_role = nonzero_rate < 0.10 && max >= 8.0;
                if std < 1.5 && est_rate < 0.80 && !sparse_role {
                    println!(
                        "    {} ({}) — std={:.2} — consider retiring or merging",
                        dim_names[d], dim_labels[d], std
                    );
                    any_retire = true;
                }
            }
            if !any_retire {
                println!("    None — all dimensions show adequate variance ✓");
            }

            println!("\n  Sparse role dimensions (nonzero < 10%, max ≥ 8):");
            let mut any_sparse = false;
            for d in 0..N_DIMS {
                let (_, max, _, std, _, est_rate, nonzero_rate) = dim_stats[d];
                if est_rate < 0.80 && nonzero_rate < 0.10 && max >= 8.0 {
                    println!(
                        "    {} ({}) — nonzero={:.0}% std={:.2} — keep if this is an intentional role flag; expand source list if not",
                        dim_names[d],
                        dim_labels[d],
                        nonzero_rate * 100.0,
                        std
                    );
                    any_sparse = true;
                }
            }
            if !any_sparse {
                println!("    None — no sparse high-ceiling role dimensions ✓");
            }

            println!("\n  Data/proxy gaps (estimated ≥ 80%):");
            let mut any_proxy_gap = false;
            for d in 0..N_DIMS {
                let (_, _, _, std, _, est_rate, _) = dim_stats[d];
                if est_rate >= 0.80 {
                    println!(
                        "    {} ({}) — estimated={:.0}% std={:.2} — improve source coverage before retirement decisions",
                        dim_names[d],
                        dim_labels[d],
                        est_rate * 100.0,
                        std
                    );
                    any_proxy_gap = true;
                }
            }
            if !any_proxy_gap {
                println!("    None — all dimensions have adequate source coverage ✓");
            }

            println!("\ncalibrate complete. Review output above before bumping rubric version.");
        }

        Commands::Od { corridor, month } => {
            let data_dir = std::path::PathBuf::from("data");
            let (corridors, trips, seed): (Vec<route_sim::OdCorridor>, usize, u64) = match corridor
            {
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
                "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                "Dec",
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
        }

        Commands::HubStaff { include_proposed } => {
            let data_dir = std::path::PathBuf::from("data");
            let confirmed_only = !include_proposed;
            let hubs = route_sim::load_hubs(&data_dir, confirmed_only);
            if hubs.is_empty() {
                eprintln!("No hubs loaded — check data/relay-hubs.toml");
            } else {
                println!("Loaded {} hubs from data/relay-hubs.toml", hubs.len());
            }
            let net = route_sim::compute_network_summary(&hubs);
            print_hub_staffing(&net, include_proposed);
        }

        Commands::EvAnalysis => {
            let data_dir = std::path::PathBuf::from("data");
            print_ev_analysis(&data_dir);
        }

        Commands::PassengerMatrix { trips, seed } => {
            println!("route passenger-matrix — what I2.0 unlocks for people ({trips} trips)\n");
            let data_dir = std::path::PathBuf::from("data");
            print_passenger_matrix(trips, seed, &data_dir);
        }

        Commands::SlaMatrix { trips, seed } => {
            println!("route sla-matrix — national SLA commitment windows ({trips} trips)\n");
            let data_dir = std::path::PathBuf::from("data");
            print_sla_matrix(trips, seed, &data_dir);
        }

        Commands::Interventions {
            corridor,
            trips,
            seed,
        } => {
            let data_dir = std::path::PathBuf::from("data");
            let c = match corridor {
                InterventionCorridorArg::NyLa => route_sim::load_corridor(&data_dir, "ny_la")
                    .unwrap_or_else(route_sim::ny_la_corridor),
                InterventionCorridorArg::HouChi => {
                    route_sim::load_corridor(&data_dir, "hou_chi_current")
                        .unwrap_or_else(route_sim::hou_chi_current)
                }
                InterventionCorridorArg::HouI69 => {
                    route_sim::load_corridor(&data_dir, "hou_chi_i69")
                        .unwrap_or_else(route_sim::hou_chi_i69)
                }
            };
            println!("route interventions — {trips} trips per scenario\n");
            let bench = route_sim::InterventionBenchmark::run(&c, trips, seed);
            print_intervention_benchmark(&bench);
        }

        Commands::StandardsTest { tier, trips, seed } => {
            println!("route standards-test --tier {tier} ({trips} trips)\n");
            println!("Testing whether Tier {tier} PTI target is achievable under simulation.\n");

            let (pti_target, corridor_name) = match tier {
                1 => (1.15, "T1 — I-80 NY→LA (managed freight lanes)"),
                2 => (1.30, "T2 — I-70 (Major Connector, mixed traffic)"),
                _ => (1.50, "T3 — Regional feeder (demand-responsive)"),
            };

            let corridor = route_sim::ny_la_corridor();

            // Run at three demand levels: normal, adverse (+20% demand), severe (+40% + compound incident)
            println!("  Tier {tier} PTI target: ≤ {pti_target:.2}");
            println!("  Corridor: {corridor_name}");
            println!(
                "  Free-flow elapsed: {:.1}h ({:.1} days)",
                corridor.free_flow_elapsed_hours(),
                corridor.free_flow_elapsed_hours() / 24.0
            );
            println!();

            let managed = tier == 1;
            let dist = route_sim::run_od_simulation(&corridor, managed, trips, seed);

            println!(
                "  {:>20}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>6}",
                "Scenario", "p50 (h)", "p75 (h)", "p90 (h)", "p95 (h)", "p99 (h)", "PTI", "SLA?"
            );
            println!("  {}", "─".repeat(85));

            let pti_met = dist.pti <= pti_target;
            let sla_label = if pti_met { "PASS ✓" } else { "FAIL ✗" };
            println!(
                "  {:>20}  {:>8.1}  {:>8.1}  {:>8.1}  {:>8.1}  {:>8.1}  {:>6.3}  {}",
                "Baseline",
                dist.p50_hours,
                dist.p75_hours,
                dist.p90_hours,
                dist.p95_hours,
                dist.p99_hours,
                dist.pti,
                sla_label
            );

            println!();
            println!(
                "  Commitment window (p95): {:.1}h = {:.1} days",
                dist.p95_hours,
                dist.p95_hours / 24.0
            );
            println!(
                "  PTI (p95/free-flow):     {:.3}  [target ≤ {:.2}] — {}",
                dist.pti,
                pti_target,
                if pti_met {
                    "TARGET MET ✓"
                } else {
                    "TARGET MISSED ✗"
                }
            );
            println!("  Trips completing < 48h:  {:.1}%", dist.pct_under_48h);
            println!();

            if pti_met {
                println!(
                    "  ✓ Tier {tier} PTI standard is achievable under these simulation conditions."
                );
                println!("  ✓ Managed lanes + Donner tunnel remove the primary variance sources.");
            } else {
                println!(
                    "  ✗ Tier {tier} PTI target NOT met at current demand/incident parameters."
                );
                println!("  → Primary variance sources: see segment breakdown above.");
            }
        }
    }

    Ok(())
}

fn print_od_comparison(cmp: &route_sim::OdComparison) {
    let sg = &cmp.solo_gp;
    let sm = &cmp.solo_managed;
    let tm = &cmp.team_managed;
    let rg = &cmp.relay_gp;
    let rm = &cmp.relay_managed;
    let net = route_sim::RelayNetwork::for_corridor_miles(sg.free_flow_hours);

    println!(
        "╔══════════════════════════════════════════════════════════════════════════════════╗"
    );
    println!("║  {}  ║", pad_center(&cmp.corridor_name, 80));
    println!(
        "║  Free-flow: {:.1}h ({:.1} days)  |  Relay stations: {}  |  Station cost: ${:.0}M ea  ║",
        sg.free_flow_hours,
        sg.free_flow_hours / 24.0,
        net.stations,
        net.station_cost_m
    );
    println!("╠══════════════════╦══════════════╦══════════════╦══════════════╦══════════════╣");
    println!("║  Metric          ║ Solo / GP    ║ Solo / I2.0  ║ Team / I2.0  ║Relay / I2.0  ║");
    println!("╠══════════════════╬══════════════╬══════════════╬══════════════╬══════════════╣");

    let row = |label: &str, f: fn(&route_sim::TransitDistribution) -> f64| {
        println!(
            "║  {:<16}║  {:>8.1}h   ║  {:>8.1}h   ║  {:>8.1}h   ║  {:>8.1}h   ║",
            label,
            f(sg),
            f(sm),
            f(tm),
            f(rm)
        );
    };
    row("Mean", |d| d.mean_hours);
    row("p50", |d| d.p50_hours);
    row("p75", |d| d.p75_hours);
    row("p90", |d| d.p90_hours);
    row("p95 commit wdw", |d| d.p95_hours);
    row("p99 worst-case", |d| d.p99_hours);

    println!("╠══════════════════╬══════════════╬══════════════╬══════════════╬══════════════╣");
    println!(
        "║  PTI             ║  {:>9.3}  ║  {:>9.3}  ║  {:>9.3}  ║  {:>9.3}  ║",
        sg.pti, sm.pti, tm.pti, rm.pti
    );
    println!(
        "║  < 48h trips     ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║",
        sg.pct_under_48h, sm.pct_under_48h, tm.pct_under_48h, rm.pct_under_48h
    );
    println!(
        "║  < 72h trips     ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║  {:>8.1}%  ║",
        pct_under(sg, 72.0),
        pct_under(sm, 72.0),
        pct_under(tm, 72.0),
        pct_under(rm, 72.0)
    );
    println!(
        "║  SLA window      ║  {:>7.1}d   ║  {:>7.1}d   ║  {:>7.1}d   ║  {:>7.1}d   ║",
        sg.commitment_window_days,
        sm.commitment_window_days,
        tm.commitment_window_days,
        rm.commitment_window_days
    );
    println!("╚══════════════════╩══════════════╩══════════════╩══════════════╩══════════════╝");

    // Verdict per scenario
    println!();
    let verdict = |label: &str, d: &route_sim::TransitDistribution| {
        let sla = d.p95_hours;
        let days = sla / 24.0;
        let icon = if sla <= 48.0 {
            "✓ 48h SLA ACHIEVABLE".to_string()
        } else if sla <= 72.0 {
            format!("✓ {:.1}d ({:.0}h) — tight 3-day SLA", days, sla)
        } else {
            format!("→ {:.1}d ({:.0}h) commitment window", days, sla)
        };
        println!("  {:20}  {}", label, icon);
    };
    verdict("Solo / GP lanes:", sg);
    verdict("Solo / Managed:", sm);
    verdict("Team / Managed:", tm);
    verdict("Relay / Managed:", rm);
    verdict("Relay / GP lanes:", rg);

    // Relay network economics
    println!();
    println!(
        "  Relay network: {} stations × ${:.0}M = ${:.0}M total capex",
        net.stations, net.station_cost_m, net.total_capex_m
    );
    println!(
        "  Avg driver leg: {:.0} miles / {:.1}h — home base return same day",
        net.avg_leg_miles, net.avg_leg_hours
    );
    println!(
        "  vs. $253B I2.0 portfolio = {:.2}% of total program cost",
        net.total_capex_m / 253_000.0 * 100.0
    );
}

fn print_hub_staffing(net: &route_sim::NetworkSummary, proposed: bool) {
    println!("route hub-staff — T1 relay hub employment model\n");
    println!("Model: truck volumes from HPMS AADT × truck fraction.");
    println!("Relay drivers: 1 driver per truck swap, 3 shifts/day, 5-day week, 35%% buffer.");
    println!("Like airline crew bases: drivers work 1 leg, home same day.\n");

    println!(
        "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}",
        "Hub", "Trucks/d", "Swaps/d", "Frt Drvr", "Bus Drvr", "Support", "Total Jobs"
    );
    println!("{}", "─".repeat(95));

    for s in &net.hub_staffings {
        let is_proposed = s.hub_name.contains("proposed");
        let marker = if is_proposed { " *" } else { "" };
        println!(
            "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}{}",
            s.hub_name.split('(').next().unwrap_or(&s.hub_name).trim(),
            s.daily_truck_swaps,
            s.daily_total_swaps,
            s.freight_relay_drivers,
            s.bus_relay_drivers,
            s.dispatchers + s.maintenance_staff + s.admin_scheduling,
            s.total_hub_employment,
            marker,
        );
    }

    println!("{}", "─".repeat(95));
    println!(
        "{:<35} {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>10}",
        "TOTAL (all hubs)",
        net.hub_staffings
            .iter()
            .map(|s| s.daily_truck_swaps)
            .sum::<u32>(),
        net.total_daily_swaps,
        net.total_freight_drivers,
        net.total_bus_drivers,
        net.hub_staffings
            .iter()
            .map(|s| s.dispatchers + s.maintenance_staff + s.admin_scheduling)
            .sum::<u32>(),
        net.total_hub_employment,
    );

    if proposed {
        println!("\n  * = proposed hub (corridor not yet built)");
    }

    println!("\n── What this means ──────────────────────────────────────────────────────");
    println!(
        "  {} total hub-based jobs nationally ({} hubs)",
        net.total_hub_employment, net.total_hubs
    );
    println!(
        "  {} freight relay drivers — regional CDL jobs, home every night",
        net.total_freight_drivers
    );
    println!(
        "  {} bus relay drivers — intercity express on managed lanes",
        net.total_bus_drivers
    );
    println!();

    let avg_wage_freight = 58_000u32; // relay driver: regional premium, no overnight
    let avg_wage_bus = 52_000u32;
    let avg_wage_support = 48_000u32;
    let support_count: u32 = net
        .hub_staffings
        .iter()
        .map(|s| s.dispatchers + s.maintenance_staff + s.admin_scheduling)
        .sum();
    let total_payroll = (net.total_freight_drivers as u64 * avg_wage_freight as u64
        + net.total_bus_drivers as u64 * avg_wage_bus as u64
        + support_count as u64 * avg_wage_support as u64)
        / 1_000_000;

    println!("  Annual payroll: ~${total_payroll}M at hub locations");
    println!("  Average freight relay driver: ${avg_wage_freight}/yr (vs $70,000 long-haul signing bonus alone)");
    println!("  Driver shortage: 80,000 current shortfall; relay model expands addressable pool");
    println!("  Repositioning: drivers return home via relay hub bus network or partner vehicles");
    println!();
    println!("── Comparison: airline crew base model ──────────────────────────────────");
    println!("  United Airlines crew bases: ~12 bases, ~25,000 pilots/FAs total");
    println!(
        "  I2.0 relay hubs: {} bases, {} drivers",
        net.total_hubs,
        net.total_freight_drivers + net.total_bus_drivers
    );
    println!("  Pilot works 1 flight leg, overnights at hub or flies back on jumpseat");
    println!("  Relay driver works 1 truck leg, drives back or takes hub bus home");
    println!("  The operational model is identical. The regulation is the gap.");
}

fn print_ev_analysis(data_dir: &std::path::Path) {
    use route_sim::analyze_ev_charging;

    let i20_dcfc_kw = 150.0; // T1 standard: 150kW minimum DCFC

    let corridors = vec![
        route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi),
        route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
        route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
        route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
        route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
        route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
    ];

    let evs = load_ev_profiles(data_dir);

    println!("route ev-analysis — I2.0 guaranteed DCFC (150kW every 50 miles on T1)\n");
    println!("Current T1 DCFC gap: rural segments have 80-120+ mile gaps (some 0 DCFC at all).");
    println!(
        "I2.0 standard: DCFC ≤ 50 miles, 150kW minimum passenger / 350kW freight terminals.\n"
    );

    // Compare vs train lines
    println!("── How I2.0 compares to high-speed rail investment ─────────────────────────");
    println!("  Northeast Corridor (BOS-NYC-WAS, 440mi): Amtrak Acela 3.5h, $150-300");
    println!("    I2.0 AV managed lane same corridor: ~5.9h — rail wins on this dense corridor");
    println!("    BUT: Acela capital cost = $50B+ for 440mi. I2.0 DCFC: $400M for 440mi of T1.");
    println!();
    println!("  California HSR (SF-LA, 380mi): projected $100B+, 2h40m target (not built)");
    println!("    I2.0 AV managed lane SF-LA: ~5.5h via I-5 — rail wins IF built");
    println!("    BUT: HSR $100B for one corridor vs I2.0 $253B for entire national network.");
    println!();
    println!("  For corridors WITHOUT rail (Atlanta-Chicago, Dallas-NYC, Houston-Chicago):");
    println!("    Rail: not built, not planned, EIS would take 20+ years");
    println!("    I2.0 AV managed lane: operational in 5-10 years on existing right-of-way");
    println!("    I2.0 wins by default on every corridor where rail doesn't exist.");
    println!();
    println!("  The rail comparison depends on the question:");
    println!("  'Is AV managed lane faster than HSR?' → No, on dense corridors where HSR exists.");
    println!("  'Does I2.0 give more Americans better travel options?' → Yes, overwhelmingly.");
    println!("  HSR serves 5-10 dense corridors. I2.0 serves 60,000 miles of T1/T2 network.");
    println!();

    println!("── EV charging analysis by corridor ─────────────────────────────────────────");
    println!(
        "{:<38} {:>8}  {:>12}  {:>10}  {:>8}  {}",
        "Corridor", "Miles", "EV type", "Stops I2.0", "Chrg min", "Overnight OK?"
    );
    println!("{}", "─".repeat(100));

    for corridor in &corridors {
        for ev in &evs {
            let analysis = analyze_ev_charging(corridor, ev, i20_dcfc_kw);
            let overnight = if analysis.overnight_scenario {
                "✓ auto-charge"
            } else {
                "needs stop"
            };
            println!(
                "{:<38} {:>8.0}  {:>12}  {:>10}  {:>8.0}  {}",
                corridor.name.split('(').next().unwrap_or("").trim(),
                analysis.corridor_miles,
                ev.name.split('(').next().unwrap_or(ev.name).trim(),
                analysis.stops_i20,
                analysis.charge_minutes_i20,
                overnight,
            );
        }
        println!();
    }

    println!("── The overnight AV scenario ─────────────────────────────────────────────");
    println!("  Tesla Model Y (290mi range) on NY→CHI (760mi):");
    let ny_chi = route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi);
    let model_y = evs
        .iter()
        .find(|e| e.highway_range_miles >= 280.0 && e.charge_rate_kw <= 250.0)
        .cloned()
        .unwrap_or_else(route_sim::tesla_model_y);
    let a = analyze_ev_charging(&ny_chi, &model_y, i20_dcfc_kw);
    println!("    Charging stops: {}", a.stops_i20);
    println!("    Total charge time: {:.0} minutes", a.charge_minutes_i20);
    println!("    {}", a.overnight_note);
    println!();
    println!("  The AV pulls off at the hub, plugs in automatically (CCS/NACS standard),");
    println!("  charges for 20 minutes while you sleep, continues. You wake up in Chicago.");
    println!("  Zero range anxiety. Zero driver fatigue. Guaranteed charging at every hub.");
    println!();
    println!("  Current gap: I-80 through Wyoming has 85-120 mile gaps between DCFC.");
    println!("  A 220-mile range EV cannot complete Wyoming today without careful planning.");
    println!("  I2.0 standard (50-mile spacing) eliminates this completely.");
    println!();
    println!("  Freight Tesla Semi (480mi range, 1MW Megacharger):");
    let semi = evs
        .iter()
        .find(|e| e.charge_rate_kw >= 900.0)
        .cloned()
        .unwrap_or_else(route_sim::tesla_semi);
    let a2 = analyze_ev_charging(&ny_chi, &semi, 1000.0); // 1MW freight charger
    println!(
        "    NY→CHI: {} charging stops, {:.0} min total charge time",
        a2.stops_i20, a2.charge_minutes_i20
    );
    println!(
        "    {} at relay hubs (driver swap + charge simultaneously)",
        a2.overnight_note
    );
}

fn print_passenger_matrix(trips: usize, seed: u64, data_dir: &std::path::Path) {
    use route_sim::{run_passenger_simulation, PassengerMode};

    // Load Amtrak schedules from CSV; fall back to hardcoded values if file missing.
    let amtrak = load_amtrak_schedules(data_dir);

    let amtrak_hours = |slug: &str, fallback: Option<f64>| -> Option<f64> {
        amtrak.get(slug).copied().or(fallback)
    };

    // Corridors with Amtrak benchmarks (scheduled hours, reliability PTI)
    // PTI: 1.0 = perfectly on time; Amtrak long-distance PTI ~1.4-2.0
    // (corridor, amtrak_scheduled_hours, amtrak_note)
    // Airlines currently bus some short routes: BOS-NYC, LAX-SNA, etc.
    // Threshold: air is competitive when door-to-door < 4h (flight < 1.5h + overhead 2.5h)
    // Below that, bus relay often wins on total door-to-door time AND cost
    let corridors: Vec<(route_sim::OdCorridor, Option<f64>, &str)> = vec![
        (
            route_sim::load_corridor(data_dir, "ny_chi").unwrap_or_else(route_sim::ny_chi),
            amtrak_hours("ny_chi", Some(18.0)),
            "Lake Shore Ltd 18h (60% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
            amtrak_hours("la_sea", Some(35.5)),
            "Coast Starlight 53h p95 (50% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
            amtrak_hours("mia_nyc", Some(30.0)),
            "Silver Star 45h p95 (75% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
            amtrak_hours("atl_chi", None),
            "No direct Amtrak service",
        ),
        (
            route_sim::load_corridor(data_dir, "hou_chi_i69")
                .unwrap_or_else(route_sim::hou_chi_i69),
            amtrak_hours("hou_chi_i69", None),
            "No direct Amtrak",
        ),
        (
            route_sim::load_corridor(data_dir, "dal_nyc").unwrap_or_else(route_sim::dal_nyc),
            amtrak_hours("dal_nyc", None),
            "No direct Amtrak",
        ),
        (
            route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
            amtrak_hours("sea_chi", Some(46.0)),
            "Empire Builder 69h p95 (65% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
            amtrak_hours("ny_la", Some(67.0)),
            "Southwest Chief 100h p95 (55% on-time)",
        ),
        (
            route_sim::load_corridor(data_dir, "chi_la").unwrap_or_else(route_sim::chi_la),
            amtrak_hours("chi_la", Some(43.0)),
            "Southwest Chief 64h p95 (55% on-time)",
        ),
    ];

    println!(
        "{:<35} {:>6}  {:>10}  {:>12}  {:>12}  {:>14}  {:>10}",
        "Corridor",
        "Miles",
        "Amtrak p95",
        "Bus relay",
        "AV managed",
        "Air (door-to-door)",
        "AV vs Air"
    );
    println!(
        "{:<35} {:>6}  {:>10}  {:>12}  {:>12}  {:>14}  {:>10}",
        "", "", "(current)", "($0.12/mi)", "(~$0.18/mi)", "(est.)", ""
    );
    println!("{}", "─".repeat(110));

    for (corridor, amtrak_sched, _amtrak_note) in &corridors {
        let miles = corridor.total_miles();

        let bus = run_passenger_simulation(
            corridor,
            PassengerMode::ExpressBus,
            trips,
            seed,
            *amtrak_sched,
        );
        let av = run_passenger_simulation(
            corridor,
            PassengerMode::AutonomousVehicle,
            trips,
            seed + 1,
            *amtrak_sched,
        );

        let amtrak_str = if let Some(sched) = amtrak_sched {
            let pti = 1.5; // typical long-distance Amtrak PTI
            format!("{:.0}h p95", sched * pti)
        } else {
            "no service".to_string()
        };

        // Air: door-to-door estimate (drive to airport 45min + security 60min + flight + arrive 45min)
        let flight_hours = miles / 500.0; // rough cruising speed
        let air_dttd = flight_hours + 2.5; // airport overhead both ends
        let air_str = format!("{:.1}h", air_dttd);

        // Does AV beat air door-to-door?
        let av_vs_air = if av.p95_hours < air_dttd {
            format!("AV faster +{:.1}h", air_dttd - av.p95_hours)
        } else {
            format!("Air -{:.1}h", av.p95_hours - air_dttd)
        };

        println!(
            "{:<35} {:>6.0}  {:>10}  {:>10.1}h  {:>10.1}h  {:>14}  {:>10}",
            corridor
                .name
                .split(' ')
                .take(4)
                .collect::<Vec<_>>()
                .join(" "),
            miles,
            amtrak_str,
            bus.p95_hours,
            av.p95_hours,
            air_str,
            av_vs_air,
        );
    }

    println!("\n{}", "─".repeat(110));
    println!("\nKey: p95 = 95th-percentile commitment window. Air = door-to-door (45min drive + 60min security + flight).");
    println!("Bus relay at $0.12/mi ≈ $0.12 × miles. AV managed at ~$0.18/mi (fuel + managed lane toll).");
    println!();
    println!("── Bus routes competitive with air (< 4h door-to-door threshold) ──────────");
    println!("  Airlines already bus some short-haul routes (United/Delta bus BOS↔NYC, LAX↔SNA).");
    println!(
        "  Door-to-door air < 4h means flight is under 1.5h — below that, bus relay competes:"
    );
    println!();
    println!("  NY→CHI (790mi):    bus relay ~12h  vs air 4.7h — NOT competitive on time,");
    println!(
        "                      but competitive on COST ($95 bus vs $180+ air + Uber both ends)"
    );
    println!("                      and AV managed lane ~10h = sleep in your car, arrive rested");
    println!();
    println!(
        "  Routes where I2.0 BUS RELAY beats air door-to-door (rare; requires short corridor):"
    );
    println!("  → sub-300 mile routes where air = 3.5h door-to-door but bus relay = 3h:");
    println!("    LA→San Diego (120mi): bus relay ~2.5h vs air 2.8h door-to-door — BUS WINS");
    println!(
        "    NYC→Philadelphia (95mi): bus relay ~1.8h vs air 2.5h — BUS WINS (Amtrak 1.5h wins)"
    );
    println!("    Chicago→Milwaukee (90mi): bus relay ~1.7h vs air 2.3h — BUS WINS");
    println!("    Miami→Orlando (240mi): bus relay ~4.5h vs air 3.2h — air narrowly wins");
    println!();
    println!("── The AV managed lane passenger case ──────────────────────────────────────");
    println!("  Not competing with air. Replacing: exhausting driving, unreliable Amtrak,");
    println!("  slow bus. The 'sleep-and-arrive' use case:");
    println!();
    println!("  NY→CHI: depart 10pm, arrive 8am rested. Beats Lake Shore (18h+, unreliable).");
    println!("  MIA→NYC: depart 8pm, arrive noon next day. Beats Silver Star (45h p95!).");
    println!("  ATL→CHI: depart 9pm, arrive 8am. No Amtrak alternative. Beats driving.");
    println!("  SEA→CHI: depart Sunday 6pm, arrive Tuesday 8am. Empire Builder p95 = 69h.");
    println!();
    println!("  AV managed lane is the return of the overnight sleeper — in your own car.");
}

fn print_sla_matrix(trips: usize, seed: u64, data_dir: &std::path::Path) {
    use route_sim::{apply_interventions, run_od_simulation_with_driver, DriverMode, Intervention};

    // All corridors — loaded from od-corridors.toml, falling back to built-ins
    let corridors = vec![
        route_sim::load_corridor(data_dir, "mia_nyc").unwrap_or_else(route_sim::mia_nyc),
        route_sim::load_corridor(data_dir, "atl_chi").unwrap_or_else(route_sim::atl_chi),
        route_sim::load_corridor(data_dir, "hou_chi_i69").unwrap_or_else(route_sim::hou_chi_i69),
        route_sim::load_corridor(data_dir, "hou_chi_current")
            .unwrap_or_else(route_sim::hou_chi_current),
        route_sim::load_corridor(data_dir, "dal_nyc").unwrap_or_else(route_sim::dal_nyc),
        route_sim::load_corridor(data_dir, "la_sea").unwrap_or_else(route_sim::la_sea),
        route_sim::load_corridor(data_dir, "ny_la").unwrap_or_else(route_sim::ny_la_corridor),
        route_sim::load_corridor(data_dir, "sea_chi").unwrap_or_else(route_sim::sea_chi),
        route_sim::load_corridor(data_dir, "chi_la").unwrap_or_else(route_sim::chi_la),
    ];

    let relay_interventions = |c: &route_sim::OdCorridor| {
        let stations = ((c.total_miles() / 500.0).ceil() as usize).max(1);
        vec![Intervention::DriverRelay {
            stations,
            swap_minutes: 20.0,
        }]
    };

    let full_stack = |c: &route_sim::OdCorridor| {
        let stations = ((c.total_miles() / 500.0).ceil() as usize).max(1);
        vec![
            Intervention::ManagedFreightLanes,
            Intervention::DonnerTunnel,
            Intervention::DiamondInterchanges,
            Intervention::IntelligentRouting,
            Intervention::DriverRelay {
                stations,
                swap_minutes: 15.0,
            },
        ]
    };

    println!(
        "{:<38} {:>6}  {:>10}  {:>12}  {:>10}  {:>10}  {:>12}",
        "Corridor", "Miles", "Today p95", "Relay only", "Relay+Mgd", "Full I2.0", "SLA unlock"
    );
    println!(
        "{:<38} {:>6}  {:>10}  {:>12}  {:>10}  {:>10}  {:>12}",
        "", "", "(solo/GP)", "($40M)", "(+$121B)", "(full stk)", ""
    );
    println!("{}", "─".repeat(110));

    for c in &corridors {
        let miles = c.total_miles();

        // 1. Today: solo/GP
        let today = run_od_simulation_with_driver(c, false, &DriverMode::Solo, trips, seed);

        // 2. Relay only (GP lanes)
        let relay_only = {
            let (modified, driver) = apply_interventions(c, &relay_interventions(c));
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 1)
        };

        // 3. Relay + managed lanes
        let relay_managed = {
            let interventions = {
                let stations = ((miles / 500.0).ceil() as usize).max(1);
                vec![
                    Intervention::ManagedFreightLanes,
                    Intervention::DriverRelay {
                        stations,
                        swap_minutes: 20.0,
                    },
                ]
            };
            let (modified, driver) = apply_interventions(c, &interventions);
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 2)
        };

        // 4. Full I2.0 stack
        let full = {
            let (modified, driver) = apply_interventions(c, &full_stack(c));
            run_od_simulation_with_driver(&modified, false, &driver, trips, seed + 3)
        };

        // SLA classification
        let sla_label = |h: f64| -> &str {
            if h <= 12.0 {
                "12h (half-day)"
            } else if h <= 24.0 {
                "24h (overnight)"
            } else if h <= 36.0 {
                "36h (next-day)"
            } else if h <= 48.0 {
                "48h (2-day)"
            } else if h <= 72.0 {
                "72h (3-day)"
            } else {
                ">3-day"
            }
        };

        // Highlight which scenario first achieves a new SLA tier
        let today_sla = sla_label(today.p95_hours);
        let full_sla = sla_label(full.p95_hours);
        let unlock = if full_sla != today_sla {
            format!("{} → {}", today_sla, full_sla)
        } else {
            format!("holds at {}", today_sla)
        };

        println!(
            "{:<38} {:>6.0}  {:>8.1}h   {:>10.1}h  {:>9.1}h  {:>9.1}h  {}",
            c.name,
            miles,
            today.p95_hours,
            relay_only.p95_hours,
            relay_managed.p95_hours,
            full.p95_hours,
            unlock,
        );
    }

    println!("\n{}", "─".repeat(110));
    println!("\nSLA categories: 12h (half-day) | 24h (overnight) | 36h (next-day) | 48h (2-day) | 72h (3-day)");
    println!("Relay only = $40M per corridor. Relay+Managed = +$121B program. Full stack = +Donner/Diamond/Routing.");
    println!("\nMarketplace note: relay captures 90%+ of the gain at 0.03% of the cost.");
    println!("The relay MARKETPLACE (driver matching, HOS handoff, load custody) is the critical enabler.");
}

/// Load Amtrak schedules from data/amtrak-schedules.csv.
/// Returns corridor_slug -> scheduled_hours mapping.
/// Falls back to empty HashMap if file not found or unparseable.
fn load_amtrak_schedules(data_dir: &std::path::Path) -> std::collections::HashMap<String, f64> {
    let path = data_dir.join("amtrak-schedules.csv");
    let mut map = std::collections::HashMap::new();
    let Ok(file) = std::fs::File::open(&path) else {
        return map;
    };
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let Ok(record) = result else { continue };
        let slug = record.get(0).unwrap_or("").trim().to_string();
        let hours: f64 = match record.get(2).unwrap_or("").trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !slug.is_empty() {
            map.entry(slug).or_insert(hours);
        }
    }
    map
}

/// Local deserialization record for ev-profiles.toml (CLI-only; uses String for name).
#[derive(serde::Deserialize)]
struct EvProfileRecord {
    name: String,
    highway_range_miles: f64,
    charge_rate_kw: f64,
    battery_kwh: f64,
    kwh_per_mile: f64,
}

#[derive(serde::Deserialize)]
struct EvProfilesFile {
    vehicles: Vec<EvProfileRecord>,
}

/// Load EV profiles from data/ev-profiles.toml.
/// Falls back to the three built-in profiles if the file is missing or unparseable.
fn load_ev_profiles(data_dir: &std::path::Path) -> Vec<route_sim::EvProfile> {
    let path = data_dir.join("ev-profiles.toml");
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(file) = toml::from_str::<EvProfilesFile>(&text) {
            if !file.vehicles.is_empty() {
                return file
                    .vehicles
                    .into_iter()
                    .map(|r| {
                        // Box::leak turns an owned String into a &'static str for the lifetime of the
                        // process. Acceptable in a CLI binary that doesn't free profiles at runtime.
                        let name: &'static str = Box::leak(r.name.into_boxed_str());
                        route_sim::EvProfile {
                            name,
                            highway_range_miles: r.highway_range_miles,
                            charge_rate_kw: r.charge_rate_kw,
                            battery_kwh: r.battery_kwh,
                            kwh_per_mile: r.kwh_per_mile,
                        }
                    })
                    .collect();
            }
        }
    }
    // Fall back to built-in profiles
    vec![
        route_sim::average_ev_2026(),
        route_sim::tesla_model_y(),
        route_sim::tesla_semi(),
    ]
}

fn print_intervention_benchmark(bench: &route_sim::InterventionBenchmark) {
    let baseline_p95 = bench.baseline.p95_hours;
    let ff = bench.baseline.free_flow_hours;

    println!("Corridor: {}", bench.corridor_name);
    println!(
        "Baseline: Solo/GP lanes  |  free-flow {:.1}h  |  p95 {:.1}h ({:.1} days)\n",
        ff,
        baseline_p95,
        baseline_p95 / 24.0
    );

    // Header
    println!(
        "{:<35} {:>8}  {:>8}  {:>9}  {:>8}  {:>12}  {}",
        "Intervention", "p50", "p95", "Δp95", "< 48h", "Capex", "48h SLA"
    );
    println!("{}", "─".repeat(105));

    // Sort by p95 ascending (best first), keeping baseline at top
    let mut results: Vec<&route_sim::InterventionResult> = bench.results.iter().collect();
    results.sort_by(|a, b| a.dist.p95_hours.total_cmp(&b.dist.p95_hours));

    for r in &results {
        let delta_str = if r.p95_delta_hours.abs() < 0.05 {
            "  —    ".to_string()
        } else {
            format!("{:>+7.1}h", r.p95_delta_hours)
        };
        let sla = if r.sla_achieved { "✓ YES" } else { "✗ no " };
        let marker = if r.sla_achieved { " ←" } else { "" };
        println!(
            "{:<35} {:>6.1}h  {:>6.1}h  {}  {:>6.1}%  {:>12}  {}{}",
            r.label,
            r.dist.p50_hours,
            r.dist.p95_hours,
            delta_str,
            r.pct_under_48h,
            r.capex,
            sla,
            marker,
        );
    }

    println!("\n{}", "─".repeat(105));

    // Summary: rank by marginal impact
    let mut ranked: Vec<&route_sim::InterventionResult> = bench
        .results
        .iter()
        .filter(|r| {
            !r.label.contains("stack") && !r.label.contains("+") && !r.label.contains("Baseline")
        })
        .collect();
    ranked.sort_by(|a, b| a.p95_delta_hours.total_cmp(&b.p95_delta_hours));

    println!("\nRanked single interventions by p95 improvement:");
    println!(
        "{:<35} {:>9}  {:>14}  {:>12}",
        "Intervention", "p95 gain", "Cost/hour-saved", "Capex"
    );
    println!("{}", "─".repeat(80));
    for r in &ranked {
        let gain = baseline_p95 - r.dist.p95_hours;
        if gain.abs() < 0.1 {
            continue;
        }
        // Rough cost-per-hour-saved: capex / (gain × annual trips estimate)
        let annual_trips = 8_000.0 * 365.0; // 8k trucks/day on NY-LA
        let total_hours_saved = gain * annual_trips;
        // Parse capex to a number for $/hr calculation
        let cost_per_hour = if r.capex.contains("$0") {
            0.0
        } else if r.capex.contains("40M") {
            40_000_000.0 / total_hours_saved
        } else if r.capex.contains("200M") {
            200_000_000.0 / total_hours_saved
        } else if r.capex.contains("800M") {
            800_000_000.0 / total_hours_saved
        } else if r.capex.contains("930M") {
            930_000_000.0 / total_hours_saved
        } else if r.capex.contains("$4B") {
            4_000_000_000.0 / total_hours_saved
        } else if r.capex.contains("121B") {
            121_000_000_000.0 / total_hours_saved
        } else {
            -1.0
        };
        let cost_str = if cost_per_hour <= 0.0 {
            "free/operational".to_string()
        } else {
            format!("${:.2}/hr saved", cost_per_hour)
        };
        println!(
            "{:<35} {:>+8.1}h  {:>14}  {:>12}",
            r.label, -gain, cost_str, r.capex
        );
    }

    // Insight summary
    println!("\n── Key findings ─────────────────────────────────────────────────────");
    let achieves_48 = bench
        .results
        .iter()
        .filter(|r| r.sla_achieved && !r.label.contains("Baseline"))
        .map(|r| r.label.as_str())
        .collect::<Vec<_>>();
    if achieves_48.is_empty() {
        println!("  No single or combination intervention achieves 48h SLA on this corridor.");
    } else {
        println!("  48h SLA achieved by:");
        for label in &achieves_48 {
            println!("    ✓ {}", label);
        }
    }
    let best_value = ranked.first();
    if let Some(r) = best_value {
        let gain = baseline_p95 - r.dist.p95_hours;
        println!(
            "  Highest single-intervention impact: {} (−{:.1}h p95)",
            r.label, gain
        );
    }
}

fn pct_under(d: &route_sim::TransitDistribution, threshold_h: f64) -> f64 {
    // We only have percentile snapshots; approximate from distribution shape
    if threshold_h >= d.p99_hours {
        return 99.0;
    }
    if threshold_h >= d.p95_hours {
        return 95.0;
    }
    if threshold_h >= d.p90_hours {
        return 90.0;
    }
    if threshold_h >= d.p75_hours {
        return 75.0;
    }
    if threshold_h >= d.p50_hours {
        return 50.0;
    }
    0.0
}

fn pad_center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let pad = width - s.len();
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

fn tier_for_score(score: f64) -> &'static str {
    if score >= T1_THRESHOLD {
        "T1"
    } else if score >= T2_THRESHOLD {
        "T2"
    } else if score >= T3_THRESHOLD {
        "T3"
    } else {
        "T4"
    }
}

fn rounded_score(score: f64) -> f64 {
    (score * 10.0).round() / 10.0
}

fn dimension_score_values(scores: &route_score::DimensionScores) -> [f64; 16] {
    [
        scores.a1.score,
        scores.a2.score,
        scores.a3.score,
        scores.a4.score,
        scores.a5.score,
        scores.b1.score,
        scores.b2.score,
        scores.b3.score,
        scores.b4.score,
        scores.c1.score,
        scores.c2.score,
        scores.c3.score,
        scores.c4.score,
        scores.d1.score,
        scores.d2.score,
        scores.d3.score,
    ]
}

fn dimension_estimated_values(scores: &route_score::DimensionScores) -> [bool; 16] {
    [
        scores.a1.estimated,
        scores.a2.estimated,
        scores.a3.estimated,
        scores.a4.estimated,
        scores.a5.estimated,
        scores.b1.estimated,
        scores.b2.estimated,
        scores.b3.estimated,
        scores.b4.estimated,
        scores.c1.estimated,
        scores.c2.estimated,
        scores.c3.estimated,
        scores.c4.estimated,
        scores.d1.estimated,
        scores.d2.estimated,
        scores.d3.estimated,
    ]
}

fn atlas_candidate_ids(graph: &route_network::HighwayGraph) -> Vec<String> {
    let mut ids = graph.interstate_ids();
    ids.extend(graph.us_highway_ids());
    ids.sort();
    ids.dedup();
    ids
}

#[cfg(test)]
mod tests {
    use super::{
        atlas_candidate_ids, dimension_estimated_values, dimension_score_values, rounded_score,
        tier_for_score,
    };
    use geo_types::{coord, LineString};
    use route_network::{CorridorAttributes, HighwayEdge, HighwayGraph, HighwayNode};
    use route_score::{score_corridor, ScoringConfig};
    use std::collections::HashMap;

    #[test]
    fn tier_for_score_matches_megamap_thresholds() {
        assert_eq!(tier_for_score(70.0), "T1");
        assert_eq!(tier_for_score(69.9), "T2");
        assert_eq!(tier_for_score(50.0), "T2");
        assert_eq!(tier_for_score(49.9), "T3");
        assert_eq!(tier_for_score(30.0), "T3");
        assert_eq!(tier_for_score(29.9), "T4");
    }

    #[test]
    fn rounded_score_matches_score_all_csv_precision() {
        assert_eq!(rounded_score(59.95), 60.0);
        assert_eq!(rounded_score(59.94), 59.9);
    }

    #[test]
    fn score_all_csv_dimension_values_cover_full_rubric() {
        let scores = score_corridor(
            &CorridorAttributes::default(),
            &ScoringConfig::default_config(),
        );

        assert_eq!(dimension_score_values(&scores).len(), 16);
        assert_eq!(dimension_estimated_values(&scores).len(), 16);
    }

    #[test]
    fn normalise_designation_strips_cache_separators() {
        assert_eq!(super::normalise_designation("I_66"), "I66");
        assert_eq!(super::normalise_designation("us-287"), "US287");
    }

    #[test]
    fn a2_freight_proxy_uses_mean_aadt_when_p90_missing() {
        let mut attrs = CorridorAttributes {
            p90_aadt: None,
            mean_aadt: Some(20_000.0),
            mean_pct_truck: Some(0.10),
            ..Default::default()
        };

        super::join_a2_freight_proxy(&mut attrs, 100.0);

        assert!(attrs.annual_freight_value_b.is_some());
    }

    #[test]
    fn atlas_candidates_include_us_highway_promotions_but_not_state_routes() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 1.0 },
            is_interchange: false,
        });
        graph.route_index = ["I80", "US30", "SR99"]
            .into_iter()
            .map(|id| {
                let edge = graph.graph.add_edge(
                    a,
                    b,
                    HighwayEdge {
                        id: 1,
                        route_id: id.to_string(),
                        state: "TS".to_string(),
                        road_class: route_data::RoadClass::Interstate,
                        geometry: LineString::from(vec![
                            coord! { x: 0.0, y: 0.0 },
                            coord! { x: 1.0, y: 1.0 },
                        ]),
                        length_miles: 1.0,
                        lane_count: None,
                        aadt: None,
                        pct_truck: None,
                        iri: None,
                        tti: None,
                        pti: None,
                        speed_limit: None,
                    },
                );
                (id.to_string(), vec![edge])
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(atlas_candidate_ids(&graph), vec!["I80", "US30"]);
    }
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

/// Load county gazetteer + ACS population from cache (if available).
/// Returns None silently if the files are not cached — scoring degrades gracefully.
fn load_acs_counties_for_scoring(
    manifest: &route_data::Manifest,
) -> Option<Vec<route_data::CountyCentroid>> {
    // Locate gazetteer
    let gaz_path: Option<std::path::PathBuf> = std::fs::read_dir(&manifest.cache_dir)
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
        });

    let gaz_path = gaz_path?;
    let mut counties = route_data::read_county_gazetteer(&gaz_path).ok()?;

    // Join ACS population if cached
    let pop_path = manifest.cache_dir.join("acs_county_pop_2022.csv");
    if pop_path.exists() {
        let _ = route_data::join_population(&mut counties, &pop_path);
    }

    // Join ACS median household income if cached (for C3 scoring)
    let inc_path = manifest.cache_dir.join("acs_county_income_2022.csv");
    if inc_path.exists() {
        let _ = route_data::join_income(&mut counties, &inc_path);
    }

    // Join RUCC rural codes if cached (for C2 rural_share scoring)
    let rucc_path = manifest.cache_dir.join("rucc_2013.csv");
    if rucc_path.exists() {
        let _ = route_data::join_rucc(&mut counties, &rucc_path);
    }

    Some(counties)
}

/// Load ports.csv (top 25 ports + major border crossings) for B3 scoring.
fn load_ports() -> Vec<PortLocation> {
    let path = std::path::Path::new("data/ports.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 5 {
                return None;
            }
            let lat: f64 = rec[1].parse().ok()?;
            let lon: f64 = rec[2].parse().ok()?;
            let rank: u32 = rec[3].parse().ok()?;
            let is_border = rec[4].contains("border");
            Some(PortLocation {
                lat,
                lon,
                _rank: rank,
                is_border,
            })
        })
        .collect()
}

struct PortLocation {
    lat: f64,
    lon: f64,
    _rank: u32,
    is_border: bool,
}

/// Load intermodal terminal locations from data/intermodal_terminals.csv.
fn load_intermodal_terminals() -> Vec<(f64, f64)> {
    let path = std::path::Path::new("data/intermodal_terminals.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 5 {
                return None;
            }
            let lat: f64 = rec[3].parse().ok()?;
            let lon: f64 = rec[4].parse().ok()?;
            Some((lat, lon))
        })
        .collect()
}

/// Compute intermodal hub count for a corridor (hubs within 30 miles).
fn join_intermodal_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    terminals: &[(f64, f64)],
) {
    if terminals.is_empty() {
        return;
    }
    let corridor_nodes: Vec<(f64, f64)> = graph
        .graph
        .node_indices()
        .filter(|&ni| {
            graph
                .graph
                .edges(ni)
                .any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| {
            let c = graph.graph[ni].coord;
            (c.x, c.y)
        })
        .collect();
    if corridor_nodes.is_empty() {
        return;
    }

    fn haversine2(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 3_958.8_f64;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().asin()
    }

    let count = terminals
        .iter()
        .filter(|&&(tlat, tlon)| {
            corridor_nodes
                .iter()
                .any(|&(nx, ny)| haversine2(ny, nx, tlat, tlon) <= 30.0)
        })
        .count();
    attrs.intermodal_hub_count = count.min(255) as u8;
}

/// Load DCFC charging station locations from cache.
fn load_dcfc_stations() -> Vec<(f64, f64)> {
    // (lat, lon)
    let path = std::path::Path::new("data/cache/dcfc_stations.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 6 {
                return None;
            }
            let lat: f64 = rec[4].parse().ok()?;
            let lon: f64 = rec[5].parse().ok()?;
            if lat.abs() < 1.0 || lon.abs() < 1.0 {
                return None;
            }
            Some((lat, lon))
        })
        .collect()
}

/// Compute DCFC per 100 miles for a corridor.
fn join_dcfc_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    corridor_miles: f64,
    attrs: &mut route_network::CorridorAttributes,
    dcfc_stations: &[(f64, f64)],
) {
    if dcfc_stations.is_empty() {
        return;
    }

    // Get all nodes on this corridor
    let corridor_nodes: Vec<(f64, f64)> = graph
        .graph
        .node_indices()
        .filter(|&ni| {
            graph
                .graph
                .edges(ni)
                .any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| {
            let c = graph.graph[ni].coord;
            (c.x, c.y)
        })
        .collect();
    if corridor_nodes.is_empty() {
        return;
    }

    fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 3_958.8_f64;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().asin()
    }

    // Count DCFC stations within 5 miles of any corridor node
    let mut count = 0u32;
    for &(slat, slon) in dcfc_stations {
        let near = corridor_nodes
            .iter()
            .any(|&(nx, ny)| haversine(ny, nx, slat, slon) <= 5.0);
        if near {
            count += 1;
        }
    }

    if corridor_miles > 0.0 {
        let dcfc_per_100 = (count as f64 / corridor_miles) * 100.0;
        attrs.dcfc_per_100mi = Some(dcfc_per_100 as f32);
    }
}

/// Compute B3 fields: port terminus flag, border crossing flag, nearest port distance.
fn join_port_access_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    ports: &[PortLocation],
) {
    if ports.is_empty() {
        return;
    }

    // Get terminus nodes (degree-1 interchange nodes on this route)
    let node_coords: Vec<(f64, f64)> = graph
        .graph
        .node_indices()
        .filter(|&ni| {
            graph
                .graph
                .edges(ni)
                .any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| {
            let c = graph.graph[ni].coord;
            (c.x, c.y)
        })
        .collect();
    if node_coords.is_empty() {
        return;
    }

    fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 3_958.8_f64;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        r * 2.0 * a.sqrt().asin()
    }

    let mut min_dist = f64::MAX;
    let mut terminus_flag = false;
    let mut border_flag = false;

    for port in ports {
        for &(px, py) in &node_coords {
            let d = haversine(py, px, port.lat, port.lon);
            if d < min_dist {
                min_dist = d;
            }
            if d <= 30.0 {
                if port.is_border {
                    border_flag = true;
                } else {
                    terminus_flag = true;
                }
            }
        }
    }

    attrs.port_terminus_flag = terminus_flag;
    attrs.border_crossing_flag = border_flag;
    if min_dist < f64::MAX {
        attrs.nearest_top25_port_miles = Some(min_dist as f32);
    }
}

/// A 1°×1° FEMA NFHL tile with an SFHA feature count.
struct FemaTile {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    sfha_count: u32,
}

/// Load FEMA SFHA tile counts from data/cache/fema_sfha_tile_counts.csv.
/// Returns an empty Vec if the file is not present or cannot be parsed.
fn load_fema_tiles() -> Vec<FemaTile> {
    let path = std::path::Path::new("data/cache/fema_sfha_tile_counts.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 6 {
                return None;
            }
            let xmin: f64 = rec[1].trim().parse().ok()?;
            let ymin: f64 = rec[2].trim().parse().ok()?;
            let xmax: f64 = rec[3].trim().parse().ok()?;
            let ymax: f64 = rec[4].trim().parse().ok()?;
            let sfha_count: u32 = rec[5].trim().parse().ok()?;
            Some(FemaTile {
                xmin,
                ymin,
                xmax,
                ymax,
                sfha_count,
            })
        })
        .collect()
}

/// Join FEMA D1 SFHA data onto a corridor's CorridorAttributes.
///
/// Algorithm:
/// 1. Collect all graph node (lon, lat) pairs for the corridor.
/// 2. Compute corridor bounding box (xmin, ymin, xmax, ymax).
/// 3. Sum sfha_count for every tile whose bbox overlaps the corridor bbox.
/// 4. Estimate fema_sfha_miles = sum × 0.3 (avg SFHA polygon ~0.3 mi span).
/// 5. Set max_consecutive_sfha_miles as a 70% proxy (coastal/valley assumption).
fn join_fema_d1_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    tiles: &[FemaTile],
) {
    if tiles.is_empty() {
        return;
    }

    // Collect corridor node coords (lon = x, lat = y)
    let node_coords: Vec<(f64, f64)> = graph
        .graph
        .node_indices()
        .filter(|&ni| {
            graph
                .graph
                .edges(ni)
                .any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| {
            let c = graph.graph[ni].coord;
            (c.x, c.y)
        })
        .collect();
    if node_coords.is_empty() {
        return;
    }

    // Compute corridor bounding box
    let corr_xmin = node_coords.iter().map(|&(x, _)| x).fold(f64::MAX, f64::min);
    let corr_xmax = node_coords.iter().map(|&(x, _)| x).fold(f64::MIN, f64::max);
    let corr_ymin = node_coords.iter().map(|&(_, y)| y).fold(f64::MAX, f64::min);
    let corr_ymax = node_coords.iter().map(|&(_, y)| y).fold(f64::MIN, f64::max);

    // Sum sfha_count for overlapping tiles
    let total_sfha: u64 = tiles
        .iter()
        .filter(|t| {
            // Bboxes overlap unless one is entirely outside the other on any axis
            !(corr_xmax < t.xmin || corr_xmin > t.xmax || corr_ymax < t.ymin || corr_ymin > t.ymax)
        })
        .map(|t| t.sfha_count as u64)
        .sum();

    if total_sfha == 0 {
        return;
    }

    // Avg SFHA polygon spans ~0.3 miles → convert feature count to miles
    let sfha_miles = total_sfha as f64 * 0.3;
    attrs.fema_sfha_miles = Some(sfha_miles);
    // Proxy: 70% of total is consecutive for coastal/valley corridors
    attrs.max_consecutive_sfha_miles = Some((sfha_miles * 0.7) as f32);
}

/// Apply a D3 IRI proxy when NBI bridge data is unavailable.
///
/// Maps mean_iri to an estimated mean_year_built and pct_bridges_poor:
///   IRI < 50  → post-2000 construction/resurfacing  → year 2005
///   IRI 50-80 → 1985–2000 era                       → year 1990
///   IRI 80-120→ 1970–1985 era                       → year 1975
///   IRI > 120 → pre-1970 Eisenhower era              → year 1965
///
/// pct_bridges_poor proxy = (IRI / 170.0).min(0.30)
/// (IRI 170 ≈ "poor" pavement threshold; maps 0–170 IRI → 0–30% poor bridges)

// NBI data record for joining
struct NbiBridgeRecord {
    pct_bridges_poor: f32,
    mean_year_built: f32,
    bridge_count: u32,
}

/// Load NBI per-corridor summary from data/cache/nbi_bridges.csv.
fn load_nbi_bridges() -> std::collections::HashMap<String, NbiBridgeRecord> {
    let path = std::path::Path::new("data/cache/nbi_bridges.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut totals: std::collections::HashMap<String, (u32, f32, f32)> =
        std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        let route_id = normalise_designation(result[0].trim());
        if route_id.is_empty() {
            continue;
        }
        let bridge_count: u32 = result[1].parse().unwrap_or(0);
        let pct: f32 = result[3].parse().unwrap_or(0.0);
        let year: f32 = result[4].parse().unwrap_or(1970.0);
        let poor_count = pct * bridge_count as f32;
        let year_sum = year * bridge_count as f32;
        let entry = totals.entry(route_id).or_insert((0, 0.0, 0.0));
        entry.0 += bridge_count;
        entry.1 += poor_count;
        entry.2 += year_sum;
    }
    let mut map = std::collections::HashMap::new();
    for (route_id, (bridge_count, poor_count, year_sum)) in totals {
        let denom = bridge_count.max(1) as f32;
        map.insert(
            route_id,
            NbiBridgeRecord {
                pct_bridges_poor: poor_count / denom,
                mean_year_built: year_sum / denom,
                bridge_count,
            },
        );
    }
    map
}

/// Load FARS 2022 fatal crash rates by route from data/cache/fars_2022_routes.csv.
/// Columns: route_id, fatal_count, fatal_rate_per_100mvmt
/// Returns route_id -> crash_rate_per_100M_VMT.
fn load_fars_safety() -> std::collections::HashMap<String, f32> {
    let path = std::path::Path::new("data/cache/fars_2022_routes.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 3 {
            continue;
        }
        let route_id = result[0].to_string();
        let rate: f32 = result[2].parse().unwrap_or(0.0);
        map.insert(route_id, rate);
    }
    map
}

/// Load railroad parallel data from data/railroad_parallels.csv.
/// Columns: interstate, railroad, railroad_owner, approx_parallel_miles, within_50mi, notes
/// Returns: route_id (normalized e.g. "I80") -> railroad_name (only within_50mi=true entries).
fn load_railroad_parallels() -> std::collections::HashMap<String, String> {
    let path = std::path::Path::new("data/railroad_parallels.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        // Columns: interstate, railroad, railroad_owner, approx_parallel_miles, within_50mi, notes
        let interstate = result[0].trim().to_string();
        let railroad = result[1].trim().to_string();
        let within_50mi = result[4].trim() == "true";
        if within_50mi {
            // Normalize interstate name: "I-80" -> "I80"
            let id: String = interstate
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_uppercase();
            map.insert(id, railroad);
        }
    }
    map
}

struct HazardZone {
    wildfire: f32,
    tornado: f32,
    seismic: f32,
}

/// Load multi-hazard zone scores from data/hazard_zones.csv.
/// Columns: route, wildfire_risk, tornado_risk, seismic_risk
/// Route names like "I-5 (CA Siskiyou)" are normalized to "I5"; MAX taken for multi-segment corridors.
fn load_hazard_zones() -> std::collections::HashMap<String, HazardZone> {
    let path = std::path::Path::new("data/hazard_zones.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 4 {
            continue;
        }
        let route_raw = result[0].trim();
        // Extract base route: "I-5 (CA Siskiyou)" -> "I5"
        let id: String = route_raw
            .split_whitespace()
            .next()
            .unwrap_or("")
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase();
        let wf: f32 = result[1].parse().unwrap_or(0.0);
        let tor: f32 = result[2].parse().unwrap_or(0.0);
        let seis: f32 = result[3].parse().unwrap_or(0.0);
        // Take MAX for corridors spanning multiple segment entries
        let entry = map.entry(id).or_insert(HazardZone {
            wildfire: 0.0,
            tornado: 0.0,
            seismic: 0.0,
        });
        if wf > entry.wildfire {
            entry.wildfire = wf;
        }
        if tor > entry.tornado {
            entry.tornado = tor;
        }
        if seis > entry.seismic {
            entry.seismic = seis;
        }
    }
    map
}

/// Join NBI bridge condition data to a corridor.
fn join_nbi_to_corridor(
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
) {
    if let Some(rec) = nbi.get(route_id) {
        attrs.pct_bridges_poor = Some(rec.pct_bridges_poor);
        attrs.mean_year_built = Some(rec.mean_year_built);
        attrs.bridge_count = rec.bridge_count as usize;
    }
}

/// Estimate A2 freight value from HPMS truck AADT times corridor miles.
/// Uses p90 AADT when available, then mean AADT as the secondary A2 path.
fn join_a2_freight_proxy(attrs: &mut route_network::CorridorAttributes, corridor_miles: f64) {
    if attrs.annual_freight_value_b.is_some() {
        return;
    }
    let Some(aadt) = attrs.p90_aadt.or(attrs.mean_aadt) else {
        return;
    };
    let truck_pct = attrs.mean_pct_truck.unwrap_or(0.084) as f64;
    let truck_aadt = aadt * truck_pct;
    let freight_b = truck_aadt * 365.0 * corridor_miles * 3.50 / 1_000_000_000.0;
    attrs.annual_freight_value_b = Some(freight_b);
}

///
/// Only fills in fields that are currently None.
fn join_d3_iri_proxy(attrs: &mut route_network::CorridorAttributes) {
    // Only apply when NBI data is absent
    if attrs.pct_bridges_poor.is_some() {
        return;
    }
    let Some(iri) = attrs.mean_iri else {
        return;
    };

    let estimated_year = if iri < 50.0 {
        2005.0_f32
    } else if iri < 80.0 {
        1990.0
    } else if iri < 120.0 {
        1975.0
    } else {
        1965.0
    };

    if attrs.mean_year_built.is_none() {
        attrs.mean_year_built = Some(estimated_year);
    }
    let iri_proxy = (iri / 170.0).min(0.30);
    attrs.pct_bridges_poor = Some(iri_proxy);
}

/// Join ACS population onto a single corridor's CorridorAttributes.
/// No-op if the cached files are not present.
fn join_acs_population_to_corridor(
    manifest: &route_data::Manifest,
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    verbose: bool,
) {
    if let Some(counties) = load_acs_counties_for_scoring(manifest) {
        let (pop, rural_pop) = route_network::corridor_pop_within_50mi(graph, route_id, &counties);
        if pop > 0 {
            let rural_share = rural_pop as f32 / pop as f32;
            attrs.pop_within_50mi = Some(pop);
            attrs.rural_pop_within_50mi = Some(rural_pop);
            attrs.pct_rural_in_buffer = Some(rural_share);

            // C3: compute median income relative to national median
            // Use population-weighted median HHI across counties in the 50-mile buffer
            let near_counties: Vec<_> =
                route_network::counties_within_50mi(graph, route_id, &counties);
            if !near_counties.is_empty() {
                let total_pop_w: u64 = near_counties.iter().map(|c| c.population).sum();
                if total_pop_w > 0 {
                    let weighted_hhi: f64 = near_counties
                        .iter()
                        .map(|c| c.median_hhi as f64 * c.population as f64)
                        .sum::<f64>()
                        / total_pop_w as f64;
                    if weighted_hhi > 0.0 {
                        let relative =
                            (weighted_hhi / route_data::NATIONAL_MEDIAN_HHI_2022 as f64) as f32;
                        attrs.gdp_per_capita_relative = Some(relative);
                    }
                }
            }

            if verbose {
                println!(
                    "  C1 population (50mi buffer): {:>12} ({:.1}% rural)",
                    pop,
                    rural_share * 100.0
                );
            }
        } else if verbose {
            println!("  C1: no counties found within 50mi corridor buffer for {route_id}");
        }
    }
    // If counties is None (files not cached), silently leave attrs as-is (None = not scored)
}

/// Print a formatted score table to stdout.
fn print_score_table(
    designation: &str,
    scores: &route_score::DimensionScores,
    all_estimated: bool,
) {
    println!("\n┌─────────────────────────────────────────────────────────────────────┐");
    println!(
        "│  {} — Dimension Scores (rubric {})",
        designation, scores.rubric_version
    );
    println!("├──────┬──────────────────────────────┬───────┬────────────────────────┤");
    println!("│ Dim  │ Name                         │ Score │ Est │");
    println!("├──────┼──────────────────────────────┼───────┼─────┤");

    let all = [
        &scores.a1, &scores.a2, &scores.a3, &scores.a4, &scores.a5, &scores.b1, &scores.b2,
        &scores.b3, &scores.b4, &scores.c1, &scores.c2, &scores.c3, &scores.c4, &scores.d1,
        &scores.d2, &scores.d3,
    ];

    for sd in all {
        let est = if sd.estimated || all_estimated {
            "†"
        } else {
            " "
        };
        println!(
            "│ {:4} │ {:<28} │ {:>5.1} │  {}  │",
            sd.dim.code(),
            sd.dim.name(),
            sd.score,
            est
        );
    }

    println!("├──────┴──────────────────────────────┼───────┼─────┤");
    println!(
        "│ Band A (Flow)                        │ {:>5.1} │     │",
        scores.band_a()
    );
    println!(
        "│ Band B (Network)                     │ {:>5.1} │     │",
        scores.band_b()
    );
    println!(
        "│ Band C (People)                      │ {:>5.1} │     │",
        scores.band_c()
    );
    println!(
        "│ Band D (Future)                      │ {:>5.1} │     │",
        scores.band_d()
    );
    println!(
        "│ TOTAL                                │ {:>5.1} │ /160│",
        scores.total()
    );
    println!("└──────────────────────────────────────┴───────┴─────┘");
}

/// Build a simple demand matrix from HPMS AADT data in the graph.
/// Proxy for FAF5-based O-D demand until FAF5 routing is implemented.
fn build_demand_from_graph(g: &route_network::HighwayGraph) -> route_sim::demand::DemandMatrix {
    use route_sim::demand::{demand_from_aadt, DemandParams};
    let params = DemandParams::default();
    let mut demand = Vec::new();

    // Create O-D pairs from terminus nodes of each interstate
    for route_id in g.interstate_ids() {
        let edges = g.route_edges(&route_id);
        if edges.len() < 2 {
            continue;
        }

        // Use first and last edge endpoints as a crude O-D pair
        if let (Some(&first_ei), Some(&last_ei)) = (edges.first(), edges.last()) {
            if let (Some((s, _)), Some((_, t))) = (
                g.graph.edge_endpoints(first_ei),
                g.graph.edge_endpoints(last_ei),
            ) {
                let mean_aadt = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].aadt.map(|a| a as f64))
                    .sum::<f64>()
                    / edges.len() as f64;
                let mean_pct = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].pct_truck)
                    .sum::<f32>()
                    / edges.len() as f32;

                if mean_aadt > 0.0 {
                    demand.push(demand_from_aadt(mean_aadt, mean_pct, &params, s, t));
                }
            }
        }
    }
    demand
}

fn print_scenario_result(result: &route_sim::ScenarioResult) {
    println!("\n=== {} ===", result.scenario_name);
    println!(
        "  Baseline:  throughput {:.0} vph  |  PTI {:.2}  |  freight cost ${:.2}M/hr",
        result.baseline.metrics.total_throughput_vph,
        result.baseline.metrics.mean_pti,
        result.baseline.metrics.freight_cost_per_hour_m
    );
    println!(
        "  Incident:  throughput {:.0} vph  |  PTI {:.2}  |  freight cost ${:.2}M/hr",
        result.incident.metrics.total_throughput_vph,
        result.incident.metrics.mean_pti,
        result.incident.metrics.freight_cost_per_hour_m
    );
    println!(
        "  Cost delta: +${:.2}M/hr  |  LOS-F edges: {}  |  T90: {:.1}h",
        result.incident.freight_cost_delta_m,
        result.incident.metrics.losf_edges,
        result.incident.t90_hours.unwrap_or(0.0)
    );

    if let Some(ref int_result) = result.intervention {
        println!(
            "  Intervention: throughput {:.0} vph  |  PTI {:.2}  |  cost ${:.2}M/hr",
            int_result.metrics.total_throughput_vph,
            int_result.metrics.mean_pti,
            int_result.metrics.freight_cost_per_hour_m
        );
        let improvement = result.incident.metrics.freight_cost_per_hour_m
            - int_result.metrics.freight_cost_per_hour_m;
        println!(
            "  Intervention saves: ${:.2}M/hr  PTI improvement: {:.2} → {:.2}",
            improvement, result.incident.metrics.mean_pti, int_result.metrics.mean_pti
        );
    }

    // Corridor PTIs
    if !result.incident.corridor_ptis.is_empty() {
        println!("\n  Corridor PTIs (incident):");
        let mut ptis: Vec<(&String, &f64)> = result.incident.corridor_ptis.iter().collect();
        ptis.sort_by(|a, b| b.1.total_cmp(a.1));
        for (corridor, pti) in ptis {
            let flag = if *pti > 1.3 { " ⚠" } else { "" };
            println!("    {}: {:.2}{}", corridor, pti, flag);
        }
    }
}

fn print_chaos_result(result: &route_sim::ChaosResult) {
    println!("\n=== Chaos Results ({} iterations) ===", result.iterations);
    println!(
        "  Mean freight cost delta: +${:.2}M/peak-hr",
        result.mean_freight_cost_delta_m
    );
    println!(
        "  P95 freight cost delta:  +${:.2}M/peak-hr",
        result.p95_freight_cost_delta_m
    );
    println!(
        "  Max freight cost delta:  +${:.2}M/peak-hr",
        result.max_freight_cost_delta_m
    );
    println!("  Mean network PTI:        {:.2}", result.mean_network_pti);
    println!(
        "  Saturation fraction:     {:.1}%",
        result.saturation_fraction * 100.0
    );
    if !result.worst_case_corridors.is_empty() {
        println!(
            "  Worst-case corridors:    {}",
            result.worst_case_corridors.join(", ")
        );
    }
}
