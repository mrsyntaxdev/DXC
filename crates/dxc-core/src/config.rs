use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxcConfig {
    pub download_path: String,
    pub cache_path: String,
    pub db_path: String,
    pub max_concurrent_downloads: u32,
}

impl Default for DxcConfig {
    fn default() -> Self {
        Self {
            download_path: dirs::downloads(),
            cache_path: dirs::cache(),
            db_path: dirs::database(),
            max_concurrent_downloads: 3,
        }
    }
}

impl DxcConfig {
    pub fn load() -> Self {
        let path = dirs::config_file();
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = dirs::config_file();
        if let Some(parent) = PathBuf::from(&path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "download_path" => self.download_path = value.to_string(),
            "cache_path" => self.cache_path = value.to_string(),
            "db_path" => self.db_path = value.to_string(),
            "max_concurrent_downloads" => {
                self.max_concurrent_downloads = value.parse().map_err(|_| "Invalid number".to_string())?;
            }
            _ => return Err(format!("Unknown config key: {key}")),
        }
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "download_path" => Some(self.download_path.clone()),
            "cache_path" => Some(self.cache_path.clone()),
            "db_path" => Some(self.db_path.clone()),
            "max_concurrent_downloads" => Some(self.max_concurrent_downloads.to_string()),
            _ => None,
        }
    }
}

pub fn ensure_dirs() -> Result<(), String> {
    let dirs = [
        dirs::config_dir(),
        dirs::cache(),
        dirs::downloads(),
    ];
    for d in &dirs {
        std::fs::create_dir_all(d).map_err(|e| format!("Failed to create {d}: {e}"))?;
    }
    Ok(())
}

pub mod dirs {
    fn home() -> String {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string())
    }

    pub fn config_dir() -> String {
        format!("{}/.config/dxc", home())
    }

    pub fn config_file() -> String {
        format!("{}/config.toml", config_dir())
    }

    pub fn database() -> String {
        format!("{}/database.db", config_dir())
    }

    pub fn cache() -> String {
        format!("{}/.cache/dxc", home())
    }

    pub fn downloads() -> String {
        format!("{}/Downloads/DXC", home())
    }
}
