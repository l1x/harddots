use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct HarddotsConfig {
    pub(crate) git_repo: String,
    pub(crate) cache_dir: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) applications: Vec<Application>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Application {
    pub(crate) name: String,
    pub(crate) target_path: String,
    pub(crate) source_git_path: String,
    pub(crate) version: Option<String>,
    pub(crate) packages: std::collections::HashMap<String, String>,
    pub(crate) custom_install: Option<std::collections::HashMap<String, String>>,
}

impl HarddotsConfig {
    pub fn load(path: &str) -> Result<Self, crate::error::HarddotsError> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(config: &HarddotsConfig, path: &str) -> Result<(), crate::error::HarddotsError> {
        let toml_content = toml::to_string(config)?;
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        file.write_all(toml_content.as_bytes())?;
        Ok(())
    }
}
