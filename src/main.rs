#![allow(clippy::needless_return)]
#![warn(clippy::implicit_return)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! A command line app for task management with notion
//! This project requires a nightly rust compiler because of LazyCell!
#![feature(once_cell)]

use std::{cell::LazyCell, env};

use clap::Parser;
use cli::Cli;
use handlers::{config::JSONConfigHandler, task::NotionAPITaskHandler};
use notion::NotionAPI;
use traits::{ConfigHandler, TaskHandler};

/// Defines clap cli types for parsing args and flags
mod cli;
/// Defines command line route handlers
mod handlers;
/// Defines types needed for talking to the Notion API
mod notion;
/// Defines types that express notion tasks =
mod task;
/// Defines traits
mod traits;

fn main() -> Result<(), anyhow::Error> {
    let cli: Cli = Cli::parse();

    // lazily initialze our handlers, as at this point we don't know which one we'll need
    let task_handler: LazyCell<Box<dyn TaskHandler>> = LazyCell::new(|| {
        let notion_api = NotionAPI::new(
            String::from("https://api.notion.com"),
            env::var("NOTION_TOKEN").expect("NOTION_TOKEN not defined"),
        )
        .expect("failed to construct notion api wrapper");

        return Box::new(NotionAPITaskHandler::new(notion_api));
    });
    let config_handler: LazyCell<Box<dyn ConfigHandler>> =
        LazyCell::new(|| return Box::new(JSONConfigHandler::new()));

    let handlers = cli::Handlers {
        config: config_handler,
        task: task_handler,
    };

    cli.route_command(&handlers)?;

    return Ok(());
}
