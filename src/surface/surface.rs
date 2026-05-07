use nalgebra::Vector3;

use crate::ray::ray::Ray;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
}
impl HitRecord {
    pub fn new(p: Vector3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        HitRecord { p, normal, t }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Surface {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool;
}
