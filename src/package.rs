use std::process::Command;
use tracing::{error, trace};

use crate::error::HarddotsError;
use crate::host::Host;

pub fn is_package_installed(host: &Host, package: &str) -> Result<bool, HarddotsError> {
    trace!("Checking if package '{}' is installed on {:?}", package, host.os_type);
    let status = match host.os_type {
        crate::host::OsType::MacOS => Command::new("brew")
            .args(["info", package])
            .status()?,
        crate::host::OsType::Debian => Command::new("dpkg")
            .args(["-l", package])
            .status()?,
        crate::host::OsType::Alpine => Command::new("apk")
            .args(["info", "-e", package])
            .status()?,
        crate::host::OsType::Unknown => {
            error!("Cannot check package status on unknown OS");
            return Err(HarddotsError::Other("Unsupported OS".to_string()));
        }
    };

    Ok(status.success())
}

pub fn install_package(host: &Host, package: &str) -> Result<(), HarddotsError> {
    trace!("Installing package '{}' on {:?}", package, host.os_type);
    if let Some(cmd) = host.package_manager_cmd() {
        let cmd_parts: Vec<&str> = cmd.split_whitespace().collect();
        let status = if host.root_cmd.is_empty() {
            Command::new(cmd_parts[0])
                .args(&cmd_parts[1..])
                .arg(package)
                .status()?
        } else {
            Command::new(&host.root_cmd)
                .arg(cmd_parts[0])
                .args(&cmd_parts[1..])
                .arg(package)
                .status()?
        };

        if status.success() {
            trace!("Successfully installed package '{}'", package);
            Ok(())
        } else {
            let error_msg = format!("Failed to install package '{}'", package);
            error!("{}", error_msg);
            Err(HarddotsError::Other(error_msg))
        }
    } else {
        error!("No package manager available for {:?}", host.os_type);
        Err(HarddotsError::Other("Unsupported OS".to_string()))
    }
}