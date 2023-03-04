use san::{
    color::Rgb,
    geometry::Geometry,
    material::BasicMaterial,
    winit::{event_loop::EventLoop, window::WindowBuilder},
    Mesh, Rgba, WGPURenderer, WGPURendererOption,
};

#[async_std::main]
async fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = WGPURenderer::new(window, WGPURendererOption::default()).await;
    let mut scene = renderer.create_scene();
    scene.set_background(Rgb::new(0.1, 0.2, 0.3));

    scene.add_mesh(Mesh::new(
        Geometry::plane(1.0, 1.0),
        BasicMaterial::new(Rgba::new(0.8, 0., 0., 0.5)),
    ));

    scene.add_mesh(Mesh::new(
        Geometry::plane(1.5, 0.5),
        BasicMaterial::new(Rgba::new(0., 0.8, 0., 0.5)),
    ));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = renderer.handle_event(&event, &scene);
    });
}
