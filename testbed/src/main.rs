use sparky::prelude::{algebra::*, graphics::types::*, *};

fn main() {
    log::init();

    let mut window = Window::new(Vec2::new(800.0, 600.0), "Sparky Game Engine");

    let vertices = [
        Vec3::new(-0.5f32, -0.5, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
    ];

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
        0,
        std::ptr::null(),
    );
    pos_attr.enable();

    let mut renderer = Renderer::new();
    renderer.set_clear_color(&Color::White);

    let game = GameLoop::new(&mut window, &mut renderer);

    game.on_update(move |renderer| -> GameResult {
        renderer.clear_viewport();
        renderer.draw(Primitive::Triangle, 0, 3);

        Ok(())
    })
    .unwrap();
}
