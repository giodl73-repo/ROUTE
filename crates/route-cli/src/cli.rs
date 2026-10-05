//! Clap surface for the route CLI.
//!
//! Kept in its own module so command definitions can evolve without growing
//! the already-large `run_cli` implementation file.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "route",
    about = "ROUTE — Interstate 2.0 analysis pipeline",
    version
)]
pub struct Cli {
    /// Path to scoring config (default: config/scoring.toml in repo root)
    #[arg(long, global = true, value_name = "FILE")]
    pub scoring_config: Option<PathBuf>,

    /// Path to data manifest (default: ~/.route/manifest.json)
    #[arg(long, global = true, value_name = "FILE")]
    pub manifest: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// HPMS functional systems to fetch (default: 1; use 1,2,3 for broader principal-arterial scope)
        #[arg(long, default_value = "1", value_name = "SYSTEMS")]
        functional_systems: String,
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

    /// Show tracked map atlas artifacts and verify PNG contracts
    MapAtlas {
        /// Path to map atlas manifest CSV
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        ledger: PathBuf,
        /// Show full render command and game use
        #[arg(long)]
        details: bool,
        /// Fail if tracked map artifacts are missing, tiny, or wrong dimensions
        #[arg(long)]
        gate: bool,
    },

    /// Certify current structural map publication readiness and held non-map claims
    MapPublicationReadiness {
        /// Path to map atlas manifest CSV
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Path to optimizer residual blocker backlog CSV
        #[arg(
            long,
            default_value = "data/optimizer-residual-blocker-backlog.csv",
            value_name = "FILE"
        )]
        backlog: PathBuf,
        /// Path to map publication scope decision CSV
        #[arg(
            long,
            default_value = "data/map-publication-scope-decision.csv",
            value_name = "FILE"
        )]
        scope_decision: PathBuf,
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/map-publication-readiness.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Show held claim families
        #[arg(long)]
        details: bool,
        /// Fail if maps are not structurally publication-ready
        #[arg(long)]
        gate: bool,
    },

    /// Verify the release-facing structural map publication inventory
    MapPublicationInventory {
        /// Path to publication inventory CSV
        #[arg(
            long,
            default_value = "data/map-publication-inventory.csv",
            value_name = "FILE"
        )]
        inventory: PathBuf,
        /// Path to map atlas manifest CSV
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Path to map publication readiness CSV
        #[arg(
            long,
            default_value = "data/map-publication-readiness.csv",
            value_name = "FILE"
        )]
        readiness: PathBuf,
        /// Show inventory rows
        #[arg(long)]
        details: bool,
        /// Fail if inventory drifts from atlas/readiness contracts
        #[arg(long)]
        gate: bool,
    },

    /// Export T2-only Beck map diagnostics for clutter and service-line review
    BeckT2Diagnostics {
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/beck-t2-diagnostics.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any T2 line is flagged for visual review
        #[arg(long)]
        gate: bool,
    },

    /// Export T1 Beck map diagnostics for backbone qualification and overlap review
    BeckT1Diagnostics {
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/beck-t1-diagnostics.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any T1 line has endpoint or overlap review flags
        #[arg(long)]
        gate: bool,
    },

    /// Rank T1 SLA promise-pair candidates and explain the selected cut line
    T1SlaCandidatePairs {
        /// Candidate SLA pair universe CSV
        #[arg(
            long,
            default_value = "data/t1-sla-candidate-universe.csv",
            value_name = "FILE"
        )]
        candidates: PathBuf,
        /// Selected T1 SLA pair portfolio CSV
        #[arg(long, default_value = "data/t1-sla-pairs.csv", value_name = "FILE")]
        selected_pairs: PathBuf,
        /// Output ranked SLA candidate pair CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-sla-candidate-pairs.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Selected T1 promise-pair budget
        #[arg(long, default_value_t = 25)]
        selected_budget: usize,
        /// Fail if selected/dropped pair decisions lack cut-line lineage
        #[arg(long)]
        gate: bool,
    },

    /// Select T1 lines/stops from scored routes and stop candidates under national budgets
    T1LineSelector {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Path to tier stop candidate CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        stop_candidates: PathBuf,
        /// Path to designated top-city SLA pair CSV
        #[arg(long, default_value = "data/t1-sla-pairs.csv", value_name = "FILE")]
        sla_pairs: PathBuf,
        /// Path to T1 score-backbone exception CSV
        #[arg(
            long,
            default_value = "data/t1-score-exceptions.csv",
            value_name = "FILE"
        )]
        score_exceptions: PathBuf,
        /// Optimizer constraint budget CSV with generalized blocker/debt rollups
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        constraint_budget: PathBuf,
        /// Output selector CSV file
        #[arg(
            long,
            short,
            default_value = "data/t1-line-selector.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Maximum T1 lines to select
        #[arg(long, default_value_t = 11)]
        route_budget: usize,
        /// Maximum nationally prominent stop/city candidates to promote
        #[arg(long, default_value_t = 25)]
        city_budget: usize,
        /// Maximum selected stop references across T1 lines
        #[arg(long, default_value_t = 100)]
        stop_budget: usize,
        /// Fail if the selected line/stop budget is exceeded or any SLA-required T1 route is unselected
        #[arg(long)]
        gate: bool,
    },

    /// Select ordered T1 stop chains and METIS stop workload regions
    T1StopSelector {
        /// Path to T1 line selector CSV
        #[arg(long, default_value = "data/t1-line-selector.csv", value_name = "FILE")]
        selector: PathBuf,
        /// Path to tier stop candidate CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        stop_candidates: PathBuf,
        /// Output T1 stop selector CSV file
        #[arg(
            long,
            short,
            default_value = "data/t1-stop-selector.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Target METIS stop regions per route
        #[arg(long, default_value_t = 4)]
        target_regions: usize,
        /// Fail if selected T1 routes do not produce valid stop-region chains
        #[arg(long)]
        gate: bool,
    },

    /// Export a T1 design review joining SLA selection to Beck-map diagnostics
    T1DesignReview {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Path to tier stop candidate CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        stop_candidates: PathBuf,
        /// Path to designated top-city SLA pair CSV
        #[arg(long, default_value = "data/t1-sla-pairs.csv", value_name = "FILE")]
        sla_pairs: PathBuf,
        /// Path to T1 score-backbone exception CSV
        #[arg(
            long,
            default_value = "data/t1-score-exceptions.csv",
            value_name = "FILE"
        )]
        score_exceptions: PathBuf,
        /// Optimizer constraint budget CSV with generalized blocker/debt rollups
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        constraint_budget: PathBuf,
        /// Output design review CSV file
        #[arg(
            long,
            short,
            default_value = "data/t1-design-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Maximum T1 lines to select
        #[arg(long, default_value_t = 11)]
        route_budget: usize,
        /// Maximum nationally prominent stop/city candidates to promote
        #[arg(long, default_value_t = 25)]
        city_budget: usize,
        /// Maximum selected stop references across T1 lines
        #[arg(long, default_value_t = 100)]
        stop_budget: usize,
        /// Fail if selected T1 lines lack stops or SLA-required lines are rejected
        #[arg(long)]
        gate: bool,
    },

    /// Emit T1 topology repair witnesses from design-review policy rows
    T1TopologyRepairs {
        /// Path to T1 design review CSV
        #[arg(long, default_value = "data/t1-design-review.csv", value_name = "FILE")]
        design_review: PathBuf,
        /// Output topology repair CSV file
        #[arg(
            long,
            short,
            default_value = "data/t1-topology-repairs.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any topology repair row lacks a concrete next action
        #[arg(long)]
        gate: bool,
    },

    /// Gate that Beck T1 diagnostics cover optimizer-selected T1 route/stop chains
    T1BeckAlignment {
        /// Path to T1 stop selector CSV
        #[arg(long, default_value = "data/t1-stop-selector.csv", value_name = "FILE")]
        stop_selector: PathBuf,
        /// Output Beck alignment CSV file
        #[arg(
            long,
            short,
            default_value = "data/t1-beck-alignment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selected optimizer routes are missing or under-covered by Beck diagnostics
        #[arg(long)]
        gate: bool,
    },

    /// Show T1 design policy actions and gate review-action coverage
    T1DesignPolicy {
        /// Path to T1 design review CSV
        #[arg(long, default_value = "data/t1-design-review.csv", value_name = "FILE")]
        review: PathBuf,
        /// Path to T1 design policy action CSV
        #[arg(
            long,
            default_value = "data/t1-design-policy-actions.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Print full policy details
        #[arg(long)]
        details: bool,
        /// Fail if design-review actions are not covered by policy rows
        #[arg(long)]
        gate: bool,
    },

    /// Show T1 score-backbone exceptions and gate score-only selected T1 routes
    T1ScoreExceptions {
        /// Path to T1 design review CSV
        #[arg(long, default_value = "data/t1-design-review.csv", value_name = "FILE")]
        review: PathBuf,
        /// Path to T1 score-backbone exception CSV
        #[arg(
            long,
            default_value = "data/t1-score-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Print full exception rationale
        #[arg(long)]
        details: bool,
        /// Fail if score-only selected T1 routes lack exception decisions
        #[arg(long)]
        gate: bool,
    },

    /// Export T2 Beck service-class standards used by diagnostics and maps
    BeckT2ServiceStandards {
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/beck-t2-service-standards.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if diagnostics reference a service class without a standard row
        #[arg(long)]
        gate: bool,
    },

    /// Export T2 Beck qualification-action rules used by duplicate-service review
    BeckT2QualificationActions {
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/beck-t2-qualification-actions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if diagnostics reference a service action without a rule row
        #[arg(long)]
        gate: bool,
    },

    /// Regenerate corpus entry markdown from current graph attributes and scores
    Report {
        /// Interstate designation
        designation: String,
        /// Write separately; complete reviewed sources are still required unless --allow-partial
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Explicitly allow a degraded report when source caches are incomplete
        #[arg(long)]
        allow_partial: bool,
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

    /// Interstate Tycoon paper/CLI game prototype commands
    Game {
        #[command(subcommand)]
        command: GameCommand,
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

    /// Fetch ACS county population from Census API
    FetchAcs,

    /// Fetch ACS county median household income from Census API (B19013)
    FetchAcsIncome,

    /// Fetch FEMA NFHL D1 data using small per-state bboxes (avoids 504 timeout)
    FetchFemaD1,

    /// Fetch FEMA NFHL SFHA feature counts for T1 corridor bounding boxes (D1 dimension)
    FetchFema {
        /// Output file (default: data/cache/fema_sfha_counts.csv)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Emit and gate source-fetch cache preservation policy coverage
    SourceFetchPolicy {
        /// Output policy ledger CSV
        #[arg(
            long,
            default_value = "data/source-fetch-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any fetch family lacks a cache-preservation contract
        #[arg(long)]
        gate: bool,
    },

    /// Validate ROUTE source families against the FLETCH registry handoff
    FletchSources {
        /// ROUTE-owned FLETCH source registry JSON
        #[arg(long, default_value = "data/fletch-registry.json", value_name = "FILE")]
        registry: PathBuf,
        /// ROUTE source-fetch preservation policy CSV
        #[arg(
            long,
            default_value = "data/source-fetch-policy.csv",
            value_name = "FILE"
        )]
        source_policy: PathBuf,
        /// Output FLETCH handoff/readiness CSV
        #[arg(
            long,
            short,
            default_value = "data/fletch-source-handoff.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print each cacheline handoff row
        #[arg(long)]
        details: bool,
        /// Fail if any source-fetch policy family is not covered by FLETCH
        #[arg(long)]
        gate: bool,
    },

    /// Compare the ROUTE-owned FLETCH registry with the local cache manifest
    FletchCacheIndex {
        /// ROUTE-owned FLETCH source registry JSON
        #[arg(long, default_value = "data/fletch-registry.json", value_name = "FILE")]
        registry: PathBuf,
        /// ROUTE-owned FLETCH cache manifest JSON
        #[arg(long, value_name = "FILE")]
        cache_manifest: Option<PathBuf>,
        /// Output FLETCH cache-index CSV
        #[arg(
            long,
            short,
            default_value = "data/fletch-cache-index.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print each cache-index row
        #[arg(long)]
        details: bool,
        /// Fail if cache entries are unexpected or unverified
        #[arg(long)]
        gate: bool,
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
        /// Fail if any standard lacks a complete Milepost 4 pressure proof record
        #[arg(long)]
        gate_pressure: bool,
    },

    /// Show Milepost 5 Forum review docket and gate review contracts
    Forum {
        /// Path to Forum review docket CSV
        #[arg(long, default_value = "data/forum-docket.csv", value_name = "FILE")]
        docket: PathBuf,
        /// Show only rows that are held, planned, or missing completion
        #[arg(long)]
        blockers: bool,
        /// Print full review target and next-action details
        #[arg(long)]
        details: bool,
        /// Fail if the Forum docket lacks complete review contracts
        #[arg(long)]
        gate: bool,
    },

    /// Show major project moments and gate the flair ledger contract
    SignificantMoments {
        /// Path to significant-moments CSV
        #[arg(
            long,
            default_value = "data/significant-moments.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows with missing contracts or unresolved next threads
        #[arg(long)]
        blockers: bool,
        /// Print artifact and next-thread details
        #[arg(long)]
        details: bool,
        /// Fail if the moments ledger has incomplete rows or missing artifacts
        #[arg(long)]
        gate: bool,
    },

    /// Show release manifest ownership and gate release metadata
    ReleaseManifest {
        /// Path to release manifest CSV
        #[arg(long, default_value = "data/release-manifest.csv", value_name = "FILE")]
        manifest: PathBuf,
        /// Show only rows with release metadata blockers
        #[arg(long)]
        blockers: bool,
        /// Print verification commands and notes
        #[arg(long)]
        details: bool,
        /// Fail if release rows have invalid metadata or missing artifacts
        #[arg(long)]
        gate: bool,
    },

    /// Show Milepost 6 Blueprint feature packages and gate Forum intake rules
    Blueprint {
        /// Path to Blueprint feature-package ledger CSV
        #[arg(
            long,
            default_value = "data/blueprint-feature-packages.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only packages that still have blocking gaps
        #[arg(long)]
        blockers: bool,
        /// Print full package evidence, delivery, and Forum constraints
        #[arg(long)]
        details: bool,
        /// Fail if feature packages violate Forum intake rules
        #[arg(long)]
        gate: bool,
    },

    /// Show Milepost 6 Blueprint evidence downgrade map and gate proof links
    BlueprintEvidence {
        /// Path to Blueprint feature-package ledger CSV
        #[arg(
            long,
            default_value = "data/blueprint-feature-packages.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to Blueprint evidence map CSV
        #[arg(
            long,
            default_value = "data/blueprint-evidence-map.csv",
            value_name = "FILE"
        )]
        evidence_map: PathBuf,
        /// Path to standards proof ledger CSV
        #[arg(
            long,
            default_value = "data/standards-proof-ledger.csv",
            value_name = "FILE"
        )]
        standards_ledger: PathBuf,
        /// Show only evidence rows that remain held or downgraded
        #[arg(long)]
        blockers: bool,
        /// Print full evidence, downgrade, and next-step details
        #[arg(long)]
        details: bool,
        /// Fail if evidence rows are detached from package/proof ledgers or promote held claims
        #[arg(long)]
        gate: bool,
    },

    /// Show Milepost 6 Blueprint cost and lifecycle range ledger
    BlueprintCosts {
        /// Path to Blueprint feature-package ledger CSV
        #[arg(
            long,
            default_value = "data/blueprint-feature-packages.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to Blueprint cost range ledger CSV
        #[arg(
            long,
            default_value = "data/blueprint-cost-ranges.csv",
            value_name = "FILE"
        )]
        costs: PathBuf,
        /// Show only rows that still need source-backed cost evidence
        #[arg(long)]
        blockers: bool,
        /// Print full cost basis, lifecycle, and next-step details
        #[arg(long)]
        details: bool,
        /// Fail if cost rows are detached from packages or lack claim labels
        #[arg(long)]
        gate: bool,
    },

    /// Show L1 inventory/source plan for standards blocked on asset or operations data
    StandardsInventory {
        /// Path to standards L1 inventory ledger CSV
        #[arg(
            long,
            default_value = "data/standards-l1-inventory.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to standards proof ledger CSV used by --gate-planned
        #[arg(
            long,
            default_value = "data/standards-proof-ledger.csv",
            value_name = "FILE"
        )]
        standards_ledger: PathBuf,
        /// Show only inventory rows that still have blocking gaps
        #[arg(long)]
        blockers: bool,
        /// Print full source, scope, and next-step details
        #[arg(long)]
        details: bool,
        /// Fail if inventory rows lack status, artifact, scope, gap, or next-step contracts
        #[arg(long)]
        gate: bool,
        /// Fail if any Planned standard lacks an L1 inventory row
        #[arg(long)]
        gate_planned: bool,
    },

    /// Show pavement and ride-quality standards for each service tier
    StandardsPavement {
        /// Path to tier pavement standards CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-standards.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows that still need source or implementation work
        #[arg(long)]
        blockers: bool,
        /// Print full freight/transit/repair details
        #[arg(long)]
        details: bool,
        /// Fail if pavement rows lack thresholds, source contract, or repair policy
        #[arg(long)]
        gate: bool,
    },

    /// Join tier segment candidates to pavement evidence and emit repair/source blockers
    TierPavementDocket {
        /// Path to generated tier segment candidate CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segments: PathBuf,
        /// Path to tier pavement standards CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-standards.csv",
            value_name = "FILE"
        )]
        standards: PathBuf,
        /// Output pavement docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print detailed blocker/source fields
        #[arg(long)]
        details: bool,
        /// Fail if docket rows lack complete pavement contracts
        #[arg(long)]
        gate: bool,
    },

    /// Aggregate pavement repair/source blockers by service bundle
    TierPavementSourceGaps {
        /// Path to tier pavement docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-docket.csv",
            value_name = "FILE"
        )]
        docket: PathBuf,
        /// Output pavement source-gap CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-gaps.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print per-bundle detail rows
        #[arg(long)]
        details: bool,
        /// Fail if source-gap rows lack action contracts
        #[arg(long)]
        gate: bool,
    },

    /// Price bundle-level pavement debt as optimizer budget penalties
    TierPavementDebtBudget {
        /// Path to pavement source-gap CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-gaps.csv",
            value_name = "FILE"
        )]
        source_gaps: PathBuf,
        /// Path to accepted route/state pavement exclusion CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-route-state-exclusions.csv",
            value_name = "FILE"
        )]
        route_state_exclusions: PathBuf,
        /// Path to accepted pavement repair funding CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-funding-acceptance.csv",
            value_name = "FILE"
        )]
        repair_funding_acceptance: PathBuf,
        /// Output pavement debt budget CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-debt-budget.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print per-bundle debt budget rows
        #[arg(long)]
        details: bool,
        /// Fail if debt rows lack cost and optimizer penalty contracts
        #[arg(long)]
        gate: bool,
    },

    /// Convert pavement source gaps into state-level acquisition tasks
    TierPavementAcquisitionPlan {
        /// Path to pavement source-gap CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-gaps.csv",
            value_name = "FILE"
        )]
        source_gaps: PathBuf,
        /// Output pavement acquisition plan CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-acquisition-plan.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print per-state acquisition rows
        #[arg(long)]
        details: bool,
        /// Fail if acquisition rows lack source/action contracts
        #[arg(long)]
        gate: bool,
    },

    /// Emit runnable pavement source acquisition tasks from the state plan
    TierPavementAcquisitionDocket {
        /// Path to pavement acquisition plan CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-acquisition-plan.csv",
            value_name = "FILE"
        )]
        acquisition_plan: PathBuf,
        /// Output pavement acquisition docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-acquisition-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Show only one priority band, e.g. A, B, or C
        #[arg(long)]
        priority: Option<String>,
        /// Print runnable commands in execution order
        #[arg(long)]
        script: bool,
        /// Fail if acquisition tasks lack command/verification contracts
        #[arg(long)]
        gate: bool,
    },

    /// Classify pavement acquisition tasks before scoped HPMS/state fetches
    TierPavementSourceAccess {
        /// Path to pavement acquisition docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-acquisition-docket.csv",
            value_name = "FILE"
        )]
        acquisition_docket: PathBuf,
        /// Output pavement source-access policy CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Source priority band to classify
        #[arg(long, default_value = "A")]
        priority: String,
        /// Fail if source-access policy rows lack scoped mutation contracts
        #[arg(long)]
        gate: bool,
    },

    /// Summarize scoped pavement source-fetch cache results without accepting evidence
    TierPavementSourceFetchAttempt {
        /// Path to pavement source-access policy CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-access.csv",
            value_name = "FILE"
        )]
        source_access: PathBuf,
        /// Output pavement source-fetch attempt summary CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-fetch-attempt.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if fetch-attempt rows do not preserve blockers
        #[arg(long)]
        gate: bool,
    },

    /// Review pavement source-fetch outcomes against current source gaps without relief
    TierPavementSourceFetchReview {
        /// Path to pavement source-fetch attempt summary CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-fetch-attempt.csv",
            value_name = "FILE"
        )]
        fetch_attempt: PathBuf,
        /// Path to pavement acquisition docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-acquisition-docket.csv",
            value_name = "FILE"
        )]
        acquisition_docket: PathBuf,
        /// Path to current pavement source-gap CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-gaps.csv",
            value_name = "FILE"
        )]
        source_gaps: PathBuf,
        /// Output pavement source-fetch review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-fetch-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Review unmatched priority-A pavement joins after populated HPMS fetches
    TierPavementUnmatchedJoinReview {
        /// Path to pavement source-fetch review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-fetch-review.csv",
            value_name = "FILE"
        )]
        fetch_review: PathBuf,
        /// Path to pavement source-gap CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-source-gaps.csv",
            value_name = "FILE"
        )]
        source_gaps: PathBuf,
        /// Path to pavement docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-docket.csv",
            value_name = "FILE"
        )]
        pavement_docket: PathBuf,
        /// Directory containing per-state HPMS cache files
        #[arg(long, default_value = "data/cache", value_name = "DIR")]
        cache_dir: PathBuf,
        /// Output unmatched pavement join review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-unmatched-join-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Plan a broader HPMS functional-system fetch for unmatched priority-A US-route pavement members
    TierPavementHpmsScopeBroadening {
        /// Path to unmatched pavement join review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-unmatched-join-review.csv",
            value_name = "FILE"
        )]
        unmatched_join_review: PathBuf,
        /// Output HPMS scope broadening plan CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-hpms-scope-broadening.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// HPMS functional systems for broadened source acquisition
        #[arg(long, default_value = "1,2,3", value_name = "SYSTEMS")]
        functional_systems: String,
        /// Fail if broadening rows do not preserve blockers
        #[arg(long)]
        gate: bool,
    },

    /// Review priority-A pavement repair debt before any asset-condition relief replay
    TierPavementRepairDebtReview {
        /// Path to unmatched pavement join review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-unmatched-join-review.csv",
            value_name = "FILE"
        )]
        unmatched_join_review: PathBuf,
        /// Path to pavement debt budget CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-debt-budget.csv",
            value_name = "FILE"
        )]
        pavement_debt_budget: PathBuf,
        /// Path to accepted route/state pavement exclusion CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-route-state-exclusions.csv",
            value_name = "FILE"
        )]
        route_state_exclusions: PathBuf,
        /// Path to accepted pavement repair funding CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-funding-acceptance.csv",
            value_name = "FILE"
        )]
        repair_funding_acceptance: PathBuf,
        /// Output pavement repair debt review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-debt-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Decide priority-A pavement repair disposition before relief eligibility
    TierPavementRepairDisposition {
        /// Path to pavement repair debt review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-debt-review.csv",
            value_name = "FILE"
        )]
        repair_debt_review: PathBuf,
        /// Output pavement repair disposition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-disposition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if disposition rows allow relief or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Package priority-A pavement repair funding requirements without granting relief
    TierPavementRepairFundingPackage {
        /// Path to pavement repair disposition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-disposition.csv",
            value_name = "FILE"
        )]
        repair_disposition: PathBuf,
        /// Output pavement repair funding package CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-funding-package.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if package rows fund repairs or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Review accepted funding commitments for priority-A pavement repair packages
    TierPavementFundingCommitmentReview {
        /// Path to pavement repair funding package CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-repair-funding-package.csv",
            value_name = "FILE"
        )]
        repair_funding_package: PathBuf,
        /// Output pavement funding commitment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-commitment-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if commitments are treated as accepted or blockers are reduced
        #[arg(long)]
        gate: bool,
    },

    /// Decide downgrade or exclusion status for unfunded priority-A pavement repair rows
    TierPavementDowngradeExclusionDecision {
        /// Path to pavement funding commitment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-commitment-review.csv",
            value_name = "FILE"
        )]
        funding_commitment_review: PathBuf,
        /// Output pavement downgrade/exclusion decision CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-downgrade-exclusion-decision.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if decisions reduce blockers or grant relief
        #[arg(long)]
        gate: bool,
    },

    /// Define accepted funding evidence requirements for priority-A pavement repair relief
    TierPavementFundingEvidenceContract {
        /// Path to pavement downgrade/exclusion decision CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-downgrade-exclusion-decision.csv",
            value_name = "FILE"
        )]
        downgrade_exclusion_decision: PathBuf,
        /// Output pavement funding evidence contract CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-contract.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if evidence contract accepts funding or allows relief
        #[arg(long)]
        gate: bool,
    },

    /// Record source-capture status for priority-A pavement funding evidence
    TierPavementFundingEvidenceSourceCapture {
        /// Path to pavement funding evidence contract CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-contract.csv",
            value_name = "FILE"
        )]
        funding_evidence_contract: PathBuf,
        /// Output pavement funding evidence source-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-source-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source capture accepts evidence or allows relief
        #[arg(long)]
        gate: bool,
    },

    /// Record artifact-attachment status for priority-A pavement funding evidence
    TierPavementFundingEvidenceArtifactAttachment {
        /// Path to pavement funding evidence source-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-source-capture.csv",
            value_name = "FILE"
        )]
        source_capture: PathBuf,
        /// Output pavement funding evidence artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if attachment rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Review priority-A pavement funding evidence artifact attachments
    TierPavementFundingEvidenceReviewDocket {
        /// Path to pavement funding evidence artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-artifact-attachment.csv",
            value_name = "FILE"
        )]
        artifact_attachment: PathBuf,
        /// Output pavement funding evidence review docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-review-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Create acquisition targets for priority-A pavement funding evidence
    TierPavementFundingEvidenceAcquisition {
        /// Path to pavement funding evidence review docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-review-docket.csv",
            value_name = "FILE"
        )]
        review_docket: PathBuf,
        /// Output pavement funding evidence acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-acquisition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acquisition rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Classify source access for priority-A pavement funding evidence targets
    TierPavementFundingEvidenceSourceAccess {
        /// Path to pavement funding evidence acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-acquisition.csv",
            value_name = "FILE"
        )]
        evidence_acquisition: PathBuf,
        /// Output pavement funding evidence source-access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-access rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Define intake requirements for priority-A pavement funding evidence
    TierPavementFundingEvidenceIntake {
        /// Path to pavement funding evidence source-access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-source-access.csv",
            value_name = "FILE"
        )]
        source_access: PathBuf,
        /// Output pavement funding evidence intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if intake rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Record metadata-capture status for priority-A pavement funding evidence
    TierPavementFundingEvidenceMetadataCapture {
        /// Path to pavement funding evidence intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-intake.csv",
            value_name = "FILE"
        )]
        evidence_intake: PathBuf,
        /// Output pavement funding evidence metadata-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-metadata-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if metadata-capture rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Attach accepted artifacts after priority-A pavement funding metadata capture
    TierPavementFundingEvidenceAcceptedArtifactAttachment {
        /// Path to pavement funding evidence metadata-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-metadata-capture.csv",
            value_name = "FILE"
        )]
        metadata_capture: PathBuf,
        /// Output accepted artifact attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if attachment rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Review accepted-artifact attachment placeholders for priority-A pavement funding evidence
    TierPavementFundingEvidenceAcceptedAttachmentReview {
        /// Path to accepted artifact attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-artifact-attachment.csv",
            value_name = "FILE"
        )]
        accepted_artifact_attachment: PathBuf,
        /// Output accepted attachment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-attachment-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Create acquisition/cache targets for accepted priority-A pavement funding artifacts
    TierPavementFundingEvidenceAcceptedArtifactAcquisition {
        /// Path to accepted attachment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-attachment-review.csv",
            value_name = "FILE"
        )]
        accepted_attachment_review: PathBuf,
        /// Output accepted artifact acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acquisition rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Classify source/cache access for accepted priority-A pavement funding artifacts
    TierPavementFundingEvidenceAcceptedSourceAccess {
        /// Path to accepted artifact acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv",
            value_name = "FILE"
        )]
        accepted_artifact_acquisition: PathBuf,
        /// Output accepted source access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-access rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Define intake requirements for accepted priority-A pavement funding artifacts
    TierPavementFundingEvidenceAcceptedIntake {
        /// Path to accepted source-access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-source-access.csv",
            value_name = "FILE"
        )]
        accepted_source_access: PathBuf,
        /// Output accepted intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if intake rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Record metadata-capture placeholders for accepted priority-A pavement funding artifacts
    TierPavementFundingEvidenceAcceptedMetadataCapture {
        /// Path to accepted intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-intake.csv",
            value_name = "FILE"
        )]
        accepted_intake: PathBuf,
        /// Output accepted metadata-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if metadata-capture rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Record artifact-attachment placeholders after accepted metadata capture
    TierPavementFundingEvidenceAcceptedMetadataArtifactAttachment {
        /// Path to accepted metadata-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-capture.csv",
            value_name = "FILE"
        )]
        accepted_metadata_capture: PathBuf,
        /// Output accepted metadata artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if attachment rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Review accepted metadata artifact-attachment placeholders for priority-A pavement funding evidence
    TierPavementFundingEvidenceAcceptedMetadataAttachmentReview {
        /// Path to accepted metadata artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-artifact-attachment.csv",
            value_name = "FILE"
        )]
        accepted_metadata_artifact_attachment: PathBuf,
        /// Output accepted metadata attachment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-attachment-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Create acquisition/cache targets from accepted metadata attachment review holds
    TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisition {
        /// Path to accepted metadata attachment review CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-attachment-review.csv",
            value_name = "FILE"
        )]
        accepted_metadata_attachment_review: PathBuf,
        /// Output accepted metadata artifact acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acquisition rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Classify source/cache access for accepted metadata artifact acquisition targets
    TierPavementFundingEvidenceAcceptedMetadataSourceAccess {
        /// Path to accepted metadata artifact acquisition CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition.csv",
            value_name = "FILE"
        )]
        accepted_metadata_artifact_acquisition: PathBuf,
        /// Output accepted metadata source access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-access rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Define intake requirements for accepted metadata source-access rows
    TierPavementFundingEvidenceAcceptedMetadataIntake {
        /// Path to accepted metadata source-access CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-source-access.csv",
            value_name = "FILE"
        )]
        accepted_metadata_source_access: PathBuf,
        /// Output accepted metadata intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if intake rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Record source-needed capture placeholders for accepted metadata intake rows
    TierPavementFundingEvidenceAcceptedMetadataSourceCapture {
        /// Path to accepted metadata intake CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-intake.csv",
            value_name = "FILE"
        )]
        accepted_metadata_intake: PathBuf,
        /// Output accepted metadata source-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-source-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-capture rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Record artifact-attachment placeholders after accepted metadata source capture
    TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachment {
        /// Path to accepted metadata source-capture CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-source-capture.csv",
            value_name = "FILE"
        )]
        accepted_metadata_source_capture: PathBuf,
        /// Output accepted metadata source-capture artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-funding-evidence-accepted-metadata-source-capture-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if attachment rows accept evidence or allow relief
        #[arg(long)]
        gate: bool,
    },

    /// Show NBI bridge-condition coverage for tier bridge standards
    StandardsBridges {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Tier to check, e.g. T1
        #[arg(long, default_value = "T1")]
        tier: String,
        /// Show per-route bridge coverage rows
        #[arg(long)]
        details: bool,
        /// Fail if selected tier routes lack cached NBI bridge-condition coverage
        #[arg(long)]
        gate_l1: bool,
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
        /// Also print standard-to-scenario coverage for pressure-tested standards
        #[arg(long)]
        coverage: bool,
        /// Path to standards proof ledger used by --coverage and --gate-coverage
        #[arg(
            long,
            default_value = "data/standards-proof-ledger.csv",
            value_name = "FILE"
        )]
        standards_ledger: PathBuf,
        /// Fail if any scenario catalog row lacks a bounded proof contract
        #[arg(long)]
        gate_l2: bool,
        /// Fail if any required L2 scenario is still only planned or stubbed
        #[arg(long)]
        gate_readiness: bool,
        /// Fail if high-stakes T1 throughput/resilience standards lack a scenario hook
        #[arg(long)]
        gate_coverage: bool,
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

    /// Show T1/T1 diamond anchor manual-validation status
    T1DiamondValidation {
        /// Path to T1/T1 diamond anchor validation ledger CSV
        #[arg(
            long,
            default_value = "data/t1-diamond-validation.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows not yet manually validated
        #[arg(long)]
        blockers: bool,
        /// Show only one priority band, e.g. A, B, or C
        #[arg(long)]
        priority: Option<String>,
        /// Print one actionable validation task per unresolved evidence dimension
        #[arg(long)]
        docket: bool,
        /// Join source-health blocker details into observed-failure docket rows
        #[arg(long)]
        with_access: bool,
        /// Path to T1/T1 source health ledger CSV for --with-access
        #[arg(long, default_value = "data/t1-source-health.csv", value_name = "FILE")]
        source_health: PathBuf,
        /// Print detailed validation blockers and next steps
        #[arg(long)]
        details: bool,
        /// Fail if the anchor catalog is incomplete or rows lack validation contracts
        #[arg(long)]
        gate_catalog: bool,
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

    /// Show polling plan for live snapshot-only T1/T1 event feeds
    T1SnapshotPlan {
        /// Path to T1/T1 snapshot polling plan CSV
        #[arg(long, default_value = "data/t1-snapshot-plan.csv", value_name = "FILE")]
        ledger: PathBuf,
        /// Show only one priority band, e.g. A, B, or C
        #[arg(long)]
        priority: Option<String>,
        /// Print fetch/import/accumulate command details
        #[arg(long)]
        details: bool,
        /// Print runnable fetch/import/accumulate commands in execution order
        #[arg(long)]
        script: bool,
        /// Fail if snapshot rows lack cadence, commands, or output paths
        #[arg(long)]
        gate_plan: bool,
    },

    /// Show source-window metadata for T1/T1 failure evidence operations
    T1EvidenceWindows {
        /// Path to T1/T1 evidence-window ledger CSV
        #[arg(
            long,
            default_value = "data/t1-evidence-windows.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Show only rows that are not promotion eligible
        #[arg(long)]
        blockers: bool,
        /// Print detailed artifacts, gaps, and next steps
        #[arg(long)]
        details: bool,
        /// Fail if source windows lack freshness metadata or over-promote snapshot-only evidence
        #[arg(long)]
        gate_windows: bool,
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
        /// Fail if normalized event observations are empty or missing required evidence fields
        #[arg(long)]
        gate_observations: bool,
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

    /// Analyze tier graph semantics: do T2 routes connect into the T1 network?
    TierConnectivity {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Path to endpoint exception ledger CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Tier to analyze against the T1 backbone
        #[arg(long, default_value = "T2")]
        tier: String,
        /// Print full per-route touch-node details
        #[arg(long)]
        details: bool,
        /// Fail if selected tier routes touch fewer than two T1 nodes
        #[arg(long)]
        gate: bool,
    },

    /// Partition tier routes into METIS-backed regional workloads
    TierRegions {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Tier to regionalize
        #[arg(long, default_value = "T2")]
        tier: String,
        /// Number of target service regions
        #[arg(long, default_value_t = 4)]
        regions: usize,
        /// Graph model to split
        #[arg(long, value_enum, default_value_t = TierRegionGraphArg::DualRoute)]
        graph: TierRegionGraphArg,
        /// Output workload CSV
        #[arg(
            long,
            short,
            default_value = "data/tier-region-workloads.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Output repair docket for disconnected or under-qualified tier routes
        #[arg(
            long,
            default_value = "data/tier-region-repairs.csv",
            value_name = "FILE"
        )]
        repairs: PathBuf,
        /// Fail if METIS cannot produce complete non-empty regions
        #[arg(long)]
        gate: bool,
    },

    /// Turn tier-region repair rows into explicit contact/exception/demotion witnesses
    TierContactWitnesses {
        /// Tier region repair docket CSV
        #[arg(
            long,
            default_value = "data/tier-region-repairs.csv",
            value_name = "FILE"
        )]
        repairs: PathBuf,
        /// Output contact witness CSV
        #[arg(
            long,
            short,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any witness is still source/review gated
        #[arg(long)]
        gate: bool,
    },

    /// Emit a focused resolution docket for unresolved T2 contact witnesses
    T2ContactResolutions {
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// Tier node exception CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Output T2 contact resolution CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-contact-resolutions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if true contact/source blockers remain unresolved
        #[arg(long)]
        gate: bool,
    },

    /// Emit terminal and relief validation rows from held T2 contact resolutions
    T2HeldContactActions {
        /// T2 contact resolution CSV
        #[arg(
            long,
            default_value = "data/t2-contact-resolutions.csv",
            value_name = "FILE"
        )]
        resolutions: PathBuf,
        /// Output held contact action CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-held-contact-actions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if held terminal/relief/contact actions lack next-step contracts
        #[arg(long)]
        gate: bool,
    },

    /// Emit graph-contact repair actions for held T2 graph rows
    T2GraphContactRepairs {
        /// T2 held contact action CSV
        #[arg(
            long,
            default_value = "data/t2-held-contact-actions.csv",
            value_name = "FILE"
        )]
        held_actions: PathBuf,
        /// Output graph-contact repair CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-graph-contact-repairs.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if graph-contact repair rows lack deterministic actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit parent-contact validation actions for held T2 relief loops
    T2ParentContactValidation {
        /// T2 held contact action CSV
        #[arg(
            long,
            default_value = "data/t2-held-contact-actions.csv",
            value_name = "FILE"
        )]
        held_actions: PathBuf,
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// Output parent-contact validation CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-parent-contact-validation.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if parent-contact validation rows lack deterministic actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit relief evidence review rows for held T2 relief candidates
    T2ReliefEvidenceDocket {
        /// T2 held contact action CSV
        #[arg(
            long,
            default_value = "data/t2-held-contact-actions.csv",
            value_name = "FILE"
        )]
        held_actions: PathBuf,
        /// ATRI bottleneck CSV
        #[arg(long, default_value = "data/atri-bottlenecks.csv", value_name = "FILE")]
        bottlenecks: PathBuf,
        /// Output relief evidence docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-relief-evidence-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief evidence rows lack source/demotion actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit terminal endpoint/contact validation rows for held T2 terminal candidates
    T2TerminalContactValidation {
        /// T2 held contact action CSV
        #[arg(
            long,
            default_value = "data/t2-held-contact-actions.csv",
            value_name = "FILE"
        )]
        held_actions: PathBuf,
        /// Endpoint exception ledger CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// Output terminal validation CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-terminal-contact-validation.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if terminal validation rows lack next-step contracts
        #[arg(long)]
        gate: bool,
    },

    /// Join T2 held repair surfaces into one route-level blocker closure docket
    T2BlockerClosure {
        /// Graph-contact repair CSV
        #[arg(
            long,
            default_value = "data/t2-graph-contact-repairs.csv",
            value_name = "FILE"
        )]
        graph_repairs: PathBuf,
        /// Parent-contact validation CSV
        #[arg(
            long,
            default_value = "data/t2-parent-contact-validation.csv",
            value_name = "FILE"
        )]
        parent_validation: PathBuf,
        /// Relief evidence docket CSV
        #[arg(
            long,
            default_value = "data/t2-relief-evidence-docket.csv",
            value_name = "FILE"
        )]
        relief_evidence: PathBuf,
        /// Terminal contact validation CSV
        #[arg(
            long,
            default_value = "data/t2-terminal-contact-validation.csv",
            value_name = "FILE"
        )]
        terminal_validation: PathBuf,
        /// National segment bundle CSV
        #[arg(
            long,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        bundles: PathBuf,
        /// Output blocker closure CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if blocker closure rows lack closure class, action, or next artifact
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-family split decisions for ambiguous numbered T2 families
    T2RouteFamilySplits {
        /// T2 blocker closure CSV
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        closure: PathBuf,
        /// T2 service diagnostic queue CSV
        #[arg(
            long,
            default_value = "data/t2-service-diagnostic-queue.csv",
            value_name = "FILE"
        )]
        service_diagnostics: PathBuf,
        /// National segment bundle CSV
        #[arg(
            long,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        bundles: PathBuf,
        /// Endpoint exception ledger CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Output route-family split CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-route-family-splits.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if route-family split decisions lack a deterministic disposition
        #[arg(long)]
        gate: bool,
    },

    /// Validate graph-contact repair blockers against observed T1/T2 contact witnesses
    T2GraphContactValidation {
        /// T2 blocker closure CSV
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        closure: PathBuf,
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// Output graph-contact validation CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-graph-contact-validation.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if graph-contact validation rows lack a deterministic disposition
        #[arg(long)]
        gate: bool,
    },

    /// Roll up remaining T2 parent, relief, and terminal contact blockers
    T2ContactClosure {
        /// T2 blocker closure CSV
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        closure: PathBuf,
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// Output contact closure CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-contact-closure.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if contact closure rows lack a deterministic disposition
        #[arg(long)]
        gate: bool,
    },

    /// Resolve T2 endpoint-exception upgrade blockers into upgrade or demotion actions
    T2EndpointClosure {
        /// T2 blocker closure CSV
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        closure: PathBuf,
        /// Endpoint exception ledger CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        exceptions: PathBuf,
        /// Output endpoint closure CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-endpoint-closure.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if endpoint closure rows lack a deterministic disposition
        #[arg(long)]
        gate: bool,
    },

    /// Emit optimizer candidate columns from accepted/reviewed tier contact witnesses
    TierCandidateColumns {
        /// Tier contact witness CSV
        #[arg(
            long,
            default_value = "data/tier-contact-witnesses.csv",
            value_name = "FILE"
        )]
        witnesses: PathBuf,
        /// T2 route-family split closure CSV
        #[arg(
            long,
            default_value = "data/t2-route-family-splits.csv",
            value_name = "FILE"
        )]
        route_family_splits: PathBuf,
        /// T2 graph-contact validation CSV
        #[arg(
            long,
            default_value = "data/t2-graph-contact-validation.csv",
            value_name = "FILE"
        )]
        graph_contact_validation: PathBuf,
        /// T2 parent/relief/terminal contact closure CSV
        #[arg(
            long,
            default_value = "data/t2-contact-closure.csv",
            value_name = "FILE"
        )]
        contact_closure: PathBuf,
        /// T2 endpoint closure CSV
        #[arg(
            long,
            default_value = "data/t2-endpoint-closure.csv",
            value_name = "FILE"
        )]
        endpoint_closure: PathBuf,
        /// T2 blocker closure CSV with bundle posture
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        blocker_closure: PathBuf,
        /// Pavement debt budget CSV with optimizer cost penalties
        #[arg(
            long,
            default_value = "data/tier-pavement-debt-budget.csv",
            value_name = "FILE"
        )]
        pavement_debt_budget: PathBuf,
        /// Optimizer constraint budget CSV with generalized blocker/debt rollups
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        constraint_budget: PathBuf,
        /// Output candidate column CSV
        #[arg(
            long,
            short,
            default_value = "data/tier-candidate-columns.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if no selected candidate columns are available
        #[arg(long)]
        gate: bool,
    },

    /// Build first-pass T2 regionalizer rows from candidate columns
    T2Regionalizer {
        /// Tier candidate column CSV
        #[arg(
            long,
            default_value = "data/tier-candidate-columns.csv",
            value_name = "FILE"
        )]
        candidates: PathBuf,
        /// Output T2 regionalizer CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-regionalizer.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if no selected regional treatments are available
        #[arg(long)]
        gate: bool,
    },

    /// Select T2 service-column actions using regionalizer rows and Beck diagnostics
    T2ServiceSelection {
        /// T2 regionalizer CSV
        #[arg(long, default_value = "data/t2-regionalizer.csv", value_name = "FILE")]
        regionalizer: PathBuf,
        /// Output T2 service selection CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selected services lack diagnostic-backed actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit Beck/service diagnostic work for bundle-ready T2 rows still held from map/game service
    T2ServiceDiagnosticQueue {
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        service_selection: PathBuf,
        /// National segment bundle CSV
        #[arg(
            long,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        bundles: PathBuf,
        /// Output T2 service diagnostic queue CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-service-diagnostic-queue.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if pending service diagnostics lack bundle identity or next work
        #[arg(long)]
        gate: bool,
    },

    /// Emit review work for T2 services that run too close to another Beck T2 line
    T2ParallelServiceQueue {
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        service_selection: PathBuf,
        /// Output T2 parallel service queue CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-parallel-service-queue.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if parallel-service rows lack a decision contract
        #[arg(long)]
        gate: bool,
    },

    /// Bind T2 game/ops service overlays to national segment bundles where available
    T2BundleOverlays {
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        service_selection: PathBuf,
        /// National segment bundle CSV
        #[arg(
            long,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        bundles: PathBuf,
        /// T2 game service-class overlay CSV
        #[arg(
            long,
            default_value = "data/game/t2-service-overlays.csv",
            value_name = "FILE"
        )]
        game_overlays: PathBuf,
        /// Output bundle-bound T2 overlay CSV
        #[arg(
            long,
            short,
            default_value = "data/game/t2-bundle-overlays.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows lack explicit bundle binding or pending-binding status
        #[arg(long)]
        gate: bool,
    },

    /// Emit bundle repair actions for T2 closure rows blocked by bundle posture
    T2BundleRepairQueue {
        /// Tier candidate column CSV
        #[arg(
            long,
            default_value = "data/tier-candidate-columns.csv",
            value_name = "FILE"
        )]
        candidates: PathBuf,
        /// T2 blocker closure CSV
        #[arg(
            long,
            default_value = "data/t2-blocker-closure.csv",
            value_name = "FILE"
        )]
        blocker_closure: PathBuf,
        /// Output T2 bundle repair queue CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-repair-queue.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if pending bundle rows lack repair actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit segment-level T1/T2 bundle candidates from selector outputs and the highway graph
    TierSegmentCandidates {
        /// T1 line selector CSV
        #[arg(long, default_value = "data/t1-line-selector.csv", value_name = "FILE")]
        t1_selector: PathBuf,
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        t2_service_selection: PathBuf,
        /// T2 bundle repair queue CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-repair-queue.csv",
            value_name = "FILE"
        )]
        t2_bundle_repair_queue: PathBuf,
        /// T2 route-family split docket CSV
        #[arg(
            long,
            default_value = "data/t2-route-family-splits.csv",
            value_name = "FILE"
        )]
        t2_route_family_splits: PathBuf,
        /// Output segment candidate CSV
        #[arg(
            long,
            short,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selected T1/T2 services do not decompose into graph segments
        #[arg(long)]
        gate: bool,
    },

    /// Emit T3/T4 pressure witnesses that can roll lower-tier gaps back upward
    LowerTierPressureWitnesses {
        /// Tier score table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Tier candidate column CSV
        #[arg(
            long,
            default_value = "data/tier-candidate-columns.csv",
            value_name = "FILE"
        )]
        candidates: PathBuf,
        /// T2 contact resolution CSV
        #[arg(
            long,
            default_value = "data/t2-contact-resolutions.csv",
            value_name = "FILE"
        )]
        resolutions: PathBuf,
        /// T2 route-family split closure CSV
        #[arg(
            long,
            default_value = "data/t2-route-family-splits.csv",
            value_name = "FILE"
        )]
        route_family_splits: PathBuf,
        /// T2 graph-contact validation CSV
        #[arg(
            long,
            default_value = "data/t2-graph-contact-validation.csv",
            value_name = "FILE"
        )]
        graph_contact_validation: PathBuf,
        /// T2 parent/relief/terminal contact closure CSV
        #[arg(
            long,
            default_value = "data/t2-contact-closure.csv",
            value_name = "FILE"
        )]
        contact_closure: PathBuf,
        /// T2 endpoint closure CSV
        #[arg(
            long,
            default_value = "data/t2-endpoint-closure.csv",
            value_name = "FILE"
        )]
        endpoint_closure: PathBuf,
        /// Output lower-tier pressure witness CSV
        #[arg(
            long,
            short,
            default_value = "data/lower-tier-pressure-witnesses.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if no lower-tier pressure witnesses are emitted
        #[arg(long)]
        gate: bool,
    },

    /// Classify lower-tier pressure into a thin T3/T4 intake and upward feedback docket
    T3T4PressureIntake {
        /// Lower-tier pressure witness CSV
        #[arg(
            long,
            default_value = "data/lower-tier-pressure-witnesses.csv",
            value_name = "FILE"
        )]
        pressure: PathBuf,
        /// Output T3/T4 pressure intake CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if pressure intake rows lack deterministic actions
        #[arg(long)]
        gate: bool,
    },

    /// Emit T3 zone access obligations from lower-tier pressure intake rows
    T3ZoneAccessObligations {
        /// T3/T4 pressure intake CSV
        #[arg(
            long,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// Map atlas CSV containing T3 zone map ids
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Output T3 zone access obligation CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-zone-access-obligations.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if zone obligations are incomplete or detached from maps
        #[arg(long)]
        gate: bool,
    },

    /// Select route columns that satisfy T3 zone access obligations
    T3ZoneRouteColumns {
        /// T3 zone access obligation CSV
        #[arg(
            long,
            default_value = "data/t3-zone-access-obligations.csv",
            value_name = "FILE"
        )]
        obligations: PathBuf,
        /// T3/T4 pressure intake CSV, used for route scores and source tiers
        #[arg(
            long,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// Optimizer constraint budget CSV with generalized blocker/debt rollups
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        constraint_budget: PathBuf,
        /// Output T3 zone route columns CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-zone-route-columns.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selected route columns do not satisfy zone obligations
        #[arg(long)]
        gate: bool,
    },

    /// Select T4 terminal/local access columns from lower-tier pressure intake
    T4TerminalAccessColumns {
        /// T3/T4 pressure intake CSV
        #[arg(
            long,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// Optimizer constraint budget CSV with generalized blocker/debt rollups
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        constraint_budget: PathBuf,
        /// Output T4 terminal/local access column CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-columns.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if T4 local pressure lacks a terminal/local access disposition
        #[arg(long)]
        gate: bool,
    },

    /// Emit T4 terminal contact evidence queue from held terminal access columns
    T4TerminalContactEvidence {
        /// T4 terminal/local access columns CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-columns.csv",
            value_name = "FILE"
        )]
        terminal_columns: PathBuf,
        /// Output T4 terminal contact evidence queue CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-contact-evidence.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if contact evidence rows violate source/decision rules
        #[arg(long)]
        gate: bool,
    },

    /// Review T4 terminal-access evidence blockers without accepting seed-only proof
    T4TerminalAccessEvidenceReview {
        /// T4 terminal contact evidence queue CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-evidence.csv",
            value_name = "FILE"
        )]
        contact_evidence: PathBuf,
        /// Output T4 terminal-access evidence review CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-evidence-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-needed rows are promoted or blockers are reduced
        #[arg(long)]
        gate: bool,
    },

    /// Emit acquisition tasks for held T4 terminal-access evidence review rows
    T4TerminalAccessProofAcquisition {
        /// T4 terminal-access evidence review CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-evidence-review.csv",
            value_name = "FILE"
        )]
        evidence_review: PathBuf,
        /// Output T4 terminal-access proof acquisition CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-acquisition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acquisition rows accept proof or omit held review rows
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof artifact placeholders for held T4 terminal-access acquisition tasks
    T4TerminalAccessProofArtifacts {
        /// T4 terminal-access proof acquisition CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-acquisition.csv",
            value_name = "FILE"
        )]
        proof_acquisition: PathBuf,
        /// Output T4 terminal-access proof artifacts CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-artifacts.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if artifact rows attach or accept proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof review rows for T4 terminal-access proof artifact placeholders
    T4TerminalAccessProofReview {
        /// T4 terminal-access proof artifacts CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-artifacts.csv",
            value_name = "FILE"
        )]
        proof_artifacts: PathBuf,
        /// Output T4 terminal-access proof review CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if proof-review rows accept proof or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit source-access policy rows for T4 terminal-access proof review holds
    T4TerminalAccessSourceAccess {
        /// T4 terminal-access proof review CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-review.csv",
            value_name = "FILE"
        )]
        proof_review: PathBuf,
        /// Output T4 terminal-access source-access CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows enable live fetch or accept proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof-intake rows for T4 terminal-access source-access policy rows
    T4TerminalAccessProofIntake {
        /// T4 terminal-access source-access CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-source-access.csv",
            value_name = "FILE"
        )]
        source_access: PathBuf,
        /// Output T4 terminal-access proof intake CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if proof-intake rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit source-capture rows for T4 terminal-access proof-intake rows
    T4TerminalAccessProofSourceCapture {
        /// T4 terminal-access proof intake CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-intake.csv",
            value_name = "FILE"
        )]
        proof_intake: PathBuf,
        /// Output T4 terminal-access proof source-capture CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-source-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-capture rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit artifact-attachment rows for T4 terminal-access source-capture rows
    T4TerminalAccessProofArtifactAttachment {
        /// T4 terminal-access proof source-capture CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-source-capture.csv",
            value_name = "FILE"
        )]
        source_capture: PathBuf,
        /// Output T4 terminal-access proof artifact-attachment CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if artifact-attachment rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Review T4 terminal-access proof artifact-attachment placeholders
    T4TerminalAccessProofAttachmentReview {
        /// T4 terminal-access proof artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-artifact-attachment.csv",
            value_name = "FILE"
        )]
        artifact_attachment: PathBuf,
        /// Output T4 terminal-access proof attachment-review CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-attachment-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if attachment-review rows accept evidence or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Create acquisition targets for held T4 terminal-access proof attachment reviews
    T4TerminalAccessProofArtifactAcquisitionTargets {
        /// T4 terminal-access proof attachment-review CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-attachment-review.csv",
            value_name = "FILE"
        )]
        attachment_review: PathBuf,
        /// Output T4 terminal-access proof artifact acquisition-target CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-artifact-acquisition-targets.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if target rows accept evidence or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Classify source/cache access for held T4 terminal-access proof artifact targets
    T4TerminalAccessProofArtifactSourceAccess {
        /// T4 terminal-access proof artifact acquisition-target CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-proof-artifact-acquisition-targets.csv",
            value_name = "FILE"
        )]
        acquisition_targets: PathBuf,
        /// Output T4 terminal-access proof artifact source-access CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-access-proof-artifact-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-access rows enable live fetches or accept proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit scenario-readiness docket from source-backed T4 terminal contact rows
    T4TerminalScenarioReadiness {
        /// T4 terminal contact evidence queue CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-evidence.csv",
            value_name = "FILE"
        )]
        contact_evidence: PathBuf,
        /// Output T4 terminal scenario-readiness docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-scenario-readiness.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if scenario-ready rows lack proof, attachment, or release hold
        #[arg(long)]
        gate: bool,
    },

    /// Emit terminal-contact source acquisition plan
    T4TerminalContactSourcePlan {
        /// T4 terminal contact evidence queue CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-evidence.csv",
            value_name = "FILE"
        )]
        contact_evidence: PathBuf,
        /// Output T4 terminal contact source plan CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-contact-source-plan.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Output terminal district source catalog CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-source-catalog.csv",
            value_name = "FILE"
        )]
        catalog_output: PathBuf,
        /// Output route contact proof docket CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-proof-docket.csv",
            value_name = "FILE"
        )]
        proof_docket_output: PathBuf,
        /// Fail if source-plan rows are incomplete or confuse seed sources with proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit terminal-contact proof artifact contract
    T4TerminalContactProofArtifactContract {
        /// Output terminal-contact proof artifact contract CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-contact-proof-artifact-contract.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if contract rows do not protect source-backed promotion
        #[arg(long)]
        gate: bool,
    },

    /// Emit terminal-contact proof source registry
    T4TerminalContactProofSourceRegistry {
        /// Route contact proof docket CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-proof-docket.csv",
            value_name = "FILE"
        )]
        proof_docket: PathBuf,
        /// Accepted non-seed terminal-contact proof sources CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-accepted-proof-sources.csv",
            value_name = "FILE"
        )]
        accepted_sources: PathBuf,
        /// Output terminal-contact proof source registry CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-contact-proof-source-registry.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if registry rows are incomplete or cite seed data as proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit one district terminal-contact proof import decision slice
    T4TerminalContactDistrictProofImport {
        /// Terminal-contact proof source registry CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-proof-source-registry.csv",
            value_name = "FILE"
        )]
        source_registry: PathBuf,
        /// Output district proof import decision CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-contact-district-proof-import.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if import rows promote without accepted registry proof
        #[arg(long)]
        gate: bool,
    },

    /// Emit Columbus South terminal-contact proof intake from Great Lakes proof docket
    T4TerminalColumbusProofIntake {
        /// Route contact proof docket CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-proof-docket.csv",
            value_name = "FILE"
        )]
        proof_docket: PathBuf,
        /// Output Columbus South terminal-contact proof intake CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-columbus-proof-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if Columbus intake rows are incomplete or include non-Columbus tasks
        #[arg(long)]
        gate: bool,
    },

    /// Emit Columbus South terminal-contact source access contract
    T4TerminalColumbusSourceAccess {
        /// Columbus South terminal-contact proof intake CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-columbus-proof-intake.csv",
            value_name = "FILE"
        )]
        columbus_intake: PathBuf,
        /// Output Columbus South source access contract CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-columbus-source-access.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if access rows are incomplete or enable unsupported live fetches
        #[arg(long)]
        gate: bool,
    },

    /// Emit Columbus South terminal-contact route proof attempts
    T4TerminalColumbusProofAttempts {
        /// Columbus South source access contract CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-columbus-source-access.csv",
            value_name = "FILE"
        )]
        source_access: PathBuf,
        /// Output Columbus South route proof attempts CSV
        #[arg(
            long,
            short,
            default_value = "data/t4-terminal-columbus-proof-attempts.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if proof attempts lack blockers or promote without proof artifacts
        #[arg(long)]
        gate: bool,
    },

    /// Emit T3/T4 access gaps from held route and terminal access columns
    #[command(name = "t3-t4-access-gaps")]
    T3T4AccessGaps {
        /// T3 zone route columns CSV
        #[arg(
            long,
            default_value = "data/t3-zone-route-columns.csv",
            value_name = "FILE"
        )]
        route_columns: PathBuf,
        /// T4 terminal/local access columns CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-columns.csv",
            value_name = "FILE"
        )]
        terminal_columns: PathBuf,
        /// Output T3/T4 access gap CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-t4-access-gaps.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if gaps lack class, evidence, or next artifact
        #[arg(long)]
        gate: bool,
    },

    /// Emit zone-level diagnostics that connect selected T3 columns and gaps to T3 maps
    T3ZoneMapDiagnostics {
        /// T3 zone route columns CSV
        #[arg(
            long,
            default_value = "data/t3-zone-route-columns.csv",
            value_name = "FILE"
        )]
        route_columns: PathBuf,
        /// T3/T4 access gaps CSV
        #[arg(
            long,
            default_value = "data/t3-t4-access-gaps.csv",
            value_name = "FILE"
        )]
        access_gaps: PathBuf,
        /// Map atlas CSV containing T3 zone map ids
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Output T3 zone map diagnostics CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-zone-map-diagnostics.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if a T3 zone map lacks selected-route or gap diagnostics
        #[arg(long)]
        gate: bool,
    },

    /// Emit optimizer-backed board rows for T3 zone renderers and game overlays
    T3ZoneRenderBoard {
        /// T3 zone map diagnostics CSV
        #[arg(
            long,
            default_value = "data/t3-zone-map-diagnostics.csv",
            value_name = "FILE"
        )]
        diagnostics: PathBuf,
        /// T3 zone route columns CSV
        #[arg(
            long,
            default_value = "data/t3-zone-route-columns.csv",
            value_name = "FILE"
        )]
        route_columns: PathBuf,
        /// T3/T4 access gaps CSV
        #[arg(
            long,
            default_value = "data/t3-t4-access-gaps.csv",
            value_name = "FILE"
        )]
        access_gaps: PathBuf,
        /// Map atlas CSV containing T3 zone map ids
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Output T3 zone render board CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-zone-render-board.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if render rows are detached from selected optimizer routes or map ids
        #[arg(long)]
        gate: bool,
    },

    /// Emit stop-placement readiness rows for selected T3 zone render-board routes
    T3ZoneStopPlacement {
        /// T3 zone render board CSV
        #[arg(
            long,
            default_value = "data/t3-zone-render-board.csv",
            value_name = "FILE"
        )]
        render_board: PathBuf,
        /// Path to stop investment candidate ledger CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        stop_candidates: PathBuf,
        /// Output T3 zone stop placement CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-zone-stop-placement.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selected T3 routes lack either a viable stop chain or an authoring action
        #[arg(long)]
        gate: bool,
    },

    /// Emit a national segment identity registry from segment-bearing optimizer artifacts
    NationalSegmentRegistry {
        /// T3 zone render board CSV
        #[arg(
            long,
            default_value = "data/t3-zone-render-board.csv",
            value_name = "FILE"
        )]
        render_board: PathBuf,
        /// T3 zone stop placement CSV
        #[arg(
            long,
            default_value = "data/t3-zone-stop-placement.csv",
            value_name = "FILE"
        )]
        stop_placement: PathBuf,
        /// T1/T2 segment candidate CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segment_candidates: PathBuf,
        /// T1/T2 pavement readiness docket CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-docket.csv",
            value_name = "FILE"
        )]
        pavement_docket: PathBuf,
        /// Output national segment registry CSV
        #[arg(
            long,
            short,
            default_value = "data/national-segment-registry.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if segment identities are incomplete, unparseable, or detached from bundles
        #[arg(long)]
        gate: bool,
    },

    /// Emit service/corridor bundles from the national segment registry
    NationalSegmentBundles {
        /// National segment registry CSV
        #[arg(
            long,
            default_value = "data/national-segment-registry.csv",
            value_name = "FILE"
        )]
        registry: PathBuf,
        /// Output national segment bundle CSV
        #[arg(
            long,
            short,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if bundles lack members, aliases, or valid segment references
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 review rows created by lower-tier bubble-up pressure
    T2BubbleUpReview {
        /// T3/T4 pressure intake CSV
        #[arg(
            long,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// Output T2 bubble-up review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bubble-up-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if bubble-up review rows lack deterministic next gates
        #[arg(long)]
        gate: bool,
    },

    /// Roll T2/T3/T4 pressure up to T1 only when it names a T1 SLA dependency
    T1FeedbackDocket {
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        service_selection: PathBuf,
        /// T2 bubble-up review CSV
        #[arg(
            long,
            default_value = "data/t2-bubble-up-review.csv",
            value_name = "FILE"
        )]
        bubble_up: PathBuf,
        /// T3/T4 pressure intake CSV
        #[arg(
            long,
            default_value = "data/t3-t4-pressure-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// T1 SLA pair portfolio CSV
        #[arg(long, default_value = "data/t1-sla-pairs.csv", value_name = "FILE")]
        sla_pairs: PathBuf,
        /// Output T1 feedback docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-feedback-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if T1 feedback rows violate conservative promotion rules
        #[arg(long)]
        gate: bool,
    },

    /// Emit the recursive tier optimizer run manifest and gate bundle
    TierOptimize {
        /// Include the full T1/T2/T3/T4 optimizer surface
        #[arg(long)]
        all_tiers: bool,
        /// Output optimizer run manifest CSV
        #[arg(
            long,
            short,
            default_value = "data/tier-optimizer-runs.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if the optimizer bundle has missing or unexpected failed artifacts
        #[arg(long)]
        gate: bool,
    },

    /// Verify an existing optimizer run manifest without regenerating it
    OptimizerManifest {
        /// Optimizer run manifest CSV
        #[arg(
            long,
            default_value = "data/tier-optimizer-runs.csv",
            value_name = "FILE"
        )]
        manifest: PathBuf,
        /// Fail if manifest rows violate bundle contract
        #[arg(long)]
        gate: bool,
    },

    /// Normalize optimizer blockers, claim holds, debts, and penalties into one ledger
    OptimizerConstraintLedger {
        /// Pavement debt budget CSV
        #[arg(
            long,
            default_value = "data/tier-pavement-debt-budget.csv",
            value_name = "FILE"
        )]
        pavement_debt_budget: PathBuf,
        /// T2 asset-condition map publication exclusion decision CSV
        #[arg(
            long,
            default_value = "data/t2-asset-condition-map-publication-exclusion.csv",
            value_name = "FILE"
        )]
        t2_asset_condition_map_publication_exclusion: PathBuf,
        /// T1 topology repair CSV
        #[arg(
            long,
            default_value = "data/t1-topology-repairs.csv",
            value_name = "FILE"
        )]
        t1_topology_repairs: PathBuf,
        /// T1 schematic geometry blocker relief CSV
        #[arg(
            long,
            default_value = "data/t1-schematic-geometry-blocker-relief.csv",
            value_name = "FILE"
        )]
        t1_schematic_geometry_blocker_relief: PathBuf,
        /// T2 Beck transfer-complexity blocker relief CSV
        #[arg(
            long,
            default_value = "data/t2-beck-transfer-complexity-blocker-relief.csv",
            value_name = "FILE"
        )]
        t2_beck_transfer_complexity_blocker_relief: PathBuf,
        /// T2 Beck label-density blocker relief CSV
        #[arg(
            long,
            default_value = "data/t2-beck-label-density-blocker-relief.csv",
            value_name = "FILE"
        )]
        t2_beck_label_density_blocker_relief: PathBuf,
        /// T2 Beck long-connector blocker relief CSV
        #[arg(
            long,
            default_value = "data/t2-beck-long-connector-blocker-relief.csv",
            value_name = "FILE"
        )]
        t2_beck_long_connector_blocker_relief: PathBuf,
        /// T2 game publication evidence blocker relief CSV
        #[arg(
            long,
            default_value = "data/t2-game-publication-evidence-blocker-relief.csv",
            value_name = "FILE"
        )]
        t2_game_publication_evidence_blocker_relief: PathBuf,
        /// T2 game/ops bundle evidence blocker relief CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-bundle-evidence-blocker-relief.csv",
            value_name = "FILE"
        )]
        t2_game_ops_bundle_evidence_blocker_relief: PathBuf,
        /// T3 lower-tier feeder-gap blocker relief CSV
        #[arg(
            long,
            default_value = "data/t3-lower-tier-feeder-gap-blocker-relief.csv",
            value_name = "FILE"
        )]
        t3_lower_tier_feeder_gap_blocker_relief: PathBuf,
        /// T2 parallel service queue CSV
        #[arg(
            long,
            default_value = "data/t2-parallel-service-queue.csv",
            value_name = "FILE"
        )]
        t2_parallel_service_queue: PathBuf,
        /// T3/T4 access gap CSV
        #[arg(
            long,
            default_value = "data/t3-t4-access-gaps.csv",
            value_name = "FILE"
        )]
        t3_t4_access_gaps: PathBuf,
        /// T4 terminal-access map publication exclusion decision CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-access-map-exclusion.csv",
            value_name = "FILE"
        )]
        t4_terminal_access_map_exclusion: PathBuf,
        /// Accepted T4 terminal-contact proof import CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-district-proof-import.csv",
            value_name = "FILE"
        )]
        t4_terminal_contact_district_proof_import: PathBuf,
        /// Rejected T4 terminal-contact proof sources CSV
        #[arg(
            long,
            default_value = "data/t4-terminal-contact-rejected-proof-sources.csv",
            value_name = "FILE"
        )]
        t4_terminal_contact_rejected_proof_sources: PathBuf,
        /// Source-fetch policy CSV
        #[arg(
            long,
            default_value = "data/source-fetch-policy.csv",
            value_name = "FILE"
        )]
        source_fetch_policy: PathBuf,
        /// Source snapshot publication exclusion decision CSV
        #[arg(
            long,
            default_value = "data/source-snapshot-publication-exclusion.csv",
            value_name = "FILE"
        )]
        source_snapshot_publication_exclusion: PathBuf,
        /// T2 game scenario hook CSV
        #[arg(
            long,
            default_value = "data/game/t2-scenario-hooks.csv",
            value_name = "FILE"
        )]
        t2_scenario_hooks: PathBuf,
        /// T2 bundle-bound game overlay CSV
        #[arg(
            long,
            default_value = "data/game/t2-bundle-overlays.csv",
            value_name = "FILE"
        )]
        t2_bundle_overlays: PathBuf,
        /// Output normalized constraint ledger CSV
        #[arg(
            long,
            short,
            default_value = "data/optimizer-constraint-ledger.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print per-class detail rows
        #[arg(long)]
        details: bool,
        /// Fail if normalized rows violate the constraint ledger contract
        #[arg(long)]
        gate: bool,
    },

    /// Roll normalized optimizer constraints up to selector-facing budget rows
    OptimizerConstraintBudget {
        /// Normalized optimizer constraint ledger CSV
        #[arg(
            long,
            default_value = "data/optimizer-constraint-ledger.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Output optimizer constraint budget CSV
        #[arg(
            long,
            short,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print per-subject budget rows
        #[arg(long)]
        details: bool,
        /// Fail if rollup rows violate selector-facing budget contract
        #[arg(long)]
        gate: bool,
    },

    /// Group remaining constraint-budget blockers into next optimizer wave families
    OptimizerResidualBlockerBacklog {
        /// Optimizer constraint budget CSV
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        budget: PathBuf,
        /// Output residual blocker backlog CSV
        #[arg(
            long,
            short,
            default_value = "data/optimizer-residual-blocker-backlog.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Print backlog rows
        #[arg(long)]
        details: bool,
        /// Fail if backlog rows reduce blockers or omit residual budget rows
        #[arg(long)]
        gate: bool,
    },

    /// Emit review docket for P1 residual optimizer claim blockers
    OptimizerClaimReview {
        /// Optimizer residual blocker backlog CSV
        #[arg(
            long,
            default_value = "data/optimizer-residual-blocker-backlog.csv",
            value_name = "FILE"
        )]
        backlog: PathBuf,
        /// Output optimizer claim review CSV
        #[arg(
            long,
            short,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows omit P1 claim blockers or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit scenario-level review rows for T2 game/ops publication blockers
    T2GamePublicationEvidenceReview {
        /// Optimizer claim review CSV
        #[arg(
            long,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        claim_review: PathBuf,
        /// T2 scenario hooks CSV
        #[arg(
            long,
            default_value = "data/game/t2-scenario-hooks.csv",
            value_name = "FILE"
        )]
        scenario_hooks: PathBuf,
        /// Output T2 game publication evidence review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-publication-evidence-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows omit scenario hooks or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T2 game/ops publication evidence blockers
    T2GamePublicationEvidencePolicy {
        /// T2 game publication evidence review CSV
        #[arg(
            long,
            default_value = "data/t2-game-publication-evidence-review.csv",
            value_name = "FILE"
        )]
        review: PathBuf,
        /// Output T2 game publication evidence policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-publication-evidence-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviews or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T2 game publication evidence policy
    T2GamePublicationEvidencePolicyAcceptance {
        /// T2 game publication evidence policy CSV
        #[arg(
            long,
            default_value = "data/t2-game-publication-evidence-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T2 game publication evidence policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-publication-evidence-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 game publication evidence blocker relief rows from accepted policy
    T2GamePublicationEvidenceBlockerRelief {
        /// T2 game publication evidence policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t2-game-publication-evidence-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T2 game publication evidence blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-publication-evidence-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-level review rows for T1 schematic-geometry claim blockers
    T1SchematicGeometryClaimReview {
        /// Optimizer claim review CSV
        #[arg(
            long,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        claim_review: PathBuf,
        /// T1 design review CSV
        #[arg(long, default_value = "data/t1-design-review.csv", value_name = "FILE")]
        design_review: PathBuf,
        /// T1 design policy action CSV
        #[arg(
            long,
            default_value = "data/t1-design-policy-actions.csv",
            value_name = "FILE"
        )]
        policy_actions: PathBuf,
        /// Output T1 schematic-geometry claim review CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-schematic-geometry-claim-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows omit overlap-review routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-level review rows for T2 Beck transfer-complexity claim blockers
    T2BeckTransferComplexityReview {
        /// Optimizer claim review CSV
        #[arg(
            long,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        claim_review: PathBuf,
        /// Output T2 Beck transfer-complexity review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-transfer-complexity-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows omit transfer-complexity routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-level review rows for T2 Beck label-density claim blockers
    T2BeckLabelDensityReview {
        /// Optimizer claim review CSV
        #[arg(
            long,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        claim_review: PathBuf,
        /// Output T2 Beck label-density review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-label-density-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows omit label-density routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-level review rows for T2 Beck long-connector claim blockers
    T2BeckLongConnectorReview {
        /// Optimizer claim review CSV
        #[arg(
            long,
            default_value = "data/optimizer-claim-review.csv",
            value_name = "FILE"
        )]
        claim_review: PathBuf,
        /// Output T2 Beck long-connector review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-long-connector-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows omit long-connector routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T2 Beck long-connector blockers
    T2BeckLongConnectorPolicy {
        /// T2 Beck long-connector review CSV
        #[arg(
            long,
            default_value = "data/t2-beck-long-connector-review.csv",
            value_name = "FILE"
        )]
        connector_review: PathBuf,
        /// Output T2 Beck long-connector policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-long-connector-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviewed routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T2 Beck long-connector policy
    T2BeckLongConnectorPolicyAcceptance {
        /// T2 Beck long-connector policy CSV
        #[arg(
            long,
            default_value = "data/t2-beck-long-connector-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T2 Beck long-connector policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-long-connector-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 Beck long-connector blocker relief rows from accepted policy
    T2BeckLongConnectorBlockerRelief {
        /// T2 Beck long-connector policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t2-beck-long-connector-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T2 Beck long-connector blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-long-connector-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T2 Beck label-density blockers
    T2BeckLabelDensityPolicy {
        /// T2 Beck label-density review CSV
        #[arg(
            long,
            default_value = "data/t2-beck-label-density-review.csv",
            value_name = "FILE"
        )]
        label_review: PathBuf,
        /// Output T2 Beck label-density policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-label-density-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviewed routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T2 Beck label-density policy
    T2BeckLabelDensityPolicyAcceptance {
        /// T2 Beck label-density policy CSV
        #[arg(
            long,
            default_value = "data/t2-beck-label-density-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T2 Beck label-density policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-label-density-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T2 Beck transfer-complexity blockers
    T2BeckTransferComplexityPolicy {
        /// T2 Beck transfer-complexity review CSV
        #[arg(
            long,
            default_value = "data/t2-beck-transfer-complexity-review.csv",
            value_name = "FILE"
        )]
        transfer_review: PathBuf,
        /// Output T2 Beck transfer-complexity policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-transfer-complexity-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviewed routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T2 Beck transfer-complexity policy
    T2BeckTransferComplexityPolicyAcceptance {
        /// T2 Beck transfer-complexity policy CSV
        #[arg(
            long,
            default_value = "data/t2-beck-transfer-complexity-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T2 Beck transfer-complexity policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-transfer-complexity-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit shared-segment map policy rows for T1 schematic geometry blockers
    T1SharedSegmentMapPolicy {
        /// T1 schematic-geometry claim review CSV
        #[arg(
            long,
            default_value = "data/t1-schematic-geometry-claim-review.csv",
            value_name = "FILE"
        )]
        schematic_review: PathBuf,
        /// Output T1 shared-segment map policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-shared-segment-map-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit route pairs or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T1 shared-segment map policy
    T1SharedSegmentPolicyAcceptance {
        /// T1 shared-segment map policy CSV
        #[arg(
            long,
            default_value = "data/t1-shared-segment-map-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T1 shared-segment policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-shared-segment-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T1 schematic-geometry blocker relief rows from accepted shared-segment policy
    T1SchematicGeometryBlockerRelief {
        /// T1 shared-segment policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t1-shared-segment-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T1 schematic-geometry blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t1-schematic-geometry-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 Beck transfer-complexity blocker relief rows from accepted policy
    T2BeckTransferComplexityBlockerRelief {
        /// T2 Beck transfer-complexity policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t2-beck-transfer-complexity-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T2 Beck transfer-complexity blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-transfer-complexity-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 Beck label-density blocker relief rows from accepted policy
    T2BeckLabelDensityBlockerRelief {
        /// T2 Beck label-density policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t2-beck-label-density-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T2 Beck label-density blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-beck-label-density-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit route-level review rows for T3 lower-tier feeder-gap claim blockers
    T3LowerTierFeederGapReview {
        /// Optimizer residual blocker backlog CSV
        #[arg(
            long,
            default_value = "data/optimizer-residual-blocker-backlog.csv",
            value_name = "FILE"
        )]
        backlog: PathBuf,
        /// T3/T4 access gap CSV
        #[arg(
            long,
            default_value = "data/t3-t4-access-gaps.csv",
            value_name = "FILE"
        )]
        access_gaps: PathBuf,
        /// Output T3 lower-tier feeder-gap review CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-lower-tier-feeder-gap-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if rows omit lower-tier feeder routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T3 lower-tier feeder-gap claim blockers
    T3LowerTierFeederGapPolicy {
        /// T3 lower-tier feeder-gap review CSV
        #[arg(
            long,
            default_value = "data/t3-lower-tier-feeder-gap-review.csv",
            value_name = "FILE"
        )]
        feeder_review: PathBuf,
        /// Output T3 lower-tier feeder-gap policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-lower-tier-feeder-gap-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviewed routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T3 lower-tier feeder-gap policy
    T3LowerTierFeederGapPolicyAcceptance {
        /// T3 lower-tier feeder-gap policy CSV
        #[arg(
            long,
            default_value = "data/t3-lower-tier-feeder-gap-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T3 lower-tier feeder-gap policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-lower-tier-feeder-gap-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policy routes or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T3 lower-tier feeder-gap blocker relief rows from accepted policy
    T3LowerTierFeederGapBlockerRelief {
        /// T3 lower-tier feeder-gap policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t3-lower-tier-feeder-gap-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T3 lower-tier feeder-gap blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t3-lower-tier-feeder-gap-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 game/ops bundle-binding blocker intake from constraint budget
    T2GameOpsBindingIntake {
        /// Optimizer constraint budget CSV
        #[arg(
            long,
            default_value = "data/optimizer-constraint-budget.csv",
            value_name = "FILE"
        )]
        budget: PathBuf,
        /// Output T2 game/ops binding intake CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-binding-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if intake rows do not represent T2 game/ops binding blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 game/ops bundle-binding decision docket
    T2GameOpsBindingDecisions {
        /// T2 game/ops binding intake CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-binding-intake.csv",
            value_name = "FILE"
        )]
        intake: PathBuf,
        /// T2 bundle overlays CSV
        #[arg(
            long,
            default_value = "data/game/t2-bundle-overlays.csv",
            value_name = "FILE"
        )]
        bundle_overlays: PathBuf,
        /// Output T2 game/ops binding decisions CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-binding-decisions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if decision rows do not preserve residual blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit repair targets for residual T2 bundle-overlay blockers
    T2BundleOverlayRepairTargets {
        /// T2 game/ops binding decisions CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-binding-decisions.csv",
            value_name = "FILE"
        )]
        decisions: PathBuf,
        /// T2 bundle overlays CSV
        #[arg(
            long,
            default_value = "data/game/t2-bundle-overlays.csv",
            value_name = "FILE"
        )]
        bundle_overlays: PathBuf,
        /// Output T2 bundle-overlay repair targets CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if residual decisions lack repair targets or classifications
        #[arg(long)]
        gate: bool,
    },

    /// Emit service-class repair docket for held T2 bundle-overlay rows
    T2ServiceClassRepairDocket {
        /// T2 bundle-overlay repair targets CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        targets: PathBuf,
        /// T2 service diagnostic queue CSV
        #[arg(
            long,
            default_value = "data/t2-service-diagnostic-queue.csv",
            value_name = "FILE"
        )]
        service_diagnostics: PathBuf,
        /// Output T2 service-class repair docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-service-class-repair-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if service-class-held rows lack repair routing
        #[arg(long)]
        gate: bool,
    },

    /// Review residual T2 game/ops bundle-binding rows against downstream repair evidence
    T2GameOpsBundleEvidenceReview {
        /// T2 game/ops binding decisions CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-binding-decisions.csv",
            value_name = "FILE"
        )]
        decisions: PathBuf,
        /// T2 bundle overlay repair targets CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        targets: PathBuf,
        /// T2 service-class repair docket CSV
        #[arg(
            long,
            default_value = "data/t2-service-class-repair-docket.csv",
            value_name = "FILE"
        )]
        service_docket: PathBuf,
        /// Output T2 game/ops bundle evidence review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-bundle-evidence-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if review rows do not preserve residual blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit policy rows for T2 game/ops bundle-binding evidence holds
    T2GameOpsBundleEvidencePolicy {
        /// T2 game/ops bundle evidence review CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-bundle-evidence-review.csv",
            value_name = "FILE"
        )]
        review: PathBuf,
        /// Output T2 game/ops bundle evidence policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-bundle-evidence-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows omit reviews or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit acceptance rows for authored T2 game/ops bundle evidence policy
    T2GameOpsBundleEvidencePolicyAcceptance {
        /// T2 game/ops bundle evidence policy CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-bundle-evidence-policy.csv",
            value_name = "FILE"
        )]
        policy: PathBuf,
        /// Output T2 game/ops bundle evidence policy acceptance CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-bundle-evidence-policy-acceptance.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acceptance rows omit policies or reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 game/ops bundle evidence blocker relief rows from accepted policy
    T2GameOpsBundleEvidenceBlockerRelief {
        /// T2 game/ops bundle evidence policy acceptance CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-bundle-evidence-policy-acceptance.csv",
            value_name = "FILE"
        )]
        acceptance: PathBuf,
        /// Output T2 game/ops bundle evidence blocker relief CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-game-ops-bundle-evidence-blocker-relief.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if relief rows omit accepted policies or do not reduce blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit diagnostic decisions for T2 service-overlay repair rows
    T2ServiceOverlayDiagnosticDecisions {
        /// T2 service-class repair docket CSV
        #[arg(
            long,
            default_value = "data/t2-service-class-repair-docket.csv",
            value_name = "FILE"
        )]
        service_docket: PathBuf,
        /// T2 bundle-overlay repair targets CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        targets: PathBuf,
        /// T2 service diagnostic queue CSV
        #[arg(
            long,
            default_value = "data/t2-service-diagnostic-queue.csv",
            value_name = "FILE"
        )]
        service_diagnostics: PathBuf,
        /// Output T2 service overlay diagnostic decisions CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-service-overlay-diagnostic-decisions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if service-overlay repair rows are not explicitly held or repaired
        #[arg(long)]
        gate: bool,
    },

    /// Emit T2 local-zone overlay handoff decisions
    T2LocalZoneOverlayHandoff {
        /// T2 service-class repair docket CSV
        #[arg(
            long,
            default_value = "data/t2-service-class-repair-docket.csv",
            value_name = "FILE"
        )]
        service_docket: PathBuf,
        /// T3 zone route columns CSV
        #[arg(
            long,
            default_value = "data/t3-zone-route-columns.csv",
            value_name = "FILE"
        )]
        zone_route_columns: PathBuf,
        /// T3 zone render board CSV
        #[arg(
            long,
            default_value = "data/t3-zone-render-board.csv",
            value_name = "FILE"
        )]
        zone_render_board: PathBuf,
        /// Output T2 local-zone overlay handoff CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-local-zone-overlay-handoff.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if local-zone rows are missing handoff decisions or promoted
        #[arg(long)]
        gate: bool,
    },

    /// Emit readiness disposition for T2 bundle-overlay rows needing bundle repair
    T2BundleReadinessDisposition {
        /// T2 bundle-overlay repair targets CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        targets: PathBuf,
        /// Output T2 bundle readiness disposition CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-readiness-disposition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if bundle-readiness blockers are not explicitly disposed
        #[arg(long)]
        gate: bool,
    },

    /// Emit repair docket for T2 bundle-readiness rows needing structural repair
    T2BundleReadinessRepairDocket {
        /// T2 bundle readiness disposition CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-readiness-disposition.csv",
            value_name = "FILE"
        )]
        readiness: PathBuf,
        /// Output T2 bundle readiness repair docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-readiness-repair-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if repair-needed readiness rows are not explicitly docketed
        #[arg(long)]
        gate: bool,
    },

    /// Emit evidence probe for T2 bundle-readiness repair tasks
    T2BundleReadinessRepairEvidence {
        /// T2 bundle readiness repair docket CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-readiness-repair-docket.csv",
            value_name = "FILE"
        )]
        repair_docket: PathBuf,
        /// National segment registry CSV
        #[arg(
            long,
            default_value = "data/national-segment-registry.csv",
            value_name = "FILE"
        )]
        registry: PathBuf,
        /// Tier segment candidates CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segment_candidates: PathBuf,
        /// T2 service selection CSV
        #[arg(
            long,
            default_value = "data/t2-service-selection.csv",
            value_name = "FILE"
        )]
        service_selection: PathBuf,
        /// Output T2 bundle readiness repair evidence CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-readiness-repair-evidence.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if repair tasks are missing evidence probe rows or promoted
        #[arg(long)]
        gate: bool,
    },

    /// Emit replay decisions for T2 bundle-readiness repair evidence
    T2BundleReadinessReplayDecisions {
        /// T2 bundle readiness repair evidence CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-readiness-repair-evidence.csv",
            value_name = "FILE"
        )]
        evidence: PathBuf,
        /// T2 bundle overlay repair delta CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-delta.csv",
            value_name = "FILE"
        )]
        repair_delta: PathBuf,
        /// Output T2 bundle readiness replay decisions CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-readiness-replay-decisions.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if evidence replay rows promote readiness or lose blockers
        #[arg(long)]
        gate: bool,
    },

    /// Audit T2 readiness replay decisions against national segment bundles
    T2NationalBundleReadinessAudit {
        /// T2 bundle readiness replay decisions CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-readiness-replay-decisions.csv",
            value_name = "FILE"
        )]
        replay_decisions: PathBuf,
        /// National segment bundles CSV
        #[arg(
            long,
            default_value = "data/national-segment-bundles.csv",
            value_name = "FILE"
        )]
        bundles: PathBuf,
        /// Output T2 national bundle readiness audit CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-national-bundle-readiness-audit.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if audit rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit stitched-member registry handoff rows for T2 readiness repair
    T2StitchedMemberRegistryHandoff {
        /// T2 national bundle readiness audit CSV
        #[arg(
            long,
            default_value = "data/t2-national-bundle-readiness-audit.csv",
            value_name = "FILE"
        )]
        audit: PathBuf,
        /// National segment registry CSV
        #[arg(
            long,
            default_value = "data/national-segment-registry.csv",
            value_name = "FILE"
        )]
        registry: PathBuf,
        /// Tier segment candidates CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segment_candidates: PathBuf,
        /// Output T2 stitched-member registry handoff CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-registry-handoff.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if handoff rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Review stitched-member route candidate scope before bundle repair
    T2StitchedMemberCandidateScopeReview {
        /// T2 stitched-member registry handoff CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-registry-handoff.csv",
            value_name = "FILE"
        )]
        handoff: PathBuf,
        /// Tier segment candidates CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segment_candidates: PathBuf,
        /// Output T2 stitched-member candidate scope review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-candidate-scope-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if scope review rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit split/merge/expand decisions for T2 stitched-member scope reviews
    T2StitchedMemberDecisionDocket {
        /// T2 stitched-member candidate scope review CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-candidate-scope-review.csv",
            value_name = "FILE"
        )]
        scope_review: PathBuf,
        /// Output T2 stitched-member decision docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-decision-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if decision rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit state-scoped split plan rows for T2 stitched-member decisions
    T2StitchedMemberSplitPlan {
        /// T2 stitched-member decision docket CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-decision-docket.csv",
            value_name = "FILE"
        )]
        decision_docket: PathBuf,
        /// Tier segment candidates CSV
        #[arg(
            long,
            default_value = "data/tier-segment-candidates.csv",
            value_name = "FILE"
        )]
        segment_candidates: PathBuf,
        /// Output T2 stitched-member split plan CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-split-plan.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if split rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit selection requirements for T2 stitched-member split-plan rows
    T2StitchedMemberSelectionDocket {
        /// T2 stitched-member split plan CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-split-plan.csv",
            value_name = "FILE"
        )]
        split_plan: PathBuf,
        /// Output T2 stitched-member selection docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-selection-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if selection rows promote readiness or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof requirements for T2 stitched-member selection rows
    T2StitchedMemberEvidenceContract {
        /// T2 stitched-member selection docket CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-selection-docket.csv",
            value_name = "FILE"
        )]
        selection_docket: PathBuf,
        /// Output T2 stitched-member evidence contract CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-evidence-contract.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if contract rows satisfy evidence or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit source-acquisition targets for T2 stitched-member proof contracts
    T2StitchedMemberEvidenceAcquisition {
        /// T2 stitched-member evidence contract CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-evidence-contract.csv",
            value_name = "FILE"
        )]
        evidence_contract: PathBuf,
        /// Output T2 stitched-member evidence acquisition CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-evidence-acquisition.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if acquisition rows satisfy evidence or lose claim blockers
        #[arg(long)]
        gate: bool,
    },

    /// Emit source-access policy rows for T2 stitched-member acquisition targets
    T2StitchedMemberSourceAccessPolicy {
        /// T2 stitched-member evidence acquisition CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-evidence-acquisition.csv",
            value_name = "FILE"
        )]
        evidence_acquisition: PathBuf,
        /// Output T2 stitched-member source-access policy CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-source-access-policy.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if policy rows enable live fetch or satisfy evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof-intake rows for T2 stitched-member source-access policy rows
    T2StitchedMemberProofIntake {
        /// T2 stitched-member source-access policy CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-source-access-policy.csv",
            value_name = "FILE"
        )]
        source_access: PathBuf,
        /// Output T2 stitched-member proof intake CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-proof-intake.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if proof-intake rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit source-capture rows for T2 stitched-member proof-intake rows
    T2StitchedMemberProofSourceCapture {
        /// T2 stitched-member proof intake CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-proof-intake.csv",
            value_name = "FILE"
        )]
        proof_intake: PathBuf,
        /// Output T2 stitched-member proof source-capture CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-proof-source-capture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if source-capture rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit artifact-attachment rows for T2 stitched-member source-capture rows
    T2StitchedMemberProofArtifactAttachment {
        /// T2 stitched-member proof source-capture CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-proof-source-capture.csv",
            value_name = "FILE"
        )]
        source_capture: PathBuf,
        /// Output T2 stitched-member proof artifact-attachment CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-proof-artifact-attachment.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if artifact-attachment rows attach or accept evidence
        #[arg(long)]
        gate: bool,
    },

    /// Emit proof-review rows for T2 stitched-member artifact-attachment rows
    T2StitchedMemberProofReviewDocket {
        /// T2 stitched-member proof artifact-attachment CSV
        #[arg(
            long,
            default_value = "data/t2-stitched-member-proof-artifact-attachment.csv",
            value_name = "FILE"
        )]
        artifact_attachment: PathBuf,
        /// Output T2 stitched-member proof review docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-stitched-member-proof-review-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if proof-review rows accept proof or leave optimizer hold routing
        #[arg(long)]
        gate: bool,
    },

    /// Emit blocker delta after T2 bundle-overlay repair dockets
    T2BundleOverlayRepairDelta {
        /// T2 game/ops binding decisions CSV
        #[arg(
            long,
            default_value = "data/t2-game-ops-binding-decisions.csv",
            value_name = "FILE"
        )]
        decisions: PathBuf,
        /// T2 bundle-overlay repair targets CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-targets.csv",
            value_name = "FILE"
        )]
        targets: PathBuf,
        /// T2 service-class repair docket CSV
        #[arg(
            long,
            default_value = "data/t2-service-class-repair-docket.csv",
            value_name = "FILE"
        )]
        service_docket: PathBuf,
        /// T2 bundle readiness disposition CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-readiness-disposition.csv",
            value_name = "FILE"
        )]
        readiness: PathBuf,
        /// Output T2 bundle-overlay repair delta CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-bundle-overlay-repair-delta.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if residual blocker accounting loses held claims
        #[arg(long)]
        gate: bool,
    },

    /// Emit optimizer action rows for residual T2 overlay repair deltas
    T2OverlayOptimizerActionDocket {
        /// T2 bundle-overlay repair delta CSV
        #[arg(
            long,
            default_value = "data/t2-bundle-overlay-repair-delta.csv",
            value_name = "FILE"
        )]
        repair_delta: PathBuf,
        /// Output T2 overlay optimizer action docket CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-overlay-optimizer-action-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if action rows reduce blockers or promote claims
        #[arg(long)]
        gate: bool,
    },

    /// Emit P1 structural-readiness review rows for T2 overlay optimizer actions
    T2OverlayP1StructuralReadinessReview {
        /// T2 overlay optimizer action docket CSV
        #[arg(
            long,
            default_value = "data/t2-overlay-optimizer-action-docket.csv",
            value_name = "FILE"
        )]
        action_docket: PathBuf,
        /// Output T2 overlay P1 structural-readiness review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-overlay-p1-structural-readiness-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if P1 review rows reduce blockers or promote claims
        #[arg(long)]
        gate: bool,
    },

    /// Emit P2 service-overlay review rows for T2 overlay optimizer actions
    T2OverlayP2ServiceOverlayReview {
        /// T2 overlay optimizer action docket CSV
        #[arg(
            long,
            default_value = "data/t2-overlay-optimizer-action-docket.csv",
            value_name = "FILE"
        )]
        action_docket: PathBuf,
        /// Output T2 overlay P2 service-overlay review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-overlay-p2-service-overlay-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if P2 review rows reduce blockers or promote claims
        #[arg(long)]
        gate: bool,
    },

    /// Emit P3 local-zone overlay review rows for T2 overlay optimizer actions
    T2OverlayP3LocalZoneOverlayReview {
        /// T2 overlay optimizer action docket CSV
        #[arg(
            long,
            default_value = "data/t2-overlay-optimizer-action-docket.csv",
            value_name = "FILE"
        )]
        action_docket: PathBuf,
        /// Output T2 overlay P3 local-zone overlay review CSV
        #[arg(
            long,
            short,
            default_value = "data/t2-overlay-p3-local-zone-overlay-review.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if P3 review rows reduce blockers or promote claims
        #[arg(long)]
        gate: bool,
    },

    /// Link optimizer outputs to map atlas and game overlay consumers
    OptimizerMapHooks {
        /// Output optimizer map hook CSV
        #[arg(
            long,
            short,
            default_value = "data/optimizer-map-hooks.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any optimizer map/game consumer artifact is missing
        #[arg(long)]
        gate: bool,
    },

    /// Verify crate-level bundle-facing API adoption
    BundleArchitecture {
        /// Output bundle architecture adoption CSV
        #[arg(
            long,
            short,
            default_value = "data/bundle-architecture.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Fail if any crate lacks its required bundle-facing API surface
        #[arg(long)]
        gate: bool,
    },

    /// Review route endpoint exception records for tier promotion/demotion decisions
    EndpointExceptions {
        /// Path to endpoint exception ledger CSV
        #[arg(
            long,
            default_value = "data/tier-node-exceptions.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Filter by requested tier, e.g. T2
        #[arg(long)]
        tier: Option<String>,
        /// Filter by route, e.g. I-65
        #[arg(long)]
        route: Option<String>,
        /// Show only incomplete or non-terminal-worthy exception rows
        #[arg(long)]
        blockers: bool,
        /// Print full artifact and next-step fields
        #[arg(long)]
        details: bool,
        /// Fail if exception rows are incomplete; with --blockers, also fail non-terminal rows
        #[arg(long)]
        gate: bool,
    },

    /// Evaluate proposed stop investments and endpoint-worthy nodes by service class
    StopCandidates {
        /// Path to stop investment candidate ledger CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Filter by stop class, e.g. S1, S2, S3
        #[arg(long = "class")]
        stop_class: Option<String>,
        /// Filter to candidates touching this route, e.g. I-95
        #[arg(long)]
        route: Option<String>,
        /// Print full evidence and next-step fields
        #[arg(long)]
        details: bool,
        /// Fail if candidate rows are not reviewable for their requested class
        #[arg(long)]
        gate: bool,
    },

    /// Show a route-ordered stop chain for a corridor map or schematic line
    StopPlan {
        /// Route to inspect, e.g. I-5
        route: String,
        /// Path to stop investment candidate ledger CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Print full evidence and next-step fields
        #[arg(long)]
        details: bool,
        /// Fail if the route has no S1/S2 endpoints or fewer than three stops
        #[arg(long)]
        gate: bool,
    },

    /// Check which routes in a tier have enough candidate stops for schematic use
    StopCoverage {
        /// Path to generated tier table CSV
        #[arg(long, default_value = "data/tier-table.csv", value_name = "FILE")]
        tier_table: PathBuf,
        /// Path to stop investment candidate ledger CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Tier to inspect
        #[arg(long, default_value = "T1")]
        tier: String,
        /// Show only routes whose stop plans fail the route-level gate
        #[arg(long)]
        blockers: bool,
        /// Fail if any route in the tier lacks a viable stop plan
        #[arg(long)]
        gate: bool,
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

    /// Relay hub outage sensitivity — missed swaps and absorption under hub disruption
    HubOutage {
        /// Include proposed hubs from missing link corridors
        #[arg(long)]
        include_proposed: bool,
        /// Hub outage duration in hours
        #[arg(long, default_value_t = 8.0)]
        outage_hours: f64,
        /// Reserve-driver fraction available for immediate absorption
        #[arg(long, default_value_t = 0.15)]
        reserve_driver_fraction: f64,
        /// Remaining disrupted swaps adjacent hubs can absorb
        #[arg(long, default_value_t = 0.35)]
        adjacent_absorption_fraction: f64,
    },

    /// EV charging analysis — guaranteed DCFC every 50mi enables overnight AV travel
    EvAnalysis,

    /// EV/rest-area outage sensitivity — charger outage queues and range-buffer viability
    EvRestOutage {
        /// Fraction of planned charging/rest stops disrupted
        #[arg(long, default_value_t = 0.20)]
        outage_station_fraction: f64,
        /// Fraction of disrupted stations covered by backup power or mobile charging
        #[arg(long, default_value_t = 0.50)]
        backup_power_fraction: f64,
        /// Added queue/dwell minutes per disrupted unbacked stop
        #[arg(long, default_value_t = 45.0)]
        queue_delay_minutes: f64,
    },

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

    /// Generate stop-to-stop SLA surface from the Beck T1/T2 topology
    StopSlaSurface {
        /// Output CSV file
        #[arg(
            long,
            short,
            default_value = "data/beck-stop-sla.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
    },

    /// Summarize the stop-to-stop SLA surface and expose worst stop gaps
    StopSlaSummary {
        /// SLA surface CSV file
        #[arg(long, default_value = "data/beck-stop-sla.csv", value_name = "FILE")]
        input: PathBuf,
        /// Number of worst gaps to show
        #[arg(long, default_value_t = 12)]
        top: usize,
        /// Fail if any stop pair exceeds this max stop gap
        #[arg(long, value_name = "MILES")]
        gate_max_gap: Option<f64>,
    },

    /// Recommend stop/intersection candidates for recurring oversized SLA gaps
    StopSlaCandidates {
        /// SLA surface CSV file
        #[arg(long, default_value = "data/beck-stop-sla.csv", value_name = "FILE")]
        input: PathBuf,
        /// Path to stop investment candidate ledger CSV
        #[arg(
            long,
            default_value = "data/tier-stop-candidates.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// City seed list used for draft midpoint candidates when the stop ledger is silent
        #[arg(long, default_value = "data/cities.json", value_name = "FILE")]
        cities: PathBuf,
        /// Only recommend for recurring gaps above this mileage
        #[arg(long, default_value_t = 300.0, value_name = "MILES")]
        target_gap: f64,
        /// Number of recurring gap segments to inspect
        #[arg(long, default_value_t = 12)]
        top: usize,
        /// Number of candidate stops to show per gap
        #[arg(long, default_value_t = 3)]
        candidates_per_gap: usize,
        /// Write the recommendation docket to CSV
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Fail if any inspected oversized gap has no generated candidate
        #[arg(long)]
        gate: bool,
        /// Fail if any inspected oversized gap still falls back to an algorithmic midpoint
        #[arg(long)]
        gate_no_algorithmic: bool,
    },

    /// Convert SLA candidate docket rows into tier-stop candidate review scaffolds
    StopSlaPromotions {
        /// SLA candidate docket CSV
        #[arg(
            long,
            default_value = "data/beck-stop-sla-candidates.csv",
            value_name = "FILE"
        )]
        input: PathBuf,
        /// Output tier-stop-candidates-shaped CSV
        #[arg(
            long,
            default_value = "data/beck-stop-sla-promotion-docket.csv",
            value_name = "FILE"
        )]
        output: PathBuf,
        /// Include rows that already exist in the stop ledger
        #[arg(long)]
        include_ledger: bool,
        /// Include non-rank-1 alternate candidates
        #[arg(long)]
        include_alternates: bool,
        /// Fail if generated promotion rows do not pass the stop candidate contract
        #[arg(long)]
        gate: bool,
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
pub enum InterventionCorridorArg {
    #[value(name = "ny-la")]
    NyLa,
    #[value(name = "hou-chi")]
    HouChi,
    #[value(name = "hou-i69")]
    HouI69,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum OdCorridorCmd {
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
pub enum GapType {
    MissingLink,
    Bottleneck,
    Resilience,
    Intermodal,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum TierRegionGraphArg {
    /// Routes are vertices; shared stops/transfers/overlaps are edges.
    DualRoute,
}

impl TierRegionGraphArg {
    pub fn service_graph_kind(&self) -> route_network::ServiceGraphKind {
        match self {
            Self::DualRoute => route_network::ServiceGraphKind::DualRouteGraph,
        }
    }
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum SimMode {
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

#[derive(clap::Subcommand, Clone, Debug)]
pub enum GameCommand {
    /// List playable Interstate Tycoon scenarios
    Scenarios,
    /// Show the map-backed Interstate Tycoon campaign spine
    Campaign {
        /// Path to campaign spine CSV
        #[arg(
            long,
            default_value = "data/game/campaign-spine.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to map atlas manifest CSV
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Fail if the campaign spine is incomplete or references missing map ids
        #[arg(long)]
        gate: bool,
    },
    /// Show game levers for T2 service classes and verify their map/standards links
    T2Overlays {
        /// Path to T2 service overlay CSV
        #[arg(
            long,
            default_value = "data/game/t2-service-overlays.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to T2 service standards CSV
        #[arg(
            long,
            default_value = "data/beck-t2-service-standards.csv",
            value_name = "FILE"
        )]
        standards: PathBuf,
        /// Path to map atlas manifest CSV
        #[arg(long, default_value = "data/map-atlas.csv", value_name = "FILE")]
        map_atlas: PathBuf,
        /// Fail if an overlay is incomplete or disconnected from standards/atlas ids
        #[arg(long)]
        gate: bool,
    },
    /// Show campaign scenario hooks that consume T2 service overlays
    T2Hooks {
        /// Path to T2 scenario hook CSV
        #[arg(
            long,
            default_value = "data/game/t2-scenario-hooks.csv",
            value_name = "FILE"
        )]
        ledger: PathBuf,
        /// Path to campaign spine CSV
        #[arg(
            long,
            default_value = "data/game/campaign-spine.csv",
            value_name = "FILE"
        )]
        campaign: PathBuf,
        /// Path to T2 service overlay CSV
        #[arg(
            long,
            default_value = "data/game/t2-service-overlays.csv",
            value_name = "FILE"
        )]
        overlays: PathBuf,
        /// Fail if T2 campaign scenarios lack a hook or reference unknown overlay classes
        #[arg(long)]
        gate: bool,
    },
    /// Print setup, cards, gates, and engine hooks for a scenario
    Inspect {
        /// Scenario id, e.g. des-moines-diamond
        scenario: String,
    },
    /// Resolve one deterministic game season
    RunSeason {
        /// Scenario id, e.g. des-moines-diamond
        scenario: String,
        /// Season number
        #[arg(long)]
        season: u8,
        /// Event card slug
        #[arg(long)]
        event: String,
        /// Project card slug; repeatable
        #[arg(long)]
        project: Vec<String>,
        /// Optional prior JSON state
        #[arg(long, value_name = "FILE")]
        state: Option<PathBuf>,
        /// Optional output JSON state
        #[arg(long, value_name = "FILE")]
        write_state: Option<PathBuf>,
        /// Optional append-only CSV session log
        #[arg(long, value_name = "FILE")]
        append_log: Option<PathBuf>,
    },
    /// Score an append-only game session log
    Score {
        /// Scenario id, e.g. des-moines-diamond
        scenario: String,
        /// CSV session log written by run-season
        #[arg(long, value_name = "FILE")]
        log: PathBuf,
        /// Print dimension-by-dimension scoring
        #[arg(long)]
        details: bool,
        /// Fail if promotion gates are not met
        #[arg(long)]
        gate_promotion: bool,
    },
}
