use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum HarddotsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("{0}")]
    Other(String),
}

impl From<&str> for HarddotsError {
    fn from(s: &str) -> Self {
        HarddotsError::Other(s.to_string())
    }
}