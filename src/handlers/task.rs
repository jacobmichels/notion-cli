use anyhow::Ok;

use crate::{
    notion::NotionAPI,
    task::{Task, TaskStatus},
    traits::{NotionCaller, TaskHandler},
};

/// A task handler that wraps a Notion client
pub struct NotionAPITaskHandler {
    /// The Notion client
    pub notion: Box<dyn NotionCaller>,
}

impl NotionAPITaskHandler {
    /// Construct a new NotionTaskHandler given a Notion API client
    pub fn new(notion: NotionAPI) -> NotionAPITaskHandler {
        return NotionAPITaskHandler {
            notion: Box::new(notion),
        };
    }
}

impl TaskHandler for NotionAPITaskHandler {
    fn add(&self, name: &[String], status: &TaskStatus) -> Result<(), anyhow::Error> {
        // convert the vec to a string, trimming the last space
        let name: String = name.iter().map(|s| s.to_string() + " ").collect::<String>();
        let name = name.trim_end();

        println!(
            "Add subcommand called. name = {:?} status = {}. This should return a link to the task it just created.",
            name,status
        );

        return Ok(());
    }

    fn list(
        &self,
        status: &Option<TaskStatus>,
        database_id: String,
    ) -> Result<Vec<Task>, anyhow::Error> {
        return self.notion.list_tasks(database_id);
    }

    fn done(&self, ids: &[String]) -> Result<(), anyhow::Error> {
        println!("Done subcommand called: id = {:?}", ids);

        return Ok(());
    }

    fn update(
        &self,
        ids: &[String],
        to: &Option<TaskStatus>,
        name: &Option<String>,
    ) -> Result<(), anyhow::Error> {
        let new_name = match name {
            Some(name) => name,
            None => "",
        };
        let to_status = match to {
            Some(status) => status.to_string(),
            None => String::from("NO_CHANGE"),
        };

        println!(
            "Update subcommand called. to = {} id = {:?} renaming to = {}",
            to_status, ids, new_name
        );

        return Ok(());
    }
}
