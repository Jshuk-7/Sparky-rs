use super::super::algebra::Vec2;

use glfw::{Action, Context, Key, WindowEvent};

use std::sync::mpsc::Receiver;

#[allow(dead_code)]
const OPENGL_MAJOR_WINDOWS: u32 = 4;
#[allow(dead_code)]
const OPENGL_MINOR_WINDOWS: u32 = 6;

#[allow(dead_code)]
const OPENGL_MAJOR_MAC_OS: u32 = 3;
#[allow(dead_code)]
const OPENGL_MINOR_MAC_OS: u32 = 3;

/// The OS independent window that a game will run in
pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Creates a new window
    ///
    /// ## Arguments
    /// - 'size' - The size of the window.
    /// - 'title' - The title of the application.
    ///
    /// ## Example
    /// ```
    /// let window = Window::new(Vec2::new(640.0, 480.0), "Hello World!");
    /// ```
    pub fn new(size: Vec2, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        Self::set_window_hints(&mut glfw);

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
    /// ## Example
    /// ```
    /// while !window.closed() {
    /// 	//...insert code here
    /// }
    /// ```
    pub fn closed(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Processes input and events, such as key presses, mouse movements, window resizes etc...
    ///
    /// ## Example
    /// ```
    /// window.update();
    /// ```
    pub fn update(&mut self) {
        self.handle_events();
        self.window_handle.swap_buffers();
        self.glfw.poll_events();
    }

    fn set_window_hints(glfw_handle: &mut glfw::Glfw) {
        // Windows
        #[cfg(target_os = "windows")]
        glfw_handle.window_hint(glfw::WindowHint::ContextVersion(
            OPENGL_MAJOR_WINDOWS,
            OPENGL_MINOR_WINDOWS,
        ));

        // Mac
        #[cfg(target_os = "macos")]
        glfw_handle.window_hint(glfw::WindowHint::ContextVersion(
            OPENGL_MAJOR_MAC_OS,
            OPENGL_MINOR_MAC_OS,
        ));
        #[cfg(target_os = "macos")]
        glfw_handle.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // All
        glfw_handle.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
    }

    fn init_graphics_api(window_handle: &mut glfw::Window) {
        window_handle.make_current();
        window_handle.set_all_polling(true);
        gl::load_with(|s| window_handle.get_proc_address(s) as *const _);
    }

    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            use WindowEvent as WndEvnt;

            match event {
                WndEvnt::Pos(_, _) => (),
                WndEvnt::Size(_, _) => (),
                WndEvnt::Close => (),
                WndEvnt::Refresh => (),
                WndEvnt::Focus(_) => (),
                WndEvnt::Iconify(_) => (),
                WndEvnt::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the resized window dimensions
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                WndEvnt::MouseButton(_, _, _) => (),
                WndEvnt::CursorPos(_, _) => (),
                WndEvnt::CursorEnter(_) => (),
                WndEvnt::Scroll(_, _) => (),
                WndEvnt::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                WndEvnt::Key(..) => (),
                WndEvnt::Char(_) => (),
                WndEvnt::CharModifiers(_, _) => (),
                WndEvnt::FileDrop(_) => (),
                WndEvnt::Maximize(_) => (),
                WndEvnt::ContentScale(_, _) => (),
            }
        }
    }
}
