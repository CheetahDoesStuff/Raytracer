use nalgebra::Vector3;
use rand::rngs::SmallRng;

use crate::{
    ray::{color::Color, ray::Ray},
    surface::surface::HitRecord,
};

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray, _rng: &mut SmallRng) -> bool {
        return false;
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f32>) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}