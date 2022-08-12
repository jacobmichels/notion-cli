#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! A command line app for task management with notion
//! This project requires a nightly rust compiler because of LazyCell!
#![feature(once_cell)]

use std::{cell::LazyCell, env};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use handlers::{config::JSONConfigHandler, task::NotionAPITaskHandler};
use notion::NotionAPI;
use traits::{ConfigHandler, TaskHandler};

/// Defines clap cli types for parsing args and flags
mod cli;
/// Defines the database type
mod database;
/// Defines command line route handlers
mod handlers;
/// Defines types needed for talking to the Notion API
mod notion;
/// Defines types that express notion tasks =
mod task;
/// Defines traits
mod traits;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    // lazily initialze our handlers, as at this point we don't know which one we'll need
    let task_handler: LazyCell<Box<dyn TaskHandler>> = LazyCell::new(|| {
        let notion_api = instantiate_notion_client();
        return Box::new(NotionAPITaskHandler::new(Box::new(notion_api)));
    });
    let config_handler: LazyCell<Box<dyn ConfigHandler>> = LazyCell::new(|| {
        let notion_api = instantiate_notion_client();
        return Box::new(JSONConfigHandler::new(Box::new(notion_api)));
    });

    let handlers = cli::Handlers {
        config: config_handler,
        task: task_handler,
    };

    cli.route_command(&handlers)?;

    return Ok(());
}

/// helper function to build a NotionAPI
/// this will only be called once throughout the apps lifetime, even through it appears more than once in LazyCell
fn instantiate_notion_client() -> NotionAPI {
    return NotionAPI::new(
        String::from("https://api.notion.com"),
        env::var("NOTION_TOKEN").expect("NOTION_TOKEN not defined"),
    )
    .expect("failed to construct notion api wrapper");
}
