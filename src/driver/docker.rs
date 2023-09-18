use crate::driver::Driver;

pub struct DockerDriver {}
impl DockerDriver {
    pub fn new() -> Self {
        Self {}
    }
}
impl Driver for DockerDriver {
    // TODO: Support --platform
    fn pull_image(&self, image: &str) -> String {
        format!("docker image pull {image}")
    }
}
