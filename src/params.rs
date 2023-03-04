use std::fmt::Debug;

use cgmath::{Matrix4, SquareMatrix};
use once_cell::sync::OnceCell;
use wgpu::util::DeviceExt;

static GLOBAL_PARAMS_LAYOUT: OnceCell<wgpu::BindGroupLayout> = OnceCell::new();

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalParams {
    view_proj: [[f32; 4]; 4],
}

impl GlobalParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn desc(device: &wgpu::Device) -> &'static wgpu::BindGroupLayout {
        GLOBAL_PARAMS_LAYOUT.get_or_init(|| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Global Params Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
        })
    }

    pub(crate) fn buffer_bind_group(
        &self,
        device: &wgpu::Device,
    ) -> (wgpu::Buffer, wgpu::BindGroup) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global Params Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Local Params Bind Group"),
            layout: Self::desc(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        (buffer, bind_group)
    }
}

impl Default for GlobalParams {
    fn default() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }
}

static LOCAL_PARAMS_LAYOUT: OnceCell<wgpu::BindGroupLayout> = OnceCell::new();

pub trait LocalParams: Debug + Clone + Copy + bytemuck::Pod {
    fn desc(device: &wgpu::Device) -> &'static wgpu::BindGroupLayout {
        LOCAL_PARAMS_LAYOUT.get_or_init(|| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Local Params Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
        })
    }

    fn buffer_bind_group(self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroup) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Local Params Buffer"),
            contents: bytemuck::cast_slice(&[self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Local Params Bind Group"),
            layout: Self::desc(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        (buffer, bind_group)
    }
}
