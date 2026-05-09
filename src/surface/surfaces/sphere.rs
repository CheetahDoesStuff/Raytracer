use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    interval::Interval,
    ray::ray::Ray,
    surface::{
        material::Material,
        surface::{HitRecord, Surface},
    },
};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        mat: Arc<dyn Material>,
    ) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Surface for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center - ray.origin();

        let a = ray.direction().norm_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal =
            (rec.p - self.center) / self.radius;

        rec.set_face_normal(ray, outward_normal);

        rec.mat = Some(self.mat.clone());

        true
    }
}