#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! A command line app for task management with notion
//! This project requires a nightly rust compiler because of LazyCell!
#![feature(once_cell)]

use anyhow::Result;
use clap::Parser;
use cli::Cli;

mod cli;
mod handlers;
mod models;
mod services;
mod traits;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    cli.route_command()?;

    return Ok(());
}
