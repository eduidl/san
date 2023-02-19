#[derive(Debug)]
pub struct Scene {
    background: wgpu::Color,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.background),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }

    pub fn set_background<T>(&mut self, background: T)
    where
        T: Into<wgpu::Color>,
    {
        self.background = background.into();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            background: wgpu::Color::WHITE,
        }
    }
}
