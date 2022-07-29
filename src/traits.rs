use crate::{
    database::Database,
    task::{Task, TaskStatus},
};

/// Defines the operations that can be performed on a task
pub trait TaskHandler {
    /// Adds a task to the database
    fn add(&self, database_id: &str, name: &str, status: &TaskStatus) -> Result<(), anyhow::Error>;
    /// Lists the tasks in the database with the specified status
    fn list(
        &self,
        database_id: &str,
        status: &Option<TaskStatus>,
        with_id: &bool,
    ) -> Result<(), anyhow::Error>;
    /// Marks a list of tasks as done
    fn done(&self, ids: &[String], name: &str) -> Result<(), anyhow::Error>;
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

    /// Prints titles and names of databases that can be used by the app
    fn print_eligible_databases(&self) -> anyhow::Result<()>;
}

/// An object that can perform Notion operations
pub trait NotionCaller {
    /// Lists the tasks in the database
    fn list_tasks(
        &self,
        database_id: &str,
        status: &Option<TaskStatus>,
    ) -> Result<Vec<Task>, anyhow::Error>;

    /// Adds a task to the database
    fn add_task(&self, database_id: &str, title: &str, status: &TaskStatus) -> anyhow::Result<()>;

    /// List all databases that have these three statuses: To Do, Doing, and Done
    fn list_eligible_databases(&self) -> anyhow::Result<Vec<Database>>;
}
