#![allow(clippy::needless_return)]

use clap::Parser;
use cli::Cli;

mod cli;
mod handlers;

// single threaded async runtime, we don't need any worker threads here
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    let task_handler = handlers::task::Task {};
    let init_handler = handlers::init::Init {};

    let handlers = cli::Handlers {
        init: Box::new(init_handler),
        task: Box::new(task_handler),
    };

    cli.handle_command(&handlers);
}
