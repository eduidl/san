pub trait MeshBase {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}

#[derive(Debug)]
pub struct Mesh {}

impl MeshBase for Mesh {
    fn render(&self, _render_pass: &mut wgpu::RenderPass) {
        println!("TODO")
    }
}
