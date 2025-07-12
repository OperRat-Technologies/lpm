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
    /// Clears the local repository
    Clear,
    /// Bundle,
    Bundle {
        /// Entry point for the bundle
        entry: String,
        /// Whether to upload the bundle to somewhere
        #[arg(long)]
        upload: bool,
        /// Output file, if omitted and now uploaded, the result will be written to "bundle.lua"
        #[arg(long)]
        out: Option<String>,
        /// Copy the output to the clipboard instead of writing to a file or uploading
        #[arg(long)]
        clipboard: bool,
        /// Minify the output bundle
        #[arg(long)]
        minify: bool,
    },
    /// Upgrades the lpm installation
    Upgrade,
}
