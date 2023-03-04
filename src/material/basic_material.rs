use super::Material;
use crate::{params::LocalParams, Rgba};

#[derive(Debug, Clone)]
pub struct BasicMaterial {
    params: BasicMaterialParams,
}

impl BasicMaterial {
    pub fn new(color: Rgba) -> Self {
        Self {
            params: BasicMaterialParams {
                color: color.into(),
            },
        }
    }
}

impl Material for BasicMaterial {
    fn render_pipeline(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        crate::pipeline::create_render_pipeline_common::<BasicMaterialParams>(
            device,
            format,
            "san::mesh::MeshBasicMaterial",
            wgpu::ShaderSource::Wgsl(include_str!("../shaders/basic_mesh.wgsl").into()),
        )
    }

    fn buffer_bind_group(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroup) {
        self.params.buffer_bind_group(device)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BasicMaterialParams {
    color: [f32; 4],
}

impl LocalParams for BasicMaterialParams {}
