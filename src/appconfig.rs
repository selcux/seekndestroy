use std::collections::HashMap;

use crate::error::AppError;
use directories::BaseDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub process: HashMap<String, String>,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, AppError> {
        let base_dirs = BaseDirs::new().ok_or(AppError::ConfigDir)?;
        let config_dir = base_dirs.config_dir();

        let content =
            std::fs::read_to_string(config_dir.join("snd.toml")).map_err(AppError::ConfigFile)?;

        let conf: AppConfig =
            toml::from_str(&content).map_err(|e| AppError::ConfigContent(e.to_string()))?;
        Ok(conf)
    }
}
