use std::{any::Any, sync::Arc};

use san::{geometry::GeometryGpuData, material::MaterialGpuData, mesh::MeshBase, AsAny, Scene};

#[derive(Debug)]
struct DummyMesh {
    label: &'static str,
}

impl DummyMesh {
    fn new(label: &'static str) -> Self {
        Self { label }
    }
}

impl AsAny for DummyMesh {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MeshBase for DummyMesh {
    fn gpu_data(
        &self,
        _device: &wgpu::Device,
        _format: wgpu::TextureFormat,
    ) -> (Arc<GeometryGpuData>, Arc<MaterialGpuData>) {
        unimplemented!()
    }
}

async fn init_scene() -> Scene {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();

    let (device, _) = adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap();

    // TextureFormat is dummy
    Scene::new(Arc::new(device), wgpu::TextureFormat::Bc1RgbaUnorm)
}

#[async_std::test]
async fn test_scene_add_mesh() {
    let mut scene = init_scene().await;

    scene.add_mesh(DummyMesh::new("mesh1"));
    scene.add_mesh(DummyMesh::new("mesh2"));

    assert_eq!(scene.meshes_len(), 2);
}

#[async_std::test]
async fn test_scene_remove_mesh() {
    let mut scene = init_scene().await;

    let mesh1 = scene.add_mesh(DummyMesh::new("mesh1"));
    scene.add_mesh(DummyMesh::new("mesh2"));

    scene.remove_mesh(mesh1);
    assert_eq!(scene.meshes_len(), 1);
}

#[async_std::test]
async fn test_scene_get_mesh_ref() {
    let mut scene = init_scene().await;

    let mesh1 = scene.add_mesh(DummyMesh::new("mesh1"));
    let mesh2 = scene.add_mesh(DummyMesh::new("mesh2"));

    assert_eq!(scene.get_mesh_ref(&mesh1).label, "mesh1");
    assert_eq!(scene.get_mesh_ref(&mesh2).label, "mesh2");
}

#[async_std::test]
async fn test_scene_get_mesh_mut() {
    let mut scene = init_scene().await;

    let mesh1 = scene.add_mesh(DummyMesh::new("mesh1"));
    let mesh2 = scene.add_mesh(DummyMesh::new("mesh2"));

    assert_eq!(scene.get_mesh_mut(&mesh1).label, "mesh1");
    assert_eq!(scene.get_mesh_mut(&mesh2).label, "mesh2");
}
