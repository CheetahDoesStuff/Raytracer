use nalgebra::Vector3;

use crate::{
    ray::ray::Ray,
    surface::surface::{HitRecord, Surface},
};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
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

        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;

            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outwards_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outwards_normal);

        true
    }
}
