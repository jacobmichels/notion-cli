use std::str::FromStr;

use anyhow::bail;
use reqwest::{blocking::Client, Url};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    task::{Task, TaskStatus},
    traits,
};

/// The notion version this app was built to work with
const NOTION_VERSION: &str = "2022-02-22";

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

/// Response from calling the endpoint /v1/databases/:id/query
#[derive(Deserialize, Debug)]
struct DatabaseQueryResponse {
    /// Either "list" or "error"
    object: String,
    /// list of pages with the database
    results: Vec<Page>,
}

#[derive(Deserialize, Debug, Clone)]
/// A notion page
struct Page {
    /// The ID of the page
    id: String,
    /// Page properties
    /// Went with a weakly typed value here to avoid making a bunch of structs
    properties: Value,
}

impl traits::NotionCaller for NotionAPI {
    fn list_tasks(
        &self,
        database_id: String,
        status: &Option<TaskStatus>,
    ) -> Result<Vec<Task>, anyhow::Error> {
        let pages = self.get_pages_from_db(database_id, status)?;

        let mut tasks: Vec<Task> = Vec::with_capacity(pages.len());

        // turn each page into a task by extracting specific information
        // could refactor this to use the From<T> trait
        for page in &pages {
            let task_title = page.properties["Name"]["title"][0]["text"]["content"]
                .as_str()
                .expect("no title for page");

            let status: anyhow::Result<TaskStatus> = page.try_into();
            if status.is_err() {
                continue;
            }
            tasks.push(Task::new(
                page.id.clone(),
                status.unwrap(),
                task_title.to_string(),
            ));
        }

        return Ok(tasks);
    }

    fn add_task(
        &self,
        database_id: String,
        title: &str,
        status: &TaskStatus,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }
}

impl NotionAPI {
    /// hit the notion API and return a list of pages found in the db
    /// optionally filters the on TaskStatus
    fn get_pages_from_db(
        &self,
        database_id: String,
        status: &Option<TaskStatus>,
    ) -> anyhow::Result<Vec<Page>> {
        let url = self
            .base_url
            .join(&format!("/v1/databases/{}/query", &database_id))?;

        let mut request = self
            .client
            .post(url)
            .header("Notion-Version", NOTION_VERSION)
            .bearer_auth(&self.token);

        if let Some(s) = status {
            let filter = s.as_status_filter();
            let payload = Some(json!({
                "filter":{
                    "property":"Status",
                    "select":{
                        "equals":filter
                    }
                }
            }));

            request = request.json(&payload);
        }

        let response = request.send()?.error_for_status()?;

        let body: DatabaseQueryResponse = response.json()?;

        if body.object != "list" {
            return Err(anyhow::Error::msg("Response was not a list of pages"));
        }

        return Ok(body.results);
    }
}

impl TryFrom<&Page> for TaskStatus {
    type Error = anyhow::Error;

    fn try_from(page: &Page) -> Result<Self, Self::Error> {
        let status_text = page.properties["Status"]["select"]["name"].as_str();

        return match status_text {
            Some("To Do") => Ok(TaskStatus::Todo),
            Some("Doing") => Ok(TaskStatus::Doing),
            Some("Done") => Ok(TaskStatus::Done),
            None => bail!("status text not found in page"),
            _ => bail!("unknown status"),
        };
    }
}
