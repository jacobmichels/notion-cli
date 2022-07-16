#![allow(clippy::needless_return)]

use std::{fmt::Display, str::FromStr};

use clap::{ArgGroup, Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    // create handler objects, pass them to handle_command
    // try to use traits
    // do the sbominator pattern, instantiate stuff then pass it down for testability

    handle_command(&cli);
}

fn handle_command(cli: &Cli) {
    match &cli.command {
        Commands::Tasks { subcommand } => {
            println!("Tasks command called");
            match subcommand {
                TaskSubcommands::Add { name, status } => {
                    // convert the vec to a string, trimming the last space
                    let name: String = name.iter().map(|s| s.to_string() + " ").collect::<String>();
                    let name = name.trim_end();

                    println!(
                        "Add subcommand called. name = {:?} status = {}. This should return a link to the task it just created.",
                        name,status
                    );
                }
                TaskSubcommands::List { status } => {
                    let status = match status {
                        Some(status) => status.to_string(),
                        None => String::from("all"),
                    };

                    println!("List subcommand called: status = {}", status);
                }
                TaskSubcommands::Done => {
                    println!("Done subcommand called")
                }
                TaskSubcommands::Update { id, to, name } => {
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
                        to_status, id, new_name
                    );
                }
            }
        }
        Commands::Init => {
            println!("Init command called")
        }
    }
}

// clap structs below

#[derive(Parser)]
#[clap(name = "github.com/jacobmichels/notion-cli")]
#[clap(author = "Jacob Michels <jacob.michels2025@gmail.com>")]
#[clap(version = "0.0.1")]
#[clap(about = "Interact with your notion board from the terminal", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
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
    Done,
}

#[derive(clap::ValueEnum, Clone)]
enum TaskStatus {
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
