use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    common::AsAny,
    geometry::{Geometry, GeometryGpuData},
    gpu::GpuCached,
    material::{Material, MaterialGpuData},
    scene::SceneID,
};

pub trait MeshBase: AsAny {
    fn gpu_data(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> (Arc<GeometryGpuData>, Arc<MaterialGpuData>);
}

pub struct Mesh<M>
where
    M: Material,
{
    geometry: GpuCached<Geometry>,
    material: GpuCached<M>,
}

impl<M> Mesh<M>
where
    M: Material,
{
    pub fn new(geometry: Geometry, material: M) -> Self
    where
        M: Material,
    {
        Self {
            geometry: GpuCached::new(geometry),
            material: GpuCached::new(material),
        }
    }
}

impl<M> AsAny for Mesh<M>
where
    M: Material + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<M> MeshBase for Mesh<M>
where
    M: Material + 'static,
{
    fn gpu_data(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> (Arc<GeometryGpuData>, Arc<MaterialGpuData>) {
        (
            self.geometry.to_gpu(device, format),
            self.material.to_gpu(device, format),
        )
    }
}

pub struct MeshID<M> {
    pub(crate) scene_id: SceneID,
    pub(crate) index: usize,
    pub(crate) _phantom: PhantomData<M>,
}

impl<M> MeshID<M> {
    pub(crate) fn new(scene_id: SceneID, index: usize) -> Self {
        Self {
            scene_id,
            index,
            _phantom: Default::default(),
        }
    }
}

pub trait DrawMesh<'b> {
    fn draw_mesh(&mut self, geometry: &'b GeometryGpuData, material: &'b MaterialGpuData);
}

impl<'a, 'b> DrawMesh<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, geometry: &'b GeometryGpuData, material: &'b MaterialGpuData) {
        self.set_pipeline(&material.pipeline);

        self.set_vertex_buffer(0, geometry.vertices.slice(..));
        if let Some(ref indices) = geometry.indices {
            self.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        }

        self.set_bind_group(0, &material.bind_group, &[]);

        if geometry.indices.is_some() {
            self.draw_indexed(0..geometry.indices_len, 0, 0..1);
        } else {
            self.draw(0..geometry.vertices_len, 0..1);
        }
    }
}
