use crate::cli_args::{Cli, Commands};
use crate::command::init::init_repository;
use clap::Parser;
use colored::Colorize;
use std::path::Path;

mod cli_args;
mod command;
mod repository;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => init_repository(Path::new(".")),
    }
}
