use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

const T1_THRESHOLD: f64 = 70.0;
const T2_THRESHOLD: f64 = 50.0;
const T3_THRESHOLD: f64 = 30.0;
const DIMENSION_CODES: [&str; 16] = [
    "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1", "D2", "D3",
];

struct ConfidenceRisk {
    route: String,
    score: f64,
    tier: &'static str,
    mean_confidence: f32,
    score_confidence: f32,
    risk_dimensions: String,
}

struct ScoreAllRow {
    route: String,
    score: f64,
    tier: &'static str,
    rubric_version: String,
    estimated: bool,
    confidence: f32,
    score_confidence: f32,
    dimensions: [f64; 16],
    dimension_confidences: [f32; 16],
}

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
        /// Path to FPM reliability CSV with ROUTE_ID, TTI, PTI columns
        #[arg(long, value_name = "FILE")]
        fpm: Option<PathBuf>,
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

    /// Show proof status for Interstate 2.0 standards
    StandardsProof {
        /// Path to standards proof ledger CSV
        #[arg(
            long,
            default_value = "data/standards-proof-ledger.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Filter by tier, e.g. T1
        #[arg(long)]
        tier: Option<String>,
        /// Filter by standard family, e.g. resilience
        #[arg(long)]
        family: Option<String>,
        /// Print full proof records, including outcome, mechanism, gate, and next test
        #[arg(long)]
        details: bool,
        /// Fail if unresolved standards would be promoted into Blueprint
        #[arg(long)]
        gate_blueprint: bool,
    },

    /// Show L2 pressure-test scenario catalog readiness
    PressureScenarios {
        /// Path to L2 pressure-test scenario catalog CSV
        #[arg(
            long,
            default_value = "data/pressure-test-scenarios.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only scenarios that still have blocking gaps
        #[arg(long)]
        blockers: bool,
        /// Print full scenario proof details
        #[arg(long)]
        details: bool,
        /// Fail if any scenario catalog row lacks a bounded proof contract
        #[arg(long)]
        gate_l2: bool,
    },

    /// Show throughput proof matrix separating congestion and resilience chokepoints
    ThroughputProof {
        /// Path to throughput proof matrix CSV
        #[arg(
            long,
            default_value = "data/throughput-proof-matrix.csv",
            value_name = "FILE"
        )]
        matrix: PathBuf,
        /// Show only rows that still have blocking gaps
        #[arg(long)]
        blockers: bool,
        /// Print full proof details
        #[arg(long)]
        details: bool,
        /// Fail if any row lacks the required proof contract
        #[arg(long)]
        gate: bool,
    },

    /// Show T1/T1 failure-rate and reroute evidence status
    T1Failures {
        /// Path to T1/T1 failure evidence ledger CSV
        #[arg(
            long,
            default_value = "data/t1-intersection-failures.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows that still need empirical sources
        #[arg(long)]
        needs_sources: bool,
        /// Print detailed blocker and next-evidence fields
        #[arg(long)]
        details: bool,
        /// Fail if failure evidence rows are unlabeled or lack next evidence steps
        #[arg(long)]
        gate_evidence: bool,
    },

    /// Show source-acquisition plan for T1/T1 failure evidence
    T1FailureSources {
        /// Path to T1/T1 failure source plan CSV
        #[arg(
            long,
            default_value = "data/t1-failure-source-plan.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows whose source endpoint still needs lookup
        #[arg(long)]
        lookup_needed: bool,
    },

    /// Show source health for T1/T1 failure evidence ingestion
    T1SourceHealth {
        /// Path to T1/T1 source health ledger CSV
        #[arg(long, default_value = "data/t1-source-health.csv", value_name = "FILE")]
        ledger: PathBuf,
        /// Show only rows that are not live/implemented
        #[arg(long)]
        blockers: bool,
        /// Print detailed blocker and next-step fields
        #[arg(long)]
        details: bool,
        /// Exit with an error if any source-health blockers are present
        #[arg(long)]
        gate_ingestion: bool,
    },

    /// Show actionable access/request docket for blocked T1/T1 evidence sources
    T1AccessDocket {
        /// Path to T1/T1 source health ledger CSV
        #[arg(long, default_value = "data/t1-source-health.csv", value_name = "FILE")]
        ledger: PathBuf,
        /// Show only one action category, e.g. api_key, account, access_request
        #[arg(long)]
        category: Option<String>,
        /// Print detailed source URLs, gaps, and next steps
        #[arg(long)]
        details: bool,
    },

    /// Summarize raw T1/T1 failure event observations into rates and durations
    T1FailureEvents {
        /// Path to normalized T1/T1 failure event observations CSV
        #[arg(
            long,
            default_value = "data/t1-failure-events.csv",
            value_name = "FILE"
        )]
        events: PathBuf,
        /// Path to T1/T1 failure evidence ledger CSV
        #[arg(
            long,
            default_value = "data/t1-intersection-failures.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Write an updated evidence ledger with empirical event summaries applied
        #[arg(long, value_name = "FILE")]
        write_ledger: Option<PathBuf>,
    },

    /// Fetch current Iowa 511 ArcGIS event JSON for source-cache ingestion
    T1FetchIowa511 {
        /// Output JSON file
        #[arg(
            long,
            default_value = "data/cache/iowa511-events.json",
            value_name = "FILE"
        )]
        output: PathBuf,
    },

    /// Normalize Iowa 511 ArcGIS event JSON into T1/T1 failure event rows
    T1ImportIowa511 {
        /// Cached Iowa 511 ArcGIS query JSON
        #[arg(
            long,
            default_value = "data/cache/iowa511-events.json",
            value_name = "FILE"
        )]
        input: PathBuf,
        /// Output normalized T1/T1 event CSV
        #[arg(
            long,
            default_value = "data/cache/iowa511-t1-failure-events.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// T1/T1 site id to assign
        #[arg(long, default_value = "T1X-I35-I80")]
        site_id: String,
        /// Site latitude for radius filtering
        #[arg(long, default_value_t = 41.658)]
        lat: f64,
        /// Site longitude for radius filtering
        #[arg(long, default_value_t = -93.800)]
        lon: f64,
        /// Maximum event distance from site center
        #[arg(long, default_value_t = 30.0)]
        radius_miles: f64,
    },

    /// Fetch current TDOT SmartWay line-event JSON for source-cache ingestion
    T1FetchTdotSmartway {
        /// Output JSON file
        #[arg(
            long,
            default_value = "data/cache/tdot-smartway-events.json",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Maximum seconds to wait for the TDOT ArcGIS query
        #[arg(long, default_value_t = 15)]
        timeout_seconds: u64,
    },

    /// Normalize TDOT SmartWay line-event JSON into T1/T1 failure event rows
    T1ImportTdotSmartway {
        /// Cached TDOT SmartWay ArcGIS query JSON
        #[arg(
            long,
            default_value = "data/cache/tdot-smartway-events.json",
            value_name = "FILE"
        )]
        input: PathBuf,
        /// Output normalized T1/T1 event CSV
        #[arg(
            long,
            default_value = "data/cache/tdot-smartway-t1-failure-events.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// T1/T1 site id to assign
        #[arg(long, default_value = "T1X-I40-I75")]
        site_id: String,
        /// Site latitude for radius filtering
        #[arg(long, default_value_t = 35.90)]
        lat: f64,
        /// Site longitude for radius filtering
        #[arg(long, default_value_t = -84.16)]
        lon: f64,
        /// Maximum event distance from site center
        #[arg(long, default_value_t = 35.0)]
        radius_miles: f64,
    },

    /// Fetch current MDOT Mi Drive incident JSON for source-cache ingestion
    T1FetchMdotMidrive {
        /// Output JSON file
        #[arg(
            long,
            default_value = "data/cache/mdot-midrive-incidents.json",
            value_name = "FILE"
        )]
        output: PathBuf,
    },

    /// Normalize MDOT Mi Drive incident JSON into T1/T1 failure event rows
    T1ImportMdotMidrive {
        /// Cached MDOT Mi Drive incident JSON
        #[arg(
            long,
            default_value = "data/cache/mdot-midrive-incidents.json",
            value_name = "FILE"
        )]
        input: PathBuf,
        /// Output normalized T1/T1 event CSV
        #[arg(
            long,
            default_value = "data/cache/mdot-midrive-t1-failure-events.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// T1/T1 site id to assign
        #[arg(long, default_value = "T1X-I75-I90")]
        site_id: String,
        /// Site latitude for radius filtering
        #[arg(long, default_value_t = 42.31)]
        lat: f64,
        /// Site longitude for radius filtering
        #[arg(long, default_value_t = -83.07)]
        lon: f64,
        /// Maximum event distance from site center
        #[arg(long, default_value_t = 60.0)]
        radius_miles: f64,
        /// Observation year for current-state snapshots
        #[arg(long)]
        observation_year: Option<u16>,
    },

    /// Fetch current INDOT TrafficWise event JSON for source-cache ingestion
    T1FetchIndotTrafficwise {
        /// Output JSON file
        #[arg(
            long,
            default_value = "data/cache/indot-trafficwise-events.json",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// North bound for the TrafficWise map query
        #[arg(long, default_value_t = 42.0)]
        north: f64,
        /// South bound for the TrafficWise map query
        #[arg(long, default_value_t = 40.8)]
        south: f64,
        /// East bound for the TrafficWise map query
        #[arg(long, default_value_t = -84.6)]
        east: f64,
        /// West bound for the TrafficWise map query
        #[arg(long, default_value_t = -87.7)]
        west: f64,
        /// TrafficWise map zoom used for event clustering
        #[arg(long, default_value_t = 8)]
        zoom: u8,
    },

    /// Normalize INDOT TrafficWise event JSON into T1/T1 failure event rows
    T1ImportIndotTrafficwise {
        /// Cached INDOT TrafficWise GraphQL response JSON
        #[arg(
            long,
            default_value = "data/cache/indot-trafficwise-events.json",
            value_name = "FILE"
        )]
        input: PathBuf,
        /// Output normalized T1/T1 event CSV
        #[arg(
            long,
            default_value = "data/cache/indot-trafficwise-t1-failure-events.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// T1/T1 site id to assign
        #[arg(long, default_value = "T1X-I80-I90")]
        site_id: String,
        /// Observation year for current-state snapshots
        #[arg(long)]
        observation_year: Option<u16>,
    },

    /// Merge normalized T1/T1 event observations into an accumulated event table
    T1AccumulateEvents {
        /// Existing accumulated T1/T1 event CSV
        #[arg(
            long,
            default_value = "data/t1-failure-events.csv",
            value_name = "FILE"
        )]
        events: PathBuf,
        /// New normalized T1/T1 event CSV to merge
        #[arg(long, value_name = "FILE")]
        input: PathBuf,
        /// Output merged event CSV
        #[arg(
            long,
            default_value = "data/t1-failure-events.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
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
    /// Run a named scenario (donner-closure, atlanta-peak, des-moines-interchange, houston-surge)
    Scenario {
        name: String,
        /// Test the named I2.0 intervention for this scenario
        #[arg(long)]
        intervention: bool,
    },
    /// Find stable graph edge IDs near a coordinate for binding scenario incidents
    Bind {
        /// Route to search, e.g. I80, I-80, I35
        #[arg(long)]
        route: String,
        /// Latitude of the incident center
        #[arg(long, allow_hyphen_values = true)]
        lat: f64,
        /// Longitude of the incident center
        #[arg(long, allow_hyphen_values = true)]
        lon: f64,
        /// Search radius in miles
        #[arg(long, default_value_t = 10.0)]
        radius: f64,
        /// Maximum candidate edges to print
        #[arg(long, default_value_t = 12)]
        top: usize,
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
    std::thread::Builder::new()
        .name("route-cli".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(run_cli)
        .context("spawning route CLI thread")?
        .join()
        .map_err(|panic| {
            if let Some(message) = panic.downcast_ref::<&str>() {
                anyhow::anyhow!("route CLI thread panicked: {message}")
            } else if let Some(message) = panic.downcast_ref::<String>() {
                anyhow::anyhow!("route CLI thread panicked: {message}")
            } else {
                anyhow::anyhow!("route CLI thread panicked")
            }
        })?
}

fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    // Load scoring config
    let scoring_config_path = cli
        .scoring_config
        .clone()
        .unwrap_or_else(|| PathBuf::from("config/scoring.toml"));
    let scoring_cfg = {
        if scoring_config_path.exists() {
            route_score::ScoringConfig::load(&scoring_config_path)
                .context("loading scoring config")?
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
            fpm: fpm_path,
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
                "A1_conf", "A2_conf", "A3_conf", "A4_conf", "A5_conf", "B1_conf", "B2_conf",
                "B3_conf", "B4_conf", "C1_conf", "C2_conf", "C3_conf", "C4_conf", "D1_conf",
                "D2_conf", "D3_conf",
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
        }

        Commands::Gap { r#type, slug } => {
            println!("route gap --type {:?}", r#type);
            let out_slug = slug.unwrap_or_else(|| gap_type_slug(&r#type).to_string());
            let out = PathBuf::from(format!("gaps/{out_slug}.md"));
            write_gap_report(&r#type, &out)?;
            println!("  wrote gap report → {}", out.display());
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

            let acs_counties = load_acs_counties_for_scoring(&manifest);
            let ports = load_ports();
            let dcfc = load_dcfc_stations();
            let intermodal = load_intermodal_terminals();
            let fema_tiles = load_fema_tiles();
            let nbi = load_nbi_bridges();
            let fars_safety = load_fars_safety();
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
            let output_path = PathBuf::from(format!("corpus/existing/{}.md", norm.to_lowercase()));
            let provenance = route_report::CorpusProvenance {
                command: format!("route report {norm}"),
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

        Commands::StandardsProof {
            ledger,
            tier,
            family,
            details,
            gate_blueprint,
        } => {
            let rows = load_standards_proof_ledger(&ledger)
                .with_context(|| format!("loading standards proof ledger {}", ledger.display()))?;
            print_standards_proof(&rows, tier.as_deref(), family.as_deref(), details);

            if gate_blueprint {
                let failures = standards_blueprint_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("Blueprint gate: FAIL");
                    println!(
                        "  {} standards still have unresolved proof gaps.",
                        failures.len()
                    );
                    println!("  First unresolved standards:");
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{} {}]: {}",
                            row.standard_id, row.tier, row.evidence_level, row.blocking_gap
                        );
                    }
                    anyhow::bail!("standards proof gate failed");
                }
                println!();
                println!("Blueprint gate: PASS");
            }
        }

        Commands::PressureScenarios {
            ledger,
            blockers,
            details,
            gate_l2,
        } => {
            let rows = load_pressure_scenarios(&ledger).with_context(|| {
                format!("loading pressure scenario ledger {}", ledger.display())
            })?;
            print_pressure_scenarios(&rows, blockers, details);

            if gate_l2 {
                let failures = pressure_scenario_gate_failures(&rows);
                let missing_adversity = pressure_scenario_missing_required_adversity(&rows);
                if !failures.is_empty() || !missing_adversity.is_empty() {
                    println!();
                    println!("L2 scenario gate: FAIL");
                    println!(
                        "  {} scenario rows still lack bounded proof contracts.",
                        failures.len()
                    );
                    if !missing_adversity.is_empty() {
                        println!(
                            "  missing required adversity classes: {}",
                            missing_adversity.join(", ")
                        );
                    }
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{}]: {}",
                            row.scenario_id, row.current_status, row.blocking_gap
                        );
                    }
                    anyhow::bail!("pressure scenario gate failed");
                }
                println!();
                println!("L2 scenario gate: PASS");
            }
        }

        Commands::ThroughputProof {
            matrix,
            blockers,
            details,
            gate,
        } => {
            let rows = load_throughput_proof_matrix(&matrix)
                .with_context(|| format!("loading throughput proof matrix {}", matrix.display()))?;
            print_throughput_proof_matrix(&rows, blockers, details);

            if gate {
                let failures = throughput_proof_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("Throughput proof gate: FAIL");
                    println!(
                        "  {} proof rows still lack bounded congestion/resilience contracts.",
                        failures.len()
                    );
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{} {}]: {}",
                            row.proof_id, row.binding_type, row.current_status, row.blocking_gap
                        );
                    }
                    anyhow::bail!("throughput proof gate failed");
                }
                println!();
                println!("Throughput proof gate: PASS");
            }
        }

        Commands::T1Failures {
            ledger,
            needs_sources,
            details,
            gate_evidence,
        } => {
            let rows = load_t1_failure_ledger(&ledger)
                .with_context(|| format!("loading T1 failure ledger {}", ledger.display()))?;
            print_t1_failures(&rows, needs_sources, details);

            if gate_evidence {
                let failures = t1_failure_evidence_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("T1/T1 failure evidence gate: FAIL");
                    println!(
                        "  {} failure rows are unlabeled or lack evidence next steps.",
                        failures.len()
                    );
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{} {}]: {}",
                            row.site_id, row.source_status, row.confidence, row.blocking_gap
                        );
                    }
                    anyhow::bail!("T1/T1 failure evidence gate failed");
                }
                println!();
                println!("T1/T1 failure evidence gate: PASS");
            }
        }

        Commands::T1FailureSources {
            ledger,
            lookup_needed,
        } => {
            let rows = load_t1_failure_source_plan(&ledger)
                .with_context(|| format!("loading T1 failure source plan {}", ledger.display()))?;
            print_t1_failure_sources(&rows, lookup_needed);
        }

        Commands::T1SourceHealth {
            ledger,
            blockers,
            details,
            gate_ingestion,
        } => {
            let rows = load_t1_source_health(&ledger)
                .with_context(|| format!("loading T1 source health {}", ledger.display()))?;
            print_t1_source_health(&rows, blockers, details);
            if gate_ingestion {
                let blocked = t1_source_health_blockers(&rows);
                if !blocked.is_empty() {
                    anyhow::bail!(
                        "{} T1 source-health blocker(s) remain; run `route t1-source-health --blockers --details`",
                        blocked.len()
                    );
                }
            }
        }

        Commands::T1AccessDocket {
            ledger,
            category,
            details,
        } => {
            let rows = load_t1_source_health(&ledger)
                .with_context(|| format!("loading T1 source health {}", ledger.display()))?;
            print_t1_access_docket(&rows, category.as_deref(), details);
        }

        Commands::T1FailureEvents {
            events,
            ledger,
            write_ledger,
        } => {
            let event_rows = load_t1_failure_events(&events)
                .with_context(|| format!("loading T1 failure events {}", events.display()))?;
            print_t1_failure_event_summary(&event_rows);
            if let Some(output) = write_ledger {
                let ledger_rows = load_t1_failure_ledger(&ledger)
                    .with_context(|| format!("loading T1 failure ledger {}", ledger.display()))?;
                let updated = apply_t1_failure_events_to_ledger(&ledger_rows, &event_rows, &events);
                write_t1_failure_ledger(&output, &updated)
                    .with_context(|| format!("writing T1 failure ledger {}", output.display()))?;
                println!();
                println!("  updated ledger -> {}", output.display());
            }
        }

        Commands::T1FetchIowa511 { output } => {
            fetch_iowa511_events(&output)
                .with_context(|| format!("fetching Iowa 511 events to {}", output.display()))?;
            println!("route t1-fetch-iowa511");
            println!("  wrote {}", output.display());
        }

        Commands::T1ImportIowa511 {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
        } => {
            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("reading Iowa 511 JSON {}", input.display()))?;
            let rows = parse_iowa511_events(&json, &site_id, lat, lon, radius_miles)
                .with_context(|| format!("normalizing Iowa 511 JSON {}", input.display()))?;
            write_t1_failure_events(&output, &rows)
                .with_context(|| format!("writing normalized events {}", output.display()))?;
            println!("route t1-import-iowa511");
            println!("  rows: {}", rows.len());
            println!("  wrote {}", output.display());
        }

        Commands::T1FetchTdotSmartway {
            output,
            timeout_seconds,
        } => {
            fetch_tdot_smartway_events(&output, timeout_seconds).with_context(|| {
                format!("fetching TDOT SmartWay events to {}", output.display())
            })?;
            println!("route t1-fetch-tdot-smartway");
            println!("  wrote {}", output.display());
        }

        Commands::T1ImportTdotSmartway {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
        } => {
            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("reading TDOT SmartWay JSON {}", input.display()))?;
            let rows = parse_tdot_smartway_events(&json, &site_id, lat, lon, radius_miles)
                .with_context(|| format!("normalizing TDOT SmartWay JSON {}", input.display()))?;
            write_t1_failure_events(&output, &rows)
                .with_context(|| format!("writing normalized events {}", output.display()))?;
            println!("route t1-import-tdot-smartway");
            println!("  rows: {}", rows.len());
            println!("  wrote {}", output.display());
        }

        Commands::T1FetchMdotMidrive { output } => {
            fetch_mdot_midrive_events(&output).with_context(|| {
                format!("fetching MDOT Mi Drive events to {}", output.display())
            })?;
            println!("route t1-fetch-mdot-midrive");
            println!("  wrote {}", output.display());
        }

        Commands::T1ImportMdotMidrive {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
            observation_year,
        } => {
            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("reading MDOT Mi Drive JSON {}", input.display()))?;
            let rows = parse_mdot_midrive_events(
                &json,
                &site_id,
                lat,
                lon,
                radius_miles,
                observation_year.unwrap_or_else(current_utc_year),
            )
            .with_context(|| format!("normalizing MDOT Mi Drive JSON {}", input.display()))?;
            write_t1_failure_events(&output, &rows)
                .with_context(|| format!("writing normalized events {}", output.display()))?;
            println!("route t1-import-mdot-midrive");
            println!("  rows: {}", rows.len());
            println!("  wrote {}", output.display());
        }

        Commands::T1FetchIndotTrafficwise {
            output,
            north,
            south,
            east,
            west,
            zoom,
        } => {
            fetch_indot_trafficwise_events(&output, north, south, east, west, zoom).with_context(
                || format!("fetching INDOT TrafficWise events to {}", output.display()),
            )?;
            println!("route t1-fetch-indot-trafficwise");
            println!("  wrote {}", output.display());
        }

        Commands::T1ImportIndotTrafficwise {
            input,
            output,
            site_id,
            observation_year,
        } => {
            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("reading INDOT TrafficWise JSON {}", input.display()))?;
            let rows = parse_indot_trafficwise_events(
                &json,
                &site_id,
                observation_year.unwrap_or_else(current_utc_year),
            )
            .with_context(|| format!("normalizing INDOT TrafficWise JSON {}", input.display()))?;
            write_t1_failure_events(&output, &rows)
                .with_context(|| format!("writing normalized events {}", output.display()))?;
            println!("route t1-import-indot-trafficwise");
            println!("  rows: {}", rows.len());
            println!("  wrote {}", output.display());
        }

        Commands::T1AccumulateEvents {
            events,
            input,
            output,
        } => {
            let existing = if events.exists() {
                load_t1_failure_events(&events)
                    .with_context(|| format!("loading accumulated events {}", events.display()))?
            } else {
                Vec::new()
            };
            let incoming = load_t1_failure_events(&input)
                .with_context(|| format!("loading incoming events {}", input.display()))?;
            let merged = merge_t1_failure_events(&existing, &incoming);
            let added = merged.len().saturating_sub(existing.len());
            write_t1_failure_events(&output, &merged)
                .with_context(|| format!("writing accumulated events {}", output.display()))?;
            println!("route t1-accumulate-events");
            println!("  existing rows: {}", existing.len());
            println!("  incoming rows: {}", incoming.len());
            println!("  merged rows: {}", merged.len());
            println!("  net new rows: {added}");
            println!("  wrote {}", output.display());
        }

        Commands::Sim { mode } => {
            match mode {
                SimMode::List => {
                    println!("Available scenarios:");
                    for name in route_sim::scenarios::available_scenarios() {
                        let status = route_sim::scenarios::load_scenario(name)
                            .and_then(|toml| toml::from_str::<route_sim::Scenario>(toml).ok())
                            .map(|scenario| {
                                if route_sim::scenario_validation_warnings(&scenario).is_empty() {
                                    "ready"
                                } else {
                                    "needs edge bindings"
                                }
                            })
                            .unwrap_or("parse error");
                        println!("  {name:<20} {status}");
                    }
                    println!("\nUsage: route sim scenario <name> [--intervention]");
                    println!("       route sim bind --route I80 --lat 39.32 --lon -120.33");
                    println!("       route sim chaos [--iterations N] [--seed S] [--t1-only]");
                }

                SimMode::Bind {
                    route,
                    lat,
                    lon,
                    radius,
                    top,
                } => {
                    let norm = normalise_designation(&route);
                    println!(
                        "route sim bind --route {norm} --lat {lat:.5} --lon {lon:.5} --radius {radius:.1}"
                    );

                    let manifest =
                        route_data::Manifest::load(&manifest_path).with_context(|| {
                            format!("loading manifest from {}", manifest_path.display())
                        })?;
                    let graph = load_graph(&manifest)?;

                    let candidates = scenario_edge_candidates(&graph, &norm, lat, lon, radius, top);
                    if candidates.is_empty() {
                        println!("  no {norm} edges found within {radius:.1} miles");
                        println!(
                            "  tip: increase --radius or verify the route exists in the graph"
                        );
                    } else {
                        println!(
                            "  {} candidate edge IDs for scenario affected_edges:",
                            candidates.len()
                        );
                        println!(
                            "  {:>12}  {:>7}  {:>7}  {:>8}  {:>5}  {:>8}  midpoint",
                            "edge_id", "dist", "length", "aadt", "lanes", "state"
                        );
                        for c in candidates {
                            println!(
                                "  {:>12}  {:>6.2}m  {:>6.2}m  {:>8}  {:>5}  {:>8}  {:.5},{:.5}",
                                c.edge_id,
                                c.distance_miles,
                                c.length_miles,
                                c.aadt
                                    .map(|v| v.to_string())
                                    .unwrap_or_else(|| "-".to_string()),
                                c.lanes
                                    .map(|v| v.to_string())
                                    .unwrap_or_else(|| "-".to_string()),
                                c.state,
                                c.mid_lat,
                                c.mid_lon
                            );
                        }
                    }
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

                    let warnings = route_sim::scenario_validation_warnings(&scenario);
                    if !warnings.is_empty() {
                        println!("  scenario warnings:");
                        for warning in warnings {
                            println!("  - {warning}");
                        }
                        println!();
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
            let mut confidence_matrix: Vec<[f32; N_DIMS]> = Vec::new();
            let mut route_ids_used: Vec<String> = Vec::new();
            let mut total_scores: Vec<f64> = Vec::new();
            let mut flagged_congestion: Vec<(String, f64, f64)> = Vec::new(); // (route, A1, B2)
            let mut confidence_risks: Vec<ConfidenceRisk> = Vec::new();
            let mut dimension_risk_totals = [0.0_f64; N_DIMS];
            let mut dimension_review_risk_totals = [0.0_f64; N_DIMS];
            let mut dimension_risk_counts = [0_usize; N_DIMS];
            let mut dimension_review_counts = [0_usize; N_DIMS];

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
                    let confidence_row = dimension_confidence_values(&s);
                    let total = rounded_score(s.total());
                    let tier = tier_for_score(total);
                    let review = total >= T2_THRESHOLD && s.score_weighted_confidence() < 0.75;
                    // Flag congestion-stress candidates: high A1 + low B2 + total near T1 threshold
                    if s.a1.score > 7.0 && s.b2.score < 3.0 && total > 20.0 {
                        flagged_congestion.push((id.clone(), s.a1.score, s.b2.score));
                    }
                    for (d, risk) in dimension_confidence_risks(&row, &confidence_row)
                        .iter()
                        .enumerate()
                    {
                        if *risk >= 1.0 {
                            dimension_risk_totals[d] += risk;
                            dimension_risk_counts[d] += 1;
                            if review {
                                dimension_review_risk_totals[d] += risk;
                                dimension_review_counts[d] += 1;
                            }
                        }
                    }
                    let risk_dimensions = confidence_risk_dimensions(&row, &confidence_row);
                    confidence_risks.push(ConfidenceRisk {
                        route: id.clone(),
                        score: total,
                        tier,
                        mean_confidence: s.mean_confidence(),
                        score_confidence: s.score_weighted_confidence(),
                        risk_dimensions,
                    });
                    matrix.push(row);
                    estimated_matrix.push(estimated_row);
                    confidence_matrix.push(confidence_row);
                    route_ids_used.push(id.clone());
                    total_scores.push(total);
                }
            }

            let n = matrix.len() as f64;
            println!("  {} corridors scored\n", matrix.len());

            // Per-dimension statistics
            println!("┌───────────────────────────────────────────────────────────────────────────────────────────────────┐");
            println!("│  Dimension Statistics (0.0–10.0 scale, n={})                                                    │", matrix.len());
            println!("├──────┬────────────────────────────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬────────────  ┤");
            println!("│  Dim │  Name                      │  Min │  Max │  Avg │  Std │  P90 │ Conf │ Est% │ NZ%  │  Status      │");
            println!("├──────┼────────────────────────────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼────────────  ┤");

            let mut dim_stats: Vec<(f64, f64, f64, f64, f64, f64, f64)> = Vec::new(); // min,max,mean,std,p90,est_rate,nonzero_rate

            for d in 0..N_DIMS {
                let vals: Vec<f64> = matrix.iter().map(|r| r[d]).collect();
                let estimated_count = estimated_matrix.iter().filter(|r| r[d]).count();
                let avg_conf = confidence_matrix.iter().map(|r| r[d] as f64).sum::<f64>() / n;
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

                println!("│  {:>2}  │  {:<26} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.2} │ {:>4.0} │ {:>4.0} │  {:<10}  │",
                    dim_names[d], dim_labels[d], min, max, mean, std, p90, avg_conf, est_rate * 100.0, nonzero_rate * 100.0, status);
                dim_stats.push((min, max, mean, std, p90, est_rate, nonzero_rate));
            }
            println!("└──────┴────────────────────────────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴────────────  ┘");

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

            confidence_risks.sort_by(|a, b| {
                a.score_confidence
                    .total_cmp(&b.score_confidence)
                    .then_with(|| b.score.total_cmp(&a.score))
                    .then_with(|| a.route.cmp(&b.route))
            });
            println!("\n  Lowest score-weighted confidence corridors:");
            println!(
                "  {:>8}  {:>6}  {:>4}  {:>7}  {:>10}  {}",
                "Route", "Score", "Tier", "Conf", "ScoreConf", "RiskDims"
            );
            println!("  {}", "─".repeat(78));
            for risk in confidence_risks.iter().take(12) {
                let flag = if risk.score >= T2_THRESHOLD && risk.score_confidence < 0.75 {
                    "  ⚠ review"
                } else {
                    ""
                };
                println!(
                    "  {:>8}  {:>6.1}  {:>4}  {:>7.2}  {:>10.2}  {:<24}{}",
                    risk.route,
                    risk.score,
                    risk.tier,
                    risk.mean_confidence,
                    risk.score_confidence,
                    risk.risk_dimensions,
                    flag
                );
            }
            println!(
                "  → Sort by score_confidence and risk_dimensions to find rankings most dependent on weak dimensions."
            );

            let mut dimension_risks: Vec<(usize, f64, f64, usize, usize)> = (0..N_DIMS)
                .map(|d| {
                    (
                        d,
                        dimension_risk_totals[d],
                        dimension_review_risk_totals[d],
                        dimension_risk_counts[d],
                        dimension_review_counts[d],
                    )
                })
                .collect();
            dimension_risks.sort_by(|a, b| {
                b.2.total_cmp(&a.2)
                    .then_with(|| b.1.total_cmp(&a.1))
                    .then_with(|| b.4.cmp(&a.4))
                    .then_with(|| DIMENSION_CODES[a.0].cmp(DIMENSION_CODES[b.0]))
            });

            println!("\n  Confidence risk by dimension:");
            println!(
                "  {:>2}  {:<28}  {:>9}  {:>10}  {:>9}  {:>9}",
                "Dim", "Name", "Risk", "ReviewRisk", "Corridors", "Reviews"
            );
            println!("  {}", "─".repeat(78));
            for (d, total_risk, review_risk, corridor_count, review_count) in
                dimension_risks.iter().take(8)
            {
                println!(
                    "  {:>2}  {:<28}  {:>9.1}  {:>10.1}  {:>9}  {:>9}",
                    dim_names[*d],
                    dim_labels[*d],
                    total_risk,
                    review_risk,
                    corridor_count,
                    review_count
                );
            }

            std::fs::create_dir_all("data")?;
            let risk_path = PathBuf::from("data/confidence-risks.csv");
            let mut risk_wtr = csv::Writer::from_path(&risk_path)?;
            risk_wtr.write_record([
                "route",
                "score",
                "tier",
                "confidence",
                "score_confidence",
                "confidence_label",
                "score_confidence_label",
                "review",
                "risk_dimensions",
            ])?;
            for risk in &confidence_risks {
                let review = risk.score >= T2_THRESHOLD && risk.score_confidence < 0.75;
                risk_wtr.write_record([
                    risk.route.clone(),
                    format!("{:.1}", risk.score),
                    risk.tier.to_string(),
                    format!("{:.2}", risk.mean_confidence),
                    format!("{:.2}", risk.score_confidence),
                    route_score::confidence_label(risk.mean_confidence).to_string(),
                    route_score::confidence_label(risk.score_confidence).to_string(),
                    review.to_string(),
                    risk.risk_dimensions.clone(),
                ])?;
            }
            risk_wtr.flush()?;
            println!("  wrote confidence risk ledger → {}", risk_path.display());

            let summary_path = PathBuf::from("data/confidence-risk-summary.csv");
            let mut summary_wtr = csv::Writer::from_path(&summary_path)?;
            summary_wtr.write_record([
                "dimension",
                "name",
                "total_risk",
                "review_risk",
                "corridors",
                "review_corridors",
            ])?;
            for (d, total_risk, review_risk, corridor_count, review_count) in &dimension_risks {
                summary_wtr.write_record([
                    dim_names[*d].to_string(),
                    dim_labels[*d].to_string(),
                    format!("{total_risk:.1}"),
                    format!("{review_risk:.1}"),
                    corridor_count.to_string(),
                    review_count.to_string(),
                ])?;
            }
            summary_wtr.flush()?;
            println!(
                "  wrote confidence risk summary → {}",
                summary_path.display()
            );

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

fn dimension_confidence_values(scores: &route_score::DimensionScores) -> [f32; 16] {
    [
        scores.a1.confidence,
        scores.a2.confidence,
        scores.a3.confidence,
        scores.a4.confidence,
        scores.a5.confidence,
        scores.b1.confidence,
        scores.b2.confidence,
        scores.b3.confidence,
        scores.b4.confidence,
        scores.c1.confidence,
        scores.c2.confidence,
        scores.c3.confidence,
        scores.c4.confidence,
        scores.d1.confidence,
        scores.d2.confidence,
        scores.d3.confidence,
    ]
}

fn dimension_confidence_risks(scores: &[f64; 16], confidences: &[f32; 16]) -> [f64; 16] {
    let mut risks = [0.0; 16];
    for d in 0..16 {
        risks[d] = scores[d] * (1.0 - confidences[d].clamp(0.0, 1.0) as f64);
    }
    risks
}

fn confidence_risk_dimensions(scores: &[f64; 16], confidences: &[f32; 16]) -> String {
    let contribution = dimension_confidence_risks(scores, confidences);
    let mut risks: Vec<(&str, f64, f32, f64)> = DIMENSION_CODES
        .iter()
        .zip(scores.iter())
        .zip(confidences.iter())
        .zip(contribution.iter())
        .filter_map(|(((code, score), confidence), risk)| {
            let score = *score;
            let confidence = confidence.clamp(0.0, 1.0);
            if *risk >= 1.0 {
                Some((*code, score, confidence, *risk))
            } else {
                None
            }
        })
        .collect();

    risks.sort_by(|a, b| {
        b.3.total_cmp(&a.3)
            .then_with(|| b.1.total_cmp(&a.1))
            .then_with(|| a.0.cmp(b.0))
    });

    risks
        .into_iter()
        .take(3)
        .map(|(code, score, confidence, _)| format!("{code}:{score:.1}@{confidence:.2}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn write_tier_artifacts(score_rows: &[ScoreAllRow]) -> Result<()> {
    write_tier_artifacts_to(score_rows, Path::new("data"))
}

fn gap_type_slug(gap_type: &GapType) -> &'static str {
    match gap_type {
        GapType::MissingLink => "missing-link",
        GapType::Bottleneck => "bottleneck",
        GapType::Resilience => "resilience",
        GapType::Intermodal => "intermodal",
    }
}

fn write_gap_report(gap_type: &GapType, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut md = String::new();
    md.push_str(&format!("# Fault Lines — {}\n\n", gap_type_slug(gap_type)));
    md.push_str(&format!(
        "Generated by `route gap --type {}` on {}.\n\n",
        gap_type_slug(gap_type),
        route_date()
    ));

    match gap_type {
        GapType::MissingLink => write_missing_link_gap_section(&mut md)?,
        GapType::Bottleneck => write_bottleneck_gap_section(&mut md)?,
        GapType::Resilience => write_score_dimension_gap_section(
            &mut md,
            "Resilience Holes",
            "Routes with high D1 climate-resilience exposure under the current score ledger.",
            "D1",
            "D1 climate-resilience exposure",
            true,
        )?,
        GapType::Intermodal => write_intermodal_gap_section(&mut md)?,
    }

    std::fs::write(output_path, md)
        .with_context(|| format!("writing gap report {}", output_path.display()))
}

fn write_missing_link_gap_section(md: &mut String) -> Result<()> {
    md.push_str(
        "Source: `data/coverage-gaps.csv`, regenerated by `route coverage --threshold 30`.\n\n",
    );
    let mut rdr = csv::Reader::from_path("data/coverage-gaps.csv")
        .context("reading data/coverage-gaps.csv")?;
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut rows = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let class = csv_get(&record, 8).to_string();
        *counts.entry(class.clone()).or_default() += 1;
        if class == "candidate_access_gap" && rows.len() < 20 {
            rows.push(record);
        }
    }

    md.push_str("| Gap class | Counties |\n|---|---:|\n");
    for (class, count) in counts {
        md.push_str(&format!("| {class} | {count} |\n"));
    }
    md.push_str("\n## Top Candidate Access Gaps\n\n");
    md.push_str(
        "| County | State | Nearest mi | Population | Land sq mi |\n|---|---|---:|---:|---:|\n",
    );
    for row in rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, 5),
            csv_get(&row, 6),
            csv_get(&row, 7)
        ));
    }
    md.push_str("\nInterpretation: `candidate_access_gap` rows are not automatic construction recommendations; they are counties whose Census internal point is beyond the threshold after separating non-CONUS and large-county centroid-risk rows.\n");
    Ok(())
}

