use std::sync::Arc;

use nalgebra::Vector3;

use crate::{interval::Interval, ray::ray::Ray, surface::material::Material};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vector3::zeros(),
            normal: Vector3::zeros(),
            t: 0.0,
            front_face: false,
            mat: None,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;

        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Surface {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
