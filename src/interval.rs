use crate::utils::INFINITY;

#[derive(Clone, Copy)]
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

    pub fn new_from_intervals(a: &Interval, b: &Interval) -> Interval {
        let min;
        let max;

        if a.min <= b.min {min = a.min} else {min = b.min}
        if a.max >= b.max {max = a.max} else {max = b.max}
        Interval {
            min: min,
            max: max
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

    pub fn clamp(self: &Self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn expand(self: &Self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval {min: self.min - padding, max: self.max + padding}
    }
}

pub const INTERVAL_EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
pub const INTERVAL_WORLD: Interval = Interval::new(-INFINITY, INFINITY);
