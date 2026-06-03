use nalgebra::Vector3;
use crate::{interval::Interval, ray::ray::Ray};

#[derive(Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn new_empty() -> AABB {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty()
        }
    }

    pub fn new_from_intervals(x: Interval, y: Interval, z: Interval) -> AABB {
        let mut bbox = AABB { x, y, z };
        bbox.pad_to_minimums();
        bbox
    }

    pub fn new_from_vec(a: Vector3<f32>, b: Vector3<f32>) -> AABB {
        let x =
            if a.x <= b.x { Interval::new(a.x, b.x) }
            else { Interval::new(b.x, a.x) };

        let y =
            if a.y <= b.y { Interval::new(a.y, b.y) }
            else { Interval::new(b.y, a.y) };

        let z =
            if a.z <= b.z { Interval::new(a.z, b.z) }
            else { Interval::new(b.z, a.z) };

        let mut bbox = AABB { x, y, z };
        bbox.pad_to_minimums();
        bbox
    }

    pub fn new_from_boxes(a: &AABB, b: &AABB) -> AABB {
        AABB::new_from_intervals(
            Interval::new_from_intervals(&a.x, &b.x),
            Interval::new_from_intervals(&a.y, &b.y),
            Interval::new_from_intervals(&a.z, &b.z)
        )
    }

    fn pad_to_minimums(&mut self) {
        let delta: f32 = 0.0001;

        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }

        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }

        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn get_axis_interval(&self, n: i8) -> &Interval {
        if n == 1 { return &self.y; }
        if n == 2 { return &self.z; }
        &self.x
    }

    pub fn get_longest_axis(&self) -> i8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 }
            else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 }
            else { 2 }
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.get_axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }

                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }

                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn offset(&self, x: f32, y: f32, z: f32) -> AABB {
        AABB::new_from_intervals(
            Interval::new(self.x.min + x, self.x.max + x),
            Interval::new(self.y.min + y, self.y.max + y),
            Interval::new(self.z.min + z, self.z.max + z),
        )
    }
}