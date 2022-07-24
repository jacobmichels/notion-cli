use std::{
    fs::{self, File},
    io::Write,
};

use crate::cli::InitHandler;

/// An InitHandler that persists the task database_id as json
pub struct JSONInitHandler {}

impl JSONInitHandler {
    /// Create a new JSONInitHandler
    pub fn new() -> JSONInitHandler {
        return JSONInitHandler {};
    }
}

impl InitHandler for JSONInitHandler {
    // creates a config file ~/.notion-cli/config.json and populates it with the database_id to use
    // maybe should be refactored eventually for testability and to optionally use a wizard to find the correct db
    fn init(&self, database_id: &str) -> anyhow::Result<()> {
        let cfg = serde_json::json!({ "database": database_id });

        let mut config_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                return Err(anyhow::Error::msg(
                    "failed to get home_dir for this platform",
                ))
            }
        };
        config_dir.push(".notion-cli");

        fs::create_dir_all(&config_dir)?;

        // config file name probably shouldn't be hardcoded here
        let mut file = File::create(config_dir.join("config.json"))?;

        write!(&mut file, "{}", cfg)?;

        return Ok(());
    }
}