fn write_bottleneck_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/atri-bottlenecks.csv` hand-curated from ATRI truck bottleneck reporting.\n\n");
    let score_rows = load_score_signal_rows()?;
    let mut rdr =
        csv::Reader::from_path("data/atri-bottlenecks.csv").context("reading ATRI bottlenecks")?;
    md.push_str("| Rank | Location | Route | State | Annual cost $M | A1 | A3 | B2 | Signal |\n|---:|---|---|---|---:|---:|---:|---:|---|\n");
    for record in rdr.records().take(20) {
        let row = record?;
        let route = normalise_designation(csv_get(&row, 2));
        let signal = score_rows.get(&route);
        let (a1, a3, b2, label) = signal
            .map(|s| {
                (
                    format!("{:.1}", s.a1),
                    format!("{:.1}", s.a3),
                    format!("{:.1}", s.b2),
                    bottleneck_signal_label(s),
                )
            })
            .unwrap_or_else(|| ("".to_string(), "".to_string(), "".to_string(), "data_gap"));
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, 3),
            csv_get(&row, 4),
            a1,
            a3,
            b2,
            label
        ));
    }
    md.push_str("\nInterpretation: ATRI bottlenecks are observed freight congestion seeds. `corridor_stress` means A1/A3 confirms broad congestion or reliability stress; `topology_chokepoint` means the route is central but the congestion is likely local/interchange-specific; `capacity_needs_flow` needs `route flow` or segment-level validation before being labeled structural capacity.\n");
    Ok(())
}

#[derive(Debug)]
struct ScoreSignalRow {
    a1: f64,
    a3: f64,
    b2: f64,
}

fn load_score_signal_rows() -> Result<std::collections::HashMap<String, ScoreSignalRow>> {
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let route_idx = headers.iter().position(|h| h == "route").unwrap_or(0);
    let a1_idx = headers.iter().position(|h| h == "A1").unwrap_or(9);
    let a3_idx = headers.iter().position(|h| h == "A3").unwrap_or(11);
    let b2_idx = headers.iter().position(|h| h == "B2").unwrap_or(15);
    let mut rows = std::collections::HashMap::new();
    for record in rdr.records() {
        let row = record?;
        rows.insert(
            normalise_designation(csv_get(&row, route_idx)),
            ScoreSignalRow {
                a1: csv_get(&row, a1_idx).parse().unwrap_or(0.0),
                a3: csv_get(&row, a3_idx).parse().unwrap_or(0.0),
                b2: csv_get(&row, b2_idx).parse().unwrap_or(0.0),
            },
        );
    }
    Ok(rows)
}

fn bottleneck_signal_label(row: &ScoreSignalRow) -> &'static str {
    if row.a1 >= 7.0 || row.a3 >= 7.0 {
        "corridor_stress"
    } else if row.b2 >= 8.0 {
        "topology_chokepoint"
    } else {
        "capacity_needs_flow"
    }
}

fn write_score_dimension_gap_section(
    md: &mut String,
    title: &str,
    description: &str,
    dimension: &str,
    dimension_label: &str,
    descending: bool,
) -> Result<()> {
    md.push_str("Source: `data/scores-all.csv`, regenerated by `route score-all`.\n\n");
    md.push_str(&format!("## {title}\n\n{description}\n\n"));
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let dim_idx = headers
        .iter()
        .position(|h| h == dimension)
        .ok_or_else(|| anyhow::anyhow!("dimension column {dimension} not found"))?;
    let conf_idx = headers
        .iter()
        .position(|h| h == format!("{dimension}_conf"))
        .ok_or_else(|| anyhow::anyhow!("dimension confidence column not found"))?;
    let mut rows = Vec::new();
    for record in rdr.records() {
        let row = record?;
        let value = csv_get(&row, dim_idx).parse::<f64>().unwrap_or(0.0);
        rows.push((value, row));
    }
    if descending {
        rows.sort_by(|a, b| b.0.total_cmp(&a.0));
    } else {
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    md.push_str(&format!("| Route | Score | Tier | {dimension_label} | Confidence | Claim label |\n|---|---:|---|---:|---:|---|\n"));
    for (_, row) in rows.into_iter().take(20) {
        let confidence = csv_get(&row, conf_idx).parse::<f32>().unwrap_or(0.0);
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, dim_idx),
            csv_get(&row, conf_idx),
            route_score::confidence_label(confidence)
        ));
    }
    md.push_str("\nInterpretation: confidence labels describe source/model support for this dimension, not the importance of the corridor. Low or Medium claims need source review before being promoted to project recommendations.\n");
    Ok(())
}

fn write_intermodal_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/scores-all.csv`; candidates here have high B3 port/border access but low D2 multimodal integration.\n\n");
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let b3_idx = headers.iter().position(|h| h == "B3").unwrap_or(16);
    let d2_idx = headers.iter().position(|h| h == "D2").unwrap_or(23);
    let b3_conf_idx = headers.iter().position(|h| h == "B3_conf").unwrap_or(32);
    let d2_conf_idx = headers.iter().position(|h| h == "D2_conf").unwrap_or(39);
    let mut rows = Vec::new();
    for record in rdr.records() {
        let row = record?;
        let b3 = csv_get(&row, b3_idx).parse::<f64>().unwrap_or(0.0);
        let d2 = csv_get(&row, d2_idx).parse::<f64>().unwrap_or(0.0);
        if b3 >= 8.0 && d2 <= 5.0 {
            rows.push((b3 - d2, row));
        }
    }
    rows.sort_by(|a, b| b.0.total_cmp(&a.0));
    md.push_str("| Route | Score | Tier | B3 port/border | B3 conf | D2 multimodal | D2 conf | Claim label |\n|---|---:|---|---:|---:|---:|---:|---|\n");
    for (_, row) in rows.into_iter().take(20) {
        let b3_conf = csv_get(&row, b3_conf_idx).parse::<f32>().unwrap_or(0.0);
        let d2_conf = csv_get(&row, d2_conf_idx).parse::<f32>().unwrap_or(0.0);
        let claim_conf = b3_conf.min(d2_conf);
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, b3_idx),
            csv_get(&row, b3_conf_idx),
            csv_get(&row, d2_idx),
            csv_get(&row, d2_conf_idx),
            route_score::confidence_label(claim_conf)
        ));
    }
    md.push_str("\nInterpretation: these are port/border-adjacent corridors whose multimodal support is weak under the current source model; validate terminal and connector data before elevating them to projects.\n");
    Ok(())
}

