use nalgebra::Vector3;

use crate::{noise::Perlin, ray::color::Color, surface::texture::Texture};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {NoiseTexture {noise: Perlin::new(), scale}}
}

impl Texture for NoiseTexture {
    fn sample(&self, _u: f32, _v: f32, pos: Vector3<f32>) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * pos))
    }
}