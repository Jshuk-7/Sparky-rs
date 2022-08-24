mod error;
pub mod logger;
mod window;
mod game_loop;

pub use error::*;
pub use logger as log;
pub use window::*;
pub use game_loop::GameLoop;

pub type GameResult = Result<(), Error>;