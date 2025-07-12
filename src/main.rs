use crate::cli_args::{Cli, Commands};
use crate::command::init::init_repository;
use crate::command::upgrade::upgrade_lpm_installation;
use crate::command::{add, bundle, clear};
use clap::Parser;
use std::path::Path;

mod bundler;
mod cli_args;
mod command;
mod compiler;
mod luarocks;
mod repository;
mod uploader;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => init_repository(Path::new(".")),
        Commands::Add { name, version } => add::add_package(name, version).await,
        Commands::Clear {} => clear::clear_local_repository(),
        Commands::Bundle {
            entry,
            upload,
            clipboard,
            minify,
            out,
        } => bundle::bundle_files(entry, upload, clipboard, minify, out).await,
        Commands::Upgrade {} => upgrade_lpm_installation().await.unwrap(),
    }
}
