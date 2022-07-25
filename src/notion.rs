use std::str::FromStr;

use reqwest::{blocking::Client, Url};

use crate::{task::Task, traits};

/// Notion API wrapper
pub struct NotionAPI {
    /// Notion's base url
    base_url: Url,
    /// Notion integration token
    token: String,
    /// HTTP client
    client: Client,
}

impl NotionAPI {
    /// Construct a new Notion object provided a base_url and token
    /// Can panic if: TLS backend cannot be initialized, or the resolver cannot load the system configuration
    pub fn new(base_url: String, token: String) -> anyhow::Result<NotionAPI> {
        let client = reqwest::blocking::ClientBuilder::new()
            .https_only(true)
            .build()?;

        let base_url = Url::from_str(&base_url)?;

        return Ok(NotionAPI {
            base_url,
            client,
            token,
        });
    }
}

impl traits::NotionCaller for NotionAPI {
    fn list_tasks(&self, database_id: String) -> Result<Vec<Task>, anyhow::Error> {
        unimplemented!()
    }
}
