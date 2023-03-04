use crate::gpu::ToGpu;

mod basic_material;
pub use basic_material::BasicMaterial;

pub trait Material {
    fn render_pipeline(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline;

    fn buffer_bind_group(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroup);
}

impl<M> ToGpu for M
where
    M: Material,
{
    type Target = MaterialGpuData;

    fn to_gpu(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> Self::Target {
        let pipeline = self.render_pipeline(device, format);
        let (buffer, bind_group) = self.buffer_bind_group(device);

        Self::Target::new(pipeline, buffer, bind_group)
    }
}

#[derive(Debug)]
pub struct MaterialGpuData {
    pub(crate) pipeline: wgpu::RenderPipeline,
    _buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl MaterialGpuData {
    fn new(
        pipeline: wgpu::RenderPipeline,
        _buffer: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
    ) -> Self {
        Self {
            pipeline,
            _buffer,
            bind_group,
        }
    }
}