fn csv_get(record: &csv::StringRecord, idx: usize) -> &str {
    record.get(idx).unwrap_or("")
}

fn route_date() -> String {
    std::env::var("ROUTE_DATE").unwrap_or_else(|_| "2026-05-06".to_string())
}

fn write_tier_artifacts_to(score_rows: &[ScoreAllRow], output_dir: &Path) -> Result<()> {
    let mut rows: Vec<&ScoreAllRow> = score_rows.iter().collect();
    rows.sort_by(|a, b| {
        a.tier
            .cmp(b.tier)
            .then_with(|| b.score.total_cmp(&a.score))
            .then_with(|| a.route.cmp(&b.route))
    });

    std::fs::create_dir_all(output_dir)?;
    let csv_path = output_dir.join("tier-table.csv");
    let mut wtr = csv::Writer::from_path(&csv_path)?;
    wtr.write_record([
        "tier",
        "route",
        "score",
        "rubric_version",
        "estimated",
        "confidence",
        "score_confidence",
        "confidence_label",
        "score_confidence_label",
    ])?;
    for row in &rows {
        wtr.write_record([
            row.tier.to_string(),
            row.route.clone(),
            format!("{:.1}", row.score),
            row.rubric_version.clone(),
            row.estimated.to_string(),
            format!("{:.2}", row.confidence),
            format!("{:.2}", row.score_confidence),
            route_score::confidence_label(row.confidence).to_string(),
            route_score::confidence_label(row.score_confidence).to_string(),
        ])?;
    }
    wtr.flush()?;

    let md_path = output_dir.join("tier-table.md");
    let mut md = String::new();
    md.push_str("# ROUTE Atlas Tier Table\n\n");
    md.push_str("Generated by `route score-all` from `data/scores-all.csv` inputs and the current scoring config.\n\n");
    md.push_str("| Tier | Corridors |\n|---|---:|\n");
    for tier in ["T1", "T2", "T3", "T4"] {
        let count = rows.iter().filter(|row| row.tier == tier).count();
        md.push_str(&format!("| {tier} | {count} |\n"));
    }
    md.push_str("\n| Tier | Route | Score | Score Confidence | Label | Estimated |\n|---|---|---:|---:|---|---|\n");
    for row in &rows {
        md.push_str(&format!(
            "| {} | {} | {:.1} | {:.2} | {} | {} |\n",
            row.tier,
            row.route,
            row.score,
            row.score_confidence,
            route_score::confidence_label(row.score_confidence),
            row.estimated
        ));
    }
    std::fs::write(&md_path, md)?;

    println!("  wrote tier table → {}", csv_path.display());
    println!("  wrote tier table markdown → {}", md_path.display());
    Ok(())
}

