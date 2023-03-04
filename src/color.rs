#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    // Red 0-1
    pub r: f32,
    // Green 0-1
    pub g: f32,
    // Blue 0-1
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl From<Rgb> for wgpu::Color {
    fn from(c: Rgb) -> Self {
        Self {
            r: c.r.into(),
            g: c.g.into(),
            b: c.b.into(),
            a: 1.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    // Red 0-1
    pub r: f32,
    // Green 0-1
    pub g: f32,
    // Blue 0-1
    pub b: f32,
    // Alpha 0-1
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Rgba> for wgpu::Color {
    fn from(c: Rgba) -> Self {
        Self {
            r: c.r.into(),
            g: c.g.into(),
            b: c.b.into(),
            a: c.a.into(),
        }
    }
}

impl From<Rgba> for [f32; 4] {
    fn from(c: Rgba) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}
