use san::{
    winit::{
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    WGPURenderer, WGPURendererOption,
};

#[async_std::main]
async fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = WGPURenderer::new(window, WGPURendererOption::default()).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = renderer.handle_event(&event);
    });
}
