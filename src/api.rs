use async_trait::async_trait;

use crate::handlers::task::NotionCaller;

pub struct Notion {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl Notion {
    pub fn new(base_url: String, token: String) -> Result<Notion, anyhow::Error> {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;

        return Ok(Notion {
            base_url,
            client,
            token,
        });
    }
}

#[async_trait]
impl NotionCaller for Notion {
    async fn list_databases(&self) -> Vec<String> {
        let res = self
            .client
            .post(format!("{}/{}", self.base_url, "v1/search"))
            .bearer_auth(self.token)
            .send()
            .await;
    }
}
