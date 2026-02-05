use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub start_hour: u8,        // Start time (0-23)
    pub end_hour: u8,          // End time (0-23)
    pub interval_minutes: u32, // Notification interval in minutes
    pub reminder_minutes: u32, // Delay before "did you drink?" reminder
}

impl Default for Config {
    fn default() -> Self {
        Config {
            start_hour: 9,
            end_hour: 18,
            interval_minutes: 30,
            reminder_minutes: 5,
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("hydration-rustminder");
        path.push("config.yaml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        info!("Loading config from {:?}", path);

        if let Ok(content) = fs::read_to_string(&path) {
            match serde_yaml::from_str(&content) {
                Ok(config) => {
                    info!("Config loaded successfully");
                    config
                }
                Err(e) => {
                    warn!("Failed to parse config: {}, using defaults", e);
                    Config::default()
                }
            }
        } else {
            info!("No config file found, using defaults");
            Config::default()
        }
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        info!("Saving config to {:?}", path);

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_yaml::to_string(self)?;
        fs::write(&path, content)?;

        info!("Config saved successfully");
        Ok(())
    }
}
