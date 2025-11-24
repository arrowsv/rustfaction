use std::{path::{Path, PathBuf}, sync::RwLock};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub game_directory: String,
    pub fps_limit: u32,
    pub use_overrides: bool,
    pub fast_start: bool,
    pub keep_launcher_open: bool,
    pub show_console: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            game_directory: String::new(),
            fps_limit: 120,
            use_overrides: true,
            fast_start: true,
            keep_launcher_open: false,
            show_console: false,
        }
    }
}

impl Config {
    pub fn init() {
        *CONFIG.write().unwrap() = Self::load_from_file(&Self::path()).unwrap_or_else(|_| {
            let default = Self::default();
            default.save().unwrap_or_else(|e| {
                log::error!("Failed to save default config: {}", e);
            });
            default
        })
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&Self::path(), content)
            .context("Failed to write config file")?;

        Ok(())
    }

    pub fn get() -> Self {
        CONFIG.read().unwrap().clone()
    }

    pub fn set(new_config: Self) {
        let mut config = CONFIG.write().unwrap();
        *config = new_config;
        config.save().unwrap_or_else(|e| {
            log::error!("Failed to save updated config: {}", e);
        })
    }

    fn path() -> PathBuf {
        let config_file = dirs::config_dir().unwrap().join("Rust Faction/config.toml");
        let config_file_dir = config_file.parent().expect("Config file should have parent");
        
        if !config_file_dir.exists() {
            std::fs::create_dir_all(&config_file_dir).unwrap_or_else(|e| {
                log::error!("Failed to create config directory ({}): {}", config_file_dir.display(), e)
            });
        }

        config_file
    }
}