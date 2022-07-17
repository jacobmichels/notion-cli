use crate::cli::{TaskHandler, TaskStatus};
use async_trait::async_trait;

#[async_trait]
pub trait NotionCaller {
    async fn list_databases(&self) -> Vec<String>;
}

pub struct Task {
    pub notion: Box<dyn NotionCaller>,
}

impl TaskHandler for Task {
    fn add(&self, name: &[String], status: &TaskStatus) {
        // convert the vec to a string, trimming the last space
        let name: String = name.iter().map(|s| s.to_string() + " ").collect::<String>();
        let name = name.trim_end();

        println!(
            "Add subcommand called. name = {:?} status = {}. This should return a link to the task it just created.",
            name,status
        );
    }

    fn list(&self, status: &Option<TaskStatus>) {}

    fn done(&self, ids: &[String]) {
        println!("Done subcommand called: id = {:?}", ids)
    }

    fn update(&self, ids: &[String], to: &Option<TaskStatus>, name: &Option<String>) {
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
    }
}
