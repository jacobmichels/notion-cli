use anyhow::Ok;

use crate::cli::{TaskHandler, TaskStatus};

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
    fn list_database_ids(&self) -> Result<Vec<String>, anyhow::Error>;
    fn list_tasks_in_db(&self, database_id: String) -> Result<Vec<Task>, anyhow::Error>;
}

pub struct NotionTaskHandler {
    pub notion: Box<dyn NotionCaller>,
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

    fn list(&self, status: &Option<TaskStatus>) -> Result<(), anyhow::Error> {
        let databases = self.notion.list_database_ids()?;
        println!("{:?}", databases);

        if databases.is_empty() {
            return Err(anyhow::Error::msg(
                "No databases found. Have you shared the database with the notion-cli integration?",
            ));
        } else if databases.len() > 1 {
            return Err(anyhow::Error::msg(
                "supplied token has access to more than one database",
            ));
        }

        return Ok(());
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
