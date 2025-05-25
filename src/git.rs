use std::path::Path;
use std::process::Command;
use tracing::{error, trace};

use crate::error::HarddotsError;

pub fn clone_repo(url: &str, cache_dir: &str) -> Result<(), HarddotsError> {
    trace!("Preparing to clone repository {} to {}", url, cache_dir);

    // Check if cache_dir already contains a Git repository
    let cache_path = Path::new(cache_dir);
    if cache_path.exists() && cache_path.join(".git").exists() {
        trace!("Repository already exists at {}, checking if it matches {}", cache_dir, url);
        let current_url = get_remote_url(cache_dir)?;
        if current_url.trim() == url.trim() {
            trace!("Existing repository matches URL {}, skipping clone", url);
            return Ok(());
        } else {
            error!("Cache directory {} contains a different repository (remote: {}), cannot clone {}", cache_dir, current_url, url);
            return Err(HarddotsError::Other(format!(
                "Cache directory {} contains a different repository (remote: {})",
                cache_dir, current_url
            )));
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
        trace!("Created parent directory {}", parent.display());
    }

    // Clone the repository
    trace!("Executing git clone {} {}", url, cache_dir);
    let status = Command::new("git")
        .args(["clone", url, cache_dir])
        .status()?;

    if status.success() {
        trace!("Successfully cloned repository {} to {}", url, cache_dir);
        Ok(())
    } else {
        let error_msg = format!("Failed to clone repository {} to {}: git exited with {}", url, cache_dir, status);
        error!("{}", error_msg);
        Err(HarddotsError::Other(error_msg))
    }
}

fn get_remote_url(cache_dir: &str) -> Result<String, HarddotsError> {
    trace!("Checking remote URL for repository at {}", cache_dir);
    let output = Command::new("git")
        .args(["-C", cache_dir, "remote", "get-url", "origin"])
        .output()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).to_string();
        trace!("Remote URL: {}", url);
        Ok(url)
    } else {
        let error_msg = format!("Failed to get remote URL for {}: {}", cache_dir, String::from_utf8_lossy(&output.stderr));
        error!("{}", error_msg);
        Err(HarddotsError::Other(error_msg))
    }
}