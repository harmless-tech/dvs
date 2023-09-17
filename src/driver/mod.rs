use crate::driver::docker::DockerDriver;

mod docker;
mod podman;

pub trait Driver {
    fn pull_image(&self, image: &str) -> String;
}

pub fn get_bundled(item: &str) -> Option<Box<dyn Driver>> {
    match item {
        "bundled-docker" => Some(Box::new(DockerDriver::new())),
        _ => None,
    }
}
