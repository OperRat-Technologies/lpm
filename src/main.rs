use crate::cli_args::{Cli, Commands};
use crate::command::init::init_repository;
use crate::command::{add, clear};
use clap::Parser;
use std::path::Path;

mod cli_args;
mod command;
mod luarocks;
mod repository;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => init_repository(Path::new(".")),
        Commands::Add { name, version } => add::add_package(name, version).await,
        Commands::Clear {} => clear::clear_local_repository(),
    }
}
