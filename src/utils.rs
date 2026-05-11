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

    Vector3::new(
        random_f32(Some(min), Some(max)),
        random_f32(Some(min), Some(max)),
        random_f32(Some(min), Some(max)),
    )
}

pub fn random_unit_vec() -> Vector3<f32> {
    loop {
        let p = random_vec(Some(-1.0), Some(1.0));
        let lensq = p.norm_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / f32::sqrt(lensq);
        }
    }
}

pub fn random_on_hemisphere(base_vec: Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vec();
    if on_unit_sphere.dot(&base_vec.normalize()) > 0.0 {
        return on_unit_sphere;
    }
    -on_unit_sphere
}

pub fn vec_near_zero(vec: Vector3<f32>) -> bool {
    let s = 1e-8;
    (vec.x < s) && (vec.y < s) && (vec.z < s)
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = -uv.dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt((1.0 - r_out_perp.norm_squared()).abs()) * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = Vector3::new(
            random_f32(Some(-1.0), Some(1.0)),
            random_f32(Some(-1.0), Some(1.0)),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}
