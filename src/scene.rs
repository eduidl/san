#[derive(Debug)]
pub struct Scene {
    pub(crate) background: wgpu::Color,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_background<T>(&mut self, background: T)
    where
        T: Into<wgpu::Color>,
    {
        self.background = background.into();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            background: wgpu::Color::WHITE,
        }
    }
}
