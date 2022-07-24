#![allow(clippy::needless_return)]
#![warn(clippy::implicit_return)]
use std::env;

use api::Notion;
use clap::Parser;
use cli::Cli;

mod api;
mod cli;
mod handlers;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let task_handler = handlers::task::NotionTaskHandler {
        notion: Box::new(Notion::new(
            String::from("https://api.notion.com"),
            env::var("NOTION_TOKEN").expect("NOTION_TOKEN not defined"),
        )?),
    };
    let init_handler = handlers::init::Init {};

    let handlers = cli::Handlers {
        init: Box::new(init_handler),
        task: Box::new(task_handler),
    };

    cli.handle_command(&handlers)?;

    return Ok(());
}
