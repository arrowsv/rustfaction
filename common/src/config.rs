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
    pub fn load() {
        let path = Self::path();

        let config = match Self::load_from_file(&path) {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to load config from {}: {}", path.display(), e);
                let default = Self::default();
                default.save().unwrap_or_else(|e| {
                    log::error!("Failed to save default config: {}", e);
                });
                default
            }
        };

        *CONFIG.write().unwrap() = config;
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::ensure_config_dir()?;

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    pub fn get() -> Self {
        CONFIG.read().unwrap().clone()
    }

    pub fn set(new_config: Self) {
        let mut config = CONFIG.write().unwrap();
        *config = new_config;
        if let Err(e) = config.save() {
            log::error!("Failed to save config: {}", e);
        }
    }

    fn ensure_config_dir() -> Result<PathBuf> {
        let dir = Self::path().parent()
            .context("Config file has no parent directory")?
            .to_path_buf();

        std::fs::create_dir_all(&dir)
            .context("Failed to create config directory")?;

        Ok(dir)
    }

    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"))
            .join("Rust Faction")
            .join("config.toml")
    }
}