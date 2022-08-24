use super::algebra::Vec2;

use glfw::{Action, Context, Key, WindowEvent};

use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(size: Vec2, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = glfw
            .create_window(
                size.x as u32,
                size.y as u32,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create Window!");

        Self::init_graphics_api(&mut window);

        Self {
            glfw,
            window_handle: window,
            events,
        }
    }

    /// Returns true or false depending on if the window is closed or not
    ///
    /// ### EXAMPLE
    /// ```
    /// while !window.closed() {
    /// 	//...insert code here
    /// }
    /// ```
    pub fn closed(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Updates input and events then swaps frame buffers
    pub fn update(&mut self) {
        self.handle_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers()
    }

    fn init_graphics_api(window_handle: &mut glfw::Window) {
        window_handle.make_current();
        window_handle.set_framebuffer_size_polling(true);
        window_handle.set_key_polling(true);
        gl::load_with(|s| window_handle.get_proc_address(s) as *const _);
    }

    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Pos(_, _) => (),
                WindowEvent::Size(_, _) => (),
                WindowEvent::Close => (),
                WindowEvent::Refresh => (),
                WindowEvent::Focus(_) => (),
                WindowEvent::Iconify(_) => (),
                WindowEvent::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the resized window dimensions
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                WindowEvent::MouseButton(_, _, _) => (),
                WindowEvent::CursorPos(_, _) => (),
                WindowEvent::CursorEnter(_) => (),
                WindowEvent::Scroll(_, _) => (),
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                WindowEvent::Key(..) => (),
                WindowEvent::Char(_) => (),
                WindowEvent::CharModifiers(_, _) => (),
                WindowEvent::FileDrop(_) => (),
                WindowEvent::Maximize(_) => (),
                WindowEvent::ContentScale(_, _) => (),
            }
        }
    }
}
