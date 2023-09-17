use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: ArgCommand,
    // TODO: Top level config stuff, like --config=$PATH
}

#[derive(Debug, Subcommand)]
pub enum ArgCommand {
    /// (TODO) Persist
    Create,
    /// (TODO) Temp
    Run {
        /// Id of the dvs
        id: String,
    },
    /// (TODO) Edit and manage dvs'
    Img,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
