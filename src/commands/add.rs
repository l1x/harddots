use std::fs::{self, OpenOptions};
use std::io::Write;
use serde::{Deserialize, Serialize};
use toml::Value;
use tracing::{error, info, trace};

use crate::config::HarddotsConfig;
use crate::error::HarddotsError;

#[derive(Serialize, Deserialize)]
struct Application {
    name: String,
    target_path: String,
    source_git_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    packages: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    custom_install: Option<std::collections::HashMap<String, String>>,
}

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
        target_path, source_git_path, macos_pkg, debian_pkg, alpine_pkg
    );

    // Read existing harddots.toml
    let config_path = "harddots.toml";
    info!("Reading existing configuration from {}", config_path);
    let mut toml_content = fs::read_to_string(config_path)?;

    // Parse TOML into a Value for manipulation
    let mut toml_value: Value = toml::from_str(&toml_content)?;

    // Check for duplicate application name
    if let Some(applications) = toml_value.get("applications").and_then(|v| v.as_array()) {
        if applications
            .iter()
            .any(|app| app.get("name").and_then(|n| n.as_str()) == Some(&name))
        {
            error!("Application '{}' already exists in harddots.toml", name);
            return Err(HarddotsError::Other(format!(
                "Application '{}' already exists",
                name
            )));
        }
    }

    // Create new application entry
    let mut packages = std::collections::HashMap::new();
    if let Some(pkg) = macos_pkg {
        packages.insert("macos".to_string(), pkg);
    }
    if let Some(pkg) = debian_pkg {
        packages.insert("debian".to_string(), pkg);
    }
    if let Some(pkg) = alpine_pkg {
        packages.insert("alpine".to_string(), pkg);
    }

    let new_app = Application {
        name,
        target_path,
        source_git_path,
        version: None,
        packages,
        custom_install: None,
    };

    // Add new application to TOML
    let new_app_value = toml::Value::try_from(&new_app)?;
    toml_value
        .as_table_mut()
        .unwrap()
        .entry("applications")
        .or_insert_with(|| Value::Array(vec![]))
        .as_array_mut()
        .unwrap()
        .push(new_app_value);

    // Serialize back to TOML
    toml_content = toml::to_string_pretty(&toml_value)?;
    trace!("Updated TOML content: {}", toml_content);

    // Write back to harddots.toml
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path)?;
    file.write_all(toml_content.as_bytes())?;
    trace!(
        "Successfully wrote updated configuration to {}",
        config_path
    );

    println!("Added application '{}' to harddots.toml", new_app.name);
    Ok(())
}
