use std::path::Path;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

#[derive(Debug, Clone)]
pub struct WGPURendererOption {
    pub power_preference: wgpu::PowerPreference,
    pub device_limits: wgpu::Limits,
    pub trace: bool,
}

impl Default for WGPURendererOption {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::default(),
            device_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            trace: false,
        }
    }
}

impl WGPURendererOption {
    pub fn power_preference(self, power_preference: wgpu::PowerPreference) -> Self {
        Self {
            power_preference,
            ..self
        }
    }

    pub fn device_limits(self, device_limits: wgpu::Limits) -> Self {
        Self {
            device_limits,
            ..self
        }
    }

    pub fn trace(self, trace: bool) -> Self {
        Self { trace, ..self }
    }
}

#[derive(Debug)]
pub struct WGPURenderer {
    window: Window,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_desc: wgpu::SurfaceConfiguration,
}

impl WGPURenderer {
    pub async fn new(window: Window, option: WGPURendererOption) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: option.power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: option.device_limits,
                },
                if option.trace {
                    Some(Path::new("trace"))
                } else {
                    None
                },
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);

        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_desc);

        Self {
            window,
            device,
            queue,
            surface,
            surface_desc,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_desc.width = width;
            self.surface_desc.height = height;
            self.surface.configure(&self.device, &self.surface_desc);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event<()>) -> ControlFlow {
        match event {
            Event::WindowEvent { event, window_id } if window_id == &self.window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => return ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.resize(new_inner_size.width, new_inner_size.height);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == &self.window.id() => {
                match self.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        self.resize(self.surface_desc.width, self.surface_desc.height);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => return ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        }

        ControlFlow::Poll
    }
}
