use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{models::config::AppConfig, traits::ConfigService};
use anyhow::{bail, Result};

pub struct JSONConfigService {}

impl JSONConfigService {
    pub fn new() -> JSONConfigService {
        JSONConfigService {}
    }

    fn get_config_dir(&self) -> Result<PathBuf> {
        let mut config_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                bail!("failed to get home_dir for this platform");
            }
        };
        config_dir.push(".notion-cli");

        return Ok(config_dir);
    }
}

impl ConfigService for JSONConfigService {
    fn get_config(&self) -> Result<AppConfig> {
        let config_dir = self.get_config_dir()?;
        let config_path = config_dir.join("config.json");

        let exists = Path::new(&config_path).exists();
        if exists {
            let file = File::open(config_path)?;
            let cfg: AppConfig = serde_json::from_reader(file)?;
            return Ok(cfg);
        }

        return Ok(AppConfig::default());
    }

    fn set_config(&self, config: AppConfig) -> Result<()> {
        let config_dir = self.get_config_dir()?;
        let config_path = config_dir.join("config.json");

        fs::create_dir_all(config_dir)?;
        let mut file = File::create(config_path)?;
        let json_str = serde_json::to_string_pretty(&config)?;
        file.write_all(json_str.as_bytes())?;

        return Ok(());
    }
}