fn atlas_candidate_ids(graph: &route_network::HighwayGraph) -> Vec<String> {
    let mut ids = graph.interstate_ids();
    ids.extend(graph.us_highway_ids());
    ids.sort();
    ids.dedup();
    ids
}

#[derive(Debug, Clone)]
struct ScenarioEdgeCandidate {
    edge_id: u64,
    distance_miles: f64,
    length_miles: f64,
    aadt: Option<u32>,
    lanes: Option<u8>,
    state: String,
    mid_lat: f64,
    mid_lon: f64,
}

fn scenario_edge_candidates(
    graph: &route_network::HighwayGraph,
    route: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    top: usize,
) -> Vec<ScenarioEdgeCandidate> {
    let mut candidates: Vec<ScenarioEdgeCandidate> = graph
        .route_edges(route)
        .iter()
        .filter_map(|&ei| {
            let edge = &graph.graph[ei];
            let (mid_lat, mid_lon) = edge_midpoint(edge)?;
            let distance_miles = haversine_miles(lat, lon, mid_lat, mid_lon);
            (distance_miles <= radius_miles).then(|| ScenarioEdgeCandidate {
                edge_id: edge.id,
                distance_miles,
                length_miles: edge.length_miles,
                aadt: edge.aadt,
                lanes: edge.lane_count,
                state: edge.state.clone(),
                mid_lat,
                mid_lon,
            })
        })
        .collect();

    candidates.sort_by(|a, b| {
        a.distance_miles
            .partial_cmp(&b.distance_miles)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.edge_id.cmp(&b.edge_id))
    });
    candidates.truncate(top);
    candidates
}

fn edge_midpoint(edge: &route_network::HighwayEdge) -> Option<(f64, f64)> {
    let coords = edge.geometry.0.as_slice();
    if coords.is_empty() {
        return None;
    }
    let idx = coords.len() / 2;
    let coord = coords[idx];
    Some((coord.y, coord.x))
}

fn haversine_miles(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 3958.8_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().asin()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct StandardsProofRow {
    standard_id: String,
    tier: String,
    standard_family: String,
    standard: String,
    outcome: String,
    mechanism: String,
    primary_stressor: String,
    acceptance_gate: String,
    evidence_level: String,
    current_artifact: String,
    blocking_gap: String,
    next_command_or_test: String,
    owner_track: String,
}

fn load_standards_proof_ledger(path: &Path) -> Result<Vec<StandardsProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_standards_proof_ledger(file)
}

fn parse_standards_proof_ledger<R: std::io::Read>(reader: R) -> Result<Vec<StandardsProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn standards_evidence_level_is_allowed(level: &str) -> bool {
    matches!(
        level.trim().to_ascii_lowercase().as_str(),
        "implemented" | "heuristic" | "stub" | "planned" | "deprecated"
    )
}

fn standards_blueprint_gate_failures(rows: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    rows.iter()
        .filter(|row| {
            !standards_evidence_level_is_allowed(&row.evidence_level)
                || !row.evidence_level.eq_ignore_ascii_case("Implemented")
                || !row.blocking_gap.trim().is_empty()
        })
        .collect()
}

fn print_standards_proof(
    rows: &[StandardsProofRow],
    tier: Option<&str>,
    family: Option<&str>,
    details: bool,
) {
    let filtered: Vec<&StandardsProofRow> = rows
        .iter()
        .filter(|row| {
            tier.map(|t| row.tier.eq_ignore_ascii_case(t))
                .unwrap_or(true)
                && family
                    .map(|f| row.standard_family.eq_ignore_ascii_case(f))
                    .unwrap_or(true)
        })
        .collect();

    println!("route standards-proof");
    println!(
        "  standards: {} shown / {} total",
        filtered.len(),
        rows.len()
    );

    let mut by_level: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in &filtered {
        *by_level.entry(row.evidence_level.clone()).or_insert(0) += 1;
    }
    if !by_level.is_empty() {
        let summary = by_level
            .iter()
            .map(|(level, count)| format!("{level}: {count}"))
            .collect::<Vec<_>>()
            .join(", ");
        println!("  evidence: {summary}");
    }
    println!();

    println!(
        "{:<24} {:<4} {:<12} {:<11} {}",
        "Standard", "Tier", "Family", "Evidence", "Blocking gap"
    );
    println!("{}", "-".repeat(110));
    for row in filtered {
        println!(
            "{:<24} {:<4} {:<12} {:<11} {}",
            row.standard_id, row.tier, row.standard_family, row.evidence_level, row.blocking_gap
        );
        if details {
            println!("  standard: {}", row.standard);
            println!("  outcome: {}", row.outcome);
            println!("  mechanism: {}", row.mechanism);
            println!("  stressor: {}", row.primary_stressor);
            println!("  gate: {}", row.acceptance_gate);
            println!("  artifact: {}", row.current_artifact);
            println!("  next: {}", row.next_command_or_test);
            println!("  owner: {}", row.owner_track);
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PressureScenarioRow {
    scenario_id: String,
    scenario_name: String,
    adversity_class: String,
    standards_tested: String,
    current_status: String,
    existing_artifact: String,
    blocking_gap: String,
    next_evidence_step: String,
}

fn load_pressure_scenarios(path: &Path) -> Result<Vec<PressureScenarioRow>> {
    let file = std::fs::File::open(path)?;
    parse_pressure_scenarios(file)
}

fn parse_pressure_scenarios<R: std::io::Read>(reader: R) -> Result<Vec<PressureScenarioRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn print_pressure_scenarios(rows: &[PressureScenarioRow], blockers: bool, details: bool) {
    let failures = pressure_scenario_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.current_status.clone()).or_insert(0) += 1;
    }

    println!("route pressure-scenarios");
    println!(
        "  scenarios: {} shown / {} total",
        filtered.len(),
        rows.len()
    );
    println!("  status: {}", format_count_map(&by_status));
    println!("  L2 gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<24} {:<14} {:<28} {}",
        "Scenario", "Name", "Status", "Adversity", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<24} {:<14} {:<28} {}",
            row.scenario_id,
            truncate_for_table(&row.scenario_name, 24),
            row.current_status,
            truncate_for_table(&row.adversity_class, 28),
            row.blocking_gap
        );
        if details {
            println!("  standards: {}", row.standards_tested);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}

fn pressure_scenario_gate_failures(rows: &[PressureScenarioRow]) -> Vec<&PressureScenarioRow> {
    rows.iter()
        .filter(|row| !pressure_scenario_has_bounded_contract(row))
        .collect()
}

fn pressure_scenario_missing_required_adversity(rows: &[PressureScenarioRow]) -> Vec<&'static str> {
    const REQUIRED: &[(&str, &[&str])] = &[
        ("T1/T1 closure", &["t1/t1"]),
        ("corridor segment closure", &["corridor segment", "closure"]),
        ("port surge", &["port surge"]),
        ("weather/flood disruption", &["weather", "flood"]),
        ("relay hub outage", &["relay hub outage"]),
        ("EV/rest-area outage", &["ev/rest-area outage"]),
        ("managed-lane sensitivity", &["managed-lane"]),
    ];

    REQUIRED
        .iter()
        .filter_map(|(label, terms)| {
            let covered = rows.iter().any(|row| {
                let class = row.adversity_class.to_ascii_lowercase();
                if *label == "weather/flood disruption" {
                    terms.iter().any(|term| class.contains(term))
                } else {
                    terms.iter().all(|term| class.contains(term))
                }
            });
            (!covered).then_some(*label)
        })
        .collect()
}

fn pressure_scenario_has_bounded_contract(row: &PressureScenarioRow) -> bool {
    let has_identity = row.scenario_id.starts_with("S-L2-")
        && !row.scenario_name.trim().is_empty()
        && !row.adversity_class.trim().is_empty();
    let has_test_scope = !row.standards_tested.trim().is_empty()
        && row
            .standards_tested
            .split(';')
            .any(|value| value.trim().starts_with('T'));
    let has_artifact = !row.existing_artifact.trim().is_empty();
    let has_next_step = !row.next_evidence_step.trim().is_empty();
    let status = row.current_status.to_ascii_lowercase();
    let status_is_labeled = matches!(
        status.as_str(),
        "implemented" | "heuristic" | "planned" | "stub" | "deprecated"
    );

    has_identity && has_test_scope && has_artifact && has_next_step && status_is_labeled
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ThroughputProofRow {
    proof_id: String,
    proof_name: String,
    binding_type: String,
    stressor: String,
    primary_metric: String,
    existing_artifact: String,
    current_status: String,
    blocking_gap: String,
    next_evidence_step: String,
}

fn load_throughput_proof_matrix(path: &Path) -> Result<Vec<ThroughputProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_throughput_proof_matrix(file)
}

fn parse_throughput_proof_matrix<R: std::io::Read>(reader: R) -> Result<Vec<ThroughputProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn print_throughput_proof_matrix(rows: &[ThroughputProofRow], blockers: bool, details: bool) {
    let failures = throughput_proof_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_binding: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_binding.entry(row.binding_type.clone()).or_insert(0) += 1;
    }

    println!("route throughput-proof");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  binding: {}", format_count_map(&by_binding));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<26} {:<20} {:<12} {}",
        "Proof", "Name", "Binding", "Status", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<26} {:<20} {:<12} {}",
            row.proof_id,
            truncate_for_table(&row.proof_name, 26),
            row.binding_type,
            row.current_status,
            row.blocking_gap
        );
        if details {
            println!("  stressor: {}", row.stressor);
            println!("  metric: {}", row.primary_metric);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}

fn throughput_proof_gate_failures(rows: &[ThroughputProofRow]) -> Vec<&ThroughputProofRow> {
    rows.iter()
        .filter(|row| !throughput_proof_has_bounded_contract(row))
        .collect()
}

fn throughput_proof_has_bounded_contract(row: &ThroughputProofRow) -> bool {
    let binding = row.binding_type.trim().to_ascii_lowercase();
    let binding_is_labeled = matches!(
        binding.as_str(),
        "congestion_binding" | "resilience_binding"
    );
    row.proof_id.starts_with("TP-")
        && !row.proof_name.trim().is_empty()
        && binding_is_labeled
        && !row.stressor.trim().is_empty()
        && !row.primary_metric.trim().is_empty()
        && !row.existing_artifact.trim().is_empty()
        && standards_evidence_level_is_allowed(&row.current_status)
        && !row.next_evidence_step.trim().is_empty()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1FailureRow {
    site_id: String,
    intersection: String,
    location: String,
    failure_mode: String,
    annual_probability: Option<f64>,
    duration_p50_hours: Option<f64>,
    duration_p95_hours: Option<f64>,
    throughput_retention_current: Option<f64>,
    throughput_retention_i2: Option<f64>,
    reroute_time_p50_hours: Option<f64>,
    reroute_time_p95_hours: Option<f64>,
    source_status: String,
    confidence: String,
    current_artifact: String,
    blocking_gap: String,
    next_evidence_step: String,
}

fn load_t1_failure_ledger(path: &Path) -> Result<Vec<T1FailureRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_ledger(file)
}

fn parse_t1_failure_ledger<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn write_t1_failure_ledger(path: &Path, rows: &[T1FailureRow]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}

fn print_t1_failures(rows: &[T1FailureRow], needs_sources: bool, details: bool) {
    let filtered: Vec<&T1FailureRow> = rows
        .iter()
        .filter(|row| !needs_sources || row.source_status.eq_ignore_ascii_case("source_needed"))
        .collect();

    let empirical = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("empirical"))
        .count();
    let modeled = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("modeled"))
        .count();
    let source_needed = rows
        .iter()
        .filter(|row| row.source_status.eq_ignore_ascii_case("source_needed"))
        .count();

    println!("route t1-failures");
    println!("  sites: {} shown / {} total", filtered.len(), rows.len());
    println!("  evidence: empirical {empirical}, modeled {modeled}, source_needed {source_needed}");
    println!();
    println!(
        "{:<18} {:<14} {:<18} {:<13} {:>8} {:>8} {:>8} {}",
        "Site", "Intersection", "Location", "Source", "P_fail", "KeepNow", "KeepI2", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<18} {:<13} {:>8} {:>8} {:>8} {}",
            row.site_id,
            row.intersection,
            row.location,
            row.source_status,
            fmt_opt(row.annual_probability),
            fmt_opt(row.throughput_retention_current),
            fmt_opt(row.throughput_retention_i2),
            row.blocking_gap
        );
        if details {
            println!("  failure mode: {}", row.failure_mode);
            println!(
                "  duration p50/p95: {} / {} h",
                fmt_opt(row.duration_p50_hours),
                fmt_opt(row.duration_p95_hours)
            );
            println!(
                "  reroute p50/p95: {} / {} h",
                fmt_opt(row.reroute_time_p50_hours),
                fmt_opt(row.reroute_time_p95_hours)
            );
            println!("  confidence: {}", row.confidence);
            println!("  artifact: {}", row.current_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}

fn t1_failure_evidence_gate_failures(rows: &[T1FailureRow]) -> Vec<&T1FailureRow> {
    rows.iter()
        .filter(|row| !t1_failure_row_has_evidence_contract(row))
        .collect()
}

fn t1_failure_row_has_evidence_contract(row: &T1FailureRow) -> bool {
    let status = row.source_status.trim().to_ascii_lowercase();
    let status_is_labeled = matches!(status.as_str(), "empirical" | "modeled" | "source_needed");
    let confidence = row.confidence.trim().to_ascii_lowercase();
    let confidence_is_labeled =
        matches!(confidence.as_str(), "high" | "medium" | "low" | "unknown");
    let source_needed_has_gap = status != "source_needed" || !row.blocking_gap.trim().is_empty();

    !row.site_id.trim().is_empty()
        && !row.intersection.trim().is_empty()
        && !row.failure_mode.trim().is_empty()
        && status_is_labeled
        && confidence_is_labeled
        && !row.current_artifact.trim().is_empty()
        && !row.next_evidence_step.trim().is_empty()
        && source_needed_has_gap
}

fn fmt_opt(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.3}"))
        .unwrap_or_else(|| "-".to_string())
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1FailureSourceRow {
    site_id: String,
    intersection: String,
    location: String,
    primary_state_sources: String,
    national_sources: String,
    fields_to_populate: String,
    access_status: String,
    source_url: String,
    notes: String,
}

fn load_t1_failure_source_plan(path: &Path) -> Result<Vec<T1FailureSourceRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_source_plan(file)
}

