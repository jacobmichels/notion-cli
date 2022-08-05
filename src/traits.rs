use anyhow::Result;

use crate::{
    database::Database,
    task::{Task, TaskStatus},
};

/// Defines the operations that can be performed on a task
pub trait TaskHandler {
    /// Adds a task to the database
    fn add(&self, database_id: &str, name: &str, status: &TaskStatus) -> Result<()>;
    /// Lists the tasks in the database with the specified status
    fn list(&self, database_id: &str, status: &Option<TaskStatus>, with_id: &bool) -> Result<()>;
    /// Marks a list of tasks as done
    fn done(&self, database_id: &str, ids: &[String], name: Option<&str>) -> Result<()>;
    /// Modifies the TaskStatus of multiple tasks
    fn update(&self, ids: &str, to: &Option<TaskStatus>, name: &Option<String>) -> Result<()>;
}

/// Defines the config operations
pub trait ConfigHandler {
    /// Saves the database_id for use in future calls
    fn set_database(&self, database_id: &str) -> Result<()>;

    /// Gets the persisted database_id
    fn get_database_id(&self) -> Result<String>;

    /// Prints titles and names of databases that can be used by the app
    fn print_eligible_databases(&self) -> Result<()>;
}

/// An object that can perform Notion operations
pub trait NotionCaller {
    /// Lists the tasks in the database
    fn list_tasks(&self, database_id: &str, status: &Option<TaskStatus>) -> Result<Vec<Task>>;

    /// Adds a task to the database
    fn add_task(&self, database_id: &str, title: &str, status: &TaskStatus) -> Result<()>;

    /// List all databases that have these three statuses: To Do, Doing, and Done
    fn list_eligible_databases(&self) -> Result<Vec<Database>>;

    /// Mark the given task ids as done
    fn mark_as_done(&self, ids: &[String]) -> Result<()>;

    /// Return the first task that contains pattern
    fn get_task_from_name(&self, database_id: &str, pattern: &str) -> Result<Task>;

    /// Update the task to the supplied status and title
    /// At least one of the supplied Optional values will be supplied
    fn update_task(&self, id: &str, to: &Option<TaskStatus>, name: &Option<String>) -> Result<()>;
}
