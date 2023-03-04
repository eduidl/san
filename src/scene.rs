use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use crate::mesh::{DrawMesh, MeshBase, MeshID};

pub(crate) type SceneID = u16;

static SCENE_COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct Scene {
    id: SceneID,
    device: Arc<wgpu::Device>,
    format: wgpu::TextureFormat,
    background: wgpu::Color,
    meshes: Vec<Option<Box<dyn MeshBase>>>,
    mesh_recycle_ids: Vec<usize>,
}

impl Scene {
    pub fn new(device: Arc<wgpu::Device>, format: wgpu::TextureFormat) -> Self {
        Self {
            id: SCENE_COUNTER.fetch_add(1, Ordering::Relaxed),
            device,
            format,
            background: wgpu::Color::WHITE,
            meshes: Vec::new(),
            mesh_recycle_ids: Vec::new(),
        }
    }

    pub(crate) fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let gpu_data: Vec<_> = self
            .meshes
            .iter()
            .flatten()
            .map(|mesh| (mesh.gpu_data(&self.device, self.format)))
            .collect();

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        for (geo, mat) in gpu_data.iter() {
            render_pass.draw_mesh(geo, mat);
        }
    }

    pub fn set_background<T>(&mut self, background: T)
    where
        T: Into<wgpu::Color>,
    {
        self.background = background.into();
    }

    pub fn meshes_len(&self) -> usize {
        self.meshes.len() - self.mesh_recycle_ids.len()
    }

    pub fn add_mesh<M>(&mut self, mesh: M) -> MeshID<M>
    where
        M: MeshBase + 'static,
    {
        let mesh = Some(Box::new(mesh) as Box<dyn MeshBase>);

        let index = match self.mesh_recycle_ids.pop() {
            Some(index) => {
                debug_assert!(self.meshes[index].is_none());
                self.meshes[index] = mesh;
                index
            }
            None => {
                self.meshes.push(mesh);
                self.meshes.len() - 1
            }
        };

        MeshID::new(self.id, index)
    }

    pub fn remove_mesh<M>(&mut self, mesh_id: MeshID<M>) {
        assert_eq!(self.id, mesh_id.scene_id);

        self.meshes.get_mut(mesh_id.index).unwrap().take().unwrap();
        self.mesh_recycle_ids.push(mesh_id.index);
    }

    pub fn get_mesh_ref<M>(&self, mesh: &MeshID<M>) -> &M
    where
        M: MeshBase + 'static,
    {
        assert_eq!(self.id, mesh.scene_id);

        self.meshes
            .get(mesh.index)
            .unwrap()
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mesh_mut<M>(&mut self, mesh: &MeshID<M>) -> &mut M
    where
        M: MeshBase + 'static,
    {
        assert_eq!(self.id, mesh.scene_id);

        self.meshes
            .get_mut(mesh.index)
            .unwrap()
            .as_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }
}
