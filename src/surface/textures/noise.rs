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
    fn sample(&self, _u: &f32, _v: &f32, pos: &Vector3<f32>) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * pos.z + 10.0 * self.noise.turbulence(*pos, 7)).sin())
    }
}