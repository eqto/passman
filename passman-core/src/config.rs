use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("config directory not found")]
    DirNotFound,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VaultConfig {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub vaults: Vec<VaultConfig>,
}

pub fn config_dir() -> Result<PathBuf, ConfigError> {
    dirs::config_dir()
        .map(|d| d.join("passman"))
        .ok_or(ConfigError::DirNotFound)
}

pub fn config_path() -> Result<PathBuf, ConfigError> {
    let dir = config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir.join("vaults.json"))
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(path)?;
    let config: AppConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<(), ConfigError> {
    let path = config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn add_vault(id: &str, name: &str, path: &str) -> Result<(), ConfigError> {
    with_config(|config| {
        config.vaults.push(VaultConfig {
            id: id.to_string(),
            name: name.to_string(),
            path: path.to_string(),
        });
    })
}

pub fn remove_vault(id: &str) -> Result<(), ConfigError> {
    with_config(|config| {
        config.vaults.retain(|v| v.id != id);
    })
}

pub fn update_vault(id: &str, name: &str, path: &str) -> Result<(), ConfigError> {
    with_config(|config| {
        if let Some(vault) = config.vaults.iter_mut().find(|v| v.id == id) {
            vault.name = name.to_string();
            vault.path = path.to_string();
        }
    })
}

fn with_config<F: FnOnce(&mut AppConfig)>(f: F) -> Result<(), ConfigError> {
    let mut config = load_config()?;
    f(&mut config);
    save_config(&config)
}
