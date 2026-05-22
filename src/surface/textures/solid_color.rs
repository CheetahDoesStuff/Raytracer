use crate::{ray::color::Color, surface::texture::Texture};

pub struct SolidColorTexture {
    col: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        SolidColorTexture { col: color }
    }
}

impl Texture for SolidColorTexture {
    fn sample(&self, _u: f32, _v: f32) -> Color {
        self.col
    }
}