#![allow(clippy::needless_return)]
#![warn(clippy::implicit_return)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! A command line app for task management with notion

use std::env;

use api::Notion;
use clap::Parser;
use cli::Cli;
use handlers::{config::JSONConfigHandler, task::NotionTaskHandler};

/// Defines types needed for talking to the Notion API
mod api;
/// Defines types needed for clap
mod cli;
/// Defines command line route handlers
mod handlers;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let notion_api = Notion::new(
        String::from("https://api.notion.com"),
        env::var("NOTION_TOKEN").expect("NOTION_TOKEN not defined"),
    );

    let task_handler = NotionTaskHandler::new(notion_api);
    let config_handler = JSONConfigHandler::new();

    let handlers = cli::Handlers {
        config: Box::new(config_handler),
        task: Box::new(task_handler),
    };

    cli.handle_command(&handlers)?;

    return Ok(());
}
