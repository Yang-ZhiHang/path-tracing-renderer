use std::f64;

#[derive(Clone, Copy, Default)]
pub struct Interval {
    /// The minimal value of a interval.
    pub min: f64,
    /// The maximal value of a interval.
    pub max: f64,
}

impl Interval {
    /// Create a interval which between `a` and `b`.
    pub fn new(a: f64, b: f64) -> Self {
        let min = if a < b { a } else { b };
        let max = if a > b { a } else { b };
        Self { min, max }
    }

    /// Create an infinitely large interval.
    pub const fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    /// Return the length of the interval.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Extend both the left and right sides of the interval outward by `delta`.
    pub fn extend(&mut self, delta: f64) {
        self.min -= delta;
        self.max += delta;
    }

    /// Determine if the interval contains specified value.
    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    /// Return the smallest interval that contains both `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }
}
