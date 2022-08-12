use anyhow::{bail, Ok, Result};
use colour::{green, red_ln};

use crate::traits::{ConfigCommandHandler, ConfigService, NotionCaller};

/// A ConfigHandler that persists the task database_id as json
pub struct JSONConfigHandler {
    /// the notion client used to list eligible databases
    notion: Box<dyn NotionCaller>,
    config: Box<dyn ConfigService>,
}

impl JSONConfigHandler {
    /// Create a new JSONConfigHandler
    pub fn new(notion: Box<dyn NotionCaller>, config: Box<dyn ConfigService>) -> JSONConfigHandler {
        return JSONConfigHandler { notion, config };
    }
}

impl ConfigCommandHandler for JSONConfigHandler {
    // creates a config file ~/.notion-cli/config.json and populates it with the database_id to use
    // maybe should be refactored eventually for testability and to optionally use a wizard to find the correct db
    fn set_database(&self, database_id: &str) -> Result<()> {
        let mut config = self.config.get_config()?;
        config.database_id = database_id.to_string();
        self.config.set_config(config)?;

        return Ok(());
    }

    fn get_database_id(&self) -> Result<()> {
        let config = self.config.get_config()?;
        if config.database_id.is_empty() {
            bail!("No database ID set");
        } else {
            green!("Database ID: ");
            red_ln!("{}", config.database_id);
            Ok(())
        }
    }

    fn list_databases(&self) -> Result<()> {
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
        let mut config = self.config.get_config()?;
        config.token = token.to_string();
        self.config.set_config(config)?;

        return Ok(());
    }
}
