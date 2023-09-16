use clap::Parser;

mod cli;
mod driver;

fn main() {
    let _args = cli::Args::parse();
    #[cfg(debug_assertions)]
    dbg!(_args);

    println!("Hello, world!");
}
