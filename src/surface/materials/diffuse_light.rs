use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    ray::{color::Color, ray::Ray},
    surface::{material::Material, surface::HitRecord, texture::Texture},
    utils::{random_f32, reflect, refract},
};

pub struct DiffuseLight {
    pub texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &nalgebra::Vector3<f32>) -> Color {
        let emitted = self.texture.sample(&(u as f32), &(v as f32), p);
        Color::new(emitted.x, emitted.y, emitted.z)
    }
}