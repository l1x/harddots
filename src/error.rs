use thiserror::Error;

#[derive(Error, Debug)]
pub enum HarddotsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("{0}")]
    Other(String),
}

impl From<&str> for HarddotsError {
    fn from(s: &str) -> Self {
        HarddotsError::Other(s.to_string())
    }
}