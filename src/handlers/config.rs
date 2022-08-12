use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Ok, Result};
use colour::red_ln;
use serde::{Deserialize, Serialize};

use crate::traits::{ConfigHandler, NotionCaller};

/// A ConfigHandler that persists the task database_id as json
pub struct JSONConfigHandler {
    /// the notion client used to list eligible databases
    notion: Box<dyn NotionCaller>,
}

impl JSONConfigHandler {
    /// Create a new JSONConfigHandler
    pub fn new(notion: Box<dyn NotionCaller>) -> JSONConfigHandler {
        return JSONConfigHandler { notion };
    }

    /// Gets the config directory
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

    fn read_config(&self) -> Result<AppConfig> {
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

    fn write_config(&self, cfg: AppConfig) -> Result<()> {
        let config_dir = self.get_config_dir()?;
        let config_path = config_dir.join("config.json");

        fs::create_dir_all(config_dir)?;
        let mut file = File::create(config_path)?;
        let json_str = serde_json::to_string_pretty(&cfg)?;
        file.write_all(json_str.as_bytes())?;

        return Ok(());
    }
}

#[derive(Deserialize, Serialize, Default)]
struct AppConfig {
    database_id: String,
    token: String,
}

impl ConfigHandler for JSONConfigHandler {
    // creates a config file ~/.notion-cli/config.json and populates it with the database_id to use
    // maybe should be refactored eventually for testability and to optionally use a wizard to find the correct db
    fn set_database(&self, database_id: &str) -> Result<()> {
        let mut config = self.read_config()?;
        config.database_id = database_id.to_string();
        self.write_config(config)?;

        return Ok(());
    }

    fn get_database_id(&self) -> Result<String> {
        let config = self.read_config()?;
        if config.database_id.is_empty() {
            bail!("No database ID set");
        } else {
            return Ok(config.database_id);
        }
    }

    fn print_eligible_databases(&self) -> Result<()> {
        let databases = self.notion.list_eligible_databases()?;

        if databases.is_empty() {
            red_ln!("No eligible databases found");
            return Ok(());
        }

        red_ln!("Eligible databases -----------------------------------------------------");
        for (i, database) in databases.iter().enumerate() {
            database.print(i);
        }
        red_ln!("------------------------------------------------------------------------");

        return Ok(());
    }

    fn set_token(&self, token: &str) -> Result<()> {
        let mut config = self.read_config()?;
        config.token = token.to_string();
        self.write_config(config)?;

        return Ok(());
    }
}
