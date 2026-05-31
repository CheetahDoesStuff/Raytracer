use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    aabb::AABB,
    interval::Interval,
    ray::ray::Ray,
    surface::{
        material::Material,
        surface::{HitRecord, Surface},
    }
};

pub struct Quad {
    q: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    normal: Vector3<f32>,
    d: f32,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Vector3<f32>, u: Vector3<f32>, v: Vector3<f32>, mat: Arc<dyn Material>) -> Self {
        let bbox1 = AABB::new_from_vec(q, q + u + v);
        let bbox2 = AABB::new_from_vec(q + u, q + v);
        let bbox = AABB::new_from_boxes(&bbox1, &bbox2);

        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Self {
            q, u, v, w,
            normal, d,
            mat,
            bbox,
        }
    }

    fn is_interior(&self, a: f32, b: f32, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        return true;
    }
}

impl Surface for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() < 1e-8 { return false; }
        
        let t = (self.d - self.normal.dot(&ray.origin())) / denom;
        if !ray_t.contains(t) { return false }

        let intersection = ray.at(t);
        let planar_hitpt_vec = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vec));
        if !self.is_interior(alpha, beta, rec) {
            return false;
        }
        

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(ray, self.normal);
        return true;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}