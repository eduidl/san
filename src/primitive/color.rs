#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    // Red 0-1
    pub r: f64,
    // Green 0-1
    pub g: f64,
    // Blue 0-1
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<Rgb> for wgpu::Color {
    fn from(c: Rgb) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 1.0,
        }
    }
}
