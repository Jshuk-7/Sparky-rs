use sparky::prelude::{algebra::*, graphics::types::*, *};

use std::{mem, ptr};

fn main() {
    logger::init();

    let mut window = Window::new(Vec2::new(800.0, 600.0), "Sparky Game Engine");

    let vertices = [
        Vec3::new(-0.5f32, -0.5, 0.0), // pos 1
        Vec3::new(1.0, 0.0, 0.0),      // color 1
        Vec3::new(0.0, 0.5, 0.0),      // pos 2
        Vec3::new(0.0, 1.0, 0.0),      // color 2
        Vec3::new(0.5, -0.5, 0.0),     // pos 3
        Vec3::new(0.0, 0.0, 1.0),      // color 3
    ];

    //let indices = [];

    let vao = Vao::new();
    vao.bind();

    let vbo = Buffer::new(BufferType::Vertex, BufferUsage::Single);
    vbo.bind();
    vbo.submit_data(&vertices);

    let pos_attr = VertexAttribute::new(
        0,
        3,
        BufferDataType::Float32,
        false,
        mem::size_of::<Vec3>() * 2,
        ptr::null(),
    );
    pos_attr.enable();

    let color_attr = VertexAttribute::new(
        1,
        3,
        BufferDataType::Float32,
        false,
        mem::size_of::<Vec3>() * 2,
        mem::size_of::<Vec3>() as CVoidPtr,
    );
    color_attr.enable();

    let shader = Shader::new("shaders/vs.glsl", "shaders/fs.glsl");
    shader.enable();

    let mut renderer = Renderer::new();
    renderer.set_clear_color(&Color::LightBlue);

    let game = GameLoop::new(&mut window, &mut renderer);

    game.on_update(move |renderer| -> GameResult {
        renderer.clear_viewport();
        renderer.draw(Primitive::Triangle, 0, 3);

        Ok(())
    })
    .unwrap();
}
