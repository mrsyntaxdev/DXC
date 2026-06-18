use thiserror::Error;

#[derive(Error, Debug)]
pub enum DxcError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Provider not found for URL: {0}")]
    ProviderNotFound(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("{0}")]
    Other(String),
}
