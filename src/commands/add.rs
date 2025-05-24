use tracing::trace;
use crate::config::HarddotsConfig;
use crate::error::HarddotsError;

pub fn run(
    config: &HarddotsConfig,
    name: String,
    target_path: String,
    source_git_path: String,
    macos_pkg: Option<String>,
    debian_pkg: Option<String>,
    alpine_pkg: Option<String>,
) -> Result<(), HarddotsError> {
    trace!("Running add command for application: {}", name);
    trace!(
        "Arguments: target_path={}, source_git_path={}, macos_pkg={:?}, debian_pkg={:?}, alpine_pkg={:?}",
        target_path,
        source_git_path,
        macos_pkg,
        debian_pkg,
        alpine_pkg
    );
    println!("add command not implemented yet");
    Ok(())
}