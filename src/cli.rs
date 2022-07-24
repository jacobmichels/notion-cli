use clap::{ArgGroup, Parser, Subcommand};
use std::fmt::Display;

pub trait TaskHandler {
    fn add(&self, name: &[String], status: &TaskStatus) -> Result<(), anyhow::Error>;
    fn list(&self, status: &Option<TaskStatus>) -> Result<(), anyhow::Error>;
    fn done(&self, ids: &[String]) -> Result<(), anyhow::Error>;
    fn update(
        &self,
        ids: &[String],
        to: &Option<TaskStatus>,
        name: &Option<String>,
    ) -> Result<(), anyhow::Error>;
}

pub trait InitHandler {
    fn init(&self);
}

pub struct Handlers {
    pub task: Box<dyn TaskHandler>,
    pub init: Box<dyn InitHandler>,
}

impl Cli {
    pub fn handle_command(&self, handlers: &Handlers) -> Result<(), anyhow::Error> {
        match &self.command {
            Commands::Tasks { subcommand } => {
                println!("Tasks command called");
                match subcommand {
                    TaskSubcommands::Add { name, status } => handlers.task.add(name, status)?,
                    TaskSubcommands::List { status } => handlers.task.list(status)?,
                    TaskSubcommands::Done { id } => handlers.task.done(id)?,
                    TaskSubcommands::Update { id, to, name } => {
                        handlers.task.update(id, to, name)?
                    }
                }
            }
            Commands::Init => handlers.init.init(),
        }

        return Ok(());
    }
}

// clap structs below

#[derive(Parser)]
#[clap(name = "github.com/jacobmichels/notion-cli")]
#[clap(author = "Jacob Michels <jacob.michels2025@gmail.com>")]
#[clap(version = "0.0.1")]
#[clap(about = "Interact with your notion board from the terminal", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tasks {
        #[clap(subcommand)]
        subcommand: TaskSubcommands,
    },
    Init,
}

#[derive(Subcommand)]
enum TaskSubcommands {
    List {
        #[clap(long, short, value_enum)]
        status: Option<TaskStatus>,
    },
    Add {
        #[clap(required = true)]
        name: Vec<String>, // use a vec so we can have spaces in the title without wrapping with quotes
        #[clap(long, short, value_enum)]
        status: TaskStatus,
    },
    #[clap(group(ArgGroup::new("update").required(true).multiple(true).args(&["to", "name"])))]
    Update {
        #[clap(required = true)]
        id: Vec<String>, // use a vec so we can take in multiple ids
        #[clap(long, short, value_enum)]
        to: Option<TaskStatus>,
        #[clap(long, short)]
        name: Option<String>,
    },
    Done {
        #[clap(required = true)]
        id: Vec<String>, // use a vec so we can take in multiple ids
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::Todo => write!(f, "todo")?,
            TaskStatus::Doing => write!(f, "doing")?,
            TaskStatus::Done => write!(f, "done")?,
        };
        Ok(())
    }
}
