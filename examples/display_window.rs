use san::{
    primitive::Rgb,
    winit::{event_loop::EventLoop, window::WindowBuilder},
    Scene, WGPURenderer, WGPURendererOption,
};

#[async_std::main]
async fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = WGPURenderer::new(window, WGPURendererOption::default()).await;
    let mut scene = Scene::new();
    scene.set_background(Rgb::new(0.1, 0.2, 0.3));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = renderer.handle_event(&event, &scene);
    });
}
