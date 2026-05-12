use nalgebra::Vector3;

use crate::interval::Interval;

struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
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

    pub fn new_from_vec(a: Vector3<f32>, b: Vector3<f32>) {
        
    }
}