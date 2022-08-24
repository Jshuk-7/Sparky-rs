mod buffer;
pub mod buffer_types;
mod vertex_array;
mod vertex_attribute;

pub use buffer::*;
pub use vertex_array::*;
pub use vertex_attribute::*;

pub enum Primitive {
    Triangle,
    Line,
    Point,
}

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn set_clear_color(&self, color: &[f32; 3]) {
        unsafe { gl::ClearColor(color[0], color[1], color[2], 1.0) }
    }

    pub fn clear_viewport(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
    }

    pub fn draw(&self, shape: Primitive, first: u32, count: u32) {
        let primitive = match shape {
            Primitive::Triangle => gl::TRIANGLES,
            Primitive::Line => gl::LINES,
            Primitive::Point => gl::POINTS,
        };

        unsafe { gl::DrawArrays(primitive, first as i32, count as i32) }
    }
}
