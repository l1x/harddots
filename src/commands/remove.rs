use crate::config::HarddotsConfig;
use crate::error::HarddotsError;
use tracing::trace;

pub fn run(config: &HarddotsConfig, name: &str, force: bool) -> Result<(), HarddotsError> {
    trace!(
        "Running remove command for application: {}, force: {}",
        name, force
    );
    println!("remove command not implemented yet");
    Ok(())
}
