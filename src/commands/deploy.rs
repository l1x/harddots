use std::path::Path;
use tracing::{error, info, trace};

use crate::config::{Application, HarddotsConfig};
use crate::error::HarddotsError;
use crate::filesystem::create_hardlink;
use crate::host::Host;
use crate::package::{install_package, is_package_installed};

pub fn run(config: &HarddotsConfig, application: &str, dry_run: bool) -> Result<(), HarddotsError> {
    info!(
        "Deploying with configuration: {:?} Application: {} Dry run: {}",
        config, application, dry_run
    );

    // Detect host OS
    let host = Host::detect();
    trace!("Detected host: {:?}", host);

    // Select applications to deploy
    let apps_to_deploy: Vec<&Application> = if application == "all" {
        config.applications.iter().collect()
    } else {
        match config
            .applications
            .iter()
            .find(|app| app.name == application)
        {
            Some(app) => vec![app],
            None => {
                error!("Application '{}' not found in harddots.toml", application);
                return Err(HarddotsError::Other(format!(
                    "Application '{}' not found",
                    application
                )));
            }
        }
    };

    for app in apps_to_deploy {
        trace!("Processing application: {}", app.name);

        // Check and install package
        if let Some(pkg) = app.packages.get(&host.os_type.to_string()) {
            trace!("Checking package '{}' for {}", pkg, app.name);
            if !is_package_installed(&host, pkg)? {
                if dry_run {
                    info!("Dry run: Would install package '{}' for {}", pkg, app.name);
                } else {
                    info!("Installing package '{}' for {}", pkg, app.name);
                    install_package(&host, pkg)?;
                }
            } else {
                trace!("Package '{}' already installed for {}", pkg, app.name);
            }
        } else {
            trace!(
                "No package specified for {} on {:?}",
                app.name, host.os_type
            );
        }

        // Create hardlink
        let source_path = format!(
            "{}/{}",
            shellexpand::tilde(&config.cache_dir.as_deref().unwrap_or("~/.cache/harddots")),
            app.source_git_path
        );
        let target_path = shellexpand::tilde(&app.target_path).to_string();
        trace!("Preparing to link: {} -> {}", source_path, target_path);

        if !Path::new(&source_path).exists() {
            error!("Source file {} does not exist in cache", source_path);
            return Err(HarddotsError::Other(format!(
                "Source file {} not found",
                source_path
            )));
        }

        if dry_run {
            info!(
                "Dry run: Would create hardlink from {} to {} for {}",
                source_path, target_path, app.name
            );
        } else {
            info!(
                "Creating hardlink from {} to {} for {}",
                source_path, target_path, app.name
            );
            create_hardlink(&source_path, &target_path)?;
        }
    }

    if dry_run {
        println!("Dry run completed, no changes made.");
    } else {
        println!("Successfully deployed {}", application);
    }

    Ok(())
}
