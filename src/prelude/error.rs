use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("This is a test error for the Sparky Game Engine")]
    Test,
}
