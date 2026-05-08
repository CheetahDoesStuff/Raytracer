use core::f32;
use nalgebra::Vector3;
use rand::Rng;
use rand::thread_rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_f32(min: Option<f32>, max: Option<f32>) -> f32 {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1.0);

    thread_rng().gen_range(min, max)
}

pub fn random_vec(min: Option<f32>, max: Option<f32>) -> Vector3<f32> {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1.0);

    Vector3::new(random_f32(Some(min), Some(max)), random_f32(Some(min), Some(max)), random_f32(Some(min), Some(max)))
}