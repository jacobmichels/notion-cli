use reqwest::blocking::Client;

use crate::{task::Task, traits};

/// Notion API wrapper
pub struct NotionAPI {
    /// Notion's base url
    base_url: String,
    /// Notion integration token
    token: String,
    /// HTTP client
    client: Client,
}

impl NotionAPI {
    /// Construct a new Notion object provided a base_url and token
    /// Can panic if: TLS backend cannot be initialized, or the resolver cannot load the system configuration
    pub fn new(base_url: String, token: String) -> NotionAPI {
        let client = reqwest::blocking::ClientBuilder::new()
            .https_only(true)
            .build()
            .expect("http client configuration failed");

        return NotionAPI {
            base_url,
            client,
            token,
        };
    }
}

impl traits::NotionCaller for NotionAPI {
    fn list_tasks(&self, database_id: String) -> Result<Vec<Task>, anyhow::Error> {
        unimplemented!()
    }
}
