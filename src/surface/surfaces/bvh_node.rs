use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB, interval::Interval, ray::ray::Ray, surface::surface::{HitRecord, Surface}
};

pub struct BvhNode {
    left: Arc<dyn Surface + Send + Sync>,
    right: Arc<dyn Surface + Send + Sync>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Surface + Send + Sync>>,
        start: usize,
        end: usize,
    ) -> Arc<dyn Surface + Send + Sync> {
        let mut bbox = AABB::new_empty();
        for object in start..end {
            bbox = AABB::new_from_boxes(&bbox, objects[object].bounding_box());
        }
        let axis = bbox.get_longest_axis();

        let span = end - start;
        objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));

        let (left, right): (
            Arc<dyn Surface + Send + Sync>,
            Arc<dyn Surface + Send + Sync>,
        ) = if span == 1 {
            let obj = objects[start].clone();
            (obj.clone(), obj)
        } else if span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            let mid = start + span / 2;

            let left = BvhNode::new(objects, start, mid);
            let right = BvhNode::new(objects, mid, end);

            (left, right)
        };

        Arc::new(Self { left, right, bbox }) as Arc<dyn Surface + Send + Sync>
    }

    pub fn new_from_list(list: Vec<Arc<dyn Surface + Send + Sync>>) -> Arc<dyn Surface + Send + Sync> {
        let mut objects = list;
        let len = objects.len();
        Self::new(&mut objects, 0, len)
    }

    fn box_compare(
        a: &Arc<dyn Surface + Send + Sync>,
        b: &Arc<dyn Surface + Send + Sync>,
        axis: i8,
    ) -> Ordering {
        let a_box = a.bounding_box();
        let b_box = b.bounding_box();

        let a_i = a_box.get_axis_interval(axis);
        let b_i = b_box.get_axis_interval(axis);

        a_i.min
            .partial_cmp(&b_i.min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Surface for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(ray, ray_t, rec);
        let right_t = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(ray, right_t, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}