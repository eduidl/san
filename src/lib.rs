#[warn(clippy::all)]
pub mod mesh;
pub use mesh::Mesh;

pub mod primitive;

mod renderer;
pub use renderer::{WGPURenderer, WGPURendererOption};

mod scene;
pub use scene::Scene;
// re-export
pub use wgpu;
pub use winit;
