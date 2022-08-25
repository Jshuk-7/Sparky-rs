use thiserror::Error;

/// Errors to be used in Results
#[derive(Error, Debug)]
pub enum Error {
    #[error("This is a test error for the Sparky Game Engine")]
    Test,
    #[error("")]
    Overflow,
}
