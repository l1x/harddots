use clap::{Parser, Subcommand};
use tracing::error;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod config;
mod error;
mod filesystem;
mod git;
mod host;
mod package;

use commands::add;
use commands::deploy;
use commands::init;
use commands::remove;
use commands::status;
use commands::update;
use config::HarddotsConfig;
use error::HarddotsError;

#[derive(Parser)]
#[command(
    name = "harddots",
    about = "A personalized dotfile manager for idempotent deployment across Unix-like systems"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the dotfiles repository by cloning it to the cache directory
    Init,

    /// Deploy one or all applications' configurations
    Deploy {
        /// Application name to deploy (or "all" for all applications)
        #[arg(default_value = "all")]
        application: String,

        /// Simulate deployment without making changes
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// Update configurations by pulling the latest from the Git repository
    Update,

    /// Display the status of managed applications
    Status,

    /// Add a new application to harddots.toml
    Add {
        /// Application name
        name: String,

        /// Target path for the configuration file (e.g., ~/.config/starship.toml)
        target_path: String,

        /// Source path in the Git repository (e.g., starship/starship.toml)
        source_git_path: String,

        /// Package name for macOS (optional)
        #[arg(long)]
        macos_pkg: Option<String>,

        /// Package name for Debian (optional)
        #[arg(long)]
        debian_pkg: Option<String>,

        /// Package name for Alpine (optional)
        #[arg(long)]
        alpine_pkg: Option<String>,
    },

    /// Remove an application from management
    Remove {
        /// Application name to remove
        name: String,

        /// Skip confirmation prompts
        #[arg(long, default_value_t = false)]
        force: bool,
    },
}

fn main() -> Result<(), HarddotsError> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration from harddots.toml
    let config_path = "harddots.toml";
    let config = match HarddotsConfig::load(config_path) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration from {}: {}", config_path, e);
            return Err(e);
        }
    };

    // Dispatch to the appropriate command
    match cli.command {
        Commands::Init => init::run(&config),
        Commands::Deploy {
            application,
            dry_run,
        } => deploy::run(&config, &application, dry_run),
        Commands::Update => update::run(&config),
        Commands::Status => status::run(&config),
        Commands::Add {
            name,
            target_path,
            source_git_path,
            macos_pkg,
            debian_pkg,
            alpine_pkg,
        } => add::run(
            &config,
            name,
            target_path,
            source_git_path,
            macos_pkg,
            debian_pkg,
            alpine_pkg,
        ),
        Commands::Remove { name, force } => remove::run(&config, &name, force),
    }
}
