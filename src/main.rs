#![allow(dead_code)] // TODO: Remove!

use crate::cli::ArgCommand;
use clap::Parser;
use std::{ffi::OsString, process::Command};
use toml::Table;

mod cli;
mod driver;
mod errors;
mod image;

#[cfg(target_family = "wasm")]
compile_error!("Wasm is not a supported target!");

fn main() {
    let args = cli::CliArgs::parse();
    #[cfg(debug_assertions)]
    dbg!(&args);

    // TODO: Get config here!
    let driver = driver::get_bundled("bundled-docker").unwrap();

    // TODO: Testing
    match &args.subcommand {
        ArgCommand::Run { id, .. } => {
            let image = image::get_bundled(id).unwrap();
            let image = image.parse::<Table>().unwrap();

            let pull_img = image["image"]["image"].as_str().unwrap();
            let pull_img_default = image["image"]["default"].as_str().unwrap();
            let pull_img = if !pull_img.contains(':') {
                format!("{pull_img}:{pull_img_default}")
            }
            else {
                pull_img.to_string()
            };

            dbg!(&pull_img);

            let cmd = driver.pull_image(&pull_img);
            run_cmd(&cmd, None).expect("FFF1");
            run_cmd(
                "docker image tag rust:latest dvs_managed--rust:latest--aarch64",
                None,
            )
            .expect("FFF2");
        }
        _ => todo!(),
    }
}

fn run_cmd(cmd: &str, end: Option<OsString>) -> Result<(), String> {
    #[cfg(target_family = "unix")]
    let mut command = Command::new("sh");
    #[cfg(target_family = "unix")]
    command.arg("-c");

    #[cfg(target_family = "windows")]
    let mut command = Command::new("cmd");
    #[cfg(target_family = "windows")]
    command.arg("/C");

    command.arg(cmd);
    if let Some(str) = end {
        command.arg(str);
    }

    #[cfg(debug_assertions)]
    dbg!(&command);

    // let out = command.status().expect("exec failed");

    let child = command.spawn().expect("Could not spawn child process.");
    let _out = child.wait_with_output();

    // match command.spawn().and_then(|mut child| child.wait()) {
    //     Ok(status) => std::process::exit(status.code().unwrap_or(1)),
    //     Err(error) => die!("fatal: {}", error),
    // };

    Ok(())
}
