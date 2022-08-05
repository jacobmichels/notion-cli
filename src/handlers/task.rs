use anyhow::{Ok, Result};
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
    fn add(&self, database_id: &str, title: &str, status: &TaskStatus) -> Result<()> {
        let title = title.trim();

        self.notion.add_task(database_id, title, status)?;

        green_ln!("Task added!");

        return Ok(());
    }

    fn list(&self, database_id: &str, status: &Option<TaskStatus>, with_id: &bool) -> Result<()> {
        let tasks = self.notion.list_tasks(database_id, status)?;

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

    fn done(&self, database_id: &str, ids: &[String], name: Option<&str>) -> Result<()> {
        println!("Done subcommand called: ");
        println!("ids = {:?}", ids);
        println!("name = {:?}", name);

        if let Some(n) = name {
            let task = self.notion.get_task_from_name(database_id, n)?;
            println!("{:?}", task);
            self.notion.mark_as_done(&[task.id])?;
        } else {
            self.notion.mark_as_done(ids)?;
        };

        return Ok(());
    }

    fn update(&self, id: &str, to: &Option<TaskStatus>, name: &Option<String>) -> Result<()> {
        self.notion.update_task(id, to, name)?;

        green_ln!("Successfully updated task");

        return Ok(());
    }
}