fn parse_t1_failure_source_plan<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureSourceRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn print_t1_failure_sources(rows: &[T1FailureSourceRow], lookup_needed: bool) {
    let filtered: Vec<&T1FailureSourceRow> = rows
        .iter()
        .filter(|row| !lookup_needed || row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .collect();
    let identified = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("identified"))
        .count();
    let lookup = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .count();

    println!("route t1-failure-sources");
    println!("  sources: {} shown / {} total", filtered.len(), rows.len());
    println!("  access: identified {identified}, lookup_needed {lookup}");
    println!();
    println!(
        "{:<18} {:<14} {:<18} {:<14} {}",
        "Site", "Intersection", "Location", "Access", "Primary sources"
    );
    println!("{}", "-".repeat(120));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<18} {:<14} {}",
            row.site_id,
            row.intersection,
            row.location,
            row.access_status,
            row.primary_state_sources
        );
        println!("  fields: {}", row.fields_to_populate);
        println!("  national: {}", row.national_sources);
        if !row.source_url.trim().is_empty() {
            println!("  url: {}", row.source_url);
        }
        println!("  notes: {}", row.notes);
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1SourceHealthRow {
    site_id: String,
    source_name: String,
    source_url: String,
    source_kind: String,
    access_health: String,
    ingestion_status: String,
    history_status: String,
    last_checked: String,
    blocking_gap: String,
    next_step: String,
}

fn load_t1_source_health(path: &Path) -> Result<Vec<T1SourceHealthRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_source_health(file)
}

fn parse_t1_source_health<R: std::io::Read>(reader: R) -> Result<Vec<T1SourceHealthRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn print_t1_source_health(rows: &[T1SourceHealthRow], blockers: bool, details: bool) {
    let blocked = t1_source_health_blockers(rows);
    let filtered = if blockers {
        blocked.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_access: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut by_ingestion: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_access.entry(row.access_health.clone()).or_insert(0) += 1;
        *by_ingestion
            .entry(row.ingestion_status.clone())
            .or_insert(0) += 1;
    }

    println!("route t1-source-health");
    println!("  sources: {} shown / {} total", filtered.len(), rows.len());
    println!("  access: {}", format_count_map(&by_access));
    println!("  ingestion: {}", format_count_map(&by_ingestion));
    println!();
    println!(
        "{:<18} {:<24} {:<16} {:<14} {:<14} {}",
        "Site", "Source", "Access", "Ingestion", "History", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<24} {:<16} {:<14} {:<14} {}",
            row.site_id,
            truncate_for_table(&row.source_name, 24),
            row.access_health,
            row.ingestion_status,
            row.history_status,
            row.blocking_gap
        );
        if details {
            println!("  kind: {}", row.source_kind);
            println!("  last checked: {}", row.last_checked);
            println!("  url: {}", row.source_url);
            println!("  next: {}", row.next_step);
        }
    }
}

fn t1_source_health_blockers(rows: &[T1SourceHealthRow]) -> Vec<&T1SourceHealthRow> {
    rows.iter()
        .filter(|row| t1_source_health_is_blocked(row))
        .collect()
}

fn t1_source_health_is_blocked(row: &T1SourceHealthRow) -> bool {
    !matches!(
        (
            row.access_health.as_str(),
            row.ingestion_status.as_str(),
            row.history_status.as_str()
        ),
        ("live", "implemented", "snapshot_only") | ("live", "documented", "historical_method")
    )
}

fn print_t1_access_docket(rows: &[T1SourceHealthRow], category: Option<&str>, details: bool) {
    let mut docket = rows
        .iter()
        .filter(|row| t1_source_health_is_blocked(row))
        .map(t1_access_docket_item)
        .filter(|item| {
            category
                .map(|category| item.category.eq_ignore_ascii_case(category))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    docket.sort_by(|a, b| {
        t1_access_priority_rank(&a.priority)
            .cmp(&t1_access_priority_rank(&b.priority))
            .then_with(|| a.category.cmp(&b.category))
            .then_with(|| a.site_id.cmp(&b.site_id))
    });

    let mut by_category: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for item in &docket {
        *by_category.entry(item.category.clone()).or_insert(0) += 1;
    }

    println!("route t1-access-docket");
    println!("  actions: {} shown", docket.len());
    println!("  categories: {}", format_count_map(&by_category));
    println!();
    println!(
        "{:<10} {:<16} {:<18} {:<24} {}",
        "Priority", "Category", "Site", "Source", "Action"
    );
    println!("{}", "-".repeat(132));
    for item in docket {
        println!(
            "{:<10} {:<16} {:<18} {:<24} {}",
            item.priority,
            item.category,
            item.site_id,
            truncate_for_table(&item.source_name, 24),
            item.action
        );
        if details {
            println!("  access: {}", item.access_health);
            println!("  history: {}", item.history_status);
            println!("  url: {}", item.source_url);
            println!("  gap: {}", item.blocking_gap);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct T1AccessDocketItem {
    site_id: String,
    source_name: String,
    source_url: String,
    access_health: String,
    history_status: String,
    blocking_gap: String,
    category: String,
    priority: String,
    action: String,
}

fn t1_access_docket_item(row: &T1SourceHealthRow) -> T1AccessDocketItem {
    let category = t1_access_category(row).to_string();
    let priority = t1_access_priority(row).to_string();
    let action = match category.as_str() {
        "api_key" => format!("Request credentials; then implement {}", row.source_name),
        "account" => format!("Obtain account/export; then map {}", row.source_name),
        "access_request" => format!(
            "Request data access or partner extract for {}",
            row.source_name
        ),
        "endpoint_tuning" => format!("Tune query/export path for {}", row.source_name),
        "records_request" => format!(
            "Request archive/export or identify allowed endpoint for {}",
            row.source_name
        ),
        _ => row.next_step.clone(),
    };
    T1AccessDocketItem {
        site_id: row.site_id.clone(),
        source_name: row.source_name.clone(),
        source_url: row.source_url.clone(),
        access_health: row.access_health.clone(),
        history_status: row.history_status.clone(),
        blocking_gap: row.blocking_gap.clone(),
        category,
        priority,
        action,
    }
}

fn t1_access_category(row: &T1SourceHealthRow) -> &'static str {
    match row.access_health.as_str() {
        "requires_key" => "api_key",
        "requires_account" => "account",
        "requires_access" => "access_request",
        "blocked_query" => "endpoint_tuning",
        "blocked_access" => "records_request",
        _ if row.ingestion_status != "implemented" => "implementation",
        _ if row.history_status == "snapshot_only" => "history_archive",
        _ => "monitoring",
    }
}

fn t1_access_priority(row: &T1SourceHealthRow) -> &'static str {
    if row.source_kind == "travel_time_reliability" {
        "critical"
    } else if row.access_health == "blocked_query" || row.access_health == "blocked_access" {
        "high"
    } else if row.access_health == "requires_access" || row.access_health == "requires_key" {
        "high"
    } else {
        "medium"
    }
}

fn t1_access_priority_rank(priority: &str) -> u8 {
    match priority {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        _ => 3,
    }
}

fn format_count_map(counts: &std::collections::BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        "none".to_string()
    } else {
        counts
            .iter()
            .map(|(key, count)| format!("{key}: {count}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn truncate_for_table(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        value.to_string()
    } else {
        value
            .chars()
            .take(width.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1FailureEventRow {
    site_id: String,
    event_id: String,
    source: String,
    source_event_id: String,
    observation_year: u16,
    start_time: String,
    end_time: String,
    duration_hours: Option<f64>,
    event_type: String,
    full_closure: bool,
    lanes_closed: Option<u8>,
    freight_relevant: bool,
    confidence: String,
    notes: String,
}

#[derive(Debug, Clone, PartialEq)]
struct T1FailureEventSummary {
    site_id: String,
    observed_years: usize,
    event_count: usize,
    annual_rate: f64,
    annual_probability: f64,
    duration_p50_hours: Option<f64>,
    duration_p95_hours: Option<f64>,
    confidence: String,
}

fn load_t1_failure_events(path: &Path) -> Result<Vec<T1FailureEventRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_events(file)
}

fn write_t1_failure_events(path: &Path, rows: &[T1FailureEventRow]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}

fn parse_t1_failure_events<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureEventRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn merge_t1_failure_events(
    existing: &[T1FailureEventRow],
    incoming: &[T1FailureEventRow],
) -> Vec<T1FailureEventRow> {
    let mut rows = existing.to_vec();
    let mut seen = rows
        .iter()
        .map(t1_failure_event_key)
        .collect::<std::collections::BTreeSet<_>>();

    for row in incoming {
        if seen.insert(t1_failure_event_key(row)) {
            rows.push(row.clone());
        }
    }

    rows.sort_by(|a, b| {
        a.site_id
            .cmp(&b.site_id)
            .then_with(|| a.observation_year.cmp(&b.observation_year))
            .then_with(|| a.event_id.cmp(&b.event_id))
    });
    rows
}

fn t1_failure_event_key(row: &T1FailureEventRow) -> (String, String) {
    (row.site_id.clone(), row.event_id.clone())
}

fn fetch_iowa511_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://services.arcgis.com/8lRhdTsQyJpO52F1/arcgis/rest/services/CARS511_Iowa_View/FeatureServer/0/query?f=json&where=1%3D1&outFields=ID,Route,StartTime,EndTime,IssueDate,IssueTime,headline,cause,Restrict_,Desc0&returnGeometry=true&outSR=4326";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    std::fs::write(output, body)?;
    Ok(())
}

fn fetch_tdot_smartway_events(output: &Path, timeout_seconds: u64) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://spatial.tdot.tn.gov/arcgis/rest/services/Smartway/Smartway_Events/FeatureServer/1/query?f=json&where=1%3D1&outFields=ID,START_DATE,END_DATE,CD_ROAD_NAMES,CD_DIRECTION,EVENT_TYPE,EVENT_SUBTYPE,DESCRIPTION,HAS_CLOSURE,MIDPOINT_LATITUDE_DD,MIDPOINT_LONGITUDE_DD,COUNTY_NAME&returnGeometry=false&resultRecordCount=200";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_seconds.max(1)))
        .build()?;
    let body = client.get(url).send()?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    std::fs::write(output, body)?;
    Ok(())
}

fn fetch_mdot_midrive_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://mdotjboss.state.mi.us/MiDrive/incidents/AllForMap/";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    std::fs::write(output, body)?;
    Ok(())
}

fn fetch_indot_trafficwise_events(
    output: &Path,
    north: f64,
    south: f64,
    east: f64,
    west: f64,
    zoom: u8,
) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let query = r#"
query MapFeatures($input: MapFeaturesArgs!) {
  mapFeaturesQuery(input: $input) {
    mapFeatures {
      bbox
      title
      tooltip
      uri
      __typename
      features {
        id
        geometry
        properties
        type
      }
    }
    error {
      message
      type
    }
  }
}
"#;
    let body = serde_json::json!({
        "query": query,
        "variables": {
            "input": {
                "north": north,
                "south": south,
                "east": east,
                "west": west,
                "zoom": zoom,
                "layerSlugs": ["incidents", "construction"]
            }
        }
    });
    let client = reqwest::blocking::Client::new();
    let request_body = serde_json::to_string(&body)?;
    let text = client
        .post("https://511in.org/api/graphql")
        .header("content-type", "application/json")
        .body(request_body)
        .send()?
        .error_for_status()?
        .text()?;
    ensure_no_graphql_errors(&text)?;
    std::fs::write(output, text)?;
    Ok(())
}

fn ensure_no_arcgis_error(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(error) = value.get("error") {
        let message = error
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("ArcGIS query failed");
        let details = error
            .get("details")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_default();
        if details.is_empty() {
            anyhow::bail!("{message}");
        } else {
            anyhow::bail!("{message}: {details}");
        }
    }
    Ok(())
}

fn ensure_no_graphql_errors(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(errors) = value.get("errors").and_then(|value| value.as_array()) {
        let messages = errors
            .iter()
            .filter_map(|error| error.get("message").and_then(|value| value.as_str()))
            .collect::<Vec<_>>()
            .join("; ");
        if messages.is_empty() {
            anyhow::bail!("GraphQL query failed");
        } else {
            anyhow::bail!("{messages}");
        }
    }
    Ok(())
}

fn parse_iowa511_events(
    json: &str,
    site_id: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    ensure_no_arcgis_error(json)?;
    let Some(features) = value.get("features").and_then(|value| value.as_array()) else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for feature in features {
        let attrs = feature
            .get("attributes")
            .and_then(|value| value.as_object())
            .cloned()
            .unwrap_or_default();
        let geometry = feature.get("geometry");
        let event_lon = geometry
            .and_then(|value| value.get("x"))
            .and_then(|value| value.as_f64());
        let event_lat = geometry
            .and_then(|value| value.get("y"))
            .and_then(|value| value.as_f64());
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let route = json_string(&attrs, "Route");
        let text = [
            route.as_str(),
            json_string(&attrs, "headline").as_str(),
            json_string(&attrs, "cause").as_str(),
            json_string(&attrs, "Restrict_").as_str(),
            json_string(&attrs, "Desc0").as_str(),
        ]
        .join(" ");
        if !iowa511_is_t1_relevant(&route, &text) {
            continue;
        }

        let issue_date = json_string(&attrs, "IssueDate");
        let observation_year = issue_date
            .get(0..4)
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(0);
        let start_time = json_string(&attrs, "StartTime");
        let end_time = json_string(&attrs, "EndTime");
        let duration_hours = same_day_duration_hours(&start_time, &end_time);
        let source_event_id = json_string(&attrs, "ID");
        let event_id = if source_event_id.trim().is_empty() {
            format!("IOWA511-{}", rows.len() + 1)
        } else {
            format!("IOWA511-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "Iowa DOT 511 ArcGIS".to_string(),
            source_event_id,
            observation_year,
            start_time: combine_iowa_date_time(&issue_date, &start_time),
            end_time: combine_iowa_date_time(&issue_date, &end_time),
            duration_hours,
            event_type: iowa511_event_type(&text).to_string(),
            full_closure: iowa511_full_closure(&text),
            lanes_closed: None,
            freight_relevant: true,
            confidence: if duration_hours.is_some() {
                "medium".to_string()
            } else {
                "low".to_string()
            },
            notes: compact_note(&text),
        });
    }
    Ok(rows)
}

fn parse_tdot_smartway_events(
    json: &str,
    site_id: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    ensure_no_arcgis_error(json)?;
    let Some(features) = value.get("features").and_then(|value| value.as_array()) else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for feature in features {
        let attrs = feature
            .get("attributes")
            .and_then(|value| value.as_object())
            .cloned()
            .unwrap_or_default();
        let event_lat = json_f64(&attrs, "MIDPOINT_LATITUDE_DD");
        let event_lon = json_f64(&attrs, "MIDPOINT_LONGITUDE_DD");
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let road_names = json_string(&attrs, "CD_ROAD_NAMES");
        let text = [
            road_names.as_str(),
            json_string(&attrs, "CD_DIRECTION").as_str(),
            json_string(&attrs, "EVENT_TYPE").as_str(),
            json_string(&attrs, "EVENT_SUBTYPE").as_str(),
            json_string(&attrs, "DESCRIPTION").as_str(),
            json_string(&attrs, "COUNTY_NAME").as_str(),
        ]
        .join(" ");
        if !tdot_smartway_is_t1_relevant(&road_names, &text) {
            continue;
        }

        let source_event_id = json_string(&attrs, "ID");
        let event_id = if source_event_id.trim().is_empty() {
            format!("TDOT-SMARTWAY-{}", rows.len() + 1)
        } else {
            format!("TDOT-SMARTWAY-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        let start_ms = json_i64(&attrs, "START_DATE");
        let end_ms = json_i64(&attrs, "END_DATE");
        let duration_hours = match (start_ms, end_ms) {
            (Some(start), Some(end)) if end >= start => Some((end - start) as f64 / 3_600_000.0),
            _ => None,
        };
        let observation_year = start_ms.and_then(epoch_millis_year).unwrap_or(0);

        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "TDOT SmartWay ArcGIS".to_string(),
            source_event_id,
            observation_year,
            start_time: start_ms
                .and_then(epoch_millis_date)
                .unwrap_or_else(|| json_string(&attrs, "START_DATE")),
            end_time: end_ms
                .and_then(epoch_millis_date)
                .unwrap_or_else(|| json_string(&attrs, "END_DATE")),
            duration_hours,
            event_type: tdot_smartway_event_type(&text).to_string(),
            full_closure: json_i64(&attrs, "HAS_CLOSURE").unwrap_or(0) > 0,
            lanes_closed: None,
            freight_relevant: true,
            confidence: if duration_hours.is_some() {
                "medium".to_string()
            } else {
                "low".to_string()
            },
            notes: compact_note(&text),
        });
    }
    Ok(rows)
}

fn parse_mdot_midrive_events(
    json: &str,
    site_id: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    observation_year: u16,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let Some(events) = value.as_array() else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for event in events {
        let event_lat = event.get("latitude").and_then(|value| value.as_f64());
        let event_lon = event.get("longitude").and_then(|value| value.as_f64());
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let title = json_value_string(event, "title");
        let message = strip_html_tags(&json_value_string(event, "message"));
        let text = compact_note(&format!("{title} {message}"));
        if !mdot_midrive_is_t1_relevant(&text) {
            continue;
        }

        let source_event_id = event
            .get("id")
            .map(json_scalar_to_string)
            .unwrap_or_default();
        let event_id = if source_event_id.trim().is_empty() {
            format!("MDOT-MIDRIVE-{}", rows.len() + 1)
        } else {
            format!("MDOT-MIDRIVE-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        let reported_time = extract_after_label(&message, "Reported:");
        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "MDOT Mi Drive".to_string(),
            source_event_id,
            observation_year,
            start_time: reported_time.unwrap_or_default(),
            end_time: String::new(),
            duration_hours: None,
            event_type: mdot_midrive_event_type(&text).to_string(),
            full_closure: mdot_midrive_full_closure(&text),
            lanes_closed: mdot_midrive_lanes_closed(&text),
            freight_relevant: true,
            confidence: "low".to_string(),
            notes: text,
        });
    }
    Ok(rows)
}

fn parse_indot_trafficwise_events(
    json: &str,
    site_id: &str,
    observation_year: u16,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    ensure_no_graphql_errors(json)?;
    let Some(features) = value
        .get("data")
        .and_then(|value| value.get("mapFeaturesQuery"))
        .and_then(|value| value.get("mapFeatures"))
        .and_then(|value| value.as_array())
    else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for feature in features {
        if json_value_string(feature, "__typename") != "Event" {
            continue;
        }
        let title = json_value_string(feature, "title");
        let tooltip = strip_html_tags(&json_value_string(feature, "tooltip"));
        let text = compact_note(&format!("{title} {tooltip}"));
        if !indot_trafficwise_is_t1_relevant(&text) {
            continue;
        }

        let uri = json_value_string(feature, "uri");
        let source_event_id = uri
            .strip_prefix("event/")
            .unwrap_or(uri.as_str())
            .to_string();
        let event_id = if source_event_id.trim().is_empty() {
            format!("INDOT-TRAFFICWISE-{}", rows.len() + 1)
        } else {
            format!("INDOT-TRAFFICWISE-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "INDOT TrafficWise GraphQL".to_string(),
            source_event_id,
            observation_year,
            start_time: String::new(),
            end_time: String::new(),
            duration_hours: None,
            event_type: indot_trafficwise_event_type(&text).to_string(),
            full_closure: indot_trafficwise_full_closure(&text),
            lanes_closed: mdot_midrive_lanes_closed(&text),
            freight_relevant: true,
            confidence: "low".to_string(),
            notes: text,
        });
    }
    Ok(rows)
}

fn json_string(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn json_value_string(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn json_scalar_to_string(value: &serde_json::Value) -> String {
    value
        .as_str()
        .map(str::to_string)
        .or_else(|| value.as_i64().map(|value| value.to_string()))
        .or_else(|| value.as_u64().map(|value| value.to_string()))
        .unwrap_or_default()
}

fn json_f64(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<f64> {
    attrs.get(key).and_then(|value| {
        value
            .as_f64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
    })
}

fn json_i64(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<i64> {
    attrs.get(key).and_then(|value| {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
    })
}

fn iowa511_is_t1_relevant(route: &str, text: &str) -> bool {
    let route_norm = route.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase();
    (route_norm.contains("I-35")
        || route_norm.contains("I35")
        || route_norm.contains("I-80")
        || route_norm.contains("I80"))
        && ["CLOSED", "CLOSURE", "CONSTRUCTION", "CRASH", "INCIDENT"]
            .iter()
            .any(|needle| text_norm.contains(needle))
}

fn tdot_smartway_is_t1_relevant(road_names: &str, text: &str) -> bool {
    let route_norm = road_names.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (route_norm.contains("I-40")
        || route_norm.contains("I40")
        || route_norm.contains("I-75")
        || route_norm.contains("I75")
        || text_norm.contains("I-40")
        || text_norm.contains("I40")
        || text_norm.contains("I-75")
        || text_norm.contains("I75"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}

fn mdot_midrive_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-75")
        || text_norm.contains("I75")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("I-96")
        || text_norm.contains("I96")
        || text_norm.contains("I-275")
        || text_norm.contains("I275")
        || text_norm.contains("I-696")
        || text_norm.contains("I696"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}

fn indot_trafficwise_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-80")
        || text_norm.contains("I80")
        || text_norm.contains("I-90")
        || text_norm.contains("I90")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("TOLLROAD"))
        && [
            "CLOSURE",
            "CLOSED",
            "CRASH",
            "INCIDENT",
            "CONSTRUCTION",
            "ROADWORK",
            "LANE CLOSED",
        ]
        .iter()
        .any(|needle| text.to_ascii_uppercase().contains(needle))
}

fn tdot_smartway_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

fn indot_trafficwise_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("roadwork") || text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

fn mdot_midrive_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

fn iowa511_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closed") || text.contains("closure") {
        "closure"
    } else {
        "incident"
    }
}

fn iowa511_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    if text.contains("shoulder") || text.contains("lane closed") || text.contains("lanes closed") {
        return false;
    }
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains(": closed")
}

fn indot_trafficwise_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains("exit ramp closed")
        || text.contains("freeway closed")
}

fn mdot_midrive_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("all lanes")
        || text.contains("road closed")
        || text.contains("freeway closed")
        || text.contains("ramp closed")
}

fn mdot_midrive_lanes_closed(text: &str) -> Option<u8> {
    let text = text.to_ascii_lowercase();
    if text.contains("center lane") && (text.contains("left lane") || text.contains("right lane")) {
        Some(2)
    } else if text.contains("left lane") && text.contains("right lane") {
        Some(2)
    } else if text.contains("two lanes") || text.contains("2 lanes") {
        Some(2)
    } else if text.contains("three lanes") || text.contains("3 lanes") {
        Some(3)
    } else if text.contains("left lane") || text.contains("right lane") || text.contains("1 lane") {
        Some(1)
    } else if text.contains("left shoulder") || text.contains("right shoulder") {
        Some(0)
    } else {
        None
    }
}

fn epoch_millis_year(millis: i64) -> Option<u16> {
    epoch_millis_ymd(millis).and_then(|(year, _, _)| u16::try_from(year).ok())
}

fn epoch_millis_date(millis: i64) -> Option<String> {
    epoch_millis_ymd(millis).map(|(year, month, day)| format!("{year:04}-{month:02}-{day:02}"))
}

fn epoch_millis_ymd(millis: i64) -> Option<(i32, u32, u32)> {
    if millis < 0 {
        return None;
    }
    let days = millis.div_euclid(86_400_000);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let day = doy - (153 * mp + 2).div_euclid(5) + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    Some((year as i32, month as u32, day as u32))
}

fn current_utc_year() -> u16 {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0);
    epoch_millis_year(millis).unwrap_or(1970)
}

