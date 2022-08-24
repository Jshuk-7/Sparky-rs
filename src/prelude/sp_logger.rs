pub use log::*;

///
/// Use 'RUST_LOG=info cargo run' to see log messages
/// 
/// Initializes the logger, this must be called only one time before using the logger
/// or else no logs will be made visible in the output
pub fn init() {
    env_logger::init()
}