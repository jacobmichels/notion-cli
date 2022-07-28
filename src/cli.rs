use clap::{ArgGroup, Parser, Subcommand};
use colour::{green, green_ln, red_ln};
use std::cell::LazyCell;

use crate::{
    task::TaskStatus,
    traits::{ConfigHandler, TaskHandler},
};

/// Struct containing all required handlers for routing
pub struct Handlers {
    /// Task handler
    pub task: LazyCell<Box<dyn TaskHandler>>,
    /// Config handler
    pub config: LazyCell<Box<dyn ConfigHandler>>,
}

impl Cli {
    /// Routes the command to the correct handler
    pub fn route_command(&self, handlers: &Handlers) -> Result<(), anyhow::Error> {
        match &self.command {
            Command::Tasks { subcommand } => {
                if !self.is_initialized() {
                    return Err(anyhow::Error::msg(
                        "App not initialized, please run config set",
                    ));
                }

                match subcommand {
                    TaskSubcommand::Add { name, status } => {
                        let database = handlers.config.get_database_id()?;
                        handlers.task.add(database, name, status)?;
                    }
                    TaskSubcommand::List { status } => {
                        let database = handlers.config.get_database_id()?;
                        handlers.task.list(database, status)?;
                    }
                    TaskSubcommand::Done { id } => handlers.task.done(id)?,
                    TaskSubcommand::Update { id, to, name } => {
                        handlers.task.update(id, to, name)?
                    }
                };

                return Ok(());
            }
            Command::Config { subcommand } => {
                green_ln!("Config command called");
                match subcommand {
                    ConfigSubcommand::Get => {
                        let id = handlers.config.get_database_id()?;
                        green!("Database ID: ");
                        red_ln!("{}", id)
                    }
                    ConfigSubcommand::Set { database_id } => {
                        handlers.config.set_database(database_id)?
                    }
                }
            }
        };

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
    command: Command,
}

/// Defines the different subcommands that can be called
#[derive(Subcommand)]
enum Command {
    /// Performs operations with tasks
    Tasks {
        /// Task operation to perform
        #[clap(subcommand)]
        subcommand: TaskSubcommand,
    },
    /// Used to configure the database task commands interact with
    Config {
        /// Config operation to perform
        #[clap(subcommand)]
        subcommand: ConfigSubcommand,
    },
}

/// Defines the task commands that can be performed
#[derive(Subcommand)]
enum TaskSubcommand {
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

/// Defines the config commands that can be performed
#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Gets the current database_id
    Get,
    /// Sets the database_id
    Set {
        /// The database id in question
        #[clap(required = true)]
        database_id: String,
    },
}