fn compact_note(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_html_tags(input: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => {
                in_tag = true;
                output.push(' ');
            }
            '>' => {
                in_tag = false;
                output.push(' ');
            }
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
}

fn extract_after_label(text: &str, label: &str) -> Option<String> {
    let (_, tail) = text.split_once(label)?;
    let value = tail.split('|').next().unwrap_or(tail).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn combine_iowa_date_time(issue_date: &str, time: &str) -> String {
    if issue_date.len() != 8 || time.trim().is_empty() {
        return time.to_string();
    }
    format!(
        "{}-{}-{} {}",
        &issue_date[0..4],
        &issue_date[4..6],
        &issue_date[6..8],
        time.trim()
    )
}

fn same_day_duration_hours(start: &str, end: &str) -> Option<f64> {
    let start = parse_12h_minutes(start)?;
    let end = parse_12h_minutes(end)?;
    if end >= start {
        Some((end - start) as f64 / 60.0)
    } else {
        None
    }
}

fn parse_12h_minutes(input: &str) -> Option<i32> {
    let input = input.trim();
    let (time, suffix) = input.rsplit_once(' ')?;
    let (hour, minute) = time.split_once(':')?;
    let mut hour = hour.parse::<i32>().ok()?;
    let minute = minute.parse::<i32>().ok()?;
    if !(1..=12).contains(&hour) || !(0..=59).contains(&minute) {
        return None;
    }
    let suffix = suffix.to_ascii_uppercase();
    if suffix == "PM" && hour != 12 {
        hour += 12;
    } else if suffix == "AM" && hour == 12 {
        hour = 0;
    } else if suffix != "AM" && suffix != "PM" {
        return None;
    }
    Some(hour * 60 + minute)
}

fn summarize_t1_failure_events(rows: &[T1FailureEventRow]) -> Vec<T1FailureEventSummary> {
    let mut by_site: std::collections::BTreeMap<&str, Vec<&T1FailureEventRow>> =
        std::collections::BTreeMap::new();
    for row in rows.iter().filter(|row| row.freight_relevant) {
        by_site.entry(&row.site_id).or_default().push(row);
    }

    by_site
        .into_iter()
        .map(|(site_id, site_rows)| {
            let mut years = site_rows
                .iter()
                .map(|row| row.observation_year)
                .collect::<Vec<_>>();
            years.sort_unstable();
            years.dedup();

            let mut event_ids = site_rows
                .iter()
                .map(|row| row.event_id.as_str())
                .collect::<Vec<_>>();
            event_ids.sort_unstable();
            event_ids.dedup();

            let mut durations = site_rows
                .iter()
                .filter_map(|row| row.duration_hours)
                .filter(|v| v.is_finite() && *v >= 0.0)
                .collect::<Vec<_>>();
            durations.sort_by(|a, b| a.total_cmp(b));

            let observed_years = years.len();
            let event_count = event_ids.len();
            let annual_rate = if observed_years > 0 {
                event_count as f64 / observed_years as f64
            } else {
                0.0
            };
            let confidence = event_summary_confidence(&site_rows);

            T1FailureEventSummary {
                site_id: site_id.to_string(),
                observed_years,
                event_count,
                annual_rate,
                annual_probability: annual_probability_from_rate(annual_rate),
                duration_p50_hours: percentile_nearest(&durations, 0.50),
                duration_p95_hours: percentile_nearest(&durations, 0.95),
                confidence,
            }
        })
        .collect()
}

fn annual_probability_from_rate(rate: f64) -> f64 {
    if rate <= 0.0 {
        0.0
    } else {
        1.0 - (-rate).exp()
    }
}

fn event_summary_confidence(rows: &[&T1FailureEventRow]) -> String {
    if rows.is_empty() {
        return "unknown".to_string();
    }
    if rows
        .iter()
        .all(|row| row.confidence.eq_ignore_ascii_case("high"))
    {
        "high".to_string()
    } else if rows
        .iter()
        .any(|row| row.confidence.eq_ignore_ascii_case("low"))
    {
        "low".to_string()
    } else {
        "medium".to_string()
    }
}

fn percentile_nearest(sorted_values: &[f64], p: f64) -> Option<f64> {
    if sorted_values.is_empty() {
        return None;
    }
    let p = p.clamp(0.0, 1.0);
    let idx = ((sorted_values.len() - 1) as f64 * p).round() as usize;
    sorted_values.get(idx).copied()
}

fn apply_t1_failure_events_to_ledger(
    ledger_rows: &[T1FailureRow],
    event_rows: &[T1FailureEventRow],
    event_artifact: &Path,
) -> Vec<T1FailureRow> {
    let summaries = summarize_t1_failure_events(event_rows)
        .into_iter()
        .map(|summary| (summary.site_id.clone(), summary))
        .collect::<std::collections::BTreeMap<_, _>>();

    ledger_rows
        .iter()
        .cloned()
        .map(|mut row| {
            if let Some(summary) = summaries.get(&row.site_id) {
                row.annual_probability = Some(summary.annual_probability);
                row.duration_p50_hours = summary.duration_p50_hours;
                row.duration_p95_hours = summary.duration_p95_hours;
                row.source_status = "empirical".to_string();
                row.confidence = summary.confidence.clone();
                row.current_artifact = append_artifact(&row.current_artifact, event_artifact);
                row.blocking_gap = "Empirical event observations loaded; reroute time and throughput retention still require route simulation/source validation".to_string();
                row.next_evidence_step = "Join event windows to NPMRDS/FPM travel-time traces and reroute simulations to validate throughput retention under closure".to_string();
            }
            row
        })
        .collect()
}

fn append_artifact(existing: &str, artifact: &Path) -> String {
    let artifact = artifact.to_string_lossy();
    if existing
        .split(';')
        .map(str::trim)
        .any(|value| value == artifact)
    {
        existing.to_string()
    } else if existing.trim().is_empty() {
        artifact.to_string()
    } else {
        format!("{}; {}", existing.trim(), artifact)
    }
}

fn print_t1_failure_event_summary(rows: &[T1FailureEventRow]) {
    let summaries = summarize_t1_failure_events(rows);
    let freight_rows = rows.iter().filter(|row| row.freight_relevant).count();
    let full_closures = rows.iter().filter(|row| row.full_closure).count();
    let lane_rows = rows.iter().filter(|row| row.lanes_closed.is_some()).count();
    let source_id_rows = rows
        .iter()
        .filter(|row| !row.source_event_id.trim().is_empty())
        .count();
    let timed_rows = rows
        .iter()
        .filter(|row| !row.start_time.trim().is_empty() && !row.end_time.trim().is_empty())
        .count();
    let noted_rows = rows
        .iter()
        .filter(|row| !row.notes.trim().is_empty())
        .count();
    let sources = rows
        .iter()
        .map(|row| row.source.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let event_types = rows
        .iter()
        .map(|row| row.event_type.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let confidence_labels = rows
        .iter()
        .map(|row| row.confidence.as_str())
        .filter(|value| !value.trim().is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    println!("route t1-failure-events");
    println!("  events: {} raw rows", rows.len());
    println!("  freight-relevant rows: {freight_rows}");
    println!(
        "  sites with freight-relevant observations: {}",
        summaries.len()
    );
    if rows.is_empty() {
        println!("  no observations loaded yet; populate data/t1-failure-events.csv from source plan records");
        return;
    }
    println!("  full closures: {full_closures}");
    println!("  rows with lane counts: {lane_rows}");
    println!("  rows with source event ids: {source_id_rows}");
    println!("  rows with start/end times: {timed_rows}");
    println!("  rows with notes: {noted_rows}");
    println!("  sources: {}", join_set(&sources));
    println!("  event types: {}", join_set(&event_types));
    println!("  confidence labels: {}", join_set(&confidence_labels));
    println!();
    println!(
        "{:<18} {:>6} {:>7} {:>8} {:>8} {:>8} {:>8}",
        "Site", "Years", "Events", "Rate/Yr", "P_ann", "P50 h", "P95 h"
    );
    println!("{}", "-".repeat(78));
    for summary in summaries {
        println!(
            "{:<18} {:>6} {:>7} {:>8.3} {:>8.3} {:>8} {:>8}",
            summary.site_id,
            summary.observed_years,
            summary.event_count,
            summary.annual_rate,
            summary.annual_probability,
            fmt_opt(summary.duration_p50_hours),
            fmt_opt(summary.duration_p95_hours)
        );
    }
}

fn join_set(values: &std::collections::BTreeSet<&str>) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().copied().collect::<Vec<_>>().join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        atlas_candidate_ids, confidence_risk_dimensions, dimension_confidence_risks,
        dimension_confidence_values, dimension_estimated_values, dimension_score_values,
        gap_type_slug, join_fema_d1_to_corridor, parse_indot_trafficwise_events,
        parse_iowa511_events, parse_mdot_midrive_events, parse_pressure_scenarios,
        parse_standards_proof_ledger, parse_t1_failure_events, parse_t1_failure_ledger,
        parse_t1_failure_source_plan, parse_t1_source_health, parse_tdot_smartway_events,
        parse_throughput_proof_matrix, pressure_scenario_gate_failures,
        pressure_scenario_has_bounded_contract, pressure_scenario_missing_required_adversity,
        rounded_score, scenario_edge_candidates, standards_blueprint_gate_failures,
        standards_evidence_level_is_allowed, summarize_t1_failure_events,
        t1_failure_evidence_gate_failures, t1_failure_row_has_evidence_contract,
        throughput_proof_gate_failures, throughput_proof_has_bounded_contract, tier_for_score,
        write_tier_artifacts_to, FemaTile, GapType, ScoreAllRow, ScoreSignalRow,
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
    fn gap_type_slugs_match_cli_values() {
        assert_eq!(gap_type_slug(&GapType::MissingLink), "missing-link");
        assert_eq!(gap_type_slug(&GapType::Bottleneck), "bottleneck");
        assert_eq!(gap_type_slug(&GapType::Resilience), "resilience");
        assert_eq!(gap_type_slug(&GapType::Intermodal), "intermodal");
    }

    #[test]
    fn bottleneck_signal_prefers_stress_then_topology() {
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 8.0,
                a3: 1.0,
                b2: 10.0,
            }),
            "corridor_stress"
        );
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 1.0,
                a3: 1.0,
                b2: 10.0,
            }),
            "topology_chokepoint"
        );
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 1.0,
                a3: 1.0,
                b2: 2.0,
            }),
            "capacity_needs_flow"
        );
    }

    #[test]
    fn score_all_csv_dimension_values_cover_full_rubric() {
        let scores = score_corridor(
            &CorridorAttributes::default(),
            &ScoringConfig::default_config(),
        );

        assert_eq!(dimension_score_values(&scores).len(), 16);
        assert_eq!(dimension_estimated_values(&scores).len(), 16);
        assert_eq!(dimension_confidence_values(&scores).len(), 16);
    }

    #[test]
    fn confidence_risk_dimensions_prioritizes_scored_low_confidence_dimensions() {
        let mut scores = [0.0; 16];
        let mut confidences = [0.9; 16];
        scores[0] = 9.0;
        scores[1] = 6.0;
        scores[2] = 10.0;
        scores[13] = 5.0;
        confidences[1] = 0.45;
        confidences[2] = 0.55;
        confidences[13] = 0.50;

        assert_eq!(
            confidence_risk_dimensions(&scores, &confidences),
            "A3:10.0@0.55;A2:6.0@0.45;D1:5.0@0.50"
        );
    }

    #[test]
    fn dimension_confidence_risks_clamps_confidence_to_valid_range() {
        let mut scores = [0.0; 16];
        let mut confidences = [1.0; 16];
        scores[0] = 10.0;
        scores[1] = 10.0;
        confidences[0] = -0.5;
        confidences[1] = 1.5;

        let risks = dimension_confidence_risks(&scores, &confidences);

        assert_eq!(risks[0], 10.0);
        assert_eq!(risks[1], 0.0);
    }

    #[test]
    fn tier_artifacts_sort_by_tier_then_descending_score() {
        let rows = vec![
            score_row("I2", 55.0, "T2"),
            score_row("I1", 75.0, "T1"),
            score_row("I3", 82.0, "T1"),
        ];

        let dir = std::env::temp_dir().join(format!("route-tier-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        write_tier_artifacts_to(&rows, &dir).expect("write tier artifacts");

        let csv = std::fs::read_to_string(dir.join("tier-table.csv")).expect("read tier csv");
        let route_order: Vec<&str> = csv
            .lines()
            .skip(1)
            .filter_map(|line| line.split(',').nth(1))
            .take(3)
            .collect();
        assert_eq!(route_order, ["I3", "I1", "I2"]);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn standards_proof_ledger_parses_and_gates_unresolved_rows() {
        let csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-DIAMOND-K,T1,resilience,k >= 3,outcome,mechanism,closure,gate,Heuristic,artifact,manual validation needed,next,B.4
T3-COVERAGE,T3,access,coverage,outcome,mechanism,gap,gate,Implemented,artifact,,next,B.1
";

        let rows = parse_standards_proof_ledger(csv.as_bytes()).expect("parse proof ledger");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].standard_id, "T1-DIAMOND-K");
        assert_eq!(rows[1].evidence_level, "Implemented");

        let failures = standards_blueprint_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "T1-DIAMOND-K");
    }

    #[test]
    fn standards_proof_evidence_levels_use_blueprint_vocabulary() {
        for level in ["Implemented", "Heuristic", "Stub", "Planned", "Deprecated"] {
            assert!(standards_evidence_level_is_allowed(level));
        }
        assert!(standards_evidence_level_is_allowed(" heuristic "));
        assert!(!standards_evidence_level_is_allowed("unknown"));
        assert!(!standards_evidence_level_is_allowed(""));
    }

    #[test]
    fn standards_blueprint_gate_rejects_unknown_evidence_levels() {
        let csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-UNKNOWN,T1,resilience,claim,outcome,mechanism,closure,gate,Unlabeled,artifact,,next,B.4
";

        let rows = parse_standards_proof_ledger(csv.as_bytes()).expect("parse proof ledger");
        let failures = standards_blueprint_gate_failures(&rows);

        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "T1-UNKNOWN");
    }

    #[test]
    fn pressure_scenarios_require_bounded_l2_contracts() {
        let csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-DES-MOINES,des-moines-interchange,T1/T1 closure,T1-DIAMOND-K; T1-FLYOVER,Heuristic,scenario.toml,gap,next
BAD,unnamed,,T1-DIAMOND-K,unknown,,gap,
";

        let rows = parse_pressure_scenarios(csv.as_bytes()).expect("parse pressure scenarios");

        assert_eq!(rows.len(), 2);
        assert!(pressure_scenario_has_bounded_contract(&rows[0]));
        assert!(!pressure_scenario_has_bounded_contract(&rows[1]));
        let failures = pressure_scenario_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].scenario_id, "BAD");
    }

    #[test]
    fn pressure_scenarios_cover_required_adversity_classes() {
        let csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-DES-MOINES,des-moines-interchange,T1/T1 closure,T1-DIAMOND-K,Heuristic,scenario.toml,gap,next
S-L2-DONNER,donner-closure,corridor segment weather closure,T1-SPURS,Heuristic,scenario.toml,gap,next
S-L2-HOUSTON,houston-surge,hurricane/flood disruption and port surge,T1-RECOVERY,Heuristic,scenario.toml,gap,next
S-L2-ATLANTA,atlanta-peak,urban peak and managed-lane stress,T1-OPS-PTI,Heuristic,scenario.toml,gap,next
S-L2-RELAY-HUB,relay-hub-outage,relay hub outage,T1-TRANSIT-HUB,Planned,route sla-matrix,gap,next
S-L2-EV-REST,ev-rest-area-outage,EV/rest-area outage,T1-EV-TRUCK,Planned,route ev-analysis,gap,next
";

        let rows = parse_pressure_scenarios(csv.as_bytes()).expect("parse pressure scenarios");
        assert!(pressure_scenario_missing_required_adversity(&rows).is_empty());

        let missing = pressure_scenario_missing_required_adversity(&rows[..5]);
        assert_eq!(missing, vec!["EV/rest-area outage"]);
    }

    #[test]
    fn throughput_proof_matrix_separates_congestion_and_resilience_contracts() {
        let csv = "\
proof_id,proof_name,binding_type,stressor,primary_metric,existing_artifact,current_status,blocking_gap,next_evidence_step
TP-CONG-I80,I-80 bottleneck,congestion_binding,peak demand,max_flow_vpd,route flow I-80,Heuristic,gap,next
TP-RES-DM,Des Moines closure,resilience_binding,T1 closure,k_connectivity; t90_hours,route diamond,Heuristic,gap,next
BAD,Missing binding,unknown,peak demand,,artifact,unknown,gap,
";

        let rows = parse_throughput_proof_matrix(csv.as_bytes()).expect("parse throughput proof");

        assert_eq!(rows.len(), 3);
        assert!(throughput_proof_has_bounded_contract(&rows[0]));
        assert!(throughput_proof_has_bounded_contract(&rows[1]));
        assert!(!throughput_proof_has_bounded_contract(&rows[2]));
        let failures = throughput_proof_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].proof_id, "BAD");
    }

    #[test]
    fn t1_failure_ledger_parses_optional_empirical_fields() {
        let csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I40-I75,I-40 x I-75,Knoxville TN,closure,,,,,,,,source_needed,unknown,artifact,gap,next
";

        let rows = parse_t1_failure_ledger(csv.as_bytes()).expect("parse T1 failure ledger");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].site_id, "T1X-I35-I80");
        assert_eq!(rows[0].throughput_retention_current, Some(0.962));
        assert_eq!(rows[0].annual_probability, None);
        assert_eq!(rows[1].source_status, "source_needed");
    }

    #[test]
    fn t1_failure_evidence_gate_requires_labeled_source_status_and_next_steps() {
        let csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I40-I75,I-40 x I-75,Knoxville TN,closure,,,,,,,,source_needed,unknown,artifact,gap,next
T1X-BAD,I-5 x I-10,Los Angeles CA,closure,,,,,,,,maybe,unknown,artifact,gap,
";

        let rows = parse_t1_failure_ledger(csv.as_bytes()).expect("parse T1 failure ledger");

        assert!(t1_failure_row_has_evidence_contract(&rows[0]));
        assert!(t1_failure_row_has_evidence_contract(&rows[1]));
        assert!(!t1_failure_row_has_evidence_contract(&rows[2]));
        let failures = t1_failure_evidence_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].site_id, "T1X-BAD");
    }

    #[test]
    fn t1_failure_source_plan_parses_source_targets() {
        let csv = "\
site_id,intersection,location,primary_state_sources,national_sources,fields_to_populate,access_status,source_url,notes
T1X-I35-I80,I-35 x I-80,Des Moines IA,Iowa DOT 511,NPMRDS,annual_probability,identified,https://example.invalid,notes
T1X-I35-I40,I-35 x I-40,Oklahoma City OK,Oklahoma 511,NPMRDS,duration,lookup_needed,,notes
";

        let rows = parse_t1_failure_source_plan(csv.as_bytes()).expect("parse source plan");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].access_status, "identified");
        assert_eq!(rows[1].source_url, "");
    }

    #[test]
    fn t1_source_health_parses_and_flags_blockers() {
        let csv = "\
site_id,source_name,source_url,source_kind,access_health,ingestion_status,history_status,last_checked,blocking_gap,next_step
T1X-I35-I80,Iowa DOT 511,https://example.invalid,live_event_feed,live,implemented,snapshot_only,2026-05-09,gap,next
T1X-I40-I75,TDOT SmartWay,https://example.invalid,live_event_feed,blocked_query,scaffolded,unknown,2026-05-09,gap,next
";

        let rows = parse_t1_source_health(csv.as_bytes()).expect("parse source health");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].access_health, "live");
        assert!(!super::t1_source_health_is_blocked(&rows[0]));
        assert!(super::t1_source_health_is_blocked(&rows[1]));
        assert_eq!(super::t1_source_health_blockers(&rows).len(), 1);
    }

    #[test]
    fn t1_access_docket_groups_blockers_by_action_type() {
        let csv = "\
site_id,source_name,source_url,source_kind,access_health,ingestion_status,history_status,last_checked,blocking_gap,next_step
T1X-I35-I80,Iowa DOT 511,https://example.invalid,live_event_feed,live,implemented,snapshot_only,2026-05-09,gap,next
T1X-I40-I75,TDOT SmartWay,https://example.invalid,live_event_feed,blocked_query,scaffolded,unknown,2026-05-09,gap,next
ALL,FHWA NPMRDS,https://example.invalid,travel_time_reliability,requires_access,not_started,historical_available,2026-05-09,gap,next
";

        let rows = parse_t1_source_health(csv.as_bytes()).expect("parse source health");
        let items = rows
            .iter()
            .filter(|row| super::t1_source_health_is_blocked(row))
            .map(super::t1_access_docket_item)
            .collect::<Vec<_>>();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].category, "endpoint_tuning");
        assert_eq!(items[0].priority, "high");
        assert_eq!(items[1].category, "access_request");
        assert_eq!(items[1].priority, "critical");
    }

    #[test]
    fn t1_failure_events_summarize_rates_and_durations() {
        let csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,e1,Iowa 511,100,2023,2023-01-01T00:00:00Z,2023-01-01T02:00:00Z,2.0,incident,true,2,true,medium,first
T1X-I35-I80,e2,Iowa 511,101,2023,2023-03-01T00:00:00Z,2023-03-01T04:00:00Z,4.0,work_zone,false,1,true,medium,second
T1X-I35-I80,e3,Iowa 511,102,2024,2024-06-01T00:00:00Z,2024-06-01T10:00:00Z,10.0,incident,true,3,true,high,third
T1X-I35-I80,e4,Iowa 511,103,2024,2024-07-01T00:00:00Z,2024-07-01T08:00:00Z,8.0,incident,true,3,false,low,non-freight
";

        let rows = parse_t1_failure_events(csv.as_bytes()).expect("parse event rows");
        let summaries = summarize_t1_failure_events(&rows);

        assert_eq!(rows.len(), 4);
        assert_eq!(summaries.len(), 1);
        let summary = &summaries[0];
        assert_eq!(summary.site_id, "T1X-I35-I80");
        assert_eq!(summary.observed_years, 2);
        assert_eq!(summary.event_count, 3);
        assert_eq!(summary.annual_rate, 1.5);
        assert!((summary.annual_probability - 0.77686984).abs() < 1e-6);
        assert_eq!(summary.duration_p50_hours, Some(4.0));
        assert_eq!(summary.duration_p95_hours, Some(10.0));
        assert_eq!(summary.confidence, "medium");
    }

    #[test]
    fn t1_failure_events_apply_empirical_fields_to_ledger() {
        let ledger_csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I35-I40,I-35 x I-40,Oklahoma City OK,closure,,,,,,,,source_needed,unknown,artifact,gap,next
";
        let events_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,e1,Iowa 511,100,2023,2023-01-01T00:00:00Z,2023-01-01T02:00:00Z,2.0,incident,true,2,true,medium,first
T1X-I35-I80,e2,Iowa 511,101,2024,2024-01-01T00:00:00Z,2024-01-01T06:00:00Z,6.0,incident,true,2,true,medium,second
";

        let ledger_rows = parse_t1_failure_ledger(ledger_csv.as_bytes()).expect("parse ledger");
        let event_rows = parse_t1_failure_events(events_csv.as_bytes()).expect("parse events");
        let updated = super::apply_t1_failure_events_to_ledger(
            &ledger_rows,
            &event_rows,
            std::path::Path::new("data/t1-failure-events.csv"),
        );

        assert_eq!(updated[0].source_status, "empirical");
        assert_eq!(updated[0].duration_p50_hours, Some(6.0));
        assert_eq!(updated[0].duration_p95_hours, Some(6.0));
        assert_eq!(updated[0].throughput_retention_current, Some(0.962));
        assert!(updated[0]
            .current_artifact
            .contains("data/t1-failure-events.csv"));
        assert_eq!(updated[1].source_status, "source_needed");
    }

    #[test]
    fn t1_failure_events_merge_dedupes_repeated_snapshots() {
        let existing_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,IOWA511-1,Iowa DOT 511 ArcGIS,1,2026,2026-05-01 08:00 AM,2026-05-01 10:00 AM,2.0,work_zone,false,,true,medium,first
";
        let incoming_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,IOWA511-1,Iowa DOT 511 ArcGIS,1,2026,2026-05-01 08:00 AM,2026-05-01 10:00 AM,2.0,work_zone,false,,true,medium,duplicate
T1X-I35-I80,IOWA511-2,Iowa DOT 511 ArcGIS,2,2026,2026-05-02 08:00 AM,2026-05-02 11:00 AM,3.0,closure,true,,true,medium,second
";

        let existing = parse_t1_failure_events(existing_csv.as_bytes()).expect("parse existing");
        let incoming = parse_t1_failure_events(incoming_csv.as_bytes()).expect("parse incoming");
        let merged = super::merge_t1_failure_events(&existing, &incoming);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].event_id, "IOWA511-1");
        assert_eq!(merged[0].notes, "first");
        assert_eq!(merged[1].event_id, "IOWA511-2");
    }

    #[test]
    fn iowa511_import_filters_radius_and_normalizes_events() {
        let json = r#"{
  "features": [
    {
      "attributes": {
        "ID": "IADOT-1",
        "Route": "I-80 WB",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "I-80 WB: Crash, left lane closed",
        "cause": "due to crash.",
        "Restrict_": "Lane closed",
        "Desc0": "near Des Moines"
      },
      "geometry": { "x": -93.80, "y": 41.66 }
    },
    {
      "attributes": {
        "ID": "IADOT-2",
        "Route": "US 218",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "US 218: Road Construction",
        "cause": "due to road construction.",
        "Restrict_": "",
        "Desc0": "not a T1 route"
      },
      "geometry": { "x": -93.80, "y": 41.66 }
    },
    {
      "attributes": {
        "ID": "IADOT-3",
        "Route": "I-80 WB",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "I-80 WB: Entrance Ramp Closed",
        "cause": "due to road construction.",
        "Restrict_": "",
        "Desc0": "Council Bluffs"
      },
      "geometry": { "x": -95.85, "y": 41.26 }
    }
  ]
}"#;

        let rows = parse_iowa511_events(json, "T1X-I35-I80", 41.658, -93.800, 30.0)
            .expect("parse Iowa 511 fixture");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I35-I80");
        assert_eq!(rows[0].source_event_id, "IADOT-1");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].duration_hours, Some(2.5));
        assert_eq!(rows[0].event_type, "crash");
        assert!(!rows[0].full_closure);
    }

    #[test]
    fn tdot_smartway_import_filters_radius_and_normalizes_events() {
        let json = r#"{
  "features": [
    {
      "attributes": {
        "ID": "TDOT-1",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "I-40 / I-75",
        "CD_DIRECTION": "Eastbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "I-40/I-75 lane closure in Knox County",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.90,
        "MIDPOINT_LONGITUDE_DD": -84.16,
        "COUNTY_NAME": "Knox"
      }
    },
    {
      "attributes": {
        "ID": "TDOT-2",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "SR-1",
        "CD_DIRECTION": "Eastbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "not a T1 route",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.90,
        "MIDPOINT_LONGITUDE_DD": -84.16,
        "COUNTY_NAME": "Knox"
      }
    },
    {
      "attributes": {
        "ID": "TDOT-3",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "I-75",
        "CD_DIRECTION": "Southbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "near Chattanooga",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.05,
        "MIDPOINT_LONGITUDE_DD": -85.20,
        "COUNTY_NAME": "Hamilton"
      }
    }
  ]
}"#;

        let rows =
            parse_tdot_smartway_events(json, "T1X-I40-I75", 35.90, -84.16, 35.0).expect("parse");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I40-I75");
        assert_eq!(rows[0].source_event_id, "TDOT-1");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].duration_hours, Some(4.0));
        assert_eq!(rows[0].event_type, "work_zone");
        assert!(rows[0].full_closure);
    }

    #[test]
    fn mdot_midrive_import_filters_radius_and_normalizes_events() {
        let json = r#"[
  {
    "latitude": 42.31,
    "longitude": -83.08,
    "id": 1092974,
    "title": "Crash on NB  I-75",
    "message": "<div><strong>Location: </strong>NB I-75 at I-94</div><div><strong>Lanes Blocked: </strong>Left Lane</div><div><strong>Event Type: </strong> Crash</div><div><strong>County: </strong>Wayne</div><div><strong>Reported:</strong> 5:14 PM</div>"
  },
  {
    "latitude": 42.31,
    "longitude": -83.08,
    "id": 1092975,
    "title": "Crash on US-23",
    "message": "<div><strong>Event Type: </strong> Crash</div>"
  },
  {
    "latitude": 43.60,
    "longitude": -84.20,
    "id": 1092976,
    "title": "Crash on SB I-75",
    "message": "<div><strong>Event Type: </strong> Crash</div>"
  }
]"#;

        let rows = parse_mdot_midrive_events(json, "T1X-I75-I90", 42.31, -83.07, 60.0, 2026)
            .expect("parse MDOT Mi Drive fixture");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I75-I90");
        assert_eq!(rows[0].source_event_id, "1092974");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].start_time, "5:14 PM");
        assert_eq!(rows[0].event_type, "crash");
        assert_eq!(rows[0].lanes_closed, Some(1));
        assert_eq!(rows[0].confidence, "low");
    }

    #[test]
    fn indot_trafficwise_import_filters_events_and_normalizes_rows() {
        let json = r#"{
  "data": {
    "mapFeaturesQuery": {
      "mapFeatures": [
        {
          "title": "I-80 westbound: Entrance ramp closed.",
          "tooltip": "I-80 westbound: Entrance ramp closed, because of roadwork.",
          "uri": "event/CARSx-333174",
          "__typename": "Event"
        },
        {
          "title": "US 30 in both directions: Paving operations.",
          "tooltip": "US 30 in both directions: Paving operations, left lane closed.",
          "uri": "event/incars-178325",
          "__typename": "Event"
        },
        {
          "title": "Show six events",
          "tooltip": "",
          "uri": "cluster/-87371644160212",
          "__typename": "Cluster"
        }
      ],
      "error": null
    }
  }
}"#;

        let rows = parse_indot_trafficwise_events(json, "T1X-I80-I90", 2026).expect("parse INDOT");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I80-I90");
        assert_eq!(rows[0].source_event_id, "CARSx-333174");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].event_type, "work_zone");
        assert!(rows[0].full_closure);
        assert_eq!(rows[0].confidence, "low");
    }

    #[test]
    fn scenario_edge_candidates_return_nearest_stable_edge_ids() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: -96.0, y: 41.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: -95.9, y: 41.0 },
            is_interchange: false,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: -90.0, y: 41.0 },
            is_interchange: false,
        });
        let d = graph.graph.add_node(HighwayNode {
            id: 4,
            coord: coord! { x: -89.9, y: 41.0 },
            is_interchange: false,
        });
        let near = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 101,
                route_id: "I80".to_string(),
                state: "NE".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: -96.0, y: 41.0 },
                    coord! { x: -95.9, y: 41.0 },
                ]),
                length_miles: 6.0,
                lane_count: Some(4),
                aadt: Some(50_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        let far = graph.graph.add_edge(
            c,
            d,
            HighwayEdge {
                id: 202,
                route_id: "I80".to_string(),
                state: "IA".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: -90.0, y: 41.0 },
                    coord! { x: -89.9, y: 41.0 },
                ]),
                length_miles: 6.0,
                lane_count: Some(4),
                aadt: Some(40_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.route_index.insert("I80".to_string(), vec![far, near]);

        let candidates = scenario_edge_candidates(&graph, "I80", 41.0, -95.95, 20.0, 5);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].edge_id, 101);
        assert_eq!(candidates[0].state, "NE");
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

        let freight_b = attrs.annual_freight_value_b.expect("A2 proxy should fill");
        assert!((freight_b - 11.68).abs() < 0.001);
        assert!(attrs.freight_value_is_hpms_proxy);
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

    #[test]
    fn fema_d1_join_uses_route_edge_boxes_not_whole_corridor_box() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: false,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: 10.0, y: 0.0 },
            is_interchange: false,
        });
        let d = graph.graph.add_node(HighwayNode {
            id: 4,
            coord: coord! { x: 11.0, y: 0.0 },
            is_interchange: false,
        });
        let edge_a = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 1,
                route_id: "I1".to_string(),
                state: "TS".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
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
        let edge_b = graph.graph.add_edge(
            c,
            d,
            HighwayEdge {
                id: 2,
                route_id: "I1".to_string(),
                state: "TS".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 10.0, y: 0.0 },
                    coord! { x: 11.0, y: 0.0 },
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
        graph
            .route_index
            .insert("I1".to_string(), vec![edge_a, edge_b]);

        let tiles = vec![
            FemaTile {
                xmin: 5.0,
                ymin: -0.5,
                xmax: 6.0,
                ymax: 0.5,
                sfha_count: 100,
            },
            FemaTile {
                xmin: 0.25,
                ymin: -0.5,
                xmax: 0.75,
                ymax: 0.5,
                sfha_count: 7,
            },
        ];
        let mut attrs = CorridorAttributes::default();

        join_fema_d1_to_corridor(&graph, "I1", &mut attrs, &tiles);

        assert_eq!(attrs.fema_sfha_miles, Some(2.1));
        assert_eq!(attrs.max_consecutive_sfha_miles, Some(1.47));
    }

    #[test]
    fn hazard_zone_loader_skips_comment_preamble_and_merges_segments() {
        let zones = super::load_hazard_zones();
        let i5 = zones
            .get("I5")
            .expect("I-5 hazard rows should load from commented CSV");
        let i80 = zones
            .get("I80")
            .expect("I-80 hazard row should load from commented CSV");

        assert_eq!(zones.len(), 12);
        assert_eq!(i5.wildfire, 8.5);
        assert_eq!(i5.seismic, 8.5);
        assert_eq!(i80.wildfire, 3.5);
        assert_eq!(i80.tornado, 0.5);
    }

    fn score_row(route: &str, score: f64, tier: &'static str) -> ScoreAllRow {
        ScoreAllRow {
            route: route.to_string(),
            score,
            tier,
            rubric_version: "test".to_string(),
            estimated: false,
            confidence: 0.9,
            score_confidence: 0.8,
            dimensions: [0.0; 16],
            dimension_confidences: [0.0; 16],
        }
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

    let fpm = load_cached_fpm(manifest);
    let (graph, _) = route_network::build_graph_with_fpm(segments, &hpms, &fpm);
    Ok(graph)
}

