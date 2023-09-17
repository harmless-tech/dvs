use thiserror::Error;

#[allow(dead_code)] // TODO: Remove
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("The child process exited with code `{0}`")]
    ExitCode(i32),
}
