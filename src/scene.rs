use std::sync::atomic::{AtomicU16, Ordering};

use crate::mesh::{MeshBase, MeshID};

pub(crate) type SceneID = u16;

static SCENE_COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct Scene {
    id: SceneID,
    background: wgpu::Color,
    meshes: Vec<Option<Box<dyn MeshBase>>>,
    mesh_recycle_ids: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
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

        for mesh in self.meshes.iter().flatten() {
            mesh.render(&mut render_pass);
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

    pub fn add_mesh<T, M>(&mut self, mesh: T) -> MeshID<M>
    where
        T: Into<Box<M>>,
        M: MeshBase,
    {
        let index = match self.mesh_recycle_ids.pop() {
            Some(index) => {
                debug_assert!(self.meshes[index].is_none());
                self.meshes[index] = Some(mesh.into());
                index
            }
            None => {
                self.meshes.push(Some(mesh.into()));
                self.meshes.len() - 1
            }
        };

        MeshID::new(self.id, index)
    }

    pub fn remove_mesh<M: MeshBase>(&mut self, mesh: MeshID<M>) -> Box<M> {
        assert_eq!(self.id, mesh.scene_id);

        let box_ = self.meshes.get_mut(mesh.index).unwrap().take().unwrap();
        self.mesh_recycle_ids.push(mesh.index);

        debug_assert!(box_.as_any().is::<M>());

        unsafe {
            let raw = Box::into_raw(box_);
            Box::from_raw(raw as *mut M)
        }
    }

    pub fn get_mesh_ref<M: MeshBase>(&self, mesh: &MeshID<M>) -> &M {
        assert_eq!(self.id, mesh.scene_id);

        self.meshes
            .get(mesh.index)
            .unwrap()
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<M>()
            .unwrap()
    }

    pub fn get_mesh_mut<M: MeshBase>(&mut self, mesh: &MeshID<M>) -> &mut M {
        assert_eq!(self.id, mesh.scene_id);

        self.meshes
            .get_mut(mesh.index)
            .unwrap()
            .as_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<M>()
            .unwrap()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            id: SCENE_COUNTER.fetch_add(1, Ordering::Relaxed),
            background: wgpu::Color::WHITE,
            meshes: Vec::new(),
            mesh_recycle_ids: Vec::new(),
        }
    }
}
