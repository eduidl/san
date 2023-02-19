use std::any::Any;

use san::{mesh::MeshBase, Scene};

#[derive(Debug)]
struct TestMesh {
    label: &'static str,
}

impl TestMesh {
    fn new(label: &'static str) -> Self {
        Self { label }
    }
}

impl MeshBase for TestMesh {
    fn render(&self, _render_pass: &mut wgpu::RenderPass) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[test]
fn test_scene_add_mesh() {
    let mut scene = Scene::new();

    scene.add_mesh(TestMesh::new("mesh1"));
    scene.add_mesh(TestMesh::new("mesh2"));

    assert_eq!(scene.meshes_len(), 2);
}

#[test]
fn test_scene_remove_mesh() {
    let mut scene = Scene::new();

    let mesh1 = scene.add_mesh(TestMesh::new("mesh1"));
    scene.add_mesh(TestMesh::new("mesh2"));

    assert_eq!(scene.remove_mesh(mesh1).label, "mesh1");
    assert_eq!(scene.meshes_len(), 1);
}

#[test]
fn test_scene_get_mesh_ref() {
    let mut scene = Scene::new();

    let mesh1 = scene.add_mesh(TestMesh::new("mesh1"));
    let mesh2 = scene.add_mesh(TestMesh::new("mesh2"));

    assert_eq!(scene.get_mesh_ref(&mesh1).label, "mesh1");
    assert_eq!(scene.get_mesh_ref(&mesh2).label, "mesh2");
}

#[test]
fn test_scene_get_mesh_mut() {
    let mut scene = Scene::new();

    let mesh1 = scene.add_mesh(TestMesh::new("mesh1"));
    let mesh2 = scene.add_mesh(TestMesh::new("mesh2"));

    assert_eq!(scene.get_mesh_mut(&mesh1).label, "mesh1");
    assert_eq!(scene.get_mesh_mut(&mesh2).label, "mesh2");
}
