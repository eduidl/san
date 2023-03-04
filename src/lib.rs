#[warn(clippy::all)]
pub mod camera;

pub mod color;
pub use color::{Rgb, Rgba};

mod common;
pub use common::AsAny;

pub mod geometry;

pub(crate) mod gpu;

mod instance;
pub use instance::{Instance, InstanceRaw};

pub mod material;

pub mod mesh;
pub use mesh::{Mesh, MeshBase};

mod params;

mod pipeline;

mod renderer;
pub use renderer::{WGPURenderer, WGPURendererOption};

mod scene;
pub use scene::Scene;

mod vertex;
pub use vertex::{Vertex, VertexIndex};
// re-export
pub use wgpu;
pub use winit;
