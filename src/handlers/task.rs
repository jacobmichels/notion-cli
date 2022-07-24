use anyhow::Ok;

use crate::{
    api::Notion,
    cli::{TaskHandler, TaskStatus},
};

/// A Notion task
pub struct Task {
    /// The task's ID
    pub id: String,
    /// The task's current status
    pub status: TaskStatus,
}

impl Task {
    /// Construct a new Task instance with an ID and status
    pub fn new(id: String, status: TaskStatus) -> Task {
        return Task { id, status };
    }
}

/// Defines Notion API operations
pub trait NotionCaller {
    /// Lists the tasks in the database
    fn list_tasks(&self) -> Result<Vec<Task>, anyhow::Error>;
}

/// A task handler that wraps a Notion client
pub struct NotionTaskHandler {
    /// The Notion client
    pub notion: Box<dyn NotionCaller>,
}

impl NotionTaskHandler {
    /// Construct a new NotionTaskHandler given a Notion API client
    pub fn new(notion: Notion) -> NotionTaskHandler {
        return NotionTaskHandler {
            notion: Box::new(notion),
        };
    }
}

impl TaskHandler for NotionTaskHandler {
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

    fn list(&self, status: &Option<TaskStatus>) -> Result<Vec<Task>, anyhow::Error> {
        return self.notion.list_tasks();
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
