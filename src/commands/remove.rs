use tracing::{info, trace};

use crate::config::HarddotsConfig;
use crate::error::HarddotsError;

pub fn run(config: &mut HarddotsConfig, name: &str, force: bool) -> Result<(), HarddotsError> {
    trace!(
        "Running remove command for application: {}, force: {}",
        name, force
    );
    trace!("Arguments: name={}, force={}", name, force);

    // Remove specified application from config
    let applications = &mut config.applications;
    let original_applications_len = applications.len();
    if let Some(index) = applications
        .iter()
        .position(|application| application.name == name)
    {
        applications.remove(index);
    };

    // Write back to harddots.toml if anything being removed
    let removed = applications.len() != original_applications_len;
    if removed {
        info!("Updating harddots.toml");
        HarddotsConfig::save(config, "harddots.toml")?;
        trace!("Successfully wrote updated configuration to harddots.toml");
    }

    println!("Removed application '{}' from harddots.toml", name);
    Ok(())
}
