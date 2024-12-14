use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a repository on the current directory
    Init,
    /// Installs a remote package into the local repository
    Add {
        name: String,
        version: Option<String>,
    },
}
