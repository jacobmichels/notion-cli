use clap::{ArgGroup, Parser, Subcommand};
use std::fmt::Display;

use crate::handlers::task::Task;

/// Defines the operations that can be performed on a task
pub trait TaskHandler {
    /// Adds a task to the database
    fn add(&self, name: &[String], status: &TaskStatus) -> Result<(), anyhow::Error>;
    /// Lists all the tasks in the database
    fn list(&self, status: &Option<TaskStatus>) -> Result<Vec<Task>, anyhow::Error>;
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

/// Defines the init operation
pub trait InitHandler {
    /// Saves the database_id for use in future calls
    fn init(&self, database_id: &str) -> anyhow::Result<()>;
}

/// Struct containing all required handlers
pub struct Handlers {
    /// Task handler
    pub task: Box<dyn TaskHandler>,
    /// Init handler
    pub init: Box<dyn InitHandler>,
}

impl Cli {
    /// Routes the command to the correct handler
    pub fn handle_command(&self, handlers: &Handlers) -> Result<(), anyhow::Error> {
        match &self.command {
            Commands::Tasks { subcommand } => {
                println!("Tasks command called");

                if !self.is_initialized() {
                    return Err(anyhow::Error::msg("App not initialized, please run init"));
                }

                match subcommand {
                    TaskSubcommands::Add { name, status } => handlers.task.add(name, status)?,
                    TaskSubcommands::List { status } => {
                        handlers.task.list(status)?;
                    }
                    TaskSubcommands::Done { id } => handlers.task.done(id)?,
                    TaskSubcommands::Update { id, to, name } => {
                        handlers.task.update(id, to, name)?
                    }
                };

                return Ok(());
            }
            Commands::Init { database_id } => handlers.init.init(database_id),
        }?;

        return Ok(());
    }

    /// Checks if the app has been initialized (database_id is available)
    fn is_initialized(&self) -> bool {
        return true;
    }
}

// clap structs below

/// Clap root
#[derive(Parser)]
#[clap(name = "github.com/jacobmichels/notion-cli")]
#[clap(author = "Jacob Michels <jacob.michels2025@gmail.com>")]
#[clap(version = "0.0.1")]
#[clap(about = "Interact with your notion board from the terminal", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

/// Defines the different subcommands that can be called
#[derive(Subcommand)]
enum Commands {
    /// Performs operations with tasks
    Tasks {
        /// Task operation to perform
        #[clap(subcommand)]
        subcommand: TaskSubcommands,
    },
    /// Used to set the database task commands interact with
    Init {
        /// Notion database_id to use with task commands
        #[clap(required = true)]
        database_id: String,
    },
}

/// Defines the task commands that can be performed
#[derive(Subcommand)]
enum TaskSubcommands {
    /// Lists the tasks in the database
    List {
        /// The status of the tasks to list
        #[clap(long, short, value_enum)]
        status: Option<TaskStatus>,
    },
    /// Add a task to the database
    Add {
        /// The name of the task
        /// Needs to be a Vec so spaces are handled
        #[clap(required = true)]
        name: Vec<String>,
        /// Status of the task to add
        #[clap(long, short, value_enum)]
        status: TaskStatus,
    },
    /// Update multiple tasks
    #[clap(group(ArgGroup::new("update").required(true).multiple(true).args(&["to", "name"])))]
    Update {
        /// Vec of IDs to update
        #[clap(required = true)]
        id: Vec<String>,
        /// Status to mark the tasks as
        #[clap(long, short, value_enum)]
        to: Option<TaskStatus>,
        /// New name for the task (only considered if single task specified)
        #[clap(long, short)]
        name: Option<String>,
    },
    /// Mark tasks as done
    Done {
        /// The list of tasks to mark as done
        #[clap(required = true)]
        id: Vec<String>,
    },
}

/// The current status of a task
#[derive(clap::ValueEnum, Clone)]
pub enum TaskStatus {
    /// Todo: not started
    Todo,
    /// Doing: started but not finished
    Doing,
    /// Done: finished
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::Todo => write!(f, "todo")?,
            TaskStatus::Doing => write!(f, "doing")?,
            TaskStatus::Done => write!(f, "done")?,
        };
        return Ok(());
    }
}
