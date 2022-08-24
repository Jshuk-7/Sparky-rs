use sparky::prelude::{buffer_types::*, math::*, *};

use std::{mem::size_of, ops::Add, ptr};

fn main() {
    let mut window = Window::new(Vec2::new(800.0, 600.0), "Sparky Game Engine");

    let vertices = [
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
    ];

    let vao = Vao::new();
    vao.bind();

    let vbo = Buffer::new(BufferType::Vertex, FillType::Single);
    vbo.store_data(&vertices);

    let pos_attr = VertexAttribute::new(
        0,
        3,
        BufferDataType::Float32,
        false,
        (3 * size_of::<Vec3<f32>>() as i32).try_into().unwrap(),
        ptr::null(),
    );
    pos_attr.enable();

    let renderer = Renderer::new();
    renderer.set_clear_color(&[0.2, 0.2, 0.8]);

    while !window.closed() {
        renderer.clear_viewport();
        //renderer.draw(Primitive::Triangle, 0, 3);
        window.update();
    }
}
