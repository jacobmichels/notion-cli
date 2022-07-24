#![allow(clippy::needless_return)]
#![warn(clippy::implicit_return)]
use std::env;

use api::Notion;
use clap::Parser;
use cli::Cli;
use handlers::{init::PersistantInitHandler, task::NotionTaskHandler};

mod api;
mod cli;
mod handlers;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let notion = Notion::new(
        String::from("https://api.notion.com"),
        env::var("NOTION_TOKEN").expect("NOTION_TOKEN not defined"),
    );

    let task_handler = NotionTaskHandler::new(notion);
    let init_handler = PersistantInitHandler::new();

    let handlers = cli::Handlers {
        init: Box::new(init_handler),
        task: Box::new(task_handler),
    };

    cli.handle_command(&handlers)?;

    return Ok(());
}
