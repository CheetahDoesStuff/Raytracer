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

pub fn random_unit_vec() -> Vector3<f32> {
    loop {
        let p = random_vec(Some(-1.0), Some(1.0));
        let lensq = p.norm_squared();
        if 1e-160 < lensq && lensq <= 1.0 { return p / f32::sqrt(lensq) }
    }
}

pub fn random_on_hemisphere(base_vec: Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vec();
    if on_unit_sphere.dot(&base_vec.normalize()) > 0.0 { return on_unit_sphere }
    -on_unit_sphere
}