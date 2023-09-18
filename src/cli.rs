use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: ArgCommand,
    // TODO: Top level config stuff, like --config=$PATH
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub enum ArgCommand {
    /// (TODO) Persist
    #[command(arg_required_else_help = true)]
    Create {
        /// Name for container
        name: String,
        /// Id of the dvs
        id: String,
        #[command(flatten)]
        args: DockerArgs,
    },
    /// (TODO)
    #[command(arg_required_else_help = true)]
    Start,
    /// (TODO)
    #[command(arg_required_else_help = true)]
    Stop,
    /// (TODO)
    #[command(arg_required_else_help = true)]
    RM,
    /// (TODO) Temp (--rm)
    #[command(arg_required_else_help = true)]
    Run {
        /// Id of the dvs
        id: String,
        #[command(flatten)]
        args: DockerArgs,
    },
    /// (TODO) Connect to running container
    #[command(arg_required_else_help = true)]
    Exec {
        /// TODO
        name: String,
        #[command(flatten)]
        args: DockerIOArgs,
    },
    /// (TODO) Edit and manage dvs' (Prune, Rm (--all), Create, Edit, Dl)
    #[command(arg_required_else_help = true)]
    Img,
}

#[derive(Args, Clone, Debug)]
pub struct DockerIOArgs {
    /// TODO
    #[arg(short, long)]
    detach: bool,
    /// TODO
    #[arg(short, long, default_value_t = true)]
    interactive: bool,
    /// TODO
    #[arg(short, long, default_value_t = true)]
    tty: bool,
}

#[derive(Args, Clone, Debug)]
pub struct DockerArgs {
    #[command(flatten)]
    args: DockerIOArgs,
    /// TODO
    #[arg(short, long)]
    env: Option<Vec<String>>,
    /// TODO
    #[arg(long)]
    network: Option<String>,
    /// TODO
    #[arg(long)]
    platform: Option<String>,
    /// TODO
    #[arg(short, long)]
    publish: Option<String>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert()
}
