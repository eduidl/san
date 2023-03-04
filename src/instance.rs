use cgmath::{Matrix4, Quaternion, Vector3, Zero};

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    pub position: Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: (f32, f32, f32),
}

impl Instance {
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = (scale, scale, scale);
        self
    }

    pub fn to_raw(self) -> InstanceRaw {
        InstanceRaw {
            model: (Matrix4::from_translation(self.position)
                * Matrix4::from(self.rotation)
                * Matrix4::from_nonuniform_scale(self.scale.0, self.scale.1, self.scale.2))
            .into(),
        }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            scale: (1., 1., 1.),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4];

    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as _,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
