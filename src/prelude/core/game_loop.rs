use super::{super::Renderer, Error, Window};

/// The core of the engine. Uses a Window and Renderer to start the game.
/// 
/// ## Usage
/// - Before creating a Game Loop, a Window and Renderer must have already been initialized
pub struct GameLoop<'a> {
    window: &'a mut Window,
    renderer: &'a mut Renderer,
}

impl<'a> GameLoop<'a> {
    /// Creates a new Game Loop</br>
    ///
    /// ## Arguments
    /// - 'window' - A mutable reference to the current Window
    /// - 'renderer' - A mutable reference to the current Renderer
    ///
    /// ## Example
    /// ```
    /// let game_loop = GameLoop::new(&mut window, &mut renderer);
    /// ```
    pub fn new(window: &'a mut Window, renderer: &'a mut Renderer) -> Self {
        Self { window, renderer }
    }

    /// Runs a callback in a loop while the window is open.</br>
    /// Will break and return Ok(()) if the window is closed
    ///
    /// ## Arguments
    /// - 'update' - This function that will be called while the application is running
    ///
    /// ## Returns
    /// - Ok(()) if window is closed with no errors
    /// - Err(Error) if an error occurred
    ///
    /// ## Example
    /// ```
    /// game_loop.on_update(move |renderer: &mut Renderer| {
    ///     //game code goes here...
    /// }).unwrap();
    /// ```
    pub fn on_update<F>(self, mut func: F) -> Result<(), Error>
    where
        F: FnMut(&mut Renderer) -> Result<(), Error>,
    {
        'game_loop: loop {
            if self.window.closed() {
                break 'game_loop;
            }

            match func(self.renderer) {
                Ok(_) => (),
                Err(err) => {
                    println!("Error: {err}");
                    return Err(err);
                }
            }

            self.window.update()
        }

        Ok(())
    }
}
