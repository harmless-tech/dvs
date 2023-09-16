use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    subcommand: ArgCommand,
}

#[derive(Debug, Subcommand)]
pub enum ArgCommand {
    /// Deals with virtual system bases
    Img {
        #[command(subcommand)]
        subcommand: ArgImgCommand,
    },
    /// List virtual systems and their statuses
    List,
    /// Runs a virtual system
    Create {
        /// Temporary system that must be removed on exit.
        /// Can use multiple of these at a time.
        #[arg(short, long)]
        temp: bool,
        /// Do not remove on exit
        #[arg(short, long)]
        no_remove: bool,
        /// Do not attach STDIO
        #[arg(short, long)]
        detach: bool,
        /// Name to be given to the virtual system
        name: String,
        /// Base of the virtual system
        base: String,
        /// Command to exec, otherwise base command will be used
        #[arg(last = true)]
        cmd: Option<OsString>,
    },
    /// Runs a command in the virtual system
    Exec {
        /// Do not attach STDIO
        #[arg(short, long)]
        detach: bool,
        /// Name of the virtual system
        name: String,
        /// Command to exec, otherwise base command will be used
        #[arg(last = true)]
        cmd: Option<OsString>,
    },
    /// Start a virtual system
    Start {
        /// Name of the virtual system
        name: String,
    },
    /// Stop a virtual system
    Stop {
        /// Name of the virtual system
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ArgImgCommand {
    /// Create a new image
    Create,
    /// Download an image definition from a url
    Download,
    /// Edit an image
    Edit, // TODO: This should allow people to open stuff like the config and Dockerfile in VSCODE
    /// Bundle an image for use in
    Bundle,
    /// List images
    List,
    /// Get info about an image
    Info,
    /// (Re)Builds base image, removing the old image and any containers that use it
    Build {
        /// Do not rebuild image if it already exists
        #[arg(short, long)]
        no_rebuild: bool,
        /// Use build cache if possible
        #[arg(short, long)]
        use_cache: bool,
        /// TODO
        base_name: String,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
