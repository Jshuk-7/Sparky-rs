use super::types::{Color, Primitive};

/// Handles graphics and rendering
pub struct Renderer;

impl Renderer {
    /// Creates a new renderer
    ///
    /// ## Example
    /// ```
    /// let renderer = Renderer::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Set the color the screen will clear to on each frame before drawing
    ///
    /// ## Usage
    /// Must be called before clearing the screen if called in a loop</br>
    /// If the clear color is not set it will default to black
    ///
    /// ## Arguments
    /// - 'color' - The color the screen will be cleared to
    ///
    /// ## Example
    /// ```
    /// renderer.set_clear_color(Color::White);
    /// ```
    pub fn set_clear_color(&self, color: &Color) {
        let new_color = color.color();
        unsafe { gl::ClearColor(new_color[0], new_color[1], new_color[2], 1.0) }
    }

    /// Clear the screen to the current clear color
    ///
    /// ## Usage
    /// Must be called once per frame and after setting the clear color
    ///
    /// ## Example
    /// ```
    /// renderer.clear_viewport();
    /// ```
    pub fn clear_viewport(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
    }

    /// Uses the currently bound vao and draws to the viewport
    ///
    /// ## Usage
    /// A vao must be bound, a vbo must be bound to the same vao,</br>
    /// and any shaders must also be bound before making a draw call
    ///
    /// ## Arguments
    /// - 'shape' - Specifies what kind of primitive to draw
    /// - 'first' - Specifies the first vertex attribute to start at
    /// - 'count' - Specifies the number of indices to be rasterized
    ///
    /// ## Example
    /// ```
    /// renderer.draw(Primitive::Triangle, 0, 3);
    /// ```
    pub fn draw(&self, shape: Primitive, first: u32, count: u32) {
        let mode = shape.prim_type();

        unsafe { gl::DrawArrays(mode, first as i32, count as i32) }
    }
}
