use tracing::info;

use crate::config::HarddotsConfig;
use crate::error::HarddotsError;

pub fn run(config: &HarddotsConfig) -> Result<(), HarddotsError> {
    info!("Initializing Harddots with configuration: {:?}", config);
    info!("init command not implemented yet");
    Ok(())
}