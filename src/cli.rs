use anyhow::Result;
use clap::{ArgGroup, Parser, Subcommand};

use crate::{
    handlers::{config::JSONConfigHandler, task::NotionAPITaskHandler},
    services::{config::JSONConfigService, notion::NotionAPI},
    task::TaskStatus,
    traits::{ConfigCommandHandler, ConfigService, TaskHandler},
};

impl Cli {
    /// Routes the command to the correct handler
    pub fn route_command(&self) -> Result<()> {
        match &self.command {
            Command::Tasks { subcommand } => {
                let config = JSONConfigService::new().get_config()?;
                let notion = NotionAPI::new("https://api.notion.com".to_string(), config.token)?;
                let task_handler = NotionAPITaskHandler::new(Box::new(notion));

                match subcommand {
                    TaskSubcommand::Add { name, status } => {
                        task_handler.add(&config.database_id, name, status)?;
                    }
                    TaskSubcommand::List { status, with_id } => {
                        task_handler.list(&config.database_id, status, with_id)?;
                    }
                    TaskSubcommand::Done { ids, name } => {
                        task_handler.done(&config.database_id, ids, name.as_deref())?;
                    }
                    TaskSubcommand::Update { id, to, name } => {
                        task_handler.update(id, to, name)?;
                    }
                };

                return Ok(());
            }
            Command::Config { subcommand } => {
                let config_service = JSONConfigService::new();
                let notion = NotionAPI::new(
                    "https://api.notion.com".to_string(),
                    config_service.get_config()?.token,
                )?;
                let handler = JSONConfigHandler::new(Box::new(notion), Box::new(config_service));

                match subcommand {
                    ConfigSubcommand::Database { subcommand } => match subcommand {
                        DatabaseConfigSubcommand::Get => {
                            handler.get_database_id()?;
                        }
                        DatabaseConfigSubcommand::List => {
                            handler.list_databases()?;
                        }
                        DatabaseConfigSubcommand::Set { database_id } => {
                            handler.set_database(database_id)?;
                        }
                    },
                    ConfigSubcommand::Token { subcommand } => match subcommand {
                        TokenConfigSubcommand::Set { token } => {
                            handler.set_token(token)?;
                        }
                    },
                }
            }
        };

        return Ok(());
    }
}

// clap structs below

/// Clap root
#[derive(Parser)]
#[clap(name = "github.com/jacobmichels/notion-cli")]
#[clap(author = "Jacob Michels <jacob.michels2025@gmail.com>")]
#[clap(version = "0.1.0")]
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
        /// Whether or not to include the task ID in the listing
        #[clap(long, short)]
        with_id: bool,
    },
    /// Add a task to the database
    Add {
        /// The name of the task
        #[clap(required = true)]
        name: String,
        /// Status of the task to add
        #[clap(long, short, value_enum)]
        status: TaskStatus,
    },
    /// Update a task
    #[clap(group(ArgGroup::new("update").required(true).multiple(true).args(&["to", "name"])))]
    Update {
        /// The ID of the task to update
        #[clap(required = true)]
        id: String,
        /// Status to mark the tasks as
        #[clap(long, short, value_enum)]
        to: Option<TaskStatus>,
        /// New name for the task (only considered if single task specified)
        #[clap(long, short)]
        name: Option<String>,
    },
    /// Mark tasks as done
    #[clap(group(ArgGroup::new("done").required(true).multiple(false).args(&["ids", "name"])))]
    Done {
        /// A list of task IDs to mark as done
        ids: Vec<String>,
        /// The name of a task to mark as done
        /// Doesn't have to be an exist match, this will match to a task if the task title contains the given string (case insensitive)
        #[clap(long, short)]
        name: Option<String>,
    },
}

/// Defines the config commands that can be performed
#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Database configuration subcommand
    Database {
        #[clap(subcommand)]
        subcommand: DatabaseConfigSubcommand,
    },
    /// Token configuration subcommand
    Token {
        #[clap(subcommand)]
        subcommand: TokenConfigSubcommand,
    },
}

#[derive(Subcommand)]
enum DatabaseConfigSubcommand {
    Get,
    Set { database_id: String },
    List,
}

#[derive(Subcommand)]
enum TokenConfigSubcommand {
    Set { token: String },
}
