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
        /// Filter to interstate-only routes (default: true)
        #[arg(long, default_value_t = true)]
        interstate_only: bool,
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

    // Load manifest
    let manifest_path = cli.manifest
        .unwrap_or_else(route_data::Manifest::default_path);

    match cli.command {
        Commands::Fetch { force, year } => {
            println!("route fetch — year {year}");
            let manifest = route_data::Manifest::load(&manifest_path)
                .context("loading manifest")?;
            route_data::fetch::fetch_all(&manifest, force)?;
            println!("fetch complete.");
        }

        Commands::Build { interstate_only } => {
            println!("route build — reading NHS shapefile…");
            // TODO: load from manifest cache path; extract .shp from .zip
            println!("  [stub] NHS shapefile parsing not yet wired to manifest paths.");
            println!("  run: cargo build, then manually provide shp path via route build --shp <path>");
        }

        Commands::Score { designation, estimated, proposed } => {
            println!("route score {designation}");
            // TODO: load cached graph, extract corridor, aggregate attributes, score
            println!("  [stub] scoring engine wired — graph cache loading in progress.");
            let _ = (estimated, proposed, scoring_cfg);
        }

        Commands::ScoreAll { workers } => {
            println!("route score-all");
            let w = workers.unwrap_or_else(num_cpus);
            println!("  [stub] will score all corridors with {w} workers.");
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
