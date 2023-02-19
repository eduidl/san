use std::{any::Any, fmt::Debug, marker::PhantomData};

use crate::scene::SceneID;

pub trait MeshBase: Any + Debug {
    fn render(&self, render_pass: &mut wgpu::RenderPass);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct Mesh {}

impl Mesh {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }
}

impl MeshBase for Mesh {
    fn render(&self, _render_pass: &mut wgpu::RenderPass) {
        println!("TODO")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct MeshID<M> {
    pub scene_id: SceneID,
    pub index: usize,
    _phantom: PhantomData<M>,
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
