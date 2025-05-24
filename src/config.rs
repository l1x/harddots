use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct HarddotsConfig {
    pub(crate) git_repo: String,
    pub(crate) cache_dir: Option<String>,
    pub(crate) applications: Vec<Application>,
}

#[derive(Deserialize, Debug, Clone)]
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
}