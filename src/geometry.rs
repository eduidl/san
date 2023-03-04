use crate::{
    gpu::{ToGpu, ToGpuBuffer},
    Vertex, VertexIndex,
};

#[derive(Debug)]
pub struct Geometry {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Option<Vec<VertexIndex>>,
}

impl Geometry {
    pub fn plane(w: f32, h: f32) -> Self {
        let x = w * 0.5;
        let y = h * 0.5;

        #[rustfmt::skip]
        let vertices = vec![
            Vertex::new([-x, -y, 0.], [0., 0., 1.]),  // top left
            Vertex::new([ x, -y, 0.], [0., 0., 1.]),  // top right
            Vertex::new([ x,  y, 0.], [0., 0., 1.]),  // bottom right
            Vertex::new([-x,  y, 0.], [0., 0., 1.]),  // bottom left
        ];

        #[rustfmt::skip]
        let indices = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        Self {
            vertices,
            indices: Some(indices),
        }
    }
}

impl ToGpu for Geometry {
    type Target = GeometryGpuData;

    fn to_gpu(&self, device: &wgpu::Device, _format: wgpu::TextureFormat) -> Self::Target {
        Self::Target {
            vertices: self.vertices.as_slice().to_gpu_buffer(device),
            vertices_len: self.vertices.len() as u32,
            indices: self
                .indices
                .as_ref()
                .map(|i| i.as_slice().to_gpu_buffer(device)),
            indices_len: self.indices.as_ref().map(|i| i.len()).unwrap_or_default() as u32,
        }
    }
}

pub struct GeometryGpuData {
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) vertices_len: u32,
    pub(crate) indices: Option<wgpu::Buffer>,
    pub(crate) indices_len: u32,
}
