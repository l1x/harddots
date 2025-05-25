use std::fs;
use std::path::Path;
use tracing::trace;

use crate::error::HarddotsError;

pub fn create_hardlink(source: &str, target: &str) -> Result<(), HarddotsError> {
    trace!("Creating hardlink from {} to {}", source, target);
    let source_path = Path::new(source);
    let target_path = Path::new(target);

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
        trace!("Created parent directory {}", parent.display());
    }

    // Check if target exists and is the same as source
    if target_path.exists() {
        if let Ok(existing) = fs::read_link(target_path) {
            if existing == source_path {
                trace!("Hardlink from {} to {} already exists", source, target);
                return Ok(());
            }
        }
        // Remove existing file if it's not a valid hardlink
        fs::remove_file(target_path)?;
        trace!("Removed existing file at {}", target);
    }

    // Create hardlink
    fs::hard_link(source_path, target_path)?;
    trace!(
        "Successfully created hardlink from {} to {}",
        source, target
    );
    Ok(())
}
