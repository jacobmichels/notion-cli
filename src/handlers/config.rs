use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::PathBuf,
};

use serde_json::Value;

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
    fn get_config_dir(&self) -> anyhow::Result<PathBuf> {
        let mut config_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                return Err(anyhow::Error::msg(
                    "failed to get home_dir for this platform",
                ))
            }
        };
        config_dir.push(".notion-cli");

        return Ok(config_dir);
    }
}

impl ConfigHandler for JSONConfigHandler {
    // creates a config file ~/.notion-cli/config.json and populates it with the database_id to use
    // maybe should be refactored eventually for testability and to optionally use a wizard to find the correct db
    fn set_database(&self, database_id: &str) -> anyhow::Result<()> {
        let cfg = serde_json::json!({ "database": database_id });

        let config_dir = self.get_config_dir()?;

        fs::create_dir_all(&config_dir)?;

        // config file name probably shouldn't be hardcoded here
        let mut file = File::create(config_dir.join("config.json"))?;

        write!(&mut file, "{}", cfg)?;

        return Ok(());
    }

    fn get_database_id(&self) -> anyhow::Result<String> {
        let config_dir = self.get_config_dir()?;

        let file = fs::File::open(config_dir.join("config.json"))?;
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader)?;
        return Ok(json["database"]
            .as_str()
            .expect("malformed config file")
            .to_string());
    }

    fn list_eligible_databases(&self) -> anyhow::Result<()> {
        let databases = self.notion.list_databases();

        return Ok(());
    }
}
