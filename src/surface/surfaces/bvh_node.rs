use std::sync::Arc;

use nalgebra::Vector3;
use rand::Rng;

use crate::{
    aabb::AABB,
    interval::Interval,
    ray::ray::Ray,
    surface::surface::{HitRecord, Surface},
};

pub struct BvhNode {
    left: Arc<dyn Surface>,
    right: Arc<dyn Surface>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(list: &mut Vec<Arc<dyn Surface>>, start: usize, end: usize) -> Self {
        let axis = rand::thread_rng().gen_range(0, 2);
        let comparator;
        if axis == 0 {comparator = }

        let left = list[start].clone();
        let right = list[start].clone();

        let bbox = AABB::new_empty();
        Self { left, right, bbox }
    }

    pub fn new_from_list(list: Vec<Arc<dyn Surface>>) -> Self {
        let mut objects = list;
        let len = objects.len();
        BvhNode::new(&mut objects, 0, len)
    }
}

impl Surface for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t.clone()) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t.clone(), rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );
        hit_left || hit_right
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}