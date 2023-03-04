use std::sync::{Arc, RwLock};

use wgpu::util::DeviceExt;

use crate::{InstanceRaw, Vertex, VertexIndex};

pub trait ToGpu {
    type Target;

    fn to_gpu(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> Self::Target;
}

pub struct GpuCached<T>
where
    T: ToGpu,
{
    base: T,
    gpu_data: RwLock<Option<Arc<T::Target>>>,
}

impl<T> GpuCached<T>
where
    T: ToGpu,
{
    pub const fn new(base: T) -> Self {
        Self {
            base,
            gpu_data: RwLock::new(None),
        }
    }

    pub fn to_gpu(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> Arc<T::Target> {
        {
            let gpu_data = self.gpu_data.read().unwrap();
            if let Some(v) = gpu_data.as_ref() {
                return Arc::clone(v);
            }
        }

        let mut gpu_data_mut = self.gpu_data.write().unwrap();
        match gpu_data_mut.as_ref() {
            Some(v) => Arc::clone(v),
            None => {
                let data = Arc::new(self.base.to_gpu(device, format));
                *gpu_data_mut = Some(Arc::clone(&data));
                data
            }
        }
    }
}

pub trait ToGpuBuffer {
    fn to_gpu_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer;
}

impl ToGpuBuffer for &[Vertex] {
    fn to_gpu_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

impl ToGpuBuffer for &[VertexIndex] {
    fn to_gpu_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(self),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}

impl ToGpuBuffer for &[InstanceRaw] {
    fn to_gpu_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(self),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}