fn load_cached_fpm(manifest: &route_data::Manifest) -> Vec<route_data::HpmsFpmRecord> {
    [
        "hpms_fpm.csv",
        "fpm_2023.csv",
        "freight_performance_measures.csv",
    ]
    .iter()
    .map(|name| manifest.cache_dir.join(name))
    .find(|path| path.exists())
    .and_then(|path| route_data::hpms::read_hpms_fpm_csv(&path).ok())
    .unwrap_or_default()
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
/// 1. Collect edge geometry bounding boxes for the corridor.
/// 2. Sum each SFHA tile whose bbox overlaps at least one route edge bbox.
/// 3. Estimate fema_sfha_miles = sum × 0.3 (avg SFHA polygon ~0.3 mi span).
/// 4. Set max_consecutive_sfha_miles as a 70% proxy (coastal/valley assumption).
fn join_fema_d1_to_corridor(
    graph: &route_network::HighwayGraph,
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    tiles: &[FemaTile],
) {
    if tiles.is_empty() {
        return;
    }

    let edge_boxes: Vec<(f64, f64, f64, f64)> = graph
        .route_edges(route_id)
        .iter()
        .filter_map(|&ei| {
            let edge = &graph.graph[ei];
            let mut coords = edge.geometry.points().map(|p| (p.x(), p.y()));
            let first = coords.next()?;
            let (mut xmin, mut ymin, mut xmax, mut ymax) = (first.0, first.1, first.0, first.1);
            for (x, y) in coords {
                xmin = xmin.min(x);
                xmax = xmax.max(x);
                ymin = ymin.min(y);
                ymax = ymax.max(y);
            }
            Some((xmin, ymin, xmax, ymax))
        })
        .collect();
    if edge_boxes.is_empty() {
        return;
    }

    let total_sfha: u64 = tiles
        .iter()
        .filter(|t| {
            edge_boxes.iter().any(|&(xmin, ymin, xmax, ymax)| {
                !(xmax < t.xmin || xmin > t.xmax || ymax < t.ymin || ymin > t.ymax)
            })
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
    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/hazard_zones.csv");
    let path = if path.exists() {
        path.to_path_buf()
    } else {
        manifest_path
    };
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .flexible(true)
        .has_headers(false)
        .from_path(path)
    else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 4 {
            continue;
        }
        let route_raw = result[0].trim();
        if route_raw.eq_ignore_ascii_case("route_id") {
            continue;
        }
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

/// Estimate A2 freight value from representative HPMS daily truck crossings.
/// Uses p90 AADT when available, then mean AADT as the secondary A2 path.
fn join_a2_freight_proxy(attrs: &mut route_network::CorridorAttributes, _corridor_miles: f64) {
    if attrs.annual_freight_value_b.is_some() {
        return;
    }
    let Some(aadt) = attrs.p90_aadt.or(attrs.mean_aadt) else {
        return;
    };
    let truck_pct = attrs.mean_pct_truck.unwrap_or(0.084) as f64;
    let truck_aadt = aadt * truck_pct;
    let freight_b = truck_aadt * 365.0 * 16.0 * 1_000.0 / 1_000_000_000.0;
    attrs.annual_freight_value_b = Some(freight_b);
    attrs.freight_value_is_hpms_proxy = true;
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
        "│  {} — Dimension Scores (rubric {}, conf {:.2}, score-conf {:.2})",
        designation,
        scores.rubric_version,
        scores.mean_confidence(),
        scores.score_weighted_confidence()
    );
    println!("├──────┬──────────────────────────────┬───────┬─────┬────────┬──────┤");
    println!("│ Dim  │ Name                         │ Score │ Est │ Quality│ Conf │");
    println!("├──────┼──────────────────────────────┼───────┼─────┼────────┼──────┤");

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
            "│ {:4} │ {:<28} │ {:>5.1} │  {}  │ {:<6} │ {:>4.2} │",
            sd.dim.code(),
            sd.dim.name(),
            sd.score,
            est,
            sd.quality_label(),
            sd.confidence
        );
    }

    println!("├──────┴──────────────────────────────┼───────┼─────┴────────┴──────┤");
    println!(
        "│ Band A (Flow)                        │ {:>5.1} │                    │",
        scores.band_a()
    );
    println!(
        "│ Band B (Network)                     │ {:>5.1} │                    │",
        scores.band_b()
    );
    println!(
        "│ Band C (People)                      │ {:>5.1} │                    │",
        scores.band_c()
    );
    println!(
        "│ Band D (Future)                      │ {:>5.1} │                    │",
        scores.band_d()
    );
    println!(
        "│ TOTAL                                │ {:>5.1} │ /160               │",
        scores.total()
    );
    println!("└──────────────────────────────────────┴───────┴─────────────────────┘");
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
