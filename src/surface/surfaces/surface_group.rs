use std::sync::Arc;

use crate::{
    aabb::AABB, interval::Interval, ray::ray::Ray, surface::surface::{HitRecord, Surface}
};

pub struct SurfaceGroup {
    pub objects: Vec<Arc<dyn Surface>>,
    bbox: AABB,
}

impl SurfaceGroup {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new_empty()
        }
    }

    pub fn with(object: Arc<dyn Surface>) -> Self {
        let mut group = Self::new();
        group.add(object);
        group
    }

    pub fn add(&mut self, object: Arc<dyn Surface>) {
        self.bbox = AABB::new_from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Surface for SurfaceGroup {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    
    fn bounding_box(&self) -> &crate::aabb::AABB {
        todo!()
    }
}
