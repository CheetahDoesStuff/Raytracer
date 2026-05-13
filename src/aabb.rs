use nalgebra::Vector3;
use crate::{interval::Interval, ray::ray::Ray};

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
        AABB {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn new_from_vec(a: Vector3<f32>, b: Vector3<f32>) -> AABB{
        let x;
        let y;
        let z;

        if  a.x <= b.x {x = Interval::new(a.x, b.x)} else {x = Interval::new(b.x, a.x)}
        if  a.y <= b.y {y = Interval::new(a.y, b.y)} else {y = Interval::new(b.y, a.y)}
        if  a.z <= b.z {z = Interval::new(a.z, b.z)} else {z = Interval::new(b.z, a.z)}

        AABB {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn new_from_boxes(a: &AABB, b: &AABB) -> AABB {
        AABB::new_from_intervals(
            Interval::new_from_intervals(&a.x, &b.x),
            Interval::new_from_intervals(&a.y, &b.y),
            Interval::new_from_intervals(&a.z, &b.z)
        )
    }

    pub fn get_axis_interval(self: &Self, n: i8) -> &Interval {
        if n == 1 {return &self.y}
        if n== 2 {return &self.z}
        return &self.x
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
}