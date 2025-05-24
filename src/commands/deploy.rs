use tracing::info;

use crate::config::{Application, HarddotsConfig};
use crate::error::HarddotsError;

pub fn run(
    config: &HarddotsConfig,
    application: &str,
    dry_run: bool,
) -> Result<(), HarddotsError> {
    info!(
        "Initializing Harddots with configuration: {:?} Application: {:?}  Dry run: {:?}",
        config, application, dry_run
    );
    info!("init command not implemented yet");
    Ok(())
}
