pub use log::*;

/// Initializes the logger, this must be called one time before using the logger,</br>
/// or else no logs will be made visible in the output
/// 
/// ## Usage
/// Use 'RUST_LOG=info cargo run' to see log messages
pub fn init() {
    env_logger::init()
}