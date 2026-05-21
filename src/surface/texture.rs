use crate::ray::color::Color;

pub trait Texture: Send + Sync {
    fn sample(&self, u: f32, v: f32) -> Color;
}