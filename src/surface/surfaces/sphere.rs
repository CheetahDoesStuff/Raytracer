use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    aabb::AABB,
    interval::Interval,
    ray::ray::Ray,
    surface::{
        material::Material,
        surface::{HitRecord, Surface},
    }, utils::PI,
};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, mat: Arc<dyn Material>) -> Self {
        let radius = radius.max(0.0);

        let rvec = Vector3::new(radius, radius, radius);

        let bbox = AABB::new_from_vec(
            center - rvec,
            center + rvec,
        );

        Self {
            center,
            radius,
            mat,
            bbox,
        }
    }

    pub fn get_sphere_uv(pos: Vector3<f32>) -> (f32, f32) {
        let theta = -pos.y.acos();
        let phi = pos.z.atan2(pos.x) + PI;

        (phi / (2.0*PI), 1.0 - theta / PI)
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
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
        let outward_normal = (rec.p - self.center) / self.radius;
        (rec.u, rec.v) = Sphere::get_sphere_uv(outward_normal);
        rec.set_face_normal(ray, outward_normal);
        rec.mat = self.mat.clone();
        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}