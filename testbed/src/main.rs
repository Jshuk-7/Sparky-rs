use sparky::prelude::*;

use std::{mem, ptr};

fn main() {
    let mut window = window::Window::new(800, 600, "Sparky Game Engine");

    let vertices = [-0.5f32, -0.5, 0.0, 0.0, 0.5, 0.0, 0.5, -0.5, 0.0];

    let vao = vertex_array::Vao::new();
    vao.bind();

    let buffer = buffer::Buffer::new(buffer_types::BufferType::Vertex, buffer_types::FillType::Single);

    buffer.store_data(&vertices);

    let pos_attr = vertex_attribute::VertexAttribute::new(
        0,
        3,
        buffer_types::BufferDataType::Float32,
        false,
        (3 * mem::size_of::<f32>() as i32).try_into().unwrap(),
        ptr::null(),
    );

    pos_attr.enable();

    let renderer = graphics::Renderer::new();
    renderer.set_clear_color(&[0.2, 0.2, 0.8]);

    while !window.closed() {
        renderer.clear_viewport();
        //renderer.draw(graphics::Primitive::Triangle, 0, 3);
        window.update();
    }
}
