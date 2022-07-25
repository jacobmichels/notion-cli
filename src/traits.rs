use crate::task::{Task, TaskStatus};

/// Defines the operations that can be performed on a task
pub trait TaskHandler {
    /// Adds a task to the database
    fn add(&self, name: &[String], status: &TaskStatus) -> Result<(), anyhow::Error>;
    /// Lists the tasks in the database with the specified status
    fn list(
        &self,
        status: &Option<TaskStatus>,
        database_id: String,
    ) -> Result<Vec<Task>, anyhow::Error>;
    /// Marks a list of tasks as done
    fn done(&self, ids: &[String]) -> Result<(), anyhow::Error>;
    /// Modifies the TaskStatus of multiple tasks
    fn update(
        &self,
        ids: &[String],
        to: &Option<TaskStatus>,
        name: &Option<String>,
    ) -> Result<(), anyhow::Error>;
}

/// Defines the config operations
pub trait ConfigHandler {
    /// Saves the database_id for use in future calls
    fn set_database(&self, database_id: &str) -> anyhow::Result<()>;

    /// Gets the persisted database_id
    fn get_database_id(&self) -> anyhow::Result<String>;
}

/// An object that can perform Notion operations
pub trait NotionCaller {
    /// Lists the tasks in the database
    fn list_tasks(&self, database_id: String) -> Result<Vec<Task>, anyhow::Error>;
}
