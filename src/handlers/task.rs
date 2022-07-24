use anyhow::Ok;

use crate::{
    api::Notion,
    cli::{TaskHandler, TaskStatus},
};

pub struct Task {
    pub id: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: String, status: TaskStatus) -> Task {
        return Task { id, status };
    }
}

pub trait NotionCaller {
    fn list_tasks(&self) -> Result<Vec<Task>, anyhow::Error>;
    // fn list_database_ids(&self) -> Result<Vec<String>, anyhow::Error>;
    // fn list_tasks_in_db(&self, database_id: String) -> Result<Vec<Task>, anyhow::Error>;
}

pub struct NotionTaskHandler {
    pub notion: Box<dyn NotionCaller>,
}

impl NotionTaskHandler {
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
