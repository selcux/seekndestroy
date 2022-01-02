use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Could not read config file.")]
    ConfigFile(#[source] std::io::Error),
    #[error("Could not read config content: {0}")]
    ConfigContent(String),
    #[error("Item {0} not found.")]
    ItemNotFound(String),
    #[error("Invalit regex pattern.")]
    Pattern(#[source] regex::Error),
    #[error("Current process id not found: {0}")]
    CurrentPid(String),
    #[error("Config directory not found")]
    ConfigDir,
}
