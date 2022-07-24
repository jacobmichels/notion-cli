use std::{
    fmt::write,
    fs::{self, File},
    io::Write,
    path::{self, PathBuf},
};

use crate::cli::InitHandler;

pub struct Init {}

impl InitHandler for Init {
    // creates a config file ~/.notion-cli/config.json and populates it with the database_id to use
    // maybe should be refactored eventually for testability and to optionally use a wizard to find the correct db
    fn init(&self, database_id: &String) -> anyhow::Result<()> {
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
