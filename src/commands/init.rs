use std::path::Path;
use tracing::trace;

use crate::config::HarddotsConfig;
use crate::error::HarddotsError;
use crate::git::clone_repo;

pub fn run(config: &HarddotsConfig) -> Result<(), HarddotsError> {
    trace!(
        "Running init command with git_repo: {} and cache_dir: {:?}",
        config.git_repo, config.cache_dir
    );

    // Expand cache_dir path (handles ~)
    let cache_dir =
        shellexpand::tilde(&config.cache_dir.as_deref().unwrap_or("~/.cache/harddots")).to_string();
    trace!("Expanded cache_dir: {}", cache_dir);

    // Check if cache_dir exists and is a valid Git repository
    let cache_path = Path::new(&cache_dir);
    if cache_path.exists() && cache_path.join(".git").exists() {
        trace!(
            "Cache directory {} already contains a Git repository, skipping clone",
            cache_dir
        );
        println!("Repository already initialized at {}", cache_dir);
        return Ok(());
    }

    // Clone the repository
    trace!("Cloning repository {} to {}", config.git_repo, cache_dir);
    clone_repo(&config.git_repo, &cache_dir)?;
    println!("Successfully cloned repository to {}", cache_dir);

    Ok(())
}
