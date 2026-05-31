use nalgebra::Vector3;

use crate::ray::color::Color;

pub trait Texture: Send + Sync {
    fn sample(&self, u: &f32, v: &f32, pos: &Vector3<f32>) -> Color;
}