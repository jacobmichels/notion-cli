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
        database_id: String,
        title: &[String],
        status: &TaskStatus,
    ) -> Result<(), anyhow::Error> {
        // convert the vec to a string, trimming the last space
        let title: String = title
            .iter()
            .map(|s| s.to_string() + " ")
            .collect::<String>();
        let title = title.trim_end();

        self.notion.add_task(database_id, title, status)?;

        green_ln!("Task added!");

        return Ok(());
    }

    fn list(&self, database_id: String, status: &Option<TaskStatus>) -> Result<(), anyhow::Error> {
        let tasks = self.notion.list_tasks(database_id, status)?;

        match status {
            Some(s) => {
                red_ln!("{} -------------------------------------------------", s);
                for (i, task) in tasks.iter().enumerate() {
                    task.print(i);
                }
                red_ln!("----------------------------------------------------");
            }
            None => {
                red_ln!("----------------------------------------------------");
                for (i, task) in tasks.iter().enumerate() {
                    task.print_with_status(i);
                }
                red_ln!("----------------------------------------------------");
            }
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
