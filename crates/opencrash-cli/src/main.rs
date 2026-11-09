//! CLI entry point for `opencrash`.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use opencrash_core::IntersectionHazardBrief;
use opencrash_pack::PackHeader;

#[derive(Debug, Parser)]
#[command(
    name = "opencrash",
    version,
    about = "Free, open-source, self-hosted traffic collision, intersection hazard & police jurisdiction engine in pure Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Download NHTSA FARS / SWITRS / OSM extracts and compile a single memory-mapped `.pack` file.
    Build {
        /// Jurisdiction region slug (e.g., `us/california`, `us/all`).
        #[arg(short, long, default_value = "us/california")]
        region: String,
        /// Output `.pack` file path.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Generate a 5-year Dangerous Intersection Hazard Brief and responsible police agency for coordinates.
    Hazard {
        /// Latitude in decimal degrees.
        #[arg(long)]
        lat: f64,
        /// Longitude in decimal degrees.
        #[arg(long)]
        lon: f64,
        /// Search radius in meters.
        #[arg(short, long, default_value_t = 150)]
        radius: u32,
    },
    /// Serve the stateless HTTP & MCP API from a memory-mapped `.pack` file.
    Serve {
        /// Path or name of the `.pack` file to serve.
        pack: String,
        /// Bind address and port.
        #[arg(short, long, default_value = "127.0.0.1:8083")]
        bind: String,
        /// Also expose a Model Context Protocol (MCP) stdio/SSE endpoint.
        #[arg(long)]
        mcp: bool,
    },
    /// Batch-enrich a CSV of collision leads or locations with hazard percentiles and police report agencies.
    Batch {
        /// Input CSV path.
        input: PathBuf,
    },
    /// Display header metadata and section offsets for a `.pack` file.
    Info {
        /// Path to the `.pack` file.
        pack: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { region, output } => {
            let out = output
                .unwrap_or_else(|| PathBuf::from(format!("{}.pack", region.replace('/', "-"))));
            let header = PackHeader::new(&region, 0, 0);
            println!(
                "Initialized OpenCrash Pack v{} builder for region '{}' -> {}",
                header.version,
                header.region,
                out.display()
            );
        }
        Commands::Hazard { lat, lon, radius } => {
            let demo = IntersectionHazardBrief {
                primary_road: format!("LAT {lat:.4}"),
                secondary_road: format!("LON {lon:.4}"),
                radius_meters: radius,
                injury_collisions_5yr: 0,
                fatal_collisions_5yr: 0,
                severe_injury_collisions_5yr: 0,
                county_hazard_percentile: 0.0,
                reporting_agency: "load_pack_to_resolve".to_owned(),
            };
            println!("{}", serde_json::to_string_pretty(&demo)?);
        }
        Commands::Serve { pack, bind, mcp } => {
            println!("Serving OpenCrash Pack '{pack}' on http://{bind} (mcp={mcp})");
        }
        Commands::Batch { input } => {
            println!(
                "Batch crash hazard & TCR agency enrichment queued for {}",
                input.display()
            );
        }
        Commands::Info { pack } => {
            println!("Inspecting OpenCrash Pack at {}", pack.display());
        }
    }
    Ok(())
}
