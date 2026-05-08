use crate::utils::INFINITY;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn empty() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn size(self: &Self) -> f32 {
        self.max - self.min
    }

    pub fn contains(self: &Self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self: &Self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

pub const INTERVAL_EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
pub const INTERVAL_WORLD: Interval = Interval::new(-INFINITY, INFINITY);
