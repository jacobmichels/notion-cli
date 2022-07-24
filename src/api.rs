use reqwest::blocking::Client;

use crate::handlers::task::NotionCaller;

/// Notion API wrapper
pub struct Notion {
    /// Notion's base url
    base_url: String,
    /// Notion integration token
    token: String,
    /// HTTP client
    client: Client,
}

impl Notion {
    /// Construct a new Notion object provided a base_url and token
    /// Can panic if: TLS backend cannot be initialized, or the resolver cannot load the system configuration
    pub fn new(base_url: String, token: String) -> Notion {
        let client = reqwest::blocking::ClientBuilder::new()
            .https_only(true)
            .build()
            .expect("http client configuration failed");

        return Notion {
            base_url,
            client,
            token,
        };
    }
}

impl NotionCaller for Notion {
    fn list_tasks(&self) -> Result<Vec<crate::handlers::task::Task>, anyhow::Error> {
        unimplemented!()
    }
}
