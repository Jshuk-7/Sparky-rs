pub mod prelude;

pub mod core {
    use super::prelude::*;

    pub struct GameLoop<'a>(pub &'a mut Window, pub &'a mut Renderer);

    impl<'a> GameLoop<'a> {
        pub fn run<F>(self, mut f: F) -> Result<(), Error>
        where
            F: FnMut(&mut Window, &mut Renderer) -> Result<(), Error>,
        {
            let mut result = Ok(());

            while !self.0.closed() {
                result = f(self.0, self.1);
            }

            result
        }
    }
}
