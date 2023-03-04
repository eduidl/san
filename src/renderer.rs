use std::{path::PathBuf, sync::Arc};

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::Scene;

#[derive(Debug, Clone)]
pub struct WGPURendererOption {
    pub power_preference: wgpu::PowerPreference,
    pub device_limits: wgpu::Limits,
    pub trace: Option<PathBuf>,
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
            trace: None,
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

    pub fn with_trace(self, trace: PathBuf) -> Self {
        Self {
            trace: Some(trace),
            ..self
        }
    }
}

#[derive(Debug)]
pub struct WGPURenderer {
    window: Window,
    device: Arc<wgpu::Device>,
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
                option.trace.as_deref(),
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
            device: Arc::new(device),
            queue,
            surface,
            surface_desc,
        }
    }

    pub fn create_scene(&self) -> Scene {
        Scene::new(Arc::clone(&self.device), self.surface_desc.format)
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

    pub fn render(&mut self, scene: &Scene) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        scene.render(&view, &mut encoder);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event<()>, scene: &Scene) -> ControlFlow {
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
                match self.render(scene) {
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
