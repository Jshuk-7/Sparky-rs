pub mod prelude;

pub mod core {
    use super::prelude::*;

    pub fn run_game<F>(mut f: F, window: &mut Window) -> Result<(), Error>
    where
        F: FnMut(&mut Window) -> Result<(), Error>,
    {
        f(window)
    }
}
