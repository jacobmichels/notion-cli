use anyhow::Ok;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::handlers::task::NotionCaller;

pub struct Notion {
    base_url: String,
    token: String,
    client: Client,
}

impl Notion {
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

    fn list_database_ids(&self) -> Result<Vec<String>, anyhow::Error> {
        let res = self
            .client
            .post(format!("{}/{}", self.base_url, "v1/search"))
            .header("Accept", "application/json")
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token)
            .send()?;

        let json = res.json::<SearchResponse>()?;

        if json.object != "list" {
            return Err(anyhow::Error::msg(format!(
                "format type is \"{}\" when it needs to be \"list\"",
                json.object
            )));
        }

        let mut databases = Vec::new();

        for obj in json.results {
            if obj.object == "database" {
                databases.push(obj)
            }
        }

        return Ok(databases
            .iter()
            .map(|db| {
                return db.into();
            })
            .collect());
    }

    // fn list_tasks_in_db(
    //     &self,
    //     database_id: String,
    // ) -> Result<Vec<crate::handlers::task::Task>, anyhow::Error> {
    //     self.client.
    //    }
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    object: String,
    results: Vec<SearchResponseObject>,
}

#[derive(Deserialize, Debug)]
struct SearchResponseObject {
    object: String,
    id: String,
}

impl From<&SearchResponseObject> for String {
    fn from(obj: &SearchResponseObject) -> Self {
        return obj.id.to_string();
    }
}

impl NotionCaller for Notion {
    fn list_tasks(&self) -> Result<Vec<crate::handlers::task::Task>, anyhow::Error> {
        unimplemented!()
    }
}
