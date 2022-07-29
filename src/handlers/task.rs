use anyhow::Ok;
use colour::{green_ln, red_ln};

use crate::{
    task::TaskStatus,
    traits::{NotionCaller, TaskHandler},
};

/// A task handler that wraps a Notion client
pub struct NotionAPITaskHandler {
    /// The Notion client
    notion: Box<dyn NotionCaller>,
}

impl NotionAPITaskHandler {
    /// Construct a new NotionTaskHandler given a Notion API client
    pub fn new(notion: Box<dyn NotionCaller>) -> NotionAPITaskHandler {
        return NotionAPITaskHandler { notion };
    }
}

impl TaskHandler for NotionAPITaskHandler {
    fn add(
        &self,
        database_id: &str,
        title: &str,
        status: &TaskStatus,
    ) -> Result<(), anyhow::Error> {
        let title = title.trim();

        self.notion.add_task(&database_id, title, status)?;

        green_ln!("Task added!");

        return Ok(());
    }

    fn list(
        &self,
        database_id: &str,
        status: &Option<TaskStatus>,
        with_id: &bool,
    ) -> Result<(), anyhow::Error> {
        let tasks = self.notion.list_tasks(&database_id, status)?;

        match status {
            Some(s) => {
                red_ln!(
                    "Tasks: {} -----------------------------------------------",
                    s
                );
                for (i, task) in tasks.iter().enumerate() {
                    task.print(i, false, *with_id);
                }
                red_ln!("----------------------------------------------------");
            }
            None => {
                red_ln!("Tasks ----------------------------------------------");
                for (i, task) in tasks.iter().enumerate() {
                    task.print(i, true, *with_id);
                }
                red_ln!("----------------------------------------------------");
            }
        }

        return Ok(());
    }

    fn done(&self, ids: &[String], name: &str) -> Result<(), anyhow::Error> {
        println!("Done subcommand called: ");
        println!("ids = {:?}", ids);
        println!("name = {:?}", name);

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
